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
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::FGraph;
use crate::Inline;
use crate::Lookup;
use crate::PrefixUtil;
use crate::Static;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn binary(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inOperator1: Absyn::Operator, mut inProp1: DAE::Properties, mut inExp1: Arc<DAE::Exp>, mut inProp2: DAE::Properties, mut inExp2: Arc<DAE::Exp>, mut AbExp: Arc<Absyn::Exp>, mut AbExp1: Arc<Absyn::Exp>, mut AbExp2: Arc<Absyn::Exp>, mut inImpl: bool, mut inPre: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outCache, outExp, outProp) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inOperator1.clone(), inProp1.clone(), inExp1.clone(), inProp2.clone(), inExp2.clone())) {
        (_, _, _, props1 @ DAE::Properties::PROP_TUPLE { .. }, _, DAE::Properties::PROP { .. }, _) if (!(Config::acceptMetaModelicaGrammar()?)) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut type1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            let mut prop: DAE::Properties;
            let ref __pa1 @ DAE::PROP { type_: ref __pa0, constFlag: _ } = (Types::propTupleFirstProp(props1.clone())?) else { bail!("pattern mismatch") };
            type1 = __pa0.clone();
            prop = __pa1.clone();
            exp = Arc::new(DAE::Exp::TSUB { exp: inExp1.clone(), ix: 1, ty: type1.clone() });
            (cache, exp, prop) = binary(inCache.clone(), inEnv.clone(), inOperator1.clone(), prop.clone(), exp.clone(), inProp2.clone(), inExp2.clone(), AbExp.clone(), AbExp1.clone(), AbExp2.clone(), inImpl.clone(), inPre.clone(), inInfo.clone())?;
            (cache.clone(), exp.clone(), prop.clone())
        },
        (_, _, _, DAE::Properties::PROP { .. }, _, props2 @ DAE::Properties::PROP_TUPLE { .. }, _) if (!(Config::acceptMetaModelicaGrammar()?)) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut type2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            let mut prop: DAE::Properties;
            let ref __pa1 @ DAE::PROP { type_: ref __pa0, constFlag: _ } = (Types::propTupleFirstProp(props2.clone())?) else { bail!("pattern mismatch") };
            type2 = __pa0.clone();
            prop = __pa1.clone();
            exp = Arc::new(DAE::Exp::TSUB { exp: inExp2.clone(), ix: 1, ty: type2.clone() });
            (cache, exp, prop) = binary(inCache.clone(), inEnv.clone(), inOperator1.clone(), inProp1.clone(), inExp1.clone(), prop.clone(), exp.clone(), AbExp.clone(), AbExp1.clone(), AbExp2.clone(), inImpl.clone(), inPre.clone(), inInfo.clone())?;
            (cache.clone(), exp.clone(), prop.clone())
        },
        (cache, env, aboper, DAE::Properties::PROP { type_: type1, constFlag: const1 }, exp1, DAE::Properties::PROP { type_: type2, constFlag: const2 }, exp2) => {
            let mut opList: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut otype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut exp: Arc<DAE::Exp>;
            let mut r#const: DAE::Const = DAE::Const::C_CONST;
            let mut oper: DAE::Operator;
            let mut prop: DAE::Properties;
            let mut functionTree; // TODO: local with unresolved type
            let mut didInline: bool = false;
            let mut cache = (*cache).clone();
            let mut type1 = (*type1).clone();
            let mut exp1 = (*exp1).clone();
            let mut type2 = (*type2).clone();
            let mut exp2 = (*exp2).clone();
            if Types::isRecord(Types::arrayElementType(type1.clone())) || Types::isRecord(Types::arrayElementType(type2.clone())) {
                (cache, exp, _, otype) = binaryUserdef(cache.clone(), env.clone(), aboper.clone(), inExp1.clone(), inExp2.clone(), type1.clone(), type2.clone(), inImpl.clone(), inPre.clone(), inInfo.clone())?;
                functionTree = FCore::getFunctionTree(cache.clone());
                (exp, _) = ExpressionSimplify::simplify1(exp.clone())?;
                (exp, _, didInline, _) = Inline::inlineExp(exp.clone(), (Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE, openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE]), DAE::emptyElementSource().clone())?;
                (exp, _) = ExpressionSimplify::condsimplify(didInline.clone(), exp.clone())?;
                r#const = Types::constAnd(const1.clone(), const2.clone());
                prop = DAE::Properties::PROP { type_: otype.clone(), constFlag: r#const.clone() };
            } else {
                if Types::isBoxedType(type1.clone()) && Types::isBoxedType(type2.clone()) {
                    (exp1, type1) = Types::matchType(exp1.clone(), type1.clone(), Types::unboxedType(type1.clone())?, true)?;
                    (exp2, type2) = Types::matchType(exp2.clone(), type2.clone(), Types::unboxedType(type2.clone())?, true)?;
                }
                (opList, type1, exp1, type2, exp2, _, _, _, _) = operatorsBinary(aboper.clone(), type1.clone(), exp1.clone(), type2.clone(), exp2.clone())?;
                let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(deoverload(opList.clone(), list![(exp1.clone(), type1.clone()), (exp2.clone(), type2.clone())], AbExp.clone(), inPre.clone(), inInfo.clone())?) {
                    (__pa0, Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                oper = __pa0.clone();
                exp1 = __pa1.clone();
                exp2 = __pa2.clone();
                otype = __pa3.clone();
                r#const = Types::constAnd(const1.clone(), const2.clone());
                exp = replaceOperatorWithFcall(AbExp.clone(), exp1.clone(), oper.clone(), Some(exp2.clone()), r#const.clone())?;
                (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
                prop = DAE::Properties::PROP { type_: otype.clone(), constFlag: r#const.clone() };
                warnUnsafeRelations(inEnv.clone(), AbExp.clone(), r#const.clone(), type1.clone(), type2.clone(), exp1.clone(), exp2.clone(), oper.clone(), inPre.clone(), inInfo.clone())?;
            }
            (cache.clone(), exp.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProp))
}

pub fn unary(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inOperator1: Absyn::Operator, mut inProp1: DAE::Properties, mut inExp1: Arc<DAE::Exp>, mut AbExp: Arc<Absyn::Exp>, mut AbExp1: Arc<Absyn::Exp>, mut inImpl: bool, mut inPre: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outCache, outExp, outProp) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inOperator1.clone(), inProp1.clone(), inExp1.clone(), AbExp.clone(), AbExp1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, DAE::Properties::PROP_TUPLE { .. }, exp1, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut type1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let false = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let ref __pa1 @ DAE::PROP { type_: ref __pa0, constFlag: _ } = (Types::propTupleFirstProp(inProp1.clone())?) else { bail!("pattern mismatch") };
                    type1 = __pa0.clone();
                    prop = __pa1.clone();
                    exp = Arc::new(DAE::Exp::TSUB { exp: exp1.clone(), ix: 1, ty: type1.clone() });
                    (cache, exp, prop) = unary(inCache.clone(), inEnv.clone(), inOperator1.clone(), prop.clone(), exp.clone(), AbExp.clone(), AbExp1.clone(), inImpl.clone(), inPre.clone(), inInfo.clone())?;
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, aboper, DAE::Properties::PROP { type_: type1, constFlag: r#const }, exp1, _, _) => {
                    let mut opList: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
                    let mut otype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut oper: DAE::Operator;
                    let mut prop: DAE::Properties;
                    let mut exp1 = (*exp1).clone();
                    let false = (Types::isRecord(Types::arrayElementType(type1.clone()))) else { bail!("pattern mismatch") };
                    opList = operatorsUnary(aboper.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(deoverload(opList.clone(), list![(exp1.clone(), type1.clone())], AbExp.clone(), inPre.clone(), inInfo.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    oper = __pa0.clone();
                    exp1 = __pa1.clone();
                    otype = __pa2.clone();
                    exp = replaceOperatorWithFcall(AbExp.clone(), exp1.clone(), oper.clone(), None, r#const.clone())?;
                    prop = DAE::Properties::PROP { type_: otype.clone(), constFlag: r#const.clone() };
                    Ok((inCache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, aboper, DAE::Properties::PROP { type_: type1, constFlag: _ }, _, _, absexp1) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut operNames: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut path: Arc<Absyn::Path>;
                    let mut operatorEnv: FCore::Graph;
                    let mut recordEnv: FCore::Graph;
                    let mut operatorCl: Arc<SCode::Element>;
                    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    path = getRecordPath(type1.clone())?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    (cache, _, recordEnv) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Dump::opSymbolCompact(aboper.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone();
                    path = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (str1.clone()).clone() }))?;
                    (cache, operatorCl, operatorEnv) = Lookup::lookupClass(cache.clone(), recordEnv.clone(), path.clone(), None)?;
                    let true = (SCodeUtil::isOperator(operatorCl.clone())) else { bail!("pattern mismatch") };
                    operNames = AbsynToSCode::getListofQualOperatorFuncsfromOperator(operatorCl.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsListInEnv(cache.clone(), operatorEnv.clone(), operNames.clone(), inInfo.clone(), metamodelica::nil())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    types = __pa1.clone();
                    let (__pa2, Some((__pa3, __pa4))) = (Static::elabCallArgs3(cache.clone(), env.clone(), types.clone(), path.clone(), list![absexp1.clone()], metamodelica::nil(), metamodelica::nil(), inImpl.clone(), inPre.clone(), inInfo.clone())?) else { bail!("pattern mismatch") };
                    cache = __pa2.clone();
                    exp = __pa3.clone();
                    prop = __pa4.clone();
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProp))
}

pub fn string(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp1: Arc<Absyn::Exp>, mut inImpl: bool, mut inDoVect: bool, mut inPre: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outCache, outExp, outProp) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExp1.clone())) {
        (cache, env, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args: Deref @ metamodelica::List::Cons { head: exp1, tail: restargs } }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "String", subscripts: _ }, .. }) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut path: Arc<Absyn::Path>;
            let mut operNames: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut recordEnv: FCore::Graph;
            let mut operatorEnv: FCore::Graph;
            let mut operatorCl: Arc<SCode::Element>;
            let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut prop: DAE::Properties;
            let mut type1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut daeExp: Arc<DAE::Exp>;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), env.clone(), exp1.clone(), inImpl.clone(), inDoVect.clone(), inPre.clone(), inInfo.clone())?) {
                (__pa0, _, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            type1 = __pa1.clone();
            path = getRecordPath(type1.clone())?;
            path = AbsynUtil::makeFullyQualified(path.clone());
            (cache, _, recordEnv) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
            str1 = (literal!("'String'")).clone();
            path = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (str1.clone()).clone() }))?;
            (cache, operatorCl, operatorEnv) = Lookup::lookupClass(cache.clone(), recordEnv.clone(), path.clone(), None)?;
            let true = (SCodeUtil::isOperator(operatorCl.clone())) else { bail!("pattern mismatch") };
            operNames = AbsynToSCode::getListofQualOperatorFuncsfromOperator(operatorCl.clone())?;
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsListInEnv(cache.clone(), operatorEnv.clone(), operNames.clone(), inInfo.clone(), metamodelica::nil())?) {
                (__pa2, __pa3 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            types = __pa3.clone();
            let (__pa4, Some((__pa5, __pa6))) = (Static::elabCallArgs3(cache.clone(), env.clone(), types.clone(), path.clone(), cons(exp1.clone(), restargs.clone()), nargs.clone(), metamodelica::nil(), inImpl.clone(), inPre.clone(), inInfo.clone())?) else { bail!("pattern mismatch") };
            cache = __pa4.clone();
            daeExp = __pa5.clone();
            prop = __pa6.clone();
            (cache.clone(), daeExp.clone(), prop.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProp))
}

pub fn elabArglist(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inArgs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Type>)>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outArgs, outTypes) = (::match_deref::match_deref! { match &((inTypes.clone(), inArgs.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: pt, tail: pts }, Deref @ metamodelica::List::Cons { head: (arg, atype), tail: args }) => {
            let mut arg_1: Arc<DAE::Exp>;
            let mut atype_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut args_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut atypes_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            (arg_1, atype_1) = Types::matchType(arg.clone(), atype.clone(), pt.clone(), false)?;
            (args_1, atypes_1) = elabArglist(pts.clone(), args.clone())?;
            (cons(arg_1.clone(), args_1.clone()), cons(atype_1.clone(), atypes_1.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outArgs, outTypes))
}

pub fn initCache() -> () {
    crate::Globals::operatorOverloadingCache.with(|__root| *__root.borrow_mut() = (Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY), Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY)));
    ()
}

/* We have these as constants instead of function calls as done previously
 * because it takes a long time to generate these types over and over again.
 * The types are a bit hard to read, but they are simply 1 through 9-dimensional
 * arrays of the basic types. */
thread_local! { static __intarrtypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })]; }
pub fn intarrtypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __intarrtypes_TLS.with(|__t| __t.clone()) }

thread_local! { static __realarrtypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })]; }
pub fn realarrtypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __realarrtypes_TLS.with(|__t| __t.clone()) }

thread_local! { static __boolarrtypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })]; }
pub fn boolarrtypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __boolarrtypes_TLS.with(|__t| __t.clone()) }

thread_local! { static __stringarrtypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })]; }
pub fn stringarrtypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __stringarrtypes_TLS.with(|__t| __t.clone()) }

/* Simply a list of 9 of that basic type; used to match with the array types */
thread_local! { static __inttypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()]; }
pub fn inttypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __inttypes_TLS.with(|__t| __t.clone()) }

thread_local! { static __realtypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()]; }
pub fn realtypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __realtypes_TLS.with(|__t| __t.clone()) }

thread_local! { static __stringtypes_TLS: Arc<metamodelica::List<Arc<DAE::Type>>> = list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()]; }
pub fn stringtypes() -> Arc<metamodelica::List<Arc<DAE::Type>>> { __stringtypes_TLS.with(|__t| __t.clone()) }

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deoverloadBinaryUserdefNoConstructor(mut inTypeList: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<DAE::Exp>, mut lhsType: Arc<DAE::Type>, mut rhsType: Arc<DAE::Type>, mut inAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>> {
    let mut outExps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
    outExps = 'mc: {
        let __mc_input = (inTypeList.clone(), inLhs.clone(), inRhs.clone(), lhsType.clone(), rhsType.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: ty1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: ty2, .. }, tail: restArgs } }, functionAttributes: attr, funcResultType: ty, path }, tail: types }, _, _, _, _, acc) => {
                    let mut daeExp: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut tpl: (Arc<DAE::Exp>, Option<Arc<DAE::Type>>);
                    let mut acc = (*acc).clone();
                    (lhs, _) = Types::matchType(inLhs.clone(), lhsType.clone(), ty1.clone(), false)?;
                    (rhs, _) = Types::matchType(inRhs.clone(), rhsType.clone(), ty2.clone(), false)?;
                    daeExp = makeCallFillRestDefaults(path.clone(), list![lhs.clone(), rhs.clone()], restArgs.clone(), Types::makeCallAttr(ty.clone(), attr.clone()));
                    tpl = (daeExp.clone(), overloadFoldType(ty1.clone(), ty2.clone(), ty.clone())?);
                    acc = deoverloadBinaryUserdefNoConstructor(types.clone(), inLhs.clone(), inRhs.clone(), lhsType.clone(), rhsType.clone(), cons(tpl.clone(), acc.clone()))?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: types }, _, _, _, _, _) => {
                    let mut acc: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
                    acc = deoverloadBinaryUserdefNoConstructor(types.clone(), inLhs.clone(), inRhs.clone(), lhsType.clone(), rhsType.clone(), inAcc.clone())?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExps)
}

fn overloadFoldType(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut inType3: Arc<DAE::Type>) -> Result<Option<Arc<DAE::Type>>> {
    let mut optType: Option<Arc<DAE::Type>> = None;
    optType = if (Types::equivtypesOrRecordSubtypeOf(inType1.clone(), inType2.clone())? && Types::equivtypesOrRecordSubtypeOf(inType1.clone(), inType3.clone())?) {Some(inType1.clone())} else {None};
    Ok(optType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deoverloadBinaryUserdefNoConstructorListLhs(mut types: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inLhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inRhs: Arc<DAE::Exp>, mut rhsType: Arc<DAE::Type>, mut inAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>> {
    let mut outExps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
    outExps = (::match_deref::match_deref! { match &((types.clone(), inLhs.clone(), inRhs.clone(), rhsType.clone(), inAcc.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: lhs, tail: rest }, _, _, acc) => {
            let mut acc = (*acc).clone();
            acc = deoverloadBinaryUserdefNoConstructor(types.clone(), lhs.clone(), inRhs.clone(), Expression::r#typeof(lhs.clone())?, rhsType.clone(), acc.clone())?;
            acc = deoverloadBinaryUserdefNoConstructorListLhs(types.clone(), rest.clone(), inRhs.clone(), rhsType.clone(), acc.clone())?;
            acc.clone()
        },
        _ => {
            inAcc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExps)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deoverloadBinaryUserdefNoConstructorListRhs(mut types: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut lhsType: Arc<DAE::Type>, mut inAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>> {
    let mut outExps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
    outExps = (::match_deref::match_deref! { match &((types.clone(), inLhs.clone(), inRhs.clone(), lhsType.clone(), inAcc.clone())) {
        (_, _, Deref @ metamodelica::List::Cons { head: rhs, tail: rest }, _, acc) => {
            let mut acc = (*acc).clone();
            acc = deoverloadBinaryUserdefNoConstructor(types.clone(), inLhs.clone(), rhs.clone(), lhsType.clone(), Expression::r#typeof(rhs.clone())?, acc.clone())?;
            acc = deoverloadBinaryUserdefNoConstructorListRhs(types.clone(), inLhs.clone(), rest.clone(), lhsType.clone(), acc.clone())?;
            acc.clone()
        },
        _ => {
            inAcc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExps)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deoverloadUnaryUserdefNoConstructor(mut inTypeList: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExps = 'mc: {
        let __mc_input = (inTypeList.clone(), inExp.clone(), inType.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: ty1, .. }, tail: restArgs }, functionAttributes: attr, funcResultType: ty, path }, tail: types }, _, _, acc) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut daeExp: Arc<DAE::Exp>;
                    let mut acc = (*acc).clone();
                    (exp, _) = Types::matchType(inExp.clone(), inType.clone(), ty1.clone(), false)?;
                    daeExp = makeCallFillRestDefaults(path.clone(), list![exp.clone()], restArgs.clone(), Types::makeCallAttr(ty.clone(), attr.clone()));
                    acc = deoverloadUnaryUserdefNoConstructor(types.clone(), inExp.clone(), ty.clone(), cons(daeExp.clone(), acc.clone()))?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: types }, _, _, _) => {
                    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    acc = deoverloadUnaryUserdefNoConstructor(types.clone(), inExp.clone(), inType.clone(), inAcc.clone())?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExps)
}

fn binaryUserdef(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inOper: Absyn::Operator, mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, Option<Arc<DAE::Type>>, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut foldType: Option<Arc<DAE::Type>> = None;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCache, outExp, foldType, outType) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inOper.clone(), inExp1.clone(), inExp2.clone(), inType1.clone(), inType2.clone())) {
        (cache, env, op, exp1, exp2, type1, type2) => {
            let mut bool1: bool = false;
            let mut bool2: bool = false;
            let mut opStr: ArcStr = arcstr::literal!("");
            let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut types1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut types2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut daeExp: Arc<DAE::Exp>;
            let mut exps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            bool1 = Types::arrayType(type1.clone());
            bool2 = Types::arrayType(type2.clone());
            if bool1.clone() && bool2.clone() && AbsynUtil::opIsElementWise(op.clone()) {
                types = metamodelica::nil();
            } else {
                opStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Dump::opSymbolCompact(op.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone();
                (cache, types1) = getOperatorFuncsOrEmpty(cache.clone(), env.clone(), list![type1.clone()], (opStr.clone()).clone(), info.clone(), metamodelica::nil())?;
                (cache, types2) = getOperatorFuncsOrEmpty(cache.clone(), env.clone(), list![type2.clone()], (opStr.clone()).clone(), info.clone(), metamodelica::nil())?;
                types = List::union(types1.clone(), types2.clone());
                types = List::select1(types.clone(), (std::sync::Arc::new(fnptr!(isOperatorBinaryFunctionOrWarn, Arc<DAE::Type>, SourceInfo)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, SourceInfo) -> Result<bool> + 'static>), info.clone());
            }
            exps = deoverloadBinaryUserdefNoConstructor(types.clone(), exp1.clone(), exp2.clone(), type1.clone(), type2.clone(), metamodelica::nil())?;
            (cache, exps) = binaryCastConstructor(cache.clone(), env.clone(), inExp1.clone(), inExp2.clone(), inType1.clone(), inType2.clone(), exps.clone(), types.clone(), info.clone())?;
            (cache, exps) = binaryUserdefArray(cache.clone(), env.clone(), exps.clone(), bool1.clone() || bool2.clone(), inOper.clone(), inExp1.clone(), inExp2.clone(), inType1.clone(), inType2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps.clone()) {
                Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: Deref @ metamodelica::List::Nil } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            daeExp = __pa0.clone();
            foldType = __pa1.clone();
            (cache.clone(), daeExp.clone(), foldType.clone(), Expression::r#typeof(daeExp.clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, foldType, outType))
}

fn binaryUserdefArray(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inExps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>, mut isArray: bool, mut inOper: Absyn::Operator, mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut exps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
    (cache, exps) = (::match_deref::match_deref! { match &((env.clone(), inExps.clone(), isArray.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, _) => {
            (inCache.clone(), inExps.clone())
        },
        (_, Deref @ metamodelica::List::Nil, true) => {
            let mut isRelation: bool = false;
            let mut isVector1: bool = false;
            let mut isVector2: bool = false;
            let mut isScalar1: bool = false;
            let mut isScalar2: bool = false;
            let mut isMatrix1: bool = false;
            let mut isMatrix2: bool = false;
            isRelation = listMember(inOper.clone(), list![openmodelica_ast::Absyn::Operator::LESS, openmodelica_ast::Absyn::Operator::LESSEQ, openmodelica_ast::Absyn::Operator::GREATER, openmodelica_ast::Absyn::Operator::GREATEREQ, openmodelica_ast::Absyn::Operator::EQUAL, openmodelica_ast::Absyn::Operator::NEQUAL]);
            Error::assertionOrAddSourceMessage(!(isRelation.clone()), Error::COMPILER_ERROR.clone(), list![(literal!("Not supporting overloading of relation array operations")).clone()], info.clone())?;
            isScalar1 = !(Types::arrayType(inType1.clone()));
            isScalar2 = !(Types::arrayType(inType2.clone()));
            isVector1 = Types::isArray1D(inType1.clone());
            isVector2 = Types::isArray1D(inType2.clone());
            isMatrix1 = Types::isArray2D(inType1.clone());
            isMatrix2 = Types::isArray2D(inType2.clone());
            (cache, exps) = binaryUserdefArray2(inCache.clone(), env.clone(), isScalar1.clone(), isVector1.clone(), isMatrix1.clone(), isScalar2.clone(), isVector2.clone(), isMatrix2.clone(), inOper.clone(), inExp1.clone(), inExp2.clone(), inType1.clone(), inType2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache.clone(), exps.clone())
        },
        _ => {
            errorMultipleValid(List::map(inExps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _))), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, exps))
}

fn binaryUserdefArray2(mut inCache: FCore::Cache, mut env: FCore::Graph, mut isScalar1: bool, mut isVector1: bool, mut isMatrix1: bool, mut isScalar2: bool, mut isVector2: bool, mut isMatrix2: bool, mut inOper: Absyn::Operator, mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut exps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
    (cache, exps) = (match (inCache.clone(), env.clone(), isScalar1.clone(), isVector1.clone(), isMatrix1.clone(), isScalar2.clone(), isVector2.clone(), isMatrix2.clone(), inOper.clone()) {
        (mut cache, _, false, _, _, true, _, _, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::Exp>;
            let mut newType1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut foldName: ArcStr = arcstr::literal!("");
            let mut resultName: ArcStr = arcstr::literal!("");
            let mut iterName: ArcStr = arcstr::literal!("");
            let mut op: Absyn::Operator = Absyn::Operator::ADD;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim1 = __pa0.clone();
            newType1 = __pa1.clone();
            op = Util::assoc(inOper.clone(), list![(openmodelica_ast::Absyn::Operator::ADD_EW, openmodelica_ast::Absyn::Operator::ADD_EW), (openmodelica_ast::Absyn::Operator::SUB_EW, openmodelica_ast::Absyn::Operator::SUB_EW), (openmodelica_ast::Absyn::Operator::MUL, openmodelica_ast::Absyn::Operator::MUL_EW), (openmodelica_ast::Absyn::Operator::MUL_EW, openmodelica_ast::Absyn::Operator::MUL_EW), (openmodelica_ast::Absyn::Operator::DIV, openmodelica_ast::Absyn::Operator::DIV_EW), (openmodelica_ast::Absyn::Operator::DIV_EW, openmodelica_ast::Absyn::Operator::DIV_EW), (openmodelica_ast::Absyn::Operator::POW_EW, openmodelica_ast::Absyn::Operator::POW_EW)])?;
            iterName = (Util::getTempVariableIndex()).clone();
            foldName = (Util::getTempVariableIndex()).clone();
            resultName = (Util::getTempVariableIndex()).clone();
            cr = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName.clone()).clone(), identType: newType1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1.clone() });
            (cache, exp, _, resType) = binaryUserdef(cache.clone(), env.clone(), op.clone(), cr.clone(), inExp2.clone(), newType1.clone(), inType2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            resType = Types::liftArray(resType.clone(), dim1.clone());
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: resType.clone(), defaultValue: None, foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: None }), expr: exp.clone(), iterators: cons(Arc::new(DAE::ReductionIterator { id: (iterName.clone()).clone(), exp: inExp1.clone(), guardExp: None, ty: newType1.clone() }), metamodelica::nil()) });
            (cache.clone(), list![(exp.clone(), None)])
        },
        (mut cache, _, true, _, _, false, _, _, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::Exp>;
            let mut newType2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut foldName: ArcStr = arcstr::literal!("");
            let mut resultName: ArcStr = arcstr::literal!("");
            let mut iterName: ArcStr = arcstr::literal!("");
            let mut op: Absyn::Operator = Absyn::Operator::ADD;
            op = Util::assoc(inOper.clone(), list![(openmodelica_ast::Absyn::Operator::ADD_EW, openmodelica_ast::Absyn::Operator::ADD_EW), (openmodelica_ast::Absyn::Operator::SUB_EW, openmodelica_ast::Absyn::Operator::SUB_EW), (openmodelica_ast::Absyn::Operator::MUL, openmodelica_ast::Absyn::Operator::MUL_EW), (openmodelica_ast::Absyn::Operator::MUL_EW, openmodelica_ast::Absyn::Operator::MUL_EW), (openmodelica_ast::Absyn::Operator::DIV_EW, openmodelica_ast::Absyn::Operator::DIV_EW), (openmodelica_ast::Absyn::Operator::POW_EW, openmodelica_ast::Absyn::Operator::POW_EW)])?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim2 = __pa0.clone();
            newType2 = __pa1.clone();
            iterName = (Util::getTempVariableIndex()).clone();
            foldName = (Util::getTempVariableIndex()).clone();
            resultName = (Util::getTempVariableIndex()).clone();
            cr = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName.clone()).clone(), identType: newType2.clone(), subscriptLst: metamodelica::nil() }), ty: newType2.clone() });
            (cache, exp, _, resType) = binaryUserdef(cache.clone(), env.clone(), op.clone(), inExp1.clone(), cr.clone(), inType1.clone(), newType2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            resType = Arc::new(DAE::Type::T_ARRAY { ty: resType.clone(), dims: list![dim2.clone()] });
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: resType.clone(), defaultValue: None, foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: None }), expr: exp.clone(), iterators: cons(Arc::new(DAE::ReductionIterator { id: (iterName.clone()).clone(), exp: inExp2.clone(), guardExp: None, ty: newType2.clone() }), metamodelica::nil()) });
            (cache.clone(), list![(exp.clone(), None)])
        },
        (_, _, _, true, _, _, true, _, Absyn::Operator::MUL) => {
            bail!("fail")
        },
        (_, _, _, true, _, _, _, true, Absyn::Operator::MUL) => {
            bail!("fail")
        },
        (mut cache, _, _, _, true, _, true, _, Absyn::Operator::MUL) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::Exp>;
            let mut cr2: Arc<DAE::Exp>;
            let mut cr3: Arc<DAE::Exp>;
            let mut cr4: Arc<DAE::Exp>;
            let mut foldExp: Arc<DAE::Exp>;
            let mut newType1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newType2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newType1_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim1_1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim1_2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut iter: Arc<DAE::ReductionIterator>;
            let mut iter1: Arc<DAE::ReductionIterator>;
            let mut iter2: Arc<DAE::ReductionIterator>;
            let mut foldName1: ArcStr = arcstr::literal!("");
            let mut resultName1: ArcStr = arcstr::literal!("");
            let mut foldName2: ArcStr = arcstr::literal!("");
            let mut resultName2: ArcStr = arcstr::literal!("");
            let mut iterName: ArcStr = arcstr::literal!("");
            let mut iterName1: ArcStr = arcstr::literal!("");
            let mut iterName2: ArcStr = arcstr::literal!("");
            let mut zeroConstructor: Option<Arc<Values::Value>> = None;
            let mut zeroTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim1_1 = __pa0.clone();
            newType1_1 = __pa1.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(newType1_1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, ty: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim1_2 = __pa3.clone();
            newType1 = __pa4.clone();
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil }, ty: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim2 = __pa6.clone();
            newType2 = __pa7.clone();
            let true = (Expression::dimensionsEqual(dim1_2.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
            foldName1 = (Util::getTempVariableIndex()).clone();
            resultName1 = (Util::getTempVariableIndex()).clone();
            foldName2 = (Util::getTempVariableIndex()).clone();
            resultName2 = (Util::getTempVariableIndex()).clone();
            iterName = (Util::getTempVariableIndex()).clone();
            iterName1 = (Util::getTempVariableIndex()).clone();
            iterName2 = (Util::getTempVariableIndex()).clone();
            cr = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName.clone()).clone(), identType: newType1_1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1.clone() });
            cr1 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName1.clone()).clone(), identType: newType1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1.clone() });
            cr2 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName2.clone()).clone(), identType: newType2.clone(), subscriptLst: metamodelica::nil() }), ty: newType2.clone() });
            cr3 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (foldName1.clone()).clone(), identType: newType1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1.clone() });
            cr4 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (resultName1.clone()).clone(), identType: newType2.clone(), subscriptLst: metamodelica::nil() }), ty: newType2.clone() });
            let (__pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(binaryUserdef(cache.clone(), env.clone(), openmodelica_ast::Absyn::Operator::ADD, cr1.clone(), cr2.clone(), newType1.clone(), newType2.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa9, __pa10, Some(__pa11), __pa12) => (__pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa9.clone();
            exp = __pa10.clone();
            ty = __pa11.clone();
            resType = __pa12.clone();
            (cache, foldExp, _, _) = binaryUserdef(cache.clone(), env.clone(), openmodelica_ast::Absyn::Operator::ADD, cr3.clone(), cr4.clone(), ty.clone(), ty.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache, zeroTypes) = getOperatorFuncsOrEmpty(cache.clone(), env.clone(), list![ty.clone()], (literal!("'0'")).clone(), info.clone(), metamodelica::nil())?;
            (cache, zeroConstructor) = getZeroConstructor(cache.clone(), env.clone(), List::filterMap(zeroTypes.clone(), (std::sync::Arc::new(getZeroConstructorExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>)), r#impl.clone(), info.clone())?;
            resType = Arc::new(DAE::Type::T_ARRAY { ty: resType.clone(), dims: list![dim1_1.clone()] });
            iter = Arc::new(DAE::ReductionIterator { id: (iterName1.clone()).clone(), exp: cr.clone(), guardExp: None, ty: newType1.clone() });
            iter1 = Arc::new(DAE::ReductionIterator { id: (iterName.clone()).clone(), exp: inExp1.clone(), guardExp: None, ty: newType1.clone() });
            iter2 = Arc::new(DAE::ReductionIterator { id: (iterName2.clone()).clone(), exp: inExp2.clone(), guardExp: None, ty: newType2.clone() });
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sum")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::THREAD, exprType: resType.clone(), defaultValue: zeroConstructor.clone(), foldName: (foldName1.clone()).clone(), resultName: (resultName1.clone()).clone(), foldExp: Some(foldExp.clone()) }), expr: exp.clone(), iterators: cons(iter.clone(), cons(iter2.clone(), metamodelica::nil())) });
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: resType.clone(), defaultValue: None, foldName: (foldName2.clone()).clone(), resultName: (resultName2.clone()).clone(), foldExp: None }), expr: exp.clone(), iterators: cons(iter1.clone(), metamodelica::nil()) });
            (cache.clone(), list![(exp.clone(), None)])
        },
        (mut cache, _, _, _, true, _, _, true, Absyn::Operator::MUL) => {
            let mut mulExp: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::Exp>;
            let mut cr2: Arc<DAE::Exp>;
            let mut cr3: Arc<DAE::Exp>;
            let mut cr4: Arc<DAE::Exp>;
            let mut cr5: Arc<DAE::Exp>;
            let mut cr6: Arc<DAE::Exp>;
            let mut foldExp: Arc<DAE::Exp>;
            let mut transposed: Arc<DAE::Exp>;
            let mut newType1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newType2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newType1_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newType2_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dim1_1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim1_2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim2_1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim2_2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut iter1: Arc<DAE::ReductionIterator>;
            let mut iter2: Arc<DAE::ReductionIterator>;
            let mut iter3: Arc<DAE::ReductionIterator>;
            let mut iter4: Arc<DAE::ReductionIterator>;
            let mut foldName: ArcStr = arcstr::literal!("");
            let mut resultName: ArcStr = arcstr::literal!("");
            let mut foldName1: ArcStr = arcstr::literal!("");
            let mut resultName1: ArcStr = arcstr::literal!("");
            let mut foldName2: ArcStr = arcstr::literal!("");
            let mut resultName2: ArcStr = arcstr::literal!("");
            let mut iterName1: ArcStr = arcstr::literal!("");
            let mut iterName2: ArcStr = arcstr::literal!("");
            let mut iterName3: ArcStr = arcstr::literal!("");
            let mut iterName4: ArcStr = arcstr::literal!("");
            let mut zeroConstructor: Option<Arc<Values::Value>> = None;
            let mut zeroTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim1_1 = __pa0.clone();
            newType1_1 = __pa1.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(newType1_1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, ty: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim1_2 = __pa3.clone();
            newType1 = __pa4.clone();
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil }, ty: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim2_1 = __pa6.clone();
            newType2_1 = __pa7.clone();
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(newType2_1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Nil }, ty: __pa10 } => (__pa9.clone(), __pa10.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim2_2 = __pa9.clone();
            newType2 = __pa10.clone();
            let true = (Expression::dimensionsEqual(dim1_2.clone(), dim2_1.clone())?) else { bail!("pattern mismatch") };
            transposed = Expression::makePureBuiltinCall((literal!("transpose")).clone(), list![inExp2.clone()], Types::liftArray(Types::liftArray(newType2.clone(), dim2_1.clone()), dim2_2.clone()));
            iterName1 = (Util::getTempVariableIndex()).clone();
            iterName2 = (Util::getTempVariableIndex()).clone();
            iterName3 = (Util::getTempVariableIndex()).clone();
            iterName4 = (Util::getTempVariableIndex()).clone();
            foldName1 = (Util::getTempVariableIndex()).clone();
            resultName1 = (Util::getTempVariableIndex()).clone();
            foldName2 = (Util::getTempVariableIndex()).clone();
            resultName2 = (Util::getTempVariableIndex()).clone();
            foldName = (Util::getTempVariableIndex()).clone();
            resultName = (Util::getTempVariableIndex()).clone();
            cr1 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName1.clone()).clone(), identType: newType1_1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1_1.clone() });
            cr2 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName2.clone()).clone(), identType: newType2_1.clone(), subscriptLst: metamodelica::nil() }), ty: newType2_1.clone() });
            cr3 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName3.clone()).clone(), identType: newType1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1.clone() });
            cr4 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName4.clone()).clone(), identType: newType2.clone(), subscriptLst: metamodelica::nil() }), ty: newType2.clone() });
            (cache, mulExp, _, ty) = binaryUserdef(cache.clone(), env.clone(), openmodelica_ast::Absyn::Operator::MUL, cr3.clone(), cr4.clone(), newType1.clone(), newType2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            cr5 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (foldName.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty.clone() });
            cr6 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (resultName.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty.clone() });
            let (__pa12, __pa13, __pa14) = ::match_deref::match_deref! { match &(binaryUserdef(cache.clone(), env.clone(), openmodelica_ast::Absyn::Operator::ADD, cr5.clone(), cr6.clone(), ty.clone(), ty.clone(), r#impl.clone(), pre.clone(), info.clone())?) {
                (__pa12, __pa13, Some(__pa14), _) => (__pa12.clone(), __pa13.clone(), __pa14.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa12.clone();
            foldExp = __pa13.clone();
            ty = __pa14.clone();
            (cache, zeroTypes) = getOperatorFuncsOrEmpty(cache.clone(), env.clone(), list![ty.clone()], (literal!("'0'")).clone(), info.clone(), metamodelica::nil())?;
            (cache, zeroConstructor) = getZeroConstructor(cache.clone(), env.clone(), List::filterMap(zeroTypes.clone(), (std::sync::Arc::new(getZeroConstructorExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>)), r#impl.clone(), info.clone())?;
            iter1 = Arc::new(DAE::ReductionIterator { id: (iterName1.clone()).clone(), exp: inExp1.clone(), guardExp: None, ty: newType1_1.clone() });
            iter2 = Arc::new(DAE::ReductionIterator { id: (iterName2.clone()).clone(), exp: transposed.clone(), guardExp: None, ty: newType2_1.clone() });
            iter3 = Arc::new(DAE::ReductionIterator { id: (iterName3.clone()).clone(), exp: cr1.clone(), guardExp: None, ty: newType1_1.clone() });
            iter4 = Arc::new(DAE::ReductionIterator { id: (iterName4.clone()).clone(), exp: cr2.clone(), guardExp: None, ty: newType2_1.clone() });
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sum")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::THREAD, exprType: ty.clone(), defaultValue: zeroConstructor.clone(), foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: Some(foldExp.clone()) }), expr: mulExp.clone(), iterators: cons(iter3.clone(), cons(iter4.clone(), metamodelica::nil())) });
            ty = Types::liftArray(ty.clone(), dim2_2.clone());
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: None, foldName: (foldName2.clone()).clone(), resultName: (resultName2.clone()).clone(), foldExp: None }), expr: exp.clone(), iterators: cons(iter2.clone(), metamodelica::nil()) });
            ty = Types::liftArray(ty.clone(), dim1_1.clone());
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: None, foldName: (foldName1.clone()).clone(), resultName: (resultName1.clone()).clone(), foldExp: None }), expr: exp.clone(), iterators: cons(iter1.clone(), metamodelica::nil()) });
            (cache.clone(), list![(exp.clone(), None)])
        },
        (mut cache, _, false, _, _, false, _, _, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::Exp>;
            let mut cr2: Arc<DAE::Exp>;
            let mut newType1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut newType2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dim1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dim2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut iter1: Arc<DAE::ReductionIterator>;
            let mut iter2: Arc<DAE::ReductionIterator>;
            let mut foldName: ArcStr = arcstr::literal!("");
            let mut resultName: ArcStr = arcstr::literal!("");
            let mut iterName1: ArcStr = arcstr::literal!("");
            let mut iterName2: ArcStr = arcstr::literal!("");
            let mut op: Absyn::Operator = Absyn::Operator::ADD;
            op = Util::assoc(inOper.clone(), list![(openmodelica_ast::Absyn::Operator::ADD, openmodelica_ast::Absyn::Operator::ADD_EW), (openmodelica_ast::Absyn::Operator::ADD_EW, openmodelica_ast::Absyn::Operator::ADD_EW), (openmodelica_ast::Absyn::Operator::SUB, openmodelica_ast::Absyn::Operator::SUB_EW), (openmodelica_ast::Absyn::Operator::SUB_EW, openmodelica_ast::Absyn::Operator::SUB_EW), (openmodelica_ast::Absyn::Operator::MUL_EW, openmodelica_ast::Absyn::Operator::MUL_EW), (openmodelica_ast::Absyn::Operator::DIV_EW, openmodelica_ast::Absyn::Operator::DIV_EW), (openmodelica_ast::Absyn::Operator::POW_EW, openmodelica_ast::Absyn::Operator::POW_EW), (openmodelica_ast::Absyn::Operator::AND, openmodelica_ast::Absyn::Operator::AND), (openmodelica_ast::Absyn::Operator::OR, openmodelica_ast::Absyn::Operator::OR)])?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim1 = __pa0.clone();
            newType1 = __pa1.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, ty: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim2 = __pa3.clone();
            newType2 = __pa4.clone();
            let true = (Expression::dimensionsEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
            foldName = (Util::getTempVariableIndex()).clone();
            resultName = (Util::getTempVariableIndex()).clone();
            iterName1 = (Util::getTempVariableIndex()).clone();
            iterName2 = (Util::getTempVariableIndex()).clone();
            cr1 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName1.clone()).clone(), identType: newType1.clone(), subscriptLst: metamodelica::nil() }), ty: newType1.clone() });
            cr2 = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iterName2.clone()).clone(), identType: newType2.clone(), subscriptLst: metamodelica::nil() }), ty: newType2.clone() });
            (cache, exp, _, resType) = binaryUserdef(cache.clone(), env.clone(), op.clone(), cr1.clone(), cr2.clone(), newType1.clone(), newType2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            resType = Arc::new(DAE::Type::T_ARRAY { ty: resType.clone(), dims: list![dim2.clone()] });
            iter1 = Arc::new(DAE::ReductionIterator { id: (iterName1.clone()).clone(), exp: inExp1.clone(), guardExp: None, ty: newType1.clone() });
            iter2 = Arc::new(DAE::ReductionIterator { id: (iterName2.clone()).clone(), exp: inExp2.clone(), guardExp: None, ty: newType2.clone() });
            exp = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::THREAD, exprType: resType.clone(), defaultValue: None, foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: None }), expr: exp.clone(), iterators: cons(iter1.clone(), cons(iter2.clone(), metamodelica::nil())) });
            (cache.clone(), list![(exp.clone(), None)])
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((cache, exps))
}

fn operatorsBinary(mut inOperator: Absyn::Operator, mut t1: Arc<DAE::Type>, mut e1: Arc<DAE::Exp>, mut t2: Arc<DAE::Type>, mut e2: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>>, Arc<DAE::Type>, Arc<DAE::Exp>, Arc<DAE::Type>, Arc<DAE::Exp>, Arc<DAE::Type>, Arc<DAE::Exp>, Arc<DAE::Type>, Arc<DAE::Exp>)> {
    let mut ops: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
    let mut t1: Arc<DAE::Type> = t1;
    let mut e1: Arc<DAE::Exp> = e1;
    let mut t2: Arc<DAE::Type> = t2;
    let mut e2: Arc<DAE::Exp> = e2;
    let mut oty1: Arc<DAE::Type> = t1.clone();
    let mut oe1: Arc<DAE::Exp> = e1.clone();
    let mut oty2: Arc<DAE::Type> = t2.clone();
    let mut oe2: Arc<DAE::Exp> = e2.clone();
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut e: Arc<DAE::Exp>;
    let mut op: Absyn::Operator = inOperator.clone();
    let mut ia1: bool = Types::isArray(t1.clone());
    let mut ia2: bool = Types::isArray(t2.clone());
    if ia2.clone() && !(ia1.clone()) {
        (e1, e2, t1, t2) = (match op.clone() {
        Absyn::Operator::ADD_EW => (e2.clone(), e1.clone(), t2.clone(), t1.clone()),
        Absyn::Operator::MUL => (e2.clone(), e1.clone(), t2.clone(), t1.clone()),
        Absyn::Operator::MUL_EW => (e2.clone(), e1.clone(), t2.clone(), t1.clone()),
        _ => (e1.clone(), e2.clone(), t1.clone(), t2.clone()),
    });
    } else if ia1.clone() && !(ia2.clone()) {
        (op, e2) = (match op.clone() {
        Absyn::Operator::SUB_EW => (openmodelica_ast::Absyn::Operator::ADD_EW, Expression::negate(e2.clone())?),
        _ => (op.clone(), e2.clone()),
    });
    }
    match '__try0: {
        ops = (match op.clone() {
        Absyn::Operator::ADD => {
            OperatorsBinary.addTypes.clone()
        },
        Absyn::Operator::ADD_EW => {
            OperatorsBinary.addEwTypes.clone()
        },
        Absyn::Operator::SUB => {
            OperatorsBinary.subTypes.clone()
        },
        Absyn::Operator::SUB_EW => {
            OperatorsBinary.subEwTypes.clone()
        },
        Absyn::Operator::MUL => {
            OperatorsBinary.mulTypes.clone()
        },
        Absyn::Operator::MUL_EW => {
            OperatorsBinary.mulEwTypes.clone()
        },
        Absyn::Operator::DIV => {
            OperatorsBinary.divTypes.clone()
        },
        Absyn::Operator::DIV_EW => {
            OperatorsBinary.divEwTypes.clone()
        },
        Absyn::Operator::POW => {
            OperatorsBinary.powTypes.clone()
        },
        Absyn::Operator::POW_EW => {
            let mut realarrs: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut realscalararrs: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut realarrsscalar: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            realarrs = operatorReturn(DAE::Operator::POW_ARR2 { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }) }, realarrtypes().clone(), realarrtypes().clone(), realarrtypes().clone());
            scalars = list![(DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_REAL_DEFAULT().clone())];
            realscalararrs = operatorReturn(DAE::Operator::POW_SCALAR_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }) }, realtypes().clone(), realarrtypes().clone(), realarrtypes().clone());
            realarrsscalar = operatorReturn(DAE::Operator::POW_ARRAY_SCALAR { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }) }, realarrtypes().clone(), realtypes().clone(), realarrtypes().clone());
            types = List::flatten(list![scalars.clone(), realscalararrs.clone(), realarrsscalar.clone(), realarrs.clone()]);
            types.clone()
        },
        Absyn::Operator::AND => {
            OperatorsBinary.andTypes.clone()
        },
        Absyn::Operator::OR => {
            OperatorsBinary.orTypes.clone()
        },
        Absyn::Operator::LESS => {
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut enum_op: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            enum_op = unwrap_break_err!(makeEnumOperator(DAE::Operator::LESS { ty: DAE::T_ENUMERATION_DEFAULT().clone() }, t1.clone(), t2.clone()), '__try0);
            scalars = list![(DAE::Operator::LESS { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), enum_op.clone(), (DAE::Operator::LESS { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::LESS { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::LESS { ty: DAE::T_STRING_DEFAULT().clone() }, list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone())];
            types = List::flatten(list![scalars.clone()]);
            types.clone()
        },
        Absyn::Operator::LESSEQ => {
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut enum_op: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            enum_op = unwrap_break_err!(makeEnumOperator(DAE::Operator::LESSEQ { ty: DAE::T_ENUMERATION_DEFAULT().clone() }, t1.clone(), t2.clone()), '__try0);
            scalars = list![(DAE::Operator::LESSEQ { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), enum_op.clone(), (DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::LESSEQ { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::LESSEQ { ty: DAE::T_STRING_DEFAULT().clone() }, list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone())];
            types = List::flatten(list![scalars.clone()]);
            types.clone()
        },
        Absyn::Operator::GREATER => {
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut enum_op: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            enum_op = unwrap_break_err!(makeEnumOperator(DAE::Operator::GREATER { ty: DAE::T_ENUMERATION_DEFAULT().clone() }, t1.clone(), t2.clone()), '__try0);
            scalars = list![(DAE::Operator::GREATER { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), enum_op.clone(), (DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::GREATER { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::GREATER { ty: DAE::T_STRING_DEFAULT().clone() }, list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone())];
            types = List::flatten(list![scalars.clone()]);
            types.clone()
        },
        Absyn::Operator::GREATEREQ => {
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut enum_op: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            enum_op = unwrap_break_err!(makeEnumOperator(DAE::Operator::GREATEREQ { ty: DAE::T_ENUMERATION_DEFAULT().clone() }, t1.clone(), t2.clone()), '__try0);
            scalars = list![(DAE::Operator::GREATEREQ { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), enum_op.clone(), (DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::GREATEREQ { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), (DAE::Operator::GREATEREQ { ty: DAE::T_STRING_DEFAULT().clone() }, list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone())];
            types = List::flatten(list![scalars.clone()]);
            types.clone()
        },
        Absyn::Operator::EQUAL => {
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut enum_op: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            enum_op = unwrap_break_err!(makeEnumOperator(DAE::Operator::EQUAL { ty: DAE::T_ENUMERATION_DEFAULT().clone() }, t1.clone(), t2.clone()), '__try0);
            types = cons((DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), cons(enum_op.clone(), cons((DAE::Operator::EQUAL { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), cons((DAE::Operator::EQUAL { ty: DAE::T_STRING_DEFAULT().clone() }, list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), cons((DAE::Operator::EQUAL { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), metamodelica::nil())))));
            types.clone()
        },
        Absyn::Operator::NEQUAL => {
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut enum_op: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            enum_op = unwrap_break_err!(makeEnumOperator(DAE::Operator::NEQUAL { ty: DAE::T_ENUMERATION_DEFAULT().clone() }, t1.clone(), t2.clone()), '__try0);
            types = cons((DAE::Operator::NEQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), cons(enum_op.clone(), cons((DAE::Operator::NEQUAL { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), cons((DAE::Operator::NEQUAL { ty: DAE::T_STRING_DEFAULT().clone() }, list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), cons((DAE::Operator::NEQUAL { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()), metamodelica::nil())))));
            types.clone()
        },
        _ => bail!("match: no arm matched"),
    });
        Ok::<_, anyhow::Error>((ops.clone(),))
    } {
        Ok((__try0_o0,)) => {
            ops = __try0_o0;
        }
        Err(_) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OperatorOverloading.operatorsBinary failed, op: ")); __mm_s.push_str(&*Dump::opSymbol(op.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail");
        }
    }
    Ok((ops, t1, e1, t2, e2, oty1, oe1, oty2, oe2))
}

fn operatorsUnary(mut op: Absyn::Operator) -> Result<Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>>> {
    let mut ops: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
    ops = (match op.clone() {
        Absyn::Operator::UMINUS => {
            let mut intarrs: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut realarrs: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            scalars = list![(DAE::Operator::UMINUS { ty: DAE::T_INTEGER_DEFAULT().clone() }, list![DAE::T_INTEGER_DEFAULT().clone()], DAE::T_INTEGER_DEFAULT().clone()), (DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, list![DAE::T_REAL_DEFAULT().clone()], DAE::T_REAL_DEFAULT().clone())];
            intarrs = operatorReturnUnary(DAE::Operator::UMINUS_ARR { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }) }, intarrtypes().clone(), intarrtypes().clone())?;
            realarrs = operatorReturnUnary(DAE::Operator::UMINUS_ARR { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }) }, realarrtypes().clone(), realarrtypes().clone())?;
            types = List::flatten(list![scalars.clone(), intarrs.clone(), realarrs.clone()]);
            types.clone()
        },
        Absyn::Operator::NOT => {
            let mut boolarrs: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut scalars: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut types: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            scalars = list![(DAE::Operator::NOT { ty: DAE::T_BOOL_DEFAULT().clone() }, list![DAE::T_BOOL_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone())];
            boolarrs = operatorReturnUnary(DAE::Operator::NOT { ty: DAE::T_BOOL_DEFAULT().clone() }, boolarrtypes().clone(), boolarrtypes().clone())?;
            types = List::flatten(list![scalars.clone(), boolarrs.clone()]);
            types.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OperatorOverloading.operatorsUnary failed, op: ")); __mm_s.push_str(&*Dump::opSymbol(op.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    });
    Ok(ops)
}

fn makeEnumOperator(mut inOp: DAE::Operator, mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>) -> Result<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)> {
    let mut outOp: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
    outOp = 'mc: {
        let __mc_input = (inType1.clone(), inType2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    let mut op_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: DAE::Operator;
                    op_ty = Types::simplifyType(inType1.clone())?;
                    op = Expression::setOpType(inOp.clone(), op_ty.clone())?;
                    Ok((op.clone(), list![inType1.clone(), inType2.clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, _) => {
                    let mut op_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: DAE::Operator;
                    op_ty = Types::simplifyType(inType1.clone())?;
                    op = Expression::setOpType(inOp.clone(), op_ty.clone())?;
                    Ok((op.clone(), list![inType1.clone(), inType1.clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    let mut op_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: DAE::Operator;
                    op_ty = Types::simplifyType(inType2.clone())?;
                    op = Expression::setOpType(inOp.clone(), op_ty.clone())?;
                    Ok((op.clone(), list![inType2.clone(), inType2.clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inOp.clone(), list![DAE::T_ENUMERATION_DEFAULT().clone(), DAE::T_ENUMERATION_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outOp)
}

fn buildOperatorTypes(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>>> {
    let mut outOperatorTypes: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
    outOperatorTypes = (::match_deref::match_deref! { match &((inTypes.clone(), inPath.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcResultType: tp, funcArg: args, .. }, tail: tps }, funcname) => {
            let mut argtypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut rest: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            argtypes = List::map(args.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
            rest = buildOperatorTypes(tps.clone(), funcname.clone())?;
            cons((DAE::Operator::USERDEFINED { fqName: funcname.clone() }, argtypes.clone(), tp.clone()), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOperatorTypes)
}

fn operatorReturn(mut inOperator: DAE::Operator, mut inLhsTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inRhsTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inReturnTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> {
    let mut outOperators: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
    outOperators = {
        let mut __acc: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
        for ((l, r), re) in (&(inLhsTypes.clone())).into_iter().zip((&(inRhsTypes.clone())).into_iter()).zip((&(inReturnTypes.clone())).into_iter()) {
            let __x = (inOperator.clone(), list![l.clone(), r.clone()], re.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outOperators
}

fn operatorReturnUnary(mut inOperator: DAE::Operator, mut inArgTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inReturnTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>>> {
    let mut outOperators: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
    outOperators = (::match_deref::match_deref! { match &((inOperator.clone(), inArgTypes.clone(), inReturnTypes.clone())) {
        (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (op, Deref @ metamodelica::List::Cons { head: l, tail: lr }, Deref @ metamodelica::List::Cons { head: re, tail: rer }) => {
            let mut rest: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>> = metamodelica::nil();
            let mut t: (DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>);
            rest = operatorReturnUnary(op.clone(), lr.clone(), rer.clone())?;
            t = (op.clone(), list![l.clone()], re.clone());
            cons(t.clone(), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOperators)
}

fn getOperatorFuncsOrEmpty(mut inCache: FCore::Cache, mut env: FCore::Graph, mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut opName: ArcStr, mut info: SourceInfo, mut acc: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut funcs: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (cache, funcs) = 'mc: {
        let __mc_input = (inCache.clone(), env.clone(), tys.clone(), opName.clone(), info.clone(), acc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: ty, tail: rest }, _, _, _) => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut funcs: Arc<metamodelica::List<Arc<DAE::Type>>> = funcs.clone();
                    (cache, funcs) = getOperatorFuncsOrEmptySingleTy(inCache.clone(), env.clone(), ty.clone(), (opName.clone()).clone(), info.clone())?;
                    (cache, funcs) = getOperatorFuncsOrEmpty(cache.clone(), env.clone(), rest.clone(), (opName.clone()).clone(), info.clone(), listAppend(funcs.clone(), acc.clone()))?;
                    Ok((cache.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _) => {
                    let mut funcs: Arc<metamodelica::List<Arc<DAE::Type>>> = funcs.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    (cache, funcs) = getOperatorFuncsOrEmpty(inCache.clone(), env.clone(), rest.clone(), (opName.clone()).clone(), info.clone(), acc.clone())?;
                    Ok((cache.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _, _, _) => {
                    let mut funcs: Arc<metamodelica::List<Arc<DAE::Type>>> = funcs.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let (__pa0, Util::SUCCESS { .. }) = (Static::instantiateDaeFunctionFromTypes(inCache.clone(), env.clone(), acc.clone(), false, None, true, openmodelica_util::Util::Status::SUCCESS)?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(Types::traverseType(Arc::new(DAE::Type::T_TUPLE { types: acc.clone(), names: None }), -1, (std::sync::Arc::new(fnptr!(Types::makeExpDimensionsUnknown, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?) {
                        (Deref @ DAE::Type::T_TUPLE { types: __pa1, names: _ }, _) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    funcs = __pa1.clone();
                    Ok((cache.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, funcs))
}

pub mod AvlTreePathPathEnv {
    use super::*;
    pub type Key = Arc<Absyn::Path>;

    pub type Value = Arc<Absyn::Path>;

    pub fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (AbsynUtil::pathString(inKey.clone(), (literal!(".")).clone(), true, false)?).clone();
        Ok(outString)
    }

    pub fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (AbsynUtil::pathString(inValue.clone(), (literal!(".")).clone(), true, false)?).clone();
        Ok(outString)
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32 = 0;
        outResult = AbsynUtil::pathCompareNoQual(inKey1.clone(), inKey2.clone())?;
        Ok(outResult)
    }

    pub use addConflictKeep as addConflictDefault;

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Tree {
        NODE {
            /// The key of the node.
            key: Key,
            value: Value,
            /// Height of tree, used for balancing
            height: i32,
            /// Left subtree.
            left: Arc<Tree>,
            /// Right subtree.
            right: Arc<Tree>,
        },
        LEAF {
            /// The key of the node.
            key: Key,
            value: Value,
        },
        EMPTY,
    }
    pub use self::Tree::{NODE,LEAF,EMPTY};

    pub type ValueNode = Arc<Absyn::Path>;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value;
            let mut key_comp: i32 = 0;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc.clone())?);
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value;
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare(inKey.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }), right: Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }) });
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::LEAF).clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::LEAF).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::LEAF; value = value.clone());
                }
                outTree = tree.clone();
            }
            if (key_comp.clone() == 0) {outTree.clone()} else {balance(outTree.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
        let mut value: Value;
        bail!("fail");
        Ok(value)
    }

    pub fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = oldValue.clone();
        value
    }

    pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = newValue.clone();
        value
    }

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<Absyn::Path>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key;
        let mut value: Value;
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Path>>) -> Result<Arc<Absyn::Path>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Path>>) -> Result<Value> + 'static>;

        let mut tree: Arc<Tree> = tree;
        let mut key_comp: i32 = 0;
        let mut new_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }),
        Deref @ Tree::NODE { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::NODE).clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else {
                assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }), right: Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }) });
            } else {
                assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                new_tree = tree.clone();
            }
            if (key_comp.clone() == 0) {new_tree.clone()} else {balance(new_tree.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    fn balance(mut inTree: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::LEAF { .. } => {
            inTree.clone()
        },
        Deref @ Tree::NODE { .. } => {
            let mut lh: i32 = 0;
            let mut rh: i32 = 0;
            let mut diff: i32 = 0;
            let mut balanced_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
            lh = height(var_field!((*outTree).left, Tree::NODE).clone());
            rh = height(var_field!((*outTree).right, Tree::NODE).clone());
            diff = lh.clone() - rh.clone();
            if diff.clone() < -1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE).clone()) > 0) {rotateLeft(setTreeLeftRight(outTree.clone(), var_field!((*outTree).left, Tree::NODE).clone(), rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?)?)?} else {rotateLeft(outTree.clone())?};
            } else if diff.clone() > 1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE).clone()) < 0) {rotateRight(setTreeLeftRight(outTree.clone(), rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?, var_field!((*outTree).right, Tree::NODE).clone())?)?} else {rotateRight(outTree.clone())?};
            } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh.clone(), rh.clone()) + 1 {
                assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh.clone(), rh.clone()) + 1);
                balanced_tree = outTree.clone();
            } else {
                balanced_tree = outTree.clone();
            }
            balanced_tree.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(outTree)
    }

    fn calculateBalance(mut inNode: Arc<Tree>) -> i32 {
        let mut outBalance: i32 = 0;
        outBalance = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => height(var_field!((*inNode).left, Tree::NODE).clone()) - height(var_field!((*inNode).right, Tree::NODE).clone()),
        Deref @ Tree::LEAF { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outBalance
    }

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue.clone();
        outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult.clone()
        },
        _ => {
            outResult.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outResult
    }

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), value.clone()).unwrap();
            if c.clone() {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value.clone());
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value.clone());
            }
            value.clone()
        },
        Deref @ Tree::LEAF { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), value.clone()).unwrap();
            value.clone()
        },
        _ => {
            value.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (foldArg1, foldArg2)
    }

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let _ = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            func(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone())?;
            ()
        },
        Deref @ Tree::EMPTY { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn fromList(mut inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<Absyn::Path>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY);
        let mut key: Key;
        let mut value: Value;
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(value)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Result<Option<Arc<Absyn::Path>>> {
        let mut value: Option<Arc<Absyn::Path>> = None;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => Some(var_field!((*tree).value, Tree::LEAF).clone()),
        (0, Deref @ Tree::NODE { .. }) => Some(var_field!((*tree).value, Tree::NODE).clone()),
        (1, Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(value)
    }

    pub fn hasKey(mut inTree: Arc<Tree>, mut inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key;
        let mut key_comp: i32 = 0;
        let mut tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        key = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inTree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*inTree).key, Tree::LEAF).clone(),
        Deref @ Tree::EMPTY { .. } => {
            return Ok(comp);
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
        key_comp = keyCompare(inKey.clone(), key.clone())?;
        comp = (::match_deref::match_deref! { match &((key_comp.clone(), inTree.clone())) {
        (0, _) => true,
        (1, Deref @ Tree::NODE { right: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        ((-1), Deref @ Tree::NODE { left: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(comp)
    }

    fn height(mut inNode: Arc<Tree>) -> i32 {
        let mut outHeight: i32 = 0;
        outHeight = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inNode).height, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outHeight
    }

    pub fn intersection() -> Result<()> {
        bail!("fail");
        Ok(())
    }

    pub fn isEmpty(mut tree: Arc<Tree>) -> bool {
        let mut isEmpty: bool = false;
        isEmpty = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEmpty
    }

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), var_field!((*treeToJoin).key, Tree::NODE).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc.clone())?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), var_field!((*treeToJoin).key, Tree::LEAF).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { key, .. } => {
            lst = listKeys(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(key.clone(), lst.clone());
            lst = listKeys(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { key, .. } => {
            cons(key.clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::LEAF { .. } => cons(var_field!((*inTree).key, Tree::LEAF).clone(), lst.clone()),
        Deref @ Tree::NODE { .. } => {
            lst = listKeysReverse(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst = cons(var_field!((*inTree).key, Tree::NODE).clone(), lst.clone());
            lst = listKeysReverse(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { value, .. } => {
            lst = listValues(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(value.clone(), lst.clone());
            lst = listValues(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, .. } => {
            cons(value.clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value;
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone());
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value;
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outTree
    }

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, FT) -> Result<(Arc<Absyn::Path>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value;
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value;
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outTree, outResult)
    }

    pub fn new() -> Arc<Tree> {
        let mut outTree: Arc<Tree> = Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(outString)
    }

    pub fn printTreeStr(mut inTree: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        let mut left: Arc<Tree> = Arc::new(Tree::EMPTY);
        let mut right: Arc<Tree> = Arc::new(Tree::EMPTY);
        outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::EMPTY { .. } => literal!("EMPTY()"),
        Deref @ Tree::LEAF { .. } => printNodeStr(inTree.clone())?,
        Deref @ Tree::NODE { right, left, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(left.clone(), true, (literal!("")).clone())?); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(right.clone(), false, (literal!("")).clone())?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(outString)
    }

    fn printTreeStr2(mut inTree: Arc<Tree>, mut isLeft: bool, mut inIndent: ArcStr) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        let mut val_node: Option<Arc<Absyn::Path>> = None;
        let mut left: Option<Arc<Tree>> = None;
        let mut right: Option<Arc<Tree>> = None;
        let mut left_str: ArcStr = arcstr::literal!("");
        let mut right_str: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).left, Tree::NODE).clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!("     ")} else {literal!(" │   ")}); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).right, Tree::NODE).clone(), false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" │   ")} else {literal!("     ")}); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(outString)
    }

    fn referenceEqOrEmpty(mut t1: Arc<Tree>, mut t2: Arc<Tree>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
        _ => referenceEq(&t1.clone(),&t2.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    fn rotateLeft(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outNode: Arc<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), var_field!((**child).left, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), node.clone(), var_field!((**child).right, Tree::NODE).clone())?
        },
        Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY))?
        },
        _ => {
            inNode.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outNode)
    }

    fn rotateRight(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outNode: Arc<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((**child).right, Tree::NODE).clone(), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node.clone())?
        },
        Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY), node.clone())?
        },
        _ => {
            inNode.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outNode)
    }

    pub fn setTreeLeftRight(mut orig: Arc<Tree>, mut left: Arc<Tree>, mut right: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut res: Arc<Tree> = Arc::new(Tree::EMPTY);
        res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig.clone(),
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn smallestKey(mut tree: Arc<Tree>) -> Result<Key> {
        let mut key: Key;
        key = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::NODE { .. } => smallestKey(var_field!((*tree).right, Tree::NODE).clone())?,
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(key)
    }

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<Absyn::Path>)>>) -> Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<Absyn::Path>)>> {
        let mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<Absyn::Path>)>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            lst = toList(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst = cons((key.clone(), value.clone()), lst.clone());
            lst = toList(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            cons((key.clone(), value.clone()), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn update(mut tree: Arc<Tree>, mut key: Key, mut value: Value) -> Arc<Tree> {
        let mut outTree: Arc<Tree> = add(tree.clone(), key.clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<Absyn::Path>, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>)).unwrap();
        outTree
    }

}

pub mod AvlTreePathOperatorTypes {
    use super::*;
    pub type Key = Arc<Absyn::Path>;

    pub type Value = Arc<metamodelica::List<Arc<DAE::Type>>>;

    pub fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (AbsynUtil::pathString(inKey.clone(), (literal!(".")).clone(), true, false)?).clone();
        Ok(outString)
    }

    pub fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (TypesDump::unparseType(Arc::new(DAE::Type::T_METATUPLE { types: inValue.clone() }))?).clone();
        Ok(outString)
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32 = 0;
        outResult = AbsynUtil::pathCompareNoQual(inKey1.clone(), inKey2.clone())?;
        Ok(outResult)
    }

    pub use addConflictKeep as addConflictDefault;

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Tree {
        NODE {
            /// The key of the node.
            key: Key,
            value: Value,
            /// Height of tree, used for balancing
            height: i32,
            /// Left subtree.
            left: Arc<Tree>,
            /// Right subtree.
            right: Arc<Tree>,
        },
        LEAF {
            /// The key of the node.
            key: Key,
            value: Value,
        },
        EMPTY,
    }
    pub use self::Tree::{NODE,LEAF,EMPTY};

    pub type ValueNode = Arc<Absyn::Path>;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = metamodelica::nil();
            let mut key_comp: i32 = 0;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey.clone(), inValue.clone(), conflictFunc.clone())?);
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value = metamodelica::nil();
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare(inKey.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }), right: Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: inKey.clone(), value: inValue.clone() }) });
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::LEAF).clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::LEAF).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::LEAF; value = value.clone());
                }
                outTree = tree.clone();
            }
            if (key_comp.clone() == 0) {outTree.clone()} else {balance(outTree.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
        let mut value: Value = metamodelica::nil();
        bail!("fail");
        Ok(value)
    }

    pub fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = oldValue.clone();
        value
    }

    pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = newValue.clone();
        value
    }

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key;
        let mut value: Value = metamodelica::nil();
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<metamodelica::List<Arc<DAE::Type>>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<metamodelica::List<Arc<DAE::Type>>>>) -> Result<Value> + 'static>;

        let mut tree: Arc<Tree> = tree;
        let mut key_comp: i32 = 0;
        let mut new_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }),
        Deref @ Tree::NODE { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::NODE).clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key.clone(), r#fn.clone())?);
            } else {
                assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp.clone() == -1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }), right: Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: key.clone(), value: r#fn(None)? }) });
            } else {
                assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                new_tree = tree.clone();
            }
            if (key_comp.clone() == 0) {new_tree.clone()} else {balance(new_tree.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    fn balance(mut inTree: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::LEAF { .. } => {
            inTree.clone()
        },
        Deref @ Tree::NODE { .. } => {
            let mut lh: i32 = 0;
            let mut rh: i32 = 0;
            let mut diff: i32 = 0;
            let mut balanced_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
            lh = height(var_field!((*outTree).left, Tree::NODE).clone());
            rh = height(var_field!((*outTree).right, Tree::NODE).clone());
            diff = lh.clone() - rh.clone();
            if diff.clone() < -1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE).clone()) > 0) {rotateLeft(setTreeLeftRight(outTree.clone(), var_field!((*outTree).left, Tree::NODE).clone(), rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?)?)?} else {rotateLeft(outTree.clone())?};
            } else if diff.clone() > 1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE).clone()) < 0) {rotateRight(setTreeLeftRight(outTree.clone(), rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?, var_field!((*outTree).right, Tree::NODE).clone())?)?} else {rotateRight(outTree.clone())?};
            } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh.clone(), rh.clone()) + 1 {
                assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh.clone(), rh.clone()) + 1);
                balanced_tree = outTree.clone();
            } else {
                balanced_tree = outTree.clone();
            }
            balanced_tree.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(outTree)
    }

    fn calculateBalance(mut inNode: Arc<Tree>) -> i32 {
        let mut outBalance: i32 = 0;
        outBalance = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => height(var_field!((*inNode).left, Tree::NODE).clone()) - height(var_field!((*inNode).right, Tree::NODE).clone()),
        Deref @ Tree::LEAF { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outBalance
    }

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue.clone();
        outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            outResult = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            outResult.clone()
        },
        _ => {
            outResult.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outResult
    }

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), value.clone()).unwrap();
            if c.clone() {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value.clone());
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value.clone());
            }
            value.clone()
        },
        Deref @ Tree::LEAF { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), value.clone()).unwrap();
            value.clone()
        },
        _ => {
            value.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (foldArg1, foldArg2)
    }

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let _ = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            func(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone())?;
            ()
        },
        Deref @ Tree::EMPTY { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn fromList(mut inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY);
        let mut key: Key;
        let mut value: Value = metamodelica::nil();
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = metamodelica::nil();
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(value)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Result<Option<Arc<metamodelica::List<Arc<DAE::Type>>>>> {
        let mut value: Option<Arc<metamodelica::List<Arc<DAE::Type>>>> = None;
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k.clone())?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => Some(var_field!((*tree).value, Tree::LEAF).clone()),
        (0, Deref @ Tree::NODE { .. }) => Some(var_field!((*tree).value, Tree::NODE).clone()),
        (1, Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).right, Tree::NODE).clone(), key.clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).left, Tree::NODE).clone(), key.clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(value)
    }

    pub fn hasKey(mut inTree: Arc<Tree>, mut inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key;
        let mut key_comp: i32 = 0;
        let mut tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        key = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inTree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*inTree).key, Tree::LEAF).clone(),
        Deref @ Tree::EMPTY { .. } => {
            return Ok(comp);
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
        key_comp = keyCompare(inKey.clone(), key.clone())?;
        comp = (::match_deref::match_deref! { match &((key_comp.clone(), inTree.clone())) {
        (0, _) => true,
        (1, Deref @ Tree::NODE { right: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        ((-1), Deref @ Tree::NODE { left: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(comp)
    }

    fn height(mut inNode: Arc<Tree>) -> i32 {
        let mut outHeight: i32 = 0;
        outHeight = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inNode).height, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outHeight
    }

    pub fn intersection() -> Result<()> {
        bail!("fail");
        Ok(())
    }

    pub fn isEmpty(mut tree: Arc<Tree>) -> bool {
        let mut isEmpty: bool = false;
        isEmpty = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEmpty
    }

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), var_field!((*treeToJoin).key, Tree::NODE).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc.clone())?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), var_field!((*treeToJoin).key, Tree::LEAF).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { key, .. } => {
            lst = listKeys(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(key.clone(), lst.clone());
            lst = listKeys(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { key, .. } => {
            cons(key.clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
        let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::LEAF { .. } => cons(var_field!((*inTree).key, Tree::LEAF).clone(), lst.clone()),
        Deref @ Tree::NODE { .. } => {
            lst = listKeysReverse(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst = cons(var_field!((*inTree).key, Tree::NODE).clone(), lst.clone());
            lst = listKeysReverse(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Type>>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Type>>>>> {
        let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Type>>>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { value, .. } => {
            lst = listValues(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(value.clone(), lst.clone());
            lst = listValues(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, .. } => {
            cons(value.clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = metamodelica::nil();
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone());
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = metamodelica::nil();
            new_value = inFunc(key.clone(), value.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outTree
    }

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>, FT) -> Result<(Arc<metamodelica::List<Arc<DAE::Type>>>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = metamodelica::nil();
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = metamodelica::nil();
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outTree, outResult)
    }

    pub fn new() -> Arc<Tree> {
        let mut outTree: Arc<Tree> = Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(outString)
    }

    pub fn printTreeStr(mut inTree: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        let mut left: Arc<Tree> = Arc::new(Tree::EMPTY);
        let mut right: Arc<Tree> = Arc::new(Tree::EMPTY);
        outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::EMPTY { .. } => literal!("EMPTY()"),
        Deref @ Tree::LEAF { .. } => printNodeStr(inTree.clone())?,
        Deref @ Tree::NODE { right, left, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(left.clone(), true, (literal!("")).clone())?); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(right.clone(), false, (literal!("")).clone())?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(outString)
    }

    fn printTreeStr2(mut inTree: Arc<Tree>, mut isLeft: bool, mut inIndent: ArcStr) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        let mut val_node: Option<Arc<Absyn::Path>> = None;
        let mut left: Option<Arc<Tree>> = None;
        let mut right: Option<Arc<Tree>> = None;
        let mut left_str: ArcStr = arcstr::literal!("");
        let mut right_str: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).left, Tree::NODE).clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!("     ")} else {literal!(" │   ")}); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).right, Tree::NODE).clone(), false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" │   ")} else {literal!("     ")}); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(outString)
    }

    fn referenceEqOrEmpty(mut t1: Arc<Tree>, mut t2: Arc<Tree>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
        _ => referenceEq(&t1.clone(),&t2.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    fn rotateLeft(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outNode: Arc<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), var_field!((**child).left, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), node.clone(), var_field!((**child).right, Tree::NODE).clone())?
        },
        Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY))?
        },
        _ => {
            inNode.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outNode)
    }

    fn rotateRight(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut outNode: Arc<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((**child).right, Tree::NODE).clone(), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node.clone())?
        },
        Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY), node.clone())?
        },
        _ => {
            inNode.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outNode)
    }

    pub fn setTreeLeftRight(mut orig: Arc<Tree>, mut left: Arc<Tree>, mut right: Arc<Tree>) -> Result<Arc<Tree>> {
        let mut res: Arc<Tree> = Arc::new(Tree::EMPTY);
        res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig.clone(),
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn smallestKey(mut tree: Arc<Tree>) -> Result<Key> {
        let mut key: Key;
        key = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::NODE { .. } => smallestKey(var_field!((*tree).right, Tree::NODE).clone())?,
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(key)
    }

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>)>> {
        let mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            lst = toList(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst = cons((key.clone(), value.clone()), lst.clone());
            lst = toList(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            cons((key.clone(), value.clone()), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn update(mut tree: Arc<Tree>, mut key: Key, mut value: Value) -> Arc<Tree> {
        let mut outTree: Arc<Tree> = add(tree.clone(), key.clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> + 'static>)).unwrap();
        outTree
    }

}

fn getOperatorFuncsOrEmptySingleTy(mut cache: FCore::Cache, mut env: FCore::Graph, mut ty: Arc<DAE::Type>, mut opName: ArcStr, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut cache: FCore::Cache = cache;
    let mut funcs: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut path: Arc<Absyn::Path>;
    let mut pathIn: Arc<Absyn::Path>;
    let mut opNamePath: Arc<Absyn::Path>;
    let mut operatorCl: Arc<SCode::Element>;
    let mut recordEnv: FCore::Graph;
    let mut operEnv: FCore::Graph;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut scalarType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tree1: Arc<AvlTreePathPathEnv::Tree> = Arc::new(AvlTreePathPathEnv::Tree::EMPTY);
    let mut tree2: Arc<AvlTreePathOperatorTypes::Tree> = Arc::new(AvlTreePathOperatorTypes::Tree::EMPTY);
    let mut trees: (Arc<AvlTreePathPathEnv::Tree>, Arc<AvlTreePathOperatorTypes::Tree>);
    scalarType = Types::arrayElementType(ty.clone());
    pathIn = AbsynUtil::makeFullyQualified(getRecordPath(scalarType.clone())?);
    trees = crate::Globals::operatorOverloadingCache.with(|__root| __root.borrow().clone());
    (tree1, tree2) = trees.clone();
    match '__try0: {
        path = unwrap_break_err!(AvlTreePathPathEnv::get(tree1.clone(), pathIn.clone()), '__try0);
        Ok::<_, anyhow::Error>((path.clone(),))
    } {
        Ok((__try0_o0,)) => {
            path = __try0_o0;
        }
        Err(_) => {
            (cache, operatorCl, recordEnv) = Lookup::lookupClass(cache.clone(), env.clone(), pathIn.clone(), None)?;
            (cache, path, recordEnv) = lookupOperatorBaseClass(cache.clone(), recordEnv.clone(), operatorCl.clone())?;
            tree1 = AvlTreePathPathEnv::add(tree1.clone(), pathIn.clone(), path.clone(), (std::sync::Arc::new(fnptr!(AvlTreePathPathEnv::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            crate::Globals::operatorOverloadingCache.with(|__root| *__root.borrow_mut() = (tree1.clone(), tree2.clone()));
        }
    }
    opNamePath = Arc::new(Absyn::Path::IDENT { name: (opName.clone()).clone() });
    path = AbsynUtil::makeFullyQualified(AbsynUtil::joinPaths(path.clone(), opNamePath.clone())?);
    match '__try1: {
        funcs = unwrap_break_err!(AvlTreePathOperatorTypes::get(tree2.clone(), path.clone()), '__try1);
        Ok::<_, anyhow::Error>((funcs.clone(),))
    } {
        Ok((__try1_o0,)) => {
            funcs = __try1_o0;
        }
        Err(_) => {
            (cache, operatorCl, operEnv) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
            let true = (SCodeUtil::isOperator(operatorCl.clone())) else { bail!("pattern mismatch") };
            paths = AbsynToSCode::getListofQualOperatorFuncsfromOperator(operatorCl.clone())?;
            (cache, funcs) = Lookup::lookupFunctionsListInEnv(cache.clone(), operEnv.clone(), paths.clone(), info.clone(), metamodelica::nil())?;
            funcs = List::select2(funcs.clone(), (if (opName.clone() == literal!("'constructor'") || opName.clone() == literal!("'0'")) { ((std::sync::Arc::new(checkOperatorFunctionOutput) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>, SourceInfo) -> Result<bool> + 'static>) as _) } else { ((std::sync::Arc::new(checkOperatorFunctionOneOutput) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>, SourceInfo) -> Result<bool> + 'static>) as _) }), scalarType.clone(), info.clone());
            tree2 = AvlTreePathOperatorTypes::add(tree2.clone(), path.clone(), funcs.clone(), (std::sync::Arc::new(fnptr!(AvlTreePathOperatorTypes::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            crate::Globals::operatorOverloadingCache.with(|__root| *__root.borrow_mut() = (tree1.clone(), tree2.clone()));
        }
    }
    Ok((cache, funcs))
}

fn lookupOperatorBaseClass(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inClass: Arc<SCode::Element>) -> Result<(FCore::Cache, Arc<Absyn::Path>, FCore::Graph)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut path: Arc<Absyn::Path>;
    let mut env: FCore::Graph;
    (cache, path, env) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inClass.clone())) {
        (cache, env, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: None }, .. }, .. }) => {
            let mut cl: Arc<SCode::Element>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let mut path = (*path).clone();
            (cache, cl, env) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
            (cache, path, env) = lookupOperatorBaseClass(cache.clone(), env.clone(), cl.clone())?;
            (cache.clone(), path.clone(), env.clone())
        },
        (cache, env, Deref @ SCode::Element::CLASS { name, .. }) => {
            path = FGraph::joinScopePath(env.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            (cache.clone(), path.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cache, path, env))
}

fn checkOperatorFunctionOneOutput(mut ty: Arc<DAE::Type>, mut opType: Arc<DAE::Type>, mut info: SourceInfo) -> Result<bool> {
    let mut isOK: bool = false;
    isOK = (::match_deref::match_deref! { match &((ty.clone(), opType.clone(), info.clone())) {
        (Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { .. }, .. }, _, _) => {
            false
        },
        (Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, ty: ty1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, ty: ty2, .. }, tail: _ } }, .. }, _, _) => {
            let mut b: bool = false;
            b = Types::equivtypesOrRecordSubtypeOf(Types::arrayElementType(ty1.clone()), opType.clone())? || Types::equivtypesOrRecordSubtypeOf(Types::arrayElementType(ty2.clone()), opType.clone())?;
            checkOperatorFunctionOneOutputError(b.clone(), opType.clone(), ty.clone(), info.clone())?;
            b.clone()
        },
        (Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, ty: ty1, .. }, tail: _ }, .. }, _, _) => {
            let mut b: bool = false;
            b = Types::equivtypesOrRecordSubtypeOf(Types::arrayElementType(ty1.clone()), opType.clone())?;
            checkOperatorFunctionOneOutputError(b.clone(), opType.clone(), ty.clone(), info.clone())?;
            b.clone()
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isOK)
}

fn checkOperatorFunctionOneOutputError(mut ok: bool, mut opType: Arc<DAE::Type>, mut ty: Arc<DAE::Type>, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((ok.clone(), opType.clone(), ty.clone(), info.clone())) {
        (true, _, _, _) => {
            ()
        },
        _ => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (TypesDump::unparseType(opType.clone())?).clone();
            str2 = (TypesDump::unparseType(ty.clone())?).clone();
            Error::addSourceMessage(Error::OP_OVERLOAD_OPERATOR_NOT_INPUT.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkOperatorFunctionOutput(mut ty: Arc<DAE::Type>, mut expected: Arc<DAE::Type>, mut info: SourceInfo) -> Result<bool> {
    let mut isOK: bool = false;
    isOK = (::match_deref::match_deref! { match &((ty.clone(), expected.clone(), info.clone())) {
        (Deref @ DAE::Type::T_FUNCTION { funcResultType: actual, .. }, _, _) => {
            isOK = Types::equivtypesOrRecordSubtypeOf(actual.clone(), expected.clone())?;
            isOK.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isOK)
}

fn isOperatorBinaryFunctionOrWarn(mut ty: Arc<DAE::Type>, mut info: SourceInfo) -> bool {
    let mut isBinaryFunc: bool = false;
    isBinaryFunc = (::match_deref::match_deref! { match &((ty.clone(), info.clone())) {
        (Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            false
        },
        (Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, .. }, tail: rest } }, .. }, _) => {
            isBinaryFunc = List::mapMapBoolAnd(rest.clone(), (std::sync::Arc::new(Types::funcArgDefaultBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), std::sync::Arc::new(fnptr!(isSome, _)));
            isBinaryFunc.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBinaryFunc
}

fn isOperatorUnaryFunction(mut ty: Arc<DAE::Type>) -> bool {
    let mut isBinaryFunc: bool = false;
    isBinaryFunc = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, .. }, tail: rest }, .. } => {
            isBinaryFunc = List::mapMapBoolAnd(rest.clone(), (std::sync::Arc::new(Types::funcArgDefaultBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), std::sync::Arc::new(fnptr!(isSome, _)));
            isBinaryFunc.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBinaryFunc
}

fn getZeroConstructorExpression(mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut result: Arc<DAE::Exp>;
    result = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { path, functionAttributes: attr, funcArg: args, .. } => {
            result = makeCallFillRestDefaults(path.clone(), metamodelica::nil(), args.clone(), Types::makeCallAttr(ty.clone(), attr.clone()));
            result.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn makeCallFillRestDefaults(mut path: Arc<Absyn::Path>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut restArgs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut attr: Arc<DAE::CallAttributes>) -> Arc<DAE::Exp> {
    let mut exp: Arc<DAE::Exp>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    exps = listAppend(inExps.clone(), List::mapMap(restArgs.clone(), (std::sync::Arc::new(Types::funcArgDefaultBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>)));
    exp = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: exps.clone(), attr: attr.clone() });
    exp
}

fn getRecordPath(mut inType1: Arc<DAE::Type>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(Types::arrayElementType(inType1.clone())) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outPath = __pa0.clone();
    Ok(outPath)
}

fn deoverload(mut inOperators: Arc<metamodelica::List<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<DAE::Type>)>>, mut inArgs: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Type>)>>, mut aexp: Arc<Absyn::Exp>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(DAE::Operator, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>)> {
    let mut outOperator: DAE::Operator;
    let mut outArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outOperator, outArgs, outType) = 'mc: {
        let __mc_input = (inOperators.clone(), inArgs.clone(), aexp.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (op, params, rtype), tail: _ }, args, _, pre, _) => {
                    let mut args_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut types_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut rtype_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op = (*op).clone();
                    (args_1, types_1) = elabArglist(params.clone(), args.clone())?;
                    rtype_1 = computeReturnType(op.clone(), types_1.clone(), rtype.clone(), pre.clone(), info.clone())?;
                    ty = Types::simplifyType(rtype_1.clone())?;
                    op = Expression::setOpType(op.clone(), ty.clone())?;
                    Ok((op.clone(), args_1.clone(), rtype_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, args, _, pre, _) => {
                    let mut args_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: DAE::Operator;
                    (op, args_1, rtype) = deoverload(xs.clone(), args.clone(), aexp.clone(), pre.clone(), info.clone())?;
                    Ok((op.clone(), args_1.clone(), rtype.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, args, _, pre, _) => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tps: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut exps_str: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut tps_str: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut tpsstr: ArcStr = arcstr::literal!("");
                    s = (Dump::printExpStr(aexp.clone())?).clone();
                    exps = List::map(args.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    tps = List::map(args.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
                    exps_str = List::map(exps.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>));
                    let _ = stringDelimitList(exps_str.clone(), (literal!(", ")).clone());
                    tps_str = List::map(tps.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>));
                    tpsstr = stringDelimitList(tps_str.clone(), (literal!(", ")).clone());
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::UNRESOLVABLE_TYPE.clone(), list![(s.clone()).clone(), (tpsstr.clone()).clone(), (pre_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outOperator, outArgs, outType))
}

fn computeReturnType(mut inOperator: DAE::Operator, mut inTypesTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inType: Arc<DAE::Type>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inOperator.clone(), inTypesTypeLst.clone(), inType.clone(), inPrefix.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("vector addition")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("vector subtraction")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("vector elementwise multiplication")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("vector elementwise division")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let mut m: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut n: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let 2 = (nDims(typ1.clone())?) else { bail!("pattern mismatch") };
                    n = Types::getDimensionNth(typ1.clone(), 1)?;
                    m = Types::getDimensionNth(typ1.clone(), 2)?;
                    let true = (Expression::dimensionsKnownAndEqual(n.clone(), m.clone())?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR2 { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR2 { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR2 { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("elementwise vector^vector")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, rtype, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(rtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, rtype, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(rtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("scalar product")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let mut rtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut etype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut n1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut n2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut m: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let 1 = (nDims(typ1.clone())?) else { bail!("pattern mismatch") };
                    let 2 = (nDims(typ2.clone())?) else { bail!("pattern mismatch") };
                    n1 = Types::getDimensionNth(typ1.clone(), 1)?;
                    n2 = Types::getDimensionNth(typ2.clone(), 1)?;
                    m = Types::getDimensionNth(typ2.clone(), 2)?;
                    let true = (isValidMatrixProductDims(n1.clone(), n2.clone())?) else { bail!("pattern mismatch") };
                    etype = elementType(typ1.clone())?;
                    rtype = Types::liftArray(etype.clone(), m.clone());
                    Ok(rtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let mut rtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut etype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut n: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut m1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut m2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let 2 = (nDims(typ1.clone())?) else { bail!("pattern mismatch") };
                    let 1 = (nDims(typ2.clone())?) else { bail!("pattern mismatch") };
                    n = Types::getDimensionNth(typ1.clone(), 1)?;
                    m1 = Types::getDimensionNth(typ1.clone(), 2)?;
                    m2 = Types::getDimensionNth(typ2.clone(), 1)?;
                    let true = (isValidMatrixProductDims(m1.clone(), m2.clone())?) else { bail!("pattern mismatch") };
                    etype = elementType(typ2.clone())?;
                    rtype = Types::liftArray(etype.clone(), n.clone());
                    Ok(rtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let mut rtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut etype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut n: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut m1: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut m2: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut p: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let 2 = (nDims(typ1.clone())?) else { bail!("pattern mismatch") };
                    let 2 = (nDims(typ2.clone())?) else { bail!("pattern mismatch") };
                    n = Types::getDimensionNth(typ1.clone(), 1)?;
                    m1 = Types::getDimensionNth(typ1.clone(), 2)?;
                    m2 = Types::getDimensionNth(typ2.clone(), 1)?;
                    p = Types::getDimensionNth(typ2.clone(), 2)?;
                    let true = (isValidMatrixProductDims(m1.clone(), m2.clone())?) else { bail!("pattern mismatch") };
                    etype = elementType(typ1.clone())?;
                    rtype = Types::liftArrayListDims(etype.clone(), list![n.clone(), p.clone()]);
                    Ok(rtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("matrix multiplication")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARRAY_SCALAR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARRAY_SCALAR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_SCALAR_ARRAY { .. }, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_SCALAR_ARRAY { .. }, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARRAY_SCALAR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARRAY_SCALAR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_SCALAR_ARRAY { .. }, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    Ok(typ2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: _ }, _, _, _) => {
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::AND { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::equivtypes(typ1.clone(), typ2.clone())?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::AND { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("and")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::OR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
                    let true = (Types::equivtypes(typ1.clone(), typ2.clone())?) else { bail!("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::OR { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Cons { head: typ2, tail: Deref @ metamodelica::List::Nil } }, _, pre, _) => {
                    let mut t1_str: ArcStr = arcstr::literal!("");
                    let mut t2_str: ArcStr = arcstr::literal!("");
                    let mut pre_str: ArcStr = arcstr::literal!("");
                    t1_str = (TypesDump::unparseType(typ1.clone())?).clone();
                    t2_str = (TypesDump::unparseType(typ2.clone())?).clone();
                    pre_str = (PrefixUtil::printPrefixStr3(pre.clone())?).clone();
                    Error::addSourceMessage(Error::INCOMPATIBLE_TYPES.clone(), list![(literal!("or")).clone(), (pre_str.clone()).clone(), (t1_str.clone()).clone(), (t2_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NOT { .. }, Deref @ metamodelica::List::Cons { head: typ1, tail: Deref @ metamodelica::List::Nil }, _, _, _) => {
                    Ok(typ1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::LESS { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::LESSEQ { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::GREATER { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::GREATEREQ { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::EQUAL { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NEQUAL { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::USERDEFINED { .. }, _, typ, _, _) => {
                    Ok(typ.clone())
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
fn nDims(mut inType: Arc<DAE::Type>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            0
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            0
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            0
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            0
        },
        Deref @ DAE::Type::T_ARRAY { ty: t, .. } => {
            let mut ns: i32 = 0;
            ns = nDims(t.clone())?;
            ns.clone() + 1
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
            let mut ns: i32 = 0;
            ns = nDims(t.clone())?;
            ns.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn isValidMatrixProductDims(mut dim1: Arc<DAE::Dimension>, mut dim2: Arc<DAE::Dimension>) -> Result<bool> {
    let mut res: bool = false;
    res = Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())? || !(Expression::dimensionKnown(dim1.clone()) || Expression::dimensionKnown(dim2.clone())) || Flags::getConfigBool(Flags::CHECK_MODEL.clone())? && Expression::dimensionsEqual(dim1.clone(), dim2.clone())?;
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn elementType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        t @ Deref @ DAE::Type::T_INTEGER { .. } => {
            t.clone()
        },
        t @ Deref @ DAE::Type::T_REAL { .. } => {
            t.clone()
        },
        t @ Deref @ DAE::Type::T_STRING { .. } => {
            t.clone()
        },
        t @ Deref @ DAE::Type::T_BOOL { .. } => {
            t.clone()
        },
        Deref @ DAE::Type::T_ARRAY { ty: t, .. } => {
            let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            t_1 = elementType(t.clone())?;
            t_1.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
            let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            t_1 = elementType(t.clone())?;
            t_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

fn replaceOperatorWithFcall(mut AbExp: Arc<Absyn::Exp>, mut inExp1: Arc<DAE::Exp>, mut inOper: DAE::Operator, mut inExp2: Option<Arc<DAE::Exp>>, mut inConst: DAE::Const) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (AbExp.clone(), inExp1.clone(), inOper.clone(), inExp2.clone(), inConst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BINARY { exp1: _, op: _, exp2: _ }, e1, DAE::Operator::USERDEFINED { fqName: funcname }, Some(e2), _) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1.clone(), e2.clone()], attr: DAE::callAttrOther().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BINARY { exp1: _, op: _, exp2: _ }, e1, _, Some(e2), _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: inOper.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: _, exp: _ }, e1, DAE::Operator::USERDEFINED { fqName: funcname }, None, _) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1.clone()], attr: DAE::callAttrOther().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: _, exp: _ }, e1, _, None, _) => {
                    Ok(Arc::new(DAE::Exp::UNARY { operator: inOper.clone(), exp: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LBINARY { exp1: _, op: _, exp2: _ }, e1, DAE::Operator::USERDEFINED { fqName: funcname }, Some(e2), _) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1.clone(), e2.clone()], attr: DAE::callAttrOther().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LBINARY { exp1: _, op: _, exp2: _ }, e1, _, Some(e2), _) => {
                    Ok(Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: inOper.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LUNARY { op: _, exp: _ }, e1, DAE::Operator::USERDEFINED { fqName: funcname }, None, _) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1.clone()], attr: DAE::callAttrOther().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LUNARY { op: _, exp: _ }, e1, _, None, _) => {
                    Ok(Arc::new(DAE::Exp::LUNARY { operator: inOper.clone(), exp: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp1: _, op: _, exp2: _ }, e1, DAE::Operator::USERDEFINED { fqName: funcname }, Some(e2), _) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1.clone(), e2.clone()], attr: DAE::callAttrOther().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp1: _, op: _, exp2: _ }, e1, _, Some(e2), _) => {
                    Ok(Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: inOper.clone(), exp2: e2.clone(), index: -1, optionExpisASUB: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn warnUnsafeRelations(mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut variability: DAE::Const, mut t1: Arc<DAE::Type>, mut t2: Arc<DAE::Type>, mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut op: DAE::Operator, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inEnv.clone(), inExp.clone(), variability.clone(), t1.clone(), t2.clone(), e1.clone(), e2.clone(), op.clone(), inPrefix.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _) => {
                    let true = (FGraph::inFunctionScope(inEnv.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Exp::RELATION { exp1: _, op: _, exp2: _ }, DAE::Const::C_VAR, _, _, _, _, _, _, _) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut stmtString: ArcStr = arcstr::literal!("");
                    let mut opString: ArcStr = arcstr::literal!("");
                    b1 = Types::isReal(t1.clone());
                    b2 = Types::isReal(t1.clone());
                    let true = (boolOr(b1.clone(), b2.clone())) else { bail!("pattern mismatch") };
                    verifyOp(op.clone())?;
                    opString = (ExpressionDump::relopSymbol(op.clone())?).clone();
                    stmtString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*opString.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::WARNING_RELATION_ON_REAL.clone(), list![(stmtString.clone()).clone(), (opString.clone()).clone()], inInfo.clone())?;
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

fn verifyOp(mut op: DAE::Operator) -> Result<()> {
    let _ = (match op.clone() {
        DAE::Operator::EQUAL { ty: _ } => (),
        DAE::Operator::NEQUAL { ty: _ } => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn errorMultipleValid(mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut info: SourceInfo) -> Result<()> {
    let mut str1: ArcStr = arcstr::literal!("");
    let mut str2: ArcStr = arcstr::literal!("");
    str1 = (intString((exps.clone().len() as i32))).clone();
    str2 = stringDelimitList(List::map(exps.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    Error::addSourceMessage(Error::OP_OVERLOAD_MULTIPLE_VALID.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
    Ok(())
}

fn binaryCastConstructor(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut exps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>, mut types: Arc<metamodelica::List<Arc<DAE::Type>>>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut resExps: Arc<metamodelica::List<(Arc<DAE::Exp>, Option<Arc<DAE::Type>>)>> = metamodelica::nil();
    (cache, resExps) = (::match_deref::match_deref! { match &((inCache.clone(), env.clone(), inExp1.clone(), inExp2.clone(), inType1.clone(), inType2.clone(), exps.clone(), types.clone(), info.clone())) {
        (_, _, _, _, _, _, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, _, _) => {
            (inCache.clone(), exps.clone())
        },
        (_, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _) => {
            let mut args: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::FuncArg>>>>> = metamodelica::nil();
            let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut exps1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut exps2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            args = List::map(types.clone(), (std::sync::Arc::new(Types::getFuncArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::FuncArg>>>> + 'static>));
            tys1 = List::mapMap(args.clone(), Arc::new(listHead.clone()), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
            args = List::map(args.clone(), Arc::new(listRest.clone()));
            tys2 = List::mapMap(args.clone(), Arc::new(listHead.clone()), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
            tys1 = List::setDifference(List::union(tys1.clone(), metamodelica::nil()), list![inType1.clone()])?;
            tys2 = List::setDifference(List::union(tys2.clone(), metamodelica::nil()), list![inType2.clone()])?;
            (cache, tys1) = getOperatorFuncsOrEmpty(inCache.clone(), env.clone(), tys1.clone(), (literal!("'constructor'")).clone(), info.clone(), metamodelica::nil())?;
            (cache, tys2) = getOperatorFuncsOrEmpty(cache.clone(), env.clone(), tys2.clone(), (literal!("'constructor'")).clone(), info.clone(), metamodelica::nil())?;
            tys1 = List::select(tys1.clone(), (std::sync::Arc::new(fnptr!(isOperatorUnaryFunction, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>));
            tys2 = List::select(tys2.clone(), (std::sync::Arc::new(fnptr!(isOperatorUnaryFunction, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>));
            exps1 = deoverloadUnaryUserdefNoConstructor(tys1.clone(), inExp1.clone(), inType1.clone(), metamodelica::nil())?;
            exps2 = deoverloadUnaryUserdefNoConstructor(tys2.clone(), inExp2.clone(), inType2.clone(), metamodelica::nil())?;
            resExps = deoverloadBinaryUserdefNoConstructorListLhs(types.clone(), exps1.clone(), inExp2.clone(), inType2.clone(), metamodelica::nil())?;
            resExps = deoverloadBinaryUserdefNoConstructorListRhs(types.clone(), inExp1.clone(), exps2.clone(), inType1.clone(), resExps.clone())?;
            (cache.clone(), resExps.clone())
        },
        _ => {
            errorMultipleValid(List::map(exps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _))), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, resExps))
}

fn getZeroConstructor(mut inCache: FCore::Cache, mut env: FCore::Graph, mut zexps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut info: SourceInfo) -> Result<(FCore::Cache, Option<Arc<Values::Value>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut zeroExpression: Option<Arc<Values::Value>> = None;
    (cache, zeroExpression) = (::match_deref::match_deref! { match &(zexps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inCache.clone(), None)
        },
        Deref @ metamodelica::List::Cons { head: zc, tail: Deref @ metamodelica::List::Nil } => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            (cache, v) = Ceval::ceval(inCache.clone(), env.clone(), zc.clone(), r#impl.clone(), Absyn::Msg::MSG { info: info.clone() }, 0)?;
            (cache.clone(), Some(v.clone()))
        },
        _ => {
            errorMultipleValid(zexps.clone(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, zeroExpression))
}

