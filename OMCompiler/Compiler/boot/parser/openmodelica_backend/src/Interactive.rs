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

use crate::CevalScript;
use crate::GlobalScriptDump;
use crate::InteractiveUtil;
use crate::NFApi;
use crate::Refactor;
use crate::StaticScript;
use crate::SymbolTable;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ConnectionGraph;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InstHashTable;
use openmodelica_frontend::InstUtil;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Parser;
use openmodelica_frontend::Static;
use openmodelica_frontend::Types;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

//public imports
// protected imports
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnnotationType {
    ICON_ANNOTATION,
    DIAGRAM_ANNOTATION,
}
pub use self::AnnotationType::{ICON_ANNOTATION,DIAGRAM_ANNOTATION};

/// Used by buildEnvForGraphicProgram to avoid excessive work.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphicEnvCache {
    GRAPHIC_ENV_NO_CACHE {
        program: Absyn::Program,
        modelPath: Arc<Absyn::Path>,
    },
    GRAPHIC_ENV_PARTIAL_CACHE {
        program: Absyn::Program,
        modelPath: Arc<Absyn::Path>,
        cache: FCore::Cache,
        env: FCore::Graph,
    },
    GRAPHIC_ENV_FULL_CACHE {
        program: Absyn::Program,
        modelPath: Arc<Absyn::Path>,
        cache: FCore::Cache,
        env: FCore::Graph,
    },
}
pub use self::GraphicEnvCache::{GRAPHIC_ENV_NO_CACHE,GRAPHIC_ENV_PARTIAL_CACHE,GRAPHIC_ENV_FULL_CACHE};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Access {
    hide = 1,
    icon = 2,
    documentation = 3,
    diagram = 4,
    nonPackageText = 5,
    nonPackageDuplicate = 6,
    packageText = 7,
    packageDuplicate = 8,
    all = 9,
}
impl PartialOrd for Access {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Access {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn evaluate(mut inStatements: GlobalScript::Statements, mut verbose: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut semicolon: bool = false;
    let mut res: ArcStr = arcstr::literal!("");
    let mut resl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut stmt in &*inStatements.interactiveStmtLst.clone() {
        let mut stmt = stmt.clone();
        semicolon = inStatements.semicolon.clone();
        showStatement(stmt.clone(), semicolon.clone(), true)?;
        res = (evaluate2(stmt.clone())?).clone();
        if getEcho() && (verbose.clone() || !(semicolon.clone())) {
            res = (stringAppend((res.clone()).clone(), (literal!("\n")).clone())).clone();
            resl = cons((res.clone()).clone(), resl.clone());
        }
        showStatement(stmt.clone(), semicolon.clone(), false)?;
    }
    outString = stringAppendList(Dangerous::listReverseInPlace(resl.clone()));
    Ok(outString)
}

pub fn evaluateToStdOut(mut statements: GlobalScript::Statements, mut verbose: bool) -> Result<()> {
    let mut semicolon: bool = false;
    let mut res: ArcStr = arcstr::literal!("");
    semicolon = statements.semicolon.clone();
    for mut stmt in &*statements.interactiveStmtLst.clone() {
        let mut stmt = stmt.clone();
        showStatement(stmt.clone(), semicolon.clone(), true)?;
        res = (evaluate2(stmt.clone())?).clone();
        if getEcho() && (verbose.clone() || !(semicolon.clone())) {
            println!("{}", (res.clone()).clone());
            println!("{}", (literal!("\n")).clone());
        }
        showStatement(stmt.clone(), semicolon.clone(), false)?;
    }
    Ok(())
}

pub fn evaluateFork(mut inTpl: (ArcStr, Arc<SymbolTable::SymbolTable>)) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mosfile, st) => {
                    let mut statements: GlobalScript::Statements = <GlobalScript::Statements as ::std::default::Default>::default();
                    SymbolTable::reset()?;
                    SymbolTable::setAbsyn(st.ast.clone())?;
                    SymbolTable::setSCode(st.explodedAst.clone());
                    openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = None);
                    statements = Parser::parseexp((mosfile.clone()).clone())?;
                    evaluateToStdOut(statements.clone(), true)?;
                    println!("{}", (Error::printMessagesStr(false)).clone());
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (Error::printMessagesStr(false)).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(b)
}

fn showStatement(mut s: GlobalScript::Statement, mut semicolon: bool, mut start: bool) -> Result<()> {
    let mut testsuite: bool = false;
    if !(Flags::isSet(Flags::SHOW_STATEMENT.clone())?) {
        return Ok(());
    }
    testsuite = Testsuite::isRunning()?;
    let _ = 'mc: {
        let __mc_input = (start.clone(), testsuite.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (true, true) = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Evaluating: ")); __mm_s.push_str(&*printIstmtStr(GlobalScript::Statements { interactiveStmtLst: list![s.clone()], semicolon: semicolon.clone() })); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::fflush();
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (false, true) = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (true, false) = __mc_input.clone() else { bail!("nomatch") };
            System::realtimeTick(ClockIndexes::RT_CLOCK_SHOW_STATEMENT.clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Evaluating:   > ")); __mm_s.push_str(&*printIstmtStr(GlobalScript::Statements { interactiveStmtLst: list![s.clone()], semicolon: semicolon.clone() })); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::fflush();
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (false, false) = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Evaluated:    < ")); __mm_s.push_str(&*realString(System::realtimeTock(ClockIndexes::RT_CLOCK_SHOW_STATEMENT.clone())?)); __mm_s.push_str(&*literal!(" / ")); __mm_s.push_str(&*printIstmtStr(GlobalScript::Statements { interactiveStmtLst: list![s.clone()], semicolon: semicolon.clone() })); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::fflush();
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getEcho() -> bool {
    let mut outBoolean: bool = false;
    outBoolean = 0 != Settings::getEcho();
    outBoolean
}

fn evaluate2(mut inStatement: GlobalScript::Statement) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut str_1: ArcStr = arcstr::literal!("");
    let mut algitem: Arc<Absyn::AlgorithmItem>;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    match '__try0: {
        outString = ((::match_deref::match_deref! { match &(inStatement.clone()) {
        GlobalScript::Statement::IALG { algItem: algitem @ Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. } } => {
            unwrap_break_err!(InstHashTable::init(), '__try0);
            unwrap_break_err!(evaluateAlgItem(algitem.clone()), '__try0)
        },
        GlobalScript::Statement::IEXP { info, exp } => {
            unwrap_break_err!(InstHashTable::init(), '__try0);
            evaluateExprToStr(exp.clone(), info.clone())
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok::<_, anyhow::Error>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            r#str = (literal!("")).clone();
            str_1 = (literal!("")).clone();
            GCExt::gcollect();
            r#str = (StackOverflow::getReadableMessage((literal!("\n")).clone())?).clone();
            if Testsuite::isRunning()? {
                Error::clearCurrentComponent()?;
            }
            Error::addMessage(Error::STACK_OVERFLOW_DETAILED.clone(), list![(GlobalScriptDump::printIstmtStr(inStatement.clone())?).clone(), (r#str.clone()).clone()])?;
            Error::clearCurrentComponent()?;
            outString = (literal!("")).clone();
        }
    }
    Ok(outString)
}

fn evaluateAlgItem(mut alg: Arc<Absyn::AlgorithmItem>) -> Result<ArcStr> {
    let mut result: ArcStr = arcstr::literal!("");
    result = ((::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. } => evaluateAlgStmt(var_field!((*alg).algorithm_, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), var_field!((*alg).info, Absyn::AlgorithmItem::ALGORITHMITEM).clone())?,
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(result)
}

fn evaluateAlgStmt(mut alg: Arc<Absyn::Algorithm>, mut info: SourceInfo) -> Result<ArcStr> {
    let mut result: ArcStr = arcstr::literal!("");
    let mut env: FCore::Graph;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut cond: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut msg: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut dcond: Arc<DAE::Exp>;
    let mut dmsg: Arc<DAE::Exp>;
    let mut dexp: Arc<DAE::Exp>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut ident: ArcStr = arcstr::literal!("");
    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut dsubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut prop: DAE::Properties;
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
    let mut startv: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut stepv: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut stopv: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut starte: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut stepe: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut stope: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    result = ('mc: {
        let __mc_input = alg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_NORETCALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: cond, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. } } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut env: FCore::Graph;
                    let mut dcond: Arc<DAE::Exp>;
                    env = SymbolTable::buildEnv()?;
                    (cache, dcond, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), cond.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    ::match_deref::match_deref! { match &(CevalScript::ceval(cache.clone(), env.clone(), dcond.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?) {
                        (_, Deref @ Values::Value::BOOL { boolean: true }) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_NORETCALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: msg, tail: Deref @ metamodelica::List::Nil } }, .. }, functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. } } => {
                    let mut env: FCore::Graph;
                    let mut r#str: ArcStr = r#str.clone();
                    let mut dmsg: Arc<DAE::Exp>;
                    let mut cache: FCore::Cache = cache.clone();
                    env = SymbolTable::buildEnv()?;
                    (cache, dmsg, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), msg.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::ceval(cache.clone(), env.clone(), dmsg.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?) {
                        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_NORETCALL { .. } => {
                    let mut exp: Arc<Absyn::Exp> = exp.clone();
                    let mut dexp: Arc<DAE::Exp>;
                    let mut env: FCore::Graph;
                    let mut cache: FCore::Cache = cache.clone();
                    env = SymbolTable::buildEnv()?;
                    exp = Arc::new(Absyn::Exp::CALL { function_: var_field!((*alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), functionArgs: var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), typeVars: metamodelica::nil() });
                    (cache, dexp, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), exp.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    (_, _) = CevalScript::ceval(cache.clone(), env.clone(), dexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { value: Deref @ Absyn::Exp::CREF { componentRef: cr }, assignComponent: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: ident } } } => {
                    let mut r#str: ArcStr = r#str.clone();
                    let mut value: Arc<Values::Value> = value.clone();
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    value = getVariableValueLst(AbsynUtil::pathToStringList(AbsynUtil::crefToPath(cr.clone())?)?, SymbolTable::getVars())?;
                    r#str = (ValuesDump::valString(value.clone())?).clone();
                    ty = Types::typeOfValue(value.clone())?;
                    SymbolTable::addVar(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), value.clone(), FGraph::empty())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: subs, name: ident } }, .. } => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut value: Arc<Values::Value> = value.clone();
                    let mut env: FCore::Graph;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut ty: Arc<DAE::Type> = ty.clone();
                    let mut r#str: ArcStr = r#str.clone();
                    let mut dsubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = dsubs.clone();
                    env = SymbolTable::buildEnv()?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(StaticScript::elabExp(FCore::emptyCache(), env.clone(), var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: _, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dexp = __pa1.clone();
                    (_, value) = CevalScript::ceval(cache.clone(), env.clone(), dexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    (_, dsubs, _) = Static::elabSubscripts(cache.clone(), env.clone(), subs.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    ty = Types::typeOfValue(value.clone())?;
                    r#str = (ValuesDump::valString(value.clone())?).clone();
                    SymbolTable::addVar(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: ty.clone(), subscriptLst: dsubs.clone() }), value.clone(), env.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::TUPLE { expressions: expl }, .. } => {
                    let mut env: FCore::Graph;
                    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = types.clone();
                    let mut dexp: Arc<DAE::Exp>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = crefs.clone();
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = values.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut prop: DAE::Properties;
                    env = SymbolTable::buildEnv()?;
                    (cache, dexp, prop) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getPropType(prop.clone())?) {
                        Deref @ DAE::Type::T_TUPLE { types: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    types = __pa0.clone();
                    crefs = makeTupleCrefs(expl.clone(), types.clone(), env.clone(), cache.clone(), info.clone())?;
                    let __pa1 = ::match_deref::match_deref! { match &(CevalScript::ceval(cache.clone(), env.clone(), dexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?) {
                        (_, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    values = __pa1.clone();
                    SymbolTable::addVars(crefs.clone(), values.clone(), env.clone())?;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_IF { .. } => {
                    let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = branches.clone();
                    branches = cons((var_field!((*alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone()), var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone());
                    branches = List::appendElt((Arc::new(Absyn::Exp::BOOL { value: true }), var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone()), branches.clone());
                    evaluateIfStatementLst(branches.clone(), info.clone())?;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_WHILE { .. } => {
                    let mut value: Arc<Values::Value> = value.clone();
                    value = evaluateExpr(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), info.clone())?;
                    evaluateWhileStmt(value.clone(), var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), info.clone())?;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(Deref @ Absyn::Exp::RANGE { stop: stope, step: None, start: starte }) }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut startv: Arc<Values::Value> = startv.clone();
                    let mut stopv: Arc<Values::Value> = stopv.clone();
                    startv = evaluateExpr(starte.clone(), info.clone())?;
                    stopv = evaluateExpr(stope.clone(), info.clone())?;
                    evaluateForStmtRangeOpt((ident.clone()).clone(), startv.clone(), Arc::new(Values::Value::INTEGER { integer: 1 }), stopv.clone(), var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone());
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(Deref @ Absyn::Exp::RANGE { stop: stope, step: Some(stepe), start: starte }) }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut startv: Arc<Values::Value> = startv.clone();
                    let mut stepv: Arc<Values::Value> = stepv.clone();
                    let mut stopv: Arc<Values::Value> = stopv.clone();
                    startv = evaluateExpr(starte.clone(), info.clone())?;
                    stepv = evaluateExpr(stepe.clone(), info.clone())?;
                    stopv = evaluateExpr(stope.clone(), info.clone())?;
                    evaluateForStmtRangeOpt((ident.clone()).clone(), startv.clone(), stepv.clone(), stopv.clone(), var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone());
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = values.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(evaluateExpr(exp.clone(), info.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    values = __pa0.clone();
                    evaluateForStmt((ident.clone()).clone(), values.clone(), var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone())?;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { range: Some(exp), .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (stringRepresOfExpr(exp.clone())?).clone();
                    Error::addSourceMessage(Error::NOT_ARRAY_TYPE_IN_FOR_STATEMENT.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(result)
}

fn evaluateForStmt(mut iter: ArcStr, mut valList: Arc<metamodelica::List<Arc<Values::Value>>>, mut algItemList: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<()> {
    for mut val in &*valList.clone() {
        let mut val = val.clone();
        SymbolTable::appendVar((iter.clone()).clone(), val.clone(), Types::typeOfValue(val.clone())?);
        evaluateAlgStmtLst(algItemList.clone())?;
        SymbolTable::deleteVarFirstEntry((iter.clone()).clone())?;
    }
    Ok(())
}

fn evaluateForStmtRangeOpt(mut iter: ArcStr, mut startVal: Arc<Values::Value>, mut stepVal: Arc<Values::Value>, mut stopVal: Arc<Values::Value>, mut algItems: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> () {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = startVal.clone();
    if '__try0: {
        while unwrap_break_err!(ValuesUtil::safeLessEq(val.clone(), stopVal.clone()), '__try0) {
            SymbolTable::appendVar((iter.clone()).clone(), val.clone(), Types::typeOfValue(val.clone()).unwrap());
            unwrap_break_err!(evaluateAlgStmtLst(algItems.clone()), '__try0);
            unwrap_break_err!(SymbolTable::deleteVarFirstEntry((iter.clone()).clone()), '__try0);
            val = unwrap_break_err!(ValuesUtil::safeIntRealOp(val.clone(), stepVal.clone(), openmodelica_frontend_types::Values::IntRealOp::ADDOP), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    ()
}

fn evaluateWhileStmt(mut inValue: Arc<Values::Value>, mut inExp: Arc<Absyn::Exp>, mut inAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut info: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inValue.clone(), inExp.clone(), inAbsynAlgorithmItemLst.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: false }, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: true }, exp, algitemlst, _) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    evaluateAlgStmtLst(algitemlst.clone())?;
                    value = evaluateExpr(exp.clone(), info.clone())?;
                    evaluateWhileStmt(value.clone(), exp.clone(), algitemlst.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: _ }, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (value, exp, _, _) => {
                    let mut estr: ArcStr = arcstr::literal!("");
                    let mut tstr: ArcStr = arcstr::literal!("");
                    let mut vtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    estr = (stringRepresOfExpr(exp.clone())?).clone();
                    vtype = Types::typeOfValue(value.clone())?;
                    tstr = (TypesDump::unparseTypeNoAttr(vtype.clone())?).clone();
                    Error::addSourceMessage(Error::WHILE_CONDITION_TYPE_ERROR.clone(), list![(estr.clone()).clone(), (tstr.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn evaluatePartOfIfStatement(mut inValue: Arc<Values::Value>, mut inExp: Arc<Absyn::Exp>, mut inAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut inTplAbsynExpAbsynAlgorithmItemLstLst: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>, mut info: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inValue.clone(), inExp.clone(), inAbsynAlgorithmItemLst.clone(), inTplAbsynExpAbsynAlgorithmItemLstLst.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: true }, _, algitemlst, _, _) => {
                    evaluateAlgStmtLst(algitemlst.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: false }, _, _, algrest, _) => {
                    evaluateIfStatementLst(algrest.clone(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (value, exp, _, _, _) => {
                    let mut estr: ArcStr = arcstr::literal!("");
                    let mut tstr: ArcStr = arcstr::literal!("");
                    let mut vtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    estr = (stringRepresOfExpr(exp.clone())?).clone();
                    vtype = Types::typeOfValue(value.clone())?;
                    tstr = (TypesDump::unparseTypeNoAttr(vtype.clone())?).clone();
                    Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(estr.clone()).clone(), (tstr.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn evaluateIfStatementLst(mut inTplAbsynExpAbsynAlgorithmItemLstLst: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inTplAbsynExpAbsynAlgorithmItemLstLst.clone(), info.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (exp, algitemlst), tail: algrest }, _) => {
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            value = evaluateExpr(exp.clone(), info.clone())?;
            evaluatePartOfIfStatement(value.clone(), exp.clone(), algitemlst.clone(), algrest.clone(), info.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn evaluateAlgStmtLst(mut inAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<()> {
    for mut algitem in &*inAbsynAlgorithmItemLst.clone() {
        let mut algitem = algitem.clone();
        evaluateAlgItem(algitem.clone())?;
    }
    Ok(())
}

fn evaluateExpr(mut inExp: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inExp.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, _) => {
                    Ok(getVariableValueLst(AbsynUtil::pathToStringList(AbsynUtil::crefToPath(cr.clone())?)?, SymbolTable::getVars())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, _) => {
                    let mut env: FCore::Graph;
                    let mut sexp: Arc<DAE::Exp>;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    env = SymbolTable::buildEnv()?;
                    (cache, sexp, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), exp.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    (_, value) = CevalScript::ceval(cache.clone(), env.clone(), sexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    Ok(value.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn stringRepresOfExpr(mut exp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut estr: ArcStr = arcstr::literal!("");
    let mut env: FCore::Graph;
    let mut sexp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    env = SymbolTable::buildEnv()?;
    (_, sexp, prop) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), exp.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
    (_, sexp, prop) = Ceval::cevalIfConstant(FCore::emptyCache(), env.clone(), sexp.clone(), prop.clone(), true, Absyn::dummyInfo.clone())?;
    estr = (ExpressionBasics::printExpStr(sexp.clone())?).clone();
    Ok(estr)
}

fn evaluateExprToStr(mut inExp: Arc<Absyn::Exp>, mut info: SourceInfo) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    match '__try0: {
        outString = (unwrap_break_err!(ValuesDump::valString(evaluateExpr(inExp.clone(), info.clone()).unwrap()), '__try0)).clone();
        Ok::<_, anyhow::Error>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            outString = (literal!("")).clone();
        }
    }
    outString
}

fn makeTupleCrefs(mut inCrefs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inEnv: FCore::Graph, mut inCache: FCore::Cache, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outCrefs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for (c, t) in (&(inCrefs.clone())).into_iter().zip((&(inTypes.clone())).into_iter()) {
            let __x = makeTupleCref(c.clone(), t.clone(), inEnv.clone(), inCache.clone(), inInfo.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outCrefs)
}

fn makeTupleCref(mut inCref: Arc<Absyn::Exp>, mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph, mut inCache: FCore::Cache, mut inInfo: SourceInfo) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &((inCref.clone(), inType.clone(), inEnv.clone(), inCache.clone(), inInfo.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: asubs } }, _, _, _, _) => {
            let mut dsubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            (_, dsubs, _) = Static::elabSubscripts(inCache.clone(), inEnv.clone(), asubs.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, inInfo.clone())?;
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: inType.clone(), subscriptLst: dsubs.clone() })
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (Dump::printExpStr(inCref.clone())?).clone();
            Error::addMessage(Error::INVALID_TUPLE_CONTENT.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

fn isClassReadOnly(mut cl: Arc<Absyn::Class>) -> Result<bool> {
    let mut readOnly: bool = false;
    readOnly = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { info: SourceInfo { isReadOnly: readOnly, .. }, .. } => readOnly.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(readOnly)
}

pub fn renameComponentOnlyInClass(mut classPath: Arc<Absyn::Path>, mut oldName: Arc<Absyn::ComponentRef>, mut newName: Arc<Absyn::ComponentRef>, mut program: Absyn::Program) -> (Absyn::Program, Arc<Values::Value>) {
    let mut program: Absyn::Program = program;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cl: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut w: Absyn::Within = Absyn::Within::TOP;
    match '__try0: {
        if unwrap_break_err!(isClassReadOnly(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false).unwrap()), '__try0) {
            result = ValuesMake::makeCodeTypeNameStr(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error: class: ")); __mm_s.push_str(&*AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false).unwrap()); __mm_s.push_str(&*literal!(" is in a read only file!")); ArcStr::from(__mm_s) }).clone());
            return (program, result);
        }
        cl = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        cl = unwrap_break_err!(renameComponentInClass(cl.clone(), oldName.clone(), newName.clone()), '__try0);
        w = unwrap_break_err!(InteractiveUtil::buildWithin(AbsynUtil::makeFullyQualified(classPath.clone())), '__try0);
        program = unwrap_break_err!(InteractiveUtil::updateProgram(Absyn::Program { classes: list![cl.clone()], within_: w.clone() }, program.clone(), false), '__try0);
        result = unwrap_break_err!(ValuesMake::makeCodeTypeNameArray(list![classPath.clone()]), '__try0);
        Ok::<_, anyhow::Error>((cl.clone(), program.clone(), result.clone(), w.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cl = __try0_o0;
            program = __try0_o1;
            result = __try0_o2;
            w = __try0_o3;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, result)
}

fn renameComponentInClass(mut cls: Arc<Absyn::Class>, mut oldName: Arc<Absyn::ComponentRef>, mut newName: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut body: Arc<Absyn::ClassDef>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            let mut body = (*body).clone();
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = renameComponentInParts(var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone(), oldName.clone(), newName.clone())?);
            assign_field!(cls.body = body.clone());
            ()
        },
        Deref @ Absyn::Class { body: body @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            let mut body = (*body).clone();
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = renameComponentInParts(var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), oldName.clone(), newName.clone())?);
            assign_field!(cls.body = body.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

fn renameComponentInParts(mut inAbsynClassPartLst1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = 'mc: {
        let __mc_input = (inAbsynClassPartLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elements }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut elements_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    elements_1 = renameComponentInElements(elements.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: elements_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elements }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut elements_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    elements_1 = renameComponentInElements(elements.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: elements_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: equations }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut equations_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(equations.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::EQUATIONS { contents: equations_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: equations }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut equations_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(equations.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::INITIALEQUATIONS { contents: equations_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algorithms }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut algorithms_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    algorithms_1 = renameComponentInAlgorithms(algorithms.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::ALGORITHMS { contents: algorithms_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algorithms }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut algorithms_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    algorithms_1 = renameComponentInAlgorithms(algorithms.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::INITIALALGORITHMS { contents: algorithms_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EXTERNAL { annotation_: ano, externalDecl: external_decl }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut external_decl_1: Arc<Absyn::ExternalDecl> = Arc::new(<Absyn::ExternalDecl as ::std::default::Default>::default());
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    external_decl_1 = renameComponentInExternalDecl(external_decl.clone(), old_comp.clone(), new_comp.clone());
                    Ok(cons(Arc::new(Absyn::ClassPart::EXTERNAL { externalDecl: external_decl_1.clone(), annotation_: ano.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: a, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    res_1 = renameComponentInParts(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(a.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynClassPartLst)
}

fn renameComponentInElements(mut inAbsynElementItemLst1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outAbsynElementItemLst = 'mc: {
        let __mc_input = (inAbsynElementItemLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { constrainClass, info, specification: elementspec, innerOuter: inner_outer, redeclareKeywords: redeclare_, finalPrefix } }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut elementspec_1: Arc<Absyn::ElementSpec>;
                    let mut element_1: Arc<Absyn::ElementItem>;
                    res_1 = renameComponentInElements(res.clone(), old_comp.clone(), new_comp.clone())?;
                    elementspec_1 = renameComponentInElementSpec(elementspec.clone(), old_comp.clone(), new_comp.clone())?;
                    element_1 = Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclare_.clone(), innerOuter: inner_outer.clone(), specification: elementspec_1.clone(), info: info.clone(), constrainClass: constrainClass.clone() }) });
                    Ok(cons(element_1.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: element, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut element_1: Arc<Absyn::ElementItem>;
                    res_1 = renameComponentInElements(res.clone(), old_comp.clone(), new_comp.clone())?;
                    element_1 = element.clone();
                    Ok(cons(element_1.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynElementItemLst)
}

fn renameComponentInElementSpec(mut inElementSpec1: Arc<Absyn::ElementSpec>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outElementSpec: Arc<Absyn::ElementSpec>;
    outElementSpec = 'mc: {
        let __mc_input = (inElementSpec1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, typeSpec: path, attributes: attr }, old_comp, new_comp) => {
                    let mut comps_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    comps_1 = renameComponentInComponentitems(comps.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: path.clone(), components: comps_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElementSpec1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElementSpec)
}

fn renameComponentInComponentitems(mut inAbsynComponentItemLst1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>> {
    let mut outAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    outAbsynComponentItemLst = 'mc: {
        let __mc_input = (inAbsynComponentItemLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { comment, condition: cond, component: Absyn::Component { modification: r#mod, arrayDim, name } }, tail: res }, old_comp, new_comp) => {
                    let mut old_comp_path: Arc<Absyn::Path>;
                    let mut new_comp_path: Arc<Absyn::Path>;
                    let mut old_comp_string: ArcStr = arcstr::literal!("");
                    let mut new_comp_string: ArcStr = arcstr::literal!("");
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut comp_1: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
                    old_comp_path = AbsynUtil::crefToPath(old_comp.clone())?;
                    old_comp_string = (AbsynUtil::pathString(old_comp_path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    let true = (stringEq((name.clone()).clone(), (old_comp_string.clone()).clone())?) else { bail!("pattern mismatch") };
                    new_comp_path = AbsynUtil::crefToPath(new_comp.clone())?;
                    new_comp_string = (AbsynUtil::pathString(new_comp_path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    res_1 = renameComponentInComponentitems(res.clone(), old_comp.clone(), new_comp.clone())?;
                    comp_1 = Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (new_comp_string.clone()).clone(), arrayDim: arrayDim.clone(), modification: r#mod.clone() }, condition: cond.clone(), comment: comment.clone() });
                    Ok(cons(comp_1.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ Absyn::ComponentItem { component: Absyn::Component { .. }, .. }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    res_1 = renameComponentInComponentitems(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(comp.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-Interactive.renameComponentInComponentitems failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynComponentItemLst)
}

fn renameComponentInEquationList(mut inAbsynEquationItemLst1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outAbsynEquationItemLst = 'mc: {
        let __mc_input = (inAbsynEquationItemLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { info, comment: cmt, equation_ }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    let mut equation_1: Arc<Absyn::Equation>;
                    res_1 = renameComponentInEquationList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    equation_1 = renameComponentInEquation(equation_.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: equation_1.clone(), comment: cmt.clone(), info: info.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: equation_item, tail: res }, old_comp, new_comp) => {
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    res_1 = renameComponentInEquationList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(equation_item.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynEquationItemLst)
}

fn renameComponentInExpEquationitemList(mut inTplAbsynExpAbsynEquationItemLstLst1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>> {
    let mut outTplAbsynExpAbsynEquationItemLstLst: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
    outTplAbsynExpAbsynEquationItemLstLst = 'mc: {
        let __mc_input = (inTplAbsynExpAbsynEquationItemLstLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp1, eqn_item), tail: res }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut eqn_item_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    let mut res_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    eqn_item_1 = renameComponentInEquationList(eqn_item.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpEquationitemList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons((exp1_1.clone(), eqn_item_1.clone()), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_exp_equationitem_list failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTplAbsynExpAbsynEquationItemLstLst)
}

fn renameComponentInEquation(mut inEquation1: Arc<Absyn::Equation>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Equation>> {
    let mut outEquation: Arc<Absyn::Equation>;
    outEquation = 'mc: {
        let __mc_input = (inEquation1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_IF { equationElseItems: elses, elseIfBranches: exp_elseifs, equationTrueItems: true_items, ifExp: exp }, old_comp, new_comp) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut true_items_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    let mut elses_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    let mut exp_elseifs_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
                    exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    true_items_1 = renameComponentInEquationList(true_items.clone(), old_comp.clone(), new_comp.clone())?;
                    exp_elseifs_1 = renameComponentInExpEquationitemList(exp_elseifs.clone(), old_comp.clone(), new_comp.clone())?;
                    elses_1 = renameComponentInEquationList(elses.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Equation::EQ_IF { ifExp: exp_1.clone(), equationTrueItems: true_items_1.clone(), elseIfBranches: exp_elseifs_1.clone(), equationElseItems: elses_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_EQUALS { rightSide: exp2, leftSide: exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Equation::EQ_EQUALS { leftSide: exp1_1.clone(), rightSide: exp2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_PDE { domain: cref1, rightSide: exp2, leftSide: exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cref1_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    cref1_1 = replaceStartInComponentRef(cref1.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Equation::EQ_PDE { leftSide: exp1_1.clone(), rightSide: exp2_1.clone(), domain: cref1_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_CONNECT { connector2: cref2, connector1: cref1 }, old_comp, new_comp) => {
                    let mut cref1_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut cref2_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cref1_1 = replaceStartInComponentRef(cref1.clone(), old_comp.clone(), new_comp.clone())?;
                    cref2_1 = replaceStartInComponentRef(cref2.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Equation::EQ_CONNECT { connector1: cref1_1.clone(), connector2: cref2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_FOR { forEquations: equations, iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::List::Nil } }, old_comp, new_comp) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut equations_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(equations.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Equation::EQ_FOR { iterators: list![Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: None, range: Some(exp_1.clone()) })], forEquations: equations_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_WHEN_E { elseWhenEquations: exp_equations, whenEquations: equations, whenExp: exp }, old_comp, new_comp) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut equations_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    let mut exp_equations_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
                    exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(equations.clone(), old_comp.clone(), new_comp.clone())?;
                    exp_equations_1 = renameComponentInExpEquationitemList(exp_equations.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Equation::EQ_WHEN_E { whenExp: exp_1.clone(), whenEquations: equations_1.clone(), elseWhenEquations: exp_equations_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: function_args, functionName: cref }, _, _) => {
                    println!("{}", (literal!("-rename_component_in_equation EQ_NORETCALL not implemented yet\n")).clone());
                    Ok(Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: cref.clone(), functionArgs: function_args.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_equation failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEquation)
}

fn renameComponentInExpList(mut inAbsynExpLst1: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outAbsynExpLst = 'mc: {
        let __mc_input = (inAbsynExpLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: exp, tail: res }, old_comp, new_comp) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(exp_1.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_exp_list failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynExpLst)
}

fn renameComponentInExpListList(mut inAbsynExpLstLst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>> {
    let mut outAbsynExpLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
    outAbsynExpLstLst = 'mc: {
        let __mc_input = (inAbsynExpLstLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: exp, tail: res }, old_comp, new_comp) => {
                    let mut exp_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut res_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
                    exp_1 = renameComponentInExpList(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpListList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(exp_1.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_exp_list_list failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynExpLstLst)
}

fn renameComponentInExpTupleList(mut inTplAbsynExpAbsynExpLst1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>> {
    let mut outTplAbsynExpAbsynExpLst: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
    outTplAbsynExpAbsynExpLst = 'mc: {
        let __mc_input = (inTplAbsynExpAbsynExpLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp1, exp2), tail: res }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut res_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpTupleList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons((exp1_1.clone(), exp2_1.clone()), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_exp_tuple_list failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTplAbsynExpAbsynExpLst)
}

fn renameComponentInElementArgList(mut inAbsynElementArgLst1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outAbsynElementArgLst = 'mc: {
        let __mc_input = (inAbsynElementArgLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: element_arg, tail: res }, old_comp, new_comp) => {
                    let mut element_arg_1: Arc<Absyn::ElementArg>;
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    element_arg_1 = renameComponentInElementArg(element_arg.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInElementArgList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(element_arg_1.clone(), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_element_arg_list failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynElementArgLst)
}

fn renameComponentInElementArg(mut inElementArg1: Arc<Absyn::ElementArg>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ElementArg>> {
    let mut outElementArg: Arc<Absyn::ElementArg>;
    outElementArg = (::match_deref::match_deref! { match &((inElementArg1.clone(), inComponentRef2.clone(), inComponentRef3.clone())) {
        (Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: r#str, modification: Some(Deref @ Absyn::Modification { elementArgLst: element_args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp, info } }), path: p, eachPrefix: each_, finalPrefix: b }, old_comp, new_comp) => {
            let mut p_1: Arc<Absyn::Path>;
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut element_args_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            p_1 = AbsynUtil::crefToPath(replaceStartInComponentRef(AbsynUtil::pathToCref(p.clone())?, old_comp.clone(), new_comp.clone())?)?;
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            element_args_1 = renameComponentInElementArgList(element_args.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: each_.clone(), path: p_1.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: element_args_1.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: exp_1.clone(), info: info.clone() }) })), comment: r#str.clone(), info: mod_info.clone() })
        },
        (Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: r#str, modification: Some(Deref @ Absyn::Modification { elementArgLst: element_args, eqMod: Deref @ Absyn::EqMod::NOMOD }), path: p, eachPrefix: each_, finalPrefix: b }, old_comp, new_comp) => {
            let mut p_1: Arc<Absyn::Path>;
            let mut element_args_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            p_1 = AbsynUtil::crefToPath(replaceStartInComponentRef(AbsynUtil::pathToCref(p.clone())?, old_comp.clone(), new_comp.clone())?)?;
            element_args_1 = renameComponentInElementArgList(element_args.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: each_.clone(), path: p_1.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: element_args_1.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: r#str.clone(), info: mod_info.clone() })
        },
        (Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: r#str, modification: None, path: p, eachPrefix: each_, finalPrefix: b }, old_comp, new_comp) => {
            let mut p_1: Arc<Absyn::Path>;
            p_1 = AbsynUtil::crefToPath(replaceStartInComponentRef(AbsynUtil::pathToCref(p.clone())?, old_comp.clone(), new_comp.clone())?)?;
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: each_.clone(), path: p_1.clone(), modification: None, comment: r#str.clone(), info: mod_info.clone() })
        },
        (Deref @ Absyn::ElementArg::REDECLARATION { info, constrainClass: Some(Deref @ Absyn::ConstrainClass { elementSpec: element_spec2, comment: c }), elementSpec: element_spec, eachPrefix: each_, redeclareKeywords: redecl, finalPrefix: b }, old_comp, new_comp) => {
            let mut element_spec_1: Arc<Absyn::ElementSpec>;
            let mut element_spec2_1: Arc<Absyn::ElementSpec>;
            element_spec_1 = renameComponentInElementSpec(element_spec.clone(), old_comp.clone(), new_comp.clone())?;
            element_spec2_1 = renameComponentInElementSpec(element_spec2.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::ElementArg::REDECLARATION { finalPrefix: b.clone(), redeclareKeywords: redecl.clone(), eachPrefix: each_.clone(), elementSpec: element_spec_1.clone(), constrainClass: Some(Arc::new(Absyn::ConstrainClass { elementSpec: element_spec2_1.clone(), comment: c.clone() })), info: info.clone() })
        },
        (Deref @ Absyn::ElementArg::REDECLARATION { info, constrainClass: None, elementSpec: element_spec, eachPrefix: each_, redeclareKeywords: redecl, finalPrefix: b }, old_comp, new_comp) => {
            let mut element_spec_1: Arc<Absyn::ElementSpec>;
            element_spec_1 = renameComponentInElementSpec(element_spec.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::ElementArg::REDECLARATION { finalPrefix: b.clone(), redeclareKeywords: redecl.clone(), eachPrefix: each_.clone(), elementSpec: element_spec_1.clone(), constrainClass: None, info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementArg)
}

fn renameComponentInCode(mut inCode1: Arc<Absyn::CodeNode>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::CodeNode>> {
    let mut outCode: Arc<Absyn::CodeNode>;
    outCode = (::match_deref::match_deref! { match &((inCode1.clone(), inComponentRef2.clone(), inComponentRef3.clone())) {
        (Deref @ Absyn::CodeNode::C_TYPENAME { path }, _, _) => {
            Arc::new(Absyn::CodeNode::C_TYPENAME { path: path.clone() })
        },
        (Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr }, old_comp, new_comp) => {
            let mut cr_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            cr_1 = replaceStartInComponentRef(cr.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: cr_1.clone() })
        },
        (Deref @ Absyn::CodeNode::C_EQUATIONSECTION { equationItemLst: eqn_items, boolean: b }, old_comp, new_comp) => {
            let mut eqn_items_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            eqn_items_1 = renameComponentInEquationList(eqn_items.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_EQUATIONSECTION { boolean: b.clone(), equationItemLst: eqn_items_1.clone() })
        },
        (Deref @ Absyn::CodeNode::C_ALGORITHMSECTION { algorithmItemLst: algs, boolean: b }, old_comp, new_comp) => {
            let mut algs_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            algs_1 = renameComponentInAlgorithms(algs.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_ALGORITHMSECTION { boolean: b.clone(), algorithmItemLst: algs_1.clone() })
        },
        (Deref @ Absyn::CodeNode::C_ELEMENT { element: Deref @ Absyn::Element::ELEMENT { constrainClass, info, specification: elementspec, innerOuter: inner_outer, redeclareKeywords: redeclare_, finalPrefix } }, old_comp, new_comp) => {
            let mut elementspec_1: Arc<Absyn::ElementSpec>;
            elementspec_1 = renameComponentInElementSpec(elementspec.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_ELEMENT { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclare_.clone(), innerOuter: inner_outer.clone(), specification: elementspec_1.clone(), info: info.clone(), constrainClass: constrainClass.clone() }) })
        },
        (Deref @ Absyn::CodeNode::C_EXPRESSION { exp }, old_comp, new_comp) => {
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: exp_1.clone() })
        },
        (Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, info }, elementArgLst: element_args } }, old_comp, new_comp) => {
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut element_args_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            element_args_1 = renameComponentInElementArgList(element_args.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_MODIFICATION { modification: Arc::new(Absyn::Modification { elementArgLst: element_args_1.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: exp_1.clone(), info: info.clone() }) }) })
        },
        (Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: element_args } }, old_comp, new_comp) => {
            let mut element_args_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            element_args_1 = renameComponentInElementArgList(element_args.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::CodeNode::C_MODIFICATION { modification: Arc::new(Absyn::Modification { elementArgLst: element_args_1.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) }) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCode)
}

fn renameComponentInExp(mut inExp1: Arc<Absyn::Exp>, mut oldPrefix: Arc<Absyn::ComponentRef>, mut newPrefix: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = 'mc: {
        let __mc_input = (inExp1.clone(), oldPrefix.clone(), newPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::REAL { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, old_comp, new_comp) => {
                    let mut cr_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cr_1 = replaceStartInComponentRef(cr.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::STRING { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BOOL { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BINARY { exp2, op, exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::BINARY { exp1: exp1_1.clone(), op: op.clone(), exp2: exp2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { exp, op }, old_comp, new_comp) => {
                    let mut exp = (*exp).clone();
                    exp = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::UNARY { op: op.clone(), exp: exp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LBINARY { exp2, op, exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::LBINARY { exp1: exp1_1.clone(), op: op.clone(), exp2: exp2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LUNARY { exp, op }, old_comp, new_comp) => {
                    let mut exp = (*exp).clone();
                    exp = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::LUNARY { op: op.clone(), exp: exp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp2, op, exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::RELATION { exp1: exp1_1.clone(), op: op.clone(), exp2: exp2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::IFEXP { elseIfBranch: exp_tuple_list, elseBranch: exp3, trueBranch: exp2, ifExp: exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp3_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp_tuple_list_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    exp3_1 = renameComponentInExp(exp3.clone(), old_comp.clone(), new_comp.clone())?;
                    exp_tuple_list_1 = renameComponentInExpTupleList(exp_tuple_list.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::IFEXP { ifExp: exp1_1.clone(), trueBranch: exp2_1.clone(), elseBranch: exp3_1.clone(), elseIfBranch: exp_tuple_list_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CALL { functionArgs: func_args, function_: cref, .. }, old_comp, new_comp) => {
                    let mut func_args = (*func_args).clone();
                    let mut cref = (*cref).clone();
                    cref = replaceStartInComponentRef(cref.clone(), old_comp.clone(), new_comp.clone())?;
                    func_args = renameComponentInFunctionArgs(func_args.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: func_args.clone(), typeVars: var_field!((*inExp1).typeVars, Absyn::Exp::CALL).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::ARRAY { arrayExp: exp_list }, old_comp, new_comp) => {
                    let mut exp_list_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    exp_list_1 = renameComponentInExpList(exp_list.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::ARRAY { arrayExp: exp_list_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::MATRIX { matrix: exp_list_list }, old_comp, new_comp) => {
                    let mut exp_list_list_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
                    exp_list_list_1 = renameComponentInExpListList(exp_list_list.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::MATRIX { matrix: exp_list_list_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RANGE { stop: exp3, step: Some(exp2), start: exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp3_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
                    exp3_1 = renameComponentInExp(exp3.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::RANGE { start: exp1_1.clone(), step: Some(exp2_1.clone()), stop: exp3_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RANGE { stop: exp3, step: None, start: exp1 }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp3_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
                    exp3_1 = renameComponentInExp(exp3.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::RANGE { start: exp1_1.clone(), step: None, stop: exp3_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::TUPLE { expressions: exp_list }, old_comp, new_comp) => {
                    let mut exp_list_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    exp_list_1 = renameComponentInExpList(exp_list.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::TUPLE { expressions: exp_list_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::END, _, _) => {
                    Ok(Arc::new(openmodelica_ast::Absyn::Exp::END))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code }, old_comp, new_comp) => {
                    let mut code_1: Arc<Absyn::CodeNode>;
                    code_1 = renameComponentInCode(code.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::Exp::CODE { code: code_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_exp failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn renameComponentInAlgorithms(mut inAbsynAlgorithmItemLst1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>> {
    let mut outAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    outAbsynAlgorithmItemLst = (::match_deref::match_deref! { match &((inAbsynAlgorithmItemLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: algorithm_, tail: res }, old_comp, new_comp) => {
            let mut res_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut algorithm_1: Arc<Absyn::AlgorithmItem>;
            res_1 = renameComponentInAlgorithms(res.clone(), old_comp.clone(), new_comp.clone())?;
            algorithm_1 = algorithm_.clone();
            cons(algorithm_1.clone(), res_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynAlgorithmItemLst)
}

fn renameComponentInAlgorithm(mut inAlgorithm1: Arc<Absyn::Algorithm>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Algorithm>> {
    let mut outAlgorithm: Arc<Absyn::Algorithm> = Arc::new(Absyn::Algorithm::ALG_BREAK);
    outAlgorithm = (::match_deref::match_deref! { match &((inAlgorithm1.clone(), inComponentRef2.clone(), inComponentRef3.clone())) {
        (Deref @ Absyn::Algorithm::ALG_ASSIGN { value: exp, assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr } }, old_comp, new_comp) => {
            let mut cr_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            cr_1 = replaceStartInComponentRef(cr.clone(), old_comp.clone(), new_comp.clone())?;
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: Arc::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }), value: exp_1.clone() })
        },
        (Deref @ Absyn::Algorithm::ALG_ASSIGN { value: exp2, assignComponent: exp1 @ Deref @ Absyn::Exp::TUPLE { expressions: _ } }, old_comp, new_comp) => {
            let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            exp1_1 = renameComponentInExp(exp1.clone(), old_comp.clone(), new_comp.clone())?;
            exp2_1 = renameComponentInExp(exp2.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: exp1_1.clone(), value: exp2_1.clone() })
        },
        (Deref @ Absyn::Algorithm::ALG_IF { elseBranch: algs2, elseIfAlgorithmBranch: exp_algs_list, trueBranch: algs1, ifExp: exp }, old_comp, new_comp) => {
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut algs1_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut algs2_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut exp_algs_list_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            algs1_1 = renameComponentInAlgorithms(algs1.clone(), old_comp.clone(), new_comp.clone())?;
            exp_algs_list_1 = renameComponentInExpAlgoritmsList(exp_algs_list.clone(), old_comp.clone(), new_comp.clone())?;
            algs2_1 = renameComponentInAlgorithms(algs2.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_IF { ifExp: exp_1.clone(), trueBranch: algs1_1.clone(), elseIfAlgorithmBranch: exp_algs_list_1.clone(), elseBranch: algs2_1.clone() })
        },
        (Deref @ Absyn::Algorithm::ALG_FOR { forBody: algs, iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::List::Nil } }, old_comp, new_comp) => {
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut algs_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            algs_1 = renameComponentInAlgorithms(algs.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_FOR { iterators: list![Arc::new(Absyn::ForIterator { name: (id.clone()).clone(), guardExp: None, range: Some(exp_1.clone()) })], forBody: algs_1.clone() })
        },
        (Deref @ Absyn::Algorithm::ALG_WHILE { whileBody: algs, boolExpr: exp }, old_comp, new_comp) => {
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut algs_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            algs_1 = renameComponentInAlgorithms(algs.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_WHILE { boolExpr: exp_1.clone(), whileBody: algs_1.clone() })
        },
        (Deref @ Absyn::Algorithm::ALG_WHEN_A { elseWhenAlgorithmBranch: exp_algs_list, whenBody: algs, boolExpr: exp }, old_comp, new_comp) => {
            let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut algs_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut exp_algs_list_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
            exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
            algs_1 = renameComponentInAlgorithms(algs.clone(), old_comp.clone(), new_comp.clone())?;
            exp_algs_list_1 = renameComponentInExpAlgoritmsList(exp_algs_list.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: exp_1.clone(), whenBody: algs_1.clone(), elseWhenAlgorithmBranch: exp_algs_list_1.clone() })
        },
        (Deref @ Absyn::Algorithm::ALG_NORETCALL { functionArgs: func_args, functionCall: cr }, old_comp, new_comp) => {
            let mut cr_1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut func_args_1: Arc<Absyn::FunctionArgs>;
            cr_1 = replaceStartInComponentRef(cr.clone(), old_comp.clone(), new_comp.clone())?;
            func_args_1 = renameComponentInFunctionArgs(func_args.clone(), old_comp.clone(), new_comp.clone())?;
            Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cr_1.clone(), functionArgs: func_args_1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAlgorithm)
}

fn renameComponentInExpAlgoritmsList(mut inTplAbsynExpAbsynAlgorithmItemLstLst1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>> {
    let mut outTplAbsynExpAbsynAlgorithmItemLstLst: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
    outTplAbsynExpAbsynAlgorithmItemLstLst = 'mc: {
        let __mc_input = (inTplAbsynExpAbsynAlgorithmItemLstLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp, algs), tail: res }, old_comp, new_comp) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut algs_1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    let mut res_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
                    exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    algs_1 = renameComponentInAlgorithms(algs.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpAlgoritmsList(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons((exp_1.clone(), algs_1.clone()), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_exp_algoritms_list failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTplAbsynExpAbsynAlgorithmItemLstLst)
}

fn renameComponentInFunctionArgs(mut inFunctionArgs1: Arc<Absyn::FunctionArgs>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::FunctionArgs>> {
    let mut outFunctionArgs: Arc<Absyn::FunctionArgs>;
    outFunctionArgs = 'mc: {
        let __mc_input = (inFunctionArgs1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: namedArg, args: exps }, old_comp, new_comp) => {
                    let mut exps_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut namedArg_1: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    exps_1 = renameComponentInExpList(exps.clone(), old_comp.clone(), new_comp.clone())?;
                    namedArg_1 = renameComponentInNamedArgs(namedArg.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: exps_1.clone(), argNames: namedArg_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators }, old_comp, new_comp) => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut iteratorsRenamed: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
                    exp1_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    iteratorsRenamed = renameComponentInIterators(iterators.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: exp1_1.clone(), iterType: iterType.clone(), iterators: iteratorsRenamed.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_function_args failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFunctionArgs)
}

fn renameComponentInIterators(mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut oldComp: Arc<Absyn::ComponentRef>, mut newComp: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ForIterator>>>> {
    let mut iteratorsRenamed: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
    iteratorsRenamed = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut it in (iterators.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(it.clone()) {
        Deref @ Absyn::ForIterator { name: i, guardExp: None, range: Some(exp) } => {
            let mut exp = (*exp).clone();
            exp = renameComponentInExp(exp.clone(), oldComp.clone(), newComp.clone())?;
            Arc::new(Absyn::ForIterator { name: (i.clone()).clone(), guardExp: None, range: Some(exp.clone()) })
        },
        Deref @ Absyn::ForIterator { name: i, guardExp: None, range: None } => {
            Arc::new(Absyn::ForIterator { name: (i.clone()).clone(), guardExp: None, range: None })
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(iteratorsRenamed)
}

fn renameComponentInNamedArgs(mut inAbsynNamedArgLst1: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::NamedArg>>>> {
    let mut outAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    outAbsynNamedArgLst = 'mc: {
        let __mc_input = (inAbsynNamedArgLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argValue: exp, argName: id }, tail: res }, old_comp, new_comp) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut res_1: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    exp_1 = renameComponentInExp(exp.clone(), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInNamedArgs(res.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (id.clone()).clone(), argValue: exp_1.clone() }), res_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("-rename_component_in_namedArgs failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynNamedArgLst)
}

fn renameComponentInExternalDecl(mut external_: Arc<Absyn::ExternalDecl>, mut old_comp: Arc<Absyn::ComponentRef>, mut new_comp: Arc<Absyn::ComponentRef>) -> Arc<Absyn::ExternalDecl> {
    let mut external_1: Arc<Absyn::ExternalDecl> = Arc::new(<Absyn::ExternalDecl as ::std::default::Default>::default());
    println!("{}", (literal!("-rename_component_in_external_decl not implemented yet\n")).clone());
    external_1 = external_.clone();
    external_1
}

fn replaceStartInComponentRef(mut cr1: Arc<Absyn::ComponentRef>, mut cr2: Arc<Absyn::ComponentRef>, mut cr3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut res: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    res = replaceStartInComponentRef2(cr1.clone(), cr2.clone(), cr3.clone())?;
    Ok(res)
}

fn replaceStartInComponentRef2(mut inComponentRef1: Arc<Absyn::ComponentRef>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outComponentRef: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outComponentRef = 'mc: {
        let __mc_input = (inComponentRef1.clone(), inComponentRef2.clone(), inComponentRef3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, .. }, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id2, .. }, res @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }) => {
                    let true = (stringEq((id.clone()).clone(), (id2.clone()).clone())?) else { bail!("pattern mismatch") };
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cr1, subscripts: a, name: id }, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id2, .. }, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id3, .. }) => {
                    let true = (stringEq((id.clone()).clone(), (id2.clone()).clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id3.clone()).clone(), subscripts: a.clone(), componentRef: cr1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cr1, subscripts: a, name: id }, Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cr2, name: id2, .. }, Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cr3, name: id3, .. }) => {
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let true = (stringEq((id.clone()).clone(), (id2.clone()).clone())?) else { bail!("pattern mismatch") };
                    cr = replaceStartInComponentRef2(cr1.clone(), cr2.clone(), cr3.clone())?;
                    Ok(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id3.clone()).clone(), subscripts: a.clone(), componentRef: cr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inComponentRef1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComponentRef)
}

fn isReadOnly(mut file_info: SourceInfo) -> Result<bool> {
    let mut res: bool = false;
    res = (match file_info.clone() {
        SourceInfo { isReadOnly: mut res, .. } => res.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(res)
}

fn isParameterElement(mut inElement: Arc<Absyn::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::PARAM, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn getParameterNames(mut path: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = (path.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut comps: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>>> = metamodelica::nil();
                    let mut compelts_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    comps = InteractiveUtil::getComponentsInClass(cdef.clone(), InteractiveUtil::Visibility::ANY.clone());
                    compelts = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
                    if !(isParameterElement(c.clone())) { continue; }
                    let __x = InteractiveUtil::getComponentitemsInElement(c.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    compelts_1 = List::flatten(compelts.clone());
                    names = List::map(compelts_1.clone(), (std::sync::Arc::new(getComponentitemName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentItem>) -> Result<ArcStr> + 'static>));
                    Ok(names.clone())
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
    Ok(outList)
}

pub fn getClassEnv(mut p: Absyn::Program, mut p_class: Arc<Absyn::Path>) -> Result<GraphicEnvCache> {
    let mut env_2: GraphicEnvCache;
    let mut ocache: Option<Arc<metamodelica::List<(Absyn::Program, Arc<Absyn::Path>, GraphicEnvCache)>>> = None;
    let mut cache: Arc<metamodelica::List<(Absyn::Program, Arc<Absyn::Path>, GraphicEnvCache)>> = metamodelica::nil();
    let mut lcache: Arc<metamodelica::List<(Absyn::Program, Arc<Absyn::Path>, GraphicEnvCache)>> = metamodelica::nil();
    let mut po: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut patho: Arc<Absyn::Path>;
    let mut envo: GraphicEnvCache;
    let mut invalidate: bool = false;
    let mut fcache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph;
    if Flags::isSet(Flags::NF_API.clone())? {
        env_2 = GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { program: p.clone(), modelPath: p_class.clone(), cache: FCore::emptyCache(), env: FGraph::empty() };
        return Ok(env_2);
    }
    ocache = crate::Globals::interactiveCache.with(|__root| __root.borrow().clone());
    if isSome(ocache.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(ocache.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cache = __pa0.clone();
        for mut x in &*cache.clone() {
            let mut x = x.clone();
            (po, patho, envo) = x.clone();
            if AbsynUtil::pathEqual(patho.clone(), p_class.clone()) {
                if referenceEq(&po.clone(),&p.clone()) {
                    env_2 = envo.clone();
                    return Ok(env_2);
                } else {
                    invalidate = true;
                    break;
                }
            }
        }
        if invalidate.clone() {
            let __pa1 = ::match_deref::match_deref! { match &(ocache.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa1.clone();
            (cache, _) = List::deleteMemberOnTrue(p_class.clone(), cache.clone(), (std::sync::Arc::new(fnptr!(matchPath, Arc<Absyn::Path>, (Absyn::Program, Arc<Absyn::Path>, GraphicEnvCache))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, (Absyn::Program, Arc<Absyn::Path>, GraphicEnvCache)) -> Result<bool> + 'static>))?;
            crate::Globals::interactiveCache.with(|__root| *__root.borrow_mut() = Some(cache.clone()));
        }
    }
    (fcache, env) = getClassEnv_dispatch(p.clone(), p_class.clone())?;
    env_2 = GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { program: p.clone(), modelPath: p_class.clone(), cache: fcache.clone(), env: env.clone() };
    ocache = crate::Globals::interactiveCache.with(|__root| __root.borrow().clone());
    if isSome(ocache.clone()) {
        let __pa2 = ::match_deref::match_deref! { match &(ocache.clone()) {
            Some(__pa2) => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cache = __pa2.clone();
        crate::Globals::interactiveCache.with(|__root| *__root.borrow_mut() = Some(cons((p.clone(), p_class.clone(), env_2.clone()), cache.clone())));
    } else {
        crate::Globals::interactiveCache.with(|__root| *__root.borrow_mut() = Some(cons((p.clone(), p_class.clone(), env_2.clone()), metamodelica::nil())));
    }
    Ok(env_2)
}

pub fn matchPath(mut p: Arc<Absyn::Path>, mut entry: (Absyn::Program, Arc<Absyn::Path>, GraphicEnvCache)) -> bool {
    let mut matches: bool = false;
    let mut po: Arc<Absyn::Path>;
    (_, po, _) = entry.clone();
    matches = AbsynUtil::pathEqual(po.clone(), p.clone());
    matches
}

fn getClassEnv_dispatch(mut p: Absyn::Program, mut p_class: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env_2: FCore::Graph;
    let mut p_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut env: FCore::Graph;
    let mut env_1: FCore::Graph;
    let mut env2: FCore::Graph;
    let mut cl: Arc<SCode::Element>;
    let mut id: ArcStr = arcstr::literal!("");
    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut ci_state: ClassInf::State;
    p_1 = AbsynToSCode::translateAbsyn2SCode(p.clone())?;
    (cache, env) = Inst::makeEnvFromProgram(p_1.clone())?;
    (cache, cl, env_1) = Lookup::lookupClass(cache.clone(), env.clone(), p_class.clone(), None)?;
    env_2 = 'mc: {
        let __mc_input = cl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, restriction: restr, encapsulatedPrefix: encflag, name: id, .. } => {
                    Ok(env_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: restr, encapsulatedPrefix: encflag, name: id, .. } => {
                    let mut ci_state: ClassInf::State;
                    let mut env2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut cache: FCore::Cache = cache.clone();
                    env2 = FGraph::openScope(env_1.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
                    ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env2.clone())?)?;
                    (cache, env_2, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), cl.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    Ok(env_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(FGraph::empty())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, env_2))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentProperties {
    pub isFinal: bool,
    pub isFlow: bool,
    pub isStream: bool,
    pub isProtected: bool,
    pub isReplaceable: bool,
    pub variability: Absyn::Variability,
    pub innerOuter: Absyn::InnerOuter,
    pub direction: Absyn::Direction,
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            isFinal: Default::default(),
            isFlow: Default::default(),
            isStream: Default::default(),
            isProtected: Default::default(),
            isReplaceable: Default::default(),
            variability: Default::default(),
            innerOuter: Default::default(),
            direction: Default::default(),
        }
    }
}

pub type PROPERTIES = ComponentProperties;


pub fn setComponentProperties(mut classPath: Arc<Absyn::Path>, mut component: ArcStr, mut prefixes: Arc<metamodelica::List<bool>>, mut variability: ArcStr, mut innerPrefix: bool, mut outerPrefix: bool, mut direction: ArcStr, mut program: Absyn::Program) -> (Absyn::Program, Arc<Values::Value>) {
    let mut program: Absyn::Program = program;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut is_final: bool = false;
    let mut is_flow: bool = false;
    let mut is_stream: bool = false;
    let mut is_protected: bool = false;
    let mut is_replaceable: bool = false;
    let mut var: Absyn::Variability = Absyn::Variability::CONST;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut props: ComponentProperties = <ComponentProperties as ::std::default::Default>::default();
    match '__try0: {
        if (prefixes.clone().len() as i32) == 5 {
            let (__pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(prefixes.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } } } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            is_final = __pa1.clone();
            is_flow = __pa2.clone();
            is_stream = __pa3.clone();
            is_protected = __pa4.clone();
            is_replaceable = __pa5.clone();
            let false = (is_flow.clone() && is_stream.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        } else {
            let (__pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(prefixes.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Nil } } } } => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            is_final = __pa7.clone();
            is_flow = __pa8.clone();
            is_protected = __pa9.clone();
            is_replaceable = __pa10.clone();
            is_stream = false;
        }
        props = ComponentProperties { isFinal: is_final.clone(), isFlow: is_flow.clone(), isStream: is_stream.clone(), isProtected: is_protected.clone(), isReplaceable: is_replaceable.clone(), variability: unwrap_break_err!(setElementVariability((variability.clone()).clone()), '__try0), innerOuter: setInnerOuterAttributes(innerPrefix.clone(), outerPrefix.clone()), direction: unwrap_break_err!(setElementCausality((direction.clone()).clone()), '__try0) };
        program = unwrap_break_err!(transformPathedClassInProgram(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = (component.clone()).clone(); let __pe_b2 = props.clone(); move |__pe_a0| setComponentPropertiesInClass(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })), '__try0);
        result = ValuesMake::makeBoolean(true);
        Ok::<_, anyhow::Error>((is_final.clone(), is_flow.clone(), is_protected.clone(), is_replaceable.clone(), is_stream.clone(), program.clone(), props.clone(), result.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
            is_final = __try0_o0;
            is_flow = __try0_o1;
            is_protected = __try0_o2;
            is_replaceable = __try0_o3;
            is_stream = __try0_o4;
            program = __try0_o5;
            props = __try0_o6;
            result = __try0_o7;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, result)
}

fn setComponentPropertiesInClass(mut cls: Arc<Absyn::Class>, mut component: ArcStr, mut properties: ComponentProperties) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut body: Arc<Absyn::ClassDef>;
    body = cls.body.clone();
    assign_field!(cls.body = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = setComponentPropertiesInClassparts(var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone(), (component.clone()).clone(), properties.clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = setComponentPropertiesInClassparts(var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), (component.clone()).clone(), properties.clone())?);
            body.clone()
        },
        _ => bail!("match: no arm matched"),
    } }));
    Ok(cls)
}

fn setComponentPropertiesInClassparts(mut inParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut component: ArcStr, mut properties: ComponentProperties) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outParts = 'mc: {
        let __mc_input = inParts.clone();
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
                parts => {
                    if !((properties.isProtected.clone())) { bail!("guard") }
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut protlst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut elt: Arc<Absyn::Element>;
                    let mut parts = (*parts).clone();
                    publst = InteractiveUtil::getPublicList(parts.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue((component.clone()).clone(), publst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isElementItemNamed, ArcStr, Arc<Absyn::ElementItem>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::ElementItem>) -> Result<bool> + 'static>))?) {
                        Deref @ Absyn::ElementItem::ELEMENTITEM { element: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    elt = __pa0.clone();
                    elt = setComponentPropertiesInElement(elt.clone(), (component.clone()).clone(), properties.clone())?;
                    (publst, _) = deleteOrUpdateComponentFromElementitems((component.clone()).clone(), publst.clone(), None)?;
                    protlst = InteractiveUtil::getProtectedList(parts.clone());
                    protlst = List::appendElt(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), protlst.clone());
                    parts = InteractiveUtil::replaceProtectedList(parts.clone(), protlst.clone())?;
                    parts = InteractiveUtil::replacePublicList(parts.clone(), publst.clone())?;
                    Ok(parts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                parts => {
                    if !((!(properties.isProtected.clone()))) { bail!("guard") }
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut protlst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut elt: Arc<Absyn::Element>;
                    let mut parts = (*parts).clone();
                    protlst = InteractiveUtil::getProtectedList(parts.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue((component.clone()).clone(), protlst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isElementItemNamed, ArcStr, Arc<Absyn::ElementItem>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::ElementItem>) -> Result<bool> + 'static>))?) {
                        Deref @ Absyn::ElementItem::ELEMENTITEM { element: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    elt = __pa0.clone();
                    elt = setComponentPropertiesInElement(elt.clone(), (component.clone()).clone(), properties.clone())?;
                    (protlst, _) = deleteOrUpdateComponentFromElementitems((component.clone()).clone(), protlst.clone(), None)?;
                    publst = InteractiveUtil::getPublicList(parts.clone());
                    publst = List::appendElt(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), publst.clone());
                    parts = InteractiveUtil::replacePublicList(parts.clone(), publst.clone())?;
                    parts = InteractiveUtil::replaceProtectedList(parts.clone(), protlst.clone())?;
                    Ok(parts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest } => {
                    let mut elts = (*elts).clone();
                    let mut rest = (*rest).clone();
                    rest = setComponentPropertiesInClassparts(rest.clone(), (component.clone()).clone(), properties.clone())?;
                    elts = setComponentPropertiesInElementitems(elts.clone(), (component.clone()).clone(), properties.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: elts.clone() }), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest } => {
                    let mut elts = (*elts).clone();
                    let mut rest = (*rest).clone();
                    rest = setComponentPropertiesInClassparts(rest.clone(), (component.clone()).clone(), properties.clone())?;
                    elts = setComponentPropertiesInElementitems(elts.clone(), (component.clone()).clone(), properties.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: elts.clone() }), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: part, tail: rest } => {
                    let mut rest = (*rest).clone();
                    rest = setComponentPropertiesInClassparts(rest.clone(), (component.clone()).clone(), properties.clone())?;
                    Ok(cons(part.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outParts)
}

fn setComponentPropertiesInElementitems(mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut component: ArcStr, mut properties: ComponentProperties) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = items;
    (items, _) = List::findAndMap(items.clone(), Arc::new({ let __pe_b0 = (component.clone()).clone(); move |__pe_a1| Ok(AbsynUtil::isElementItemNamed(__pe_b0.clone(), __pe_a1)) }), Arc::new({ let __pe_b1 = (component.clone()).clone(); let __pe_b2 = properties.clone(); move |__pe_a0| setComponentPropertiesInElementItem(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    Ok(items)
}

fn setComponentPropertiesInElementItem(mut item: Arc<Absyn::ElementItem>, mut component: ArcStr, mut properties: ComponentProperties) -> Result<Arc<Absyn::ElementItem>> {
    let mut item: Arc<Absyn::ElementItem> = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = setComponentPropertiesInElement(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone(), (component.clone()).clone(), properties.clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(item)
}

fn setComponentPropertiesInElement(mut element: Arc<Absyn::Element>, mut component: ArcStr, mut properties: ComponentProperties) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let mut spec: Arc<Absyn::ElementSpec>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => {
            let mut spec = (*spec).clone();
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                finalPrefix = properties.isFinal.clone(),
                redeclareKeywords = setReplaceableKeywordAttributes(var_field!((*element).redeclareKeywords, Absyn::Element::ELEMENT).clone(), properties.isReplaceable.clone())?,
                innerOuter = properties.innerOuter.clone()
            );
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; attributes = setElementAttributes(var_field!((*spec).attributes, Absyn::ElementSpec::COMPONENTS).clone(), properties.clone()));
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

fn setReplaceableKeywordAttributes(mut inAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>, mut inBoolean: bool) -> Result<Option<Absyn::RedeclareKeywords>> {
    let mut outAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords> = None;
    outAbsynRedeclareKeywordsOption = (match (inAbsynRedeclareKeywordsOption.clone(), inBoolean.clone()) {
        (None, false) => None,
        (Some(Absyn::RedeclareKeywords::REPLACEABLE), false) => None,
        (Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE), false) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE),
        (Some(Absyn::RedeclareKeywords::REDECLARE), false) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE),
        (None, true) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE),
        (Some(Absyn::RedeclareKeywords::REDECLARE), true) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE),
        (Some(Absyn::RedeclareKeywords::REPLACEABLE), true) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE),
        (Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE), true) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE),
        _ => bail!("match: no arm matched"),
    });
    Ok(outAbsynRedeclareKeywordsOption)
}

fn setInnerOuterAttributes(mut isInner: bool, mut isOuter: bool) -> Absyn::InnerOuter {
    let mut outInnerOuter: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    outInnerOuter = (match (isInner.clone(), isOuter.clone()) {
        (false, false) => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        (true, false) => openmodelica_ast::Absyn::InnerOuter::INNER,
        (false, true) => openmodelica_ast::Absyn::InnerOuter::OUTER,
        _ => openmodelica_ast::Absyn::InnerOuter::INNER_OUTER,
    });
    outInnerOuter
}

fn setElementVariability(mut inString: ArcStr) -> Result<Absyn::Variability> {
    let mut outVariability: Absyn::Variability = Absyn::Variability::CONST;
    outVariability = (::match_deref::match_deref! { match &(inString.clone()) {
        Deref @ "" => openmodelica_ast::Absyn::Variability::VAR,
        Deref @ "discrete" => openmodelica_ast::Absyn::Variability::DISCRETE,
        Deref @ "parameter" => openmodelica_ast::Absyn::Variability::PARAM,
        Deref @ "constant" => openmodelica_ast::Absyn::Variability::CONST,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVariability)
}

fn setElementCausality(mut inString: ArcStr) -> Result<Absyn::Direction> {
    let mut outDirection: Absyn::Direction = Absyn::Direction::BIDIR;
    outDirection = (::match_deref::match_deref! { match &(inString.clone()) {
        Deref @ "" => openmodelica_ast::Absyn::Direction::BIDIR,
        Deref @ "input" => openmodelica_ast::Absyn::Direction::INPUT,
        Deref @ "output" => openmodelica_ast::Absyn::Direction::OUTPUT,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDirection)
}

fn setElementAttributes(mut attributes: Absyn::ElementAttributes, mut properties: ComponentProperties) -> Absyn::ElementAttributes {
    let mut attributes: Absyn::ElementAttributes = attributes;
    attributes = Absyn::ElementAttributes { flowPrefix: properties.isFlow.clone(), streamPrefix: properties.isStream.clone(), parallelism: attributes.parallelism.clone(), variability: properties.variability.clone(), direction: properties.direction.clone(), isField: attributes.isField.clone(), arrayDim: attributes.arrayDim.clone() };
    attributes
}

pub fn getCrefInfo(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        info = cls.info.clone();
        result = unwrap_break_err!(ValuesMake::makeArray(list![ValuesMake::makeCodeTypeNameStr((Testsuite::friendly(info.fileName.clone()).unwrap()).clone()), ValuesMake::makeCodeTypeNameStr((if (info.isReadOnly.clone()) {literal!("readonly")} else {literal!("writable")}).clone()), ValuesMake::makeInteger(info.lineNumberStart.clone()), ValuesMake::makeInteger(info.columnNumberStart.clone()), ValuesMake::makeInteger(info.lineNumberEnd.clone()), ValuesMake::makeInteger(info.columnNumberEnd.clone())]), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

fn getImportString(mut inImport: Absyn::Import) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { path: mut path, name: mut id } => {
            let mut path_str: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            r#str = stringAppendList(list![(literal!("kind=named, id=")).clone(), (id.clone()).clone(), (literal!(", path=")).clone(), (path_str.clone()).clone()]);
            r#str.clone()
        },
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut path_str: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            r#str = stringAppendList(list![(literal!("kind=qualified, path=")).clone(), (path_str.clone()).clone()]);
            r#str.clone()
        },
        Absyn::Import::UNQUAL_IMPORT { path: mut path } => {
            let mut path_str: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            r#str = stringAppendList(list![(literal!("kind=unqualified, path=")).clone(), (path_str.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn getElementType(mut inElementSpec: Arc<Absyn::ElementSpec>, mut inElement: Arc<Absyn::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElementSpec.clone()) {
        Deref @ Absyn::ElementSpec::EXTENDS { path, .. } => {
            let mut path_str: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            r#str = stringAppendList(list![(literal!("elementtype=extends, path=")).clone(), (path_str.clone()).clone()]);
            r#str.clone()
        },
        Deref @ Absyn::ElementSpec::IMPORT { import_, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut import_str: ArcStr = arcstr::literal!("");
            import_str = (getImportString(import_.clone())?).clone();
            r#str = stringAppendList(list![(literal!("elementtype=import, ")).clone(), (import_str.clone()).clone()]);
            r#str.clone()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { components: lst, typeSpec, attributes: attr } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut typename: ArcStr = arcstr::literal!("");
            let mut flowPrefixstr: ArcStr = arcstr::literal!("");
            let mut streamPrefixstr: ArcStr = arcstr::literal!("");
            let mut variability_str: ArcStr = arcstr::literal!("");
            let mut dir_str: ArcStr = arcstr::literal!("");
            let mut names_str: ArcStr = arcstr::literal!("");
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            typename = (Dump::unparseTypeSpec(typeSpec.clone())?).clone();
            let __pa0 = ::match_deref::match_deref! { match &(InteractiveUtil::getComponentItemsNameAndComment(lst.clone(), inElement.clone())) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            names = __pa0.clone();
            flowPrefixstr = (InteractiveUtil::attrFlowStr(attr.clone())?).clone();
            streamPrefixstr = (InteractiveUtil::attrStreamStr(attr.clone())?).clone();
            variability_str = (InteractiveUtil::attrVariabilityStr(attr.clone())?).clone();
            dir_str = (InteractiveUtil::attrDirectionStr(attr.clone())?).clone();
            names_str = stringDelimitList(names.clone(), (literal!(", ")).clone());
            r#str = stringAppendList(list![(literal!("elementtype=component, typename=")).clone(), (typename.clone()).clone(), (literal!(", names={")).clone(), (names_str.clone()).clone(), (literal!("}, flow=")).clone(), (flowPrefixstr.clone()).clone(), (literal!(", stream=")).clone(), (streamPrefixstr.clone()).clone(), (literal!(", variability=\"")).clone(), (variability_str.clone()).clone(), (literal!("\", direction=\"")).clone(), (dir_str.clone()).clone(), (literal!("\"")).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getElementInfo(mut inElementItem: Arc<Absyn::ElementItem>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inElementItem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { info: SourceInfo { columnNumberEnd: ecol, lineNumberEnd: eline, columnNumberStart: scol, lineNumberStart: sline, isReadOnly, fileName: file, .. }, restriction: restr, name: id, .. }, .. }, innerOuter: inout, redeclareKeywords: r, finalPrefix: f, .. } } => {
                    let mut finalPrefix: ArcStr = arcstr::literal!("");
                    let mut repl: ArcStr = arcstr::literal!("");
                    let mut inout_str: ArcStr = arcstr::literal!("");
                    let mut str_restriction: ArcStr = arcstr::literal!("");
                    let mut element_str: ArcStr = arcstr::literal!("");
                    let mut sline_str: ArcStr = arcstr::literal!("");
                    let mut scol_str: ArcStr = arcstr::literal!("");
                    let mut eline_str: ArcStr = arcstr::literal!("");
                    let mut ecol_str: ArcStr = arcstr::literal!("");
                    let mut readonly_str: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut r_1: bool = false;
                    let mut file = (*file).clone();
                    finalPrefix = (boolString(f.clone())).clone();
                    r_1 = keywordReplaceable(r.clone());
                    repl = (boolString(r_1.clone())).clone();
                    inout_str = (InteractiveUtil::innerOuterStr(inout.clone())?).clone();
                    str_restriction = (AbsynUtil::restrString(restr.clone())).clone();
                    element_str = stringAppendList(list![(literal!("elementtype=classdef, classname=")).clone(), (id.clone()).clone(), (literal!(", classrestriction=")).clone(), (str_restriction.clone()).clone()]);
                    file = (Testsuite::friendly((file.clone()).clone())?).clone();
                    sline_str = (intString(sline.clone())).clone();
                    scol_str = (intString(scol.clone())).clone();
                    eline_str = (intString(eline.clone())).clone();
                    ecol_str = (intString(ecol.clone())).clone();
                    readonly_str = (if (isReadOnly.clone()) {literal!("readonly")} else {literal!("writable")}).clone();
                    r#str = stringAppendList(list![(literal!("elementfile=\"")).clone(), (file.clone()).clone(), (literal!("\", elementreadonly=\"")).clone(), (readonly_str.clone()).clone(), (literal!("\", elementStartLine=")).clone(), (sline_str.clone()).clone(), (literal!(", elementStartColumn=")).clone(), (scol_str.clone()).clone(), (literal!(", elementEndLine=")).clone(), (eline_str.clone()).clone(), (literal!(", elementEndColumn=")).clone(), (ecol_str.clone()).clone(), (literal!(", final=")).clone(), (finalPrefix.clone()).clone(), (literal!(", replaceable=")).clone(), (repl.clone()).clone(), (literal!(", inout=\"")).clone(), (inout_str.clone()).clone(), (literal!("\", ")).clone(), (element_str.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: el @ Deref @ Absyn::Element::ELEMENT { info: SourceInfo { columnNumberEnd: ecol, lineNumberEnd: eline, columnNumberStart: scol, lineNumberStart: sline, isReadOnly, fileName: file, .. }, specification: elementSpec, innerOuter: inout, redeclareKeywords: r, finalPrefix: f, .. } } => {
                    let mut finalPrefix: ArcStr = arcstr::literal!("");
                    let mut repl: ArcStr = arcstr::literal!("");
                    let mut inout_str: ArcStr = arcstr::literal!("");
                    let mut element_str: ArcStr = arcstr::literal!("");
                    let mut sline_str: ArcStr = arcstr::literal!("");
                    let mut scol_str: ArcStr = arcstr::literal!("");
                    let mut eline_str: ArcStr = arcstr::literal!("");
                    let mut ecol_str: ArcStr = arcstr::literal!("");
                    let mut readonly_str: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut r_1: bool = false;
                    let mut file = (*file).clone();
                    finalPrefix = (boolString(f.clone())).clone();
                    r_1 = keywordReplaceable(r.clone());
                    repl = (boolString(r_1.clone())).clone();
                    inout_str = (InteractiveUtil::innerOuterStr(inout.clone())?).clone();
                    element_str = (getElementType(elementSpec.clone(), el.clone())?).clone();
                    sline_str = (intString(sline.clone())).clone();
                    scol_str = (intString(scol.clone())).clone();
                    eline_str = (intString(eline.clone())).clone();
                    ecol_str = (intString(ecol.clone())).clone();
                    readonly_str = (if (isReadOnly.clone()) {literal!("readonly")} else {literal!("writable")}).clone();
                    file = (Testsuite::friendly((file.clone()).clone())?).clone();
                    r#str = stringAppendList(list![(literal!("elementfile=\"")).clone(), (file.clone()).clone(), (literal!("\", elementreadonly=\"")).clone(), (readonly_str.clone()).clone(), (literal!("\", elementStartLine=")).clone(), (sline_str.clone()).clone(), (literal!(", elementStartColumn=")).clone(), (scol_str.clone()).clone(), (literal!(", elementEndLine=")).clone(), (eline_str.clone()).clone(), (literal!(", elementEndColumn=")).clone(), (ecol_str.clone()).clone(), (literal!(", final=")).clone(), (finalPrefix.clone()).clone(), (literal!(", replaceable=")).clone(), (repl.clone()).clone(), (literal!(", inout=\"")).clone(), (inout_str.clone()).clone(), (literal!("\", ")).clone(), (element_str.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::LEXER_COMMENT { .. } => {
                    Ok(literal!("elementtype=comment"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("elementtype=annotation"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn constructElementsInfo(mut visibility: ArcStr, mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<ArcStr> {
    let mut result: ArcStr = arcstr::literal!("");
    let mut elements_strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut element_str: ArcStr = arcstr::literal!("");
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        element_str = (getElementInfo(e.clone())?).clone();
        element_str = stringAppendList(list![(literal!("{ rec(elementvisibility=")).clone(), (visibility.clone()).clone(), (literal!(", ")).clone(), (element_str.clone()).clone(), (literal!(") }")).clone()]);
        elements_strl = cons((element_str.clone()).clone(), elements_strl.clone());
    }
    elements_strl = Dangerous::listReverseInPlace(elements_strl.clone());
    result = stringDelimitList(elements_strl.clone(), (literal!(",\n")).clone());
    if !(elements.clone().is_empty()) {
        result = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*result.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(result)
}

fn appendNonEmptyStrings(mut str1: ArcStr, mut str2: ArcStr, mut delim: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    if stringEmpty((str1.clone()).clone()) {
        outString = (str2.clone()).clone();
    } else if stringEmpty((str2.clone()).clone()) {
        outString = (str1.clone()).clone();
    } else {
        outString = stringAppendList(list![(str1.clone()).clone(), (delim.clone()).clone(), (str2.clone()).clone()]);
    }
    outString
}

pub fn getElementsInfo(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut result_str: ArcStr = arcstr::literal!("");
    let mut public_str: ArcStr = arcstr::literal!("");
    let mut protected_str: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut public_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut protected_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        parts = AbsynUtil::getClassPartsInClass(cls.clone());
        public_str = (unwrap_break_err!(constructElementsInfo((literal!("public")).clone(), InteractiveUtil::getPublicList(parts.clone())), '__try0)).clone();
        protected_str = (unwrap_break_err!(constructElementsInfo((literal!("protected")).clone(), InteractiveUtil::getProtectedList(parts.clone())), '__try0)).clone();
        result_str = (appendNonEmptyStrings((public_str.clone()).clone(), (protected_str.clone()).clone(), (literal!(", ")).clone())).clone();
        result_str = stringAppendList(list![(literal!("{ ")).clone(), (result_str.clone()).clone(), (literal!(" }")).clone()]);
        Ok::<_, anyhow::Error>((result_str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result_str = __try0_o0;
        }
        Err(_) => {
            result_str = (literal!("Error")).clone();
        }
    }
    result = ValuesMake::makeCodeTypeNameStr((result_str.clone()).clone());
    result
}

pub fn getSourceFile(mut p_class: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (p_class.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut filename: ArcStr = arcstr::literal!("");
                    cdef = InteractiveUtil::getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    filename = (AbsynUtil::classFilename(cdef.clone())?).clone();
                    Ok(filename.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn setSourceFile(mut path: Arc<Absyn::Path>, mut inString: ArcStr, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut success: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (success, outProgram) = 'mc: {
        let __mc_input = (path.clone(), inString.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, filename, p @ Absyn::Program { .. }) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut cdef_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut within_: Absyn::Within = Absyn::Within::TOP;
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    within_ = InteractiveUtil::buildWithin(path.clone())?;
                    cdef_1 = AbsynUtil::setClassFilename(cdef.clone(), (filename.clone()).clone())?;
                    newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cdef_1.clone()], within_: within_.clone() }, p.clone(), false)?;
                    Ok((true, newp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, inProgram.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((success, outProgram))
}

pub fn removeExtendsModifiers(mut inClassPath: Arc<Absyn::Path>, mut inBaseClassPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut keepRedeclares: bool) -> Result<(Absyn::Program, bool)> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut outResult: bool = false;
    (outProgram, outResult) = 'mc: {
        let __mc_input = (inClassPath.clone(), inBaseClassPath.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, inherit_class, p @ Absyn::Program { .. }) => {
                    let mut within_: Absyn::Within = Absyn::Within::TOP;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut cdef_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut env: GraphicEnvCache;
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    within_ = InteractiveUtil::buildWithin(p_class.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    env = getClassEnv(p.clone(), p_class.clone())?;
                    cdef_1 = removeExtendsModifiersInClass(cdef.clone(), inherit_class.clone(), env.clone(), keepRedeclares.clone())?;
                    newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cdef_1.clone()], within_: within_.clone() }, p.clone(), false)?;
                    Ok((newp.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inProgram.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outProgram, outResult))
}

fn removeExtendsModifiersInClass(mut inClass: Arc<Absyn::Class>, mut inPath: Arc<Absyn::Path>, mut inEnv: GraphicEnvCache, mut keepRedeclares: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &((inClass.clone(), inPath.clone(), inEnv.clone())) {
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }, inherit_name, env) => {
            let mut parts_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            parts_1 = removeExtendsModifiersInClassparts(parts.clone(), inherit_name.clone(), env.clone(), keepRedeclares.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts_1.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, modifications: modif, parts, baseClassName: bcname }, .. }, inherit_name, env) => {
            let mut parts_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            parts_1 = removeExtendsModifiersInClassparts(parts.clone(), inherit_name.clone(), env.clone(), keepRedeclares.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts_1.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn removeExtendsModifiersInClassparts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inPath: Arc<Absyn::Path>, mut inEnv: GraphicEnvCache, mut keepRedeclares: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = 'mc: {
        let __mc_input = (inAbsynClassPartLst.clone(), inPath.clone(), inEnv.clone(), keepRedeclares.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest }, inherit, env, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut elts_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    res = removeExtendsModifiersInClassparts(rest.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    elts_1 = removeExtendsModifiersInElementitems(elts.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: elts_1.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest }, inherit, env, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut elts_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    res = removeExtendsModifiersInClassparts(rest.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    elts_1 = removeExtendsModifiersInElementitems(elts.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    Ok(cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: elts_1.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: elt, tail: rest }, inherit, env, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    res = removeExtendsModifiersInClassparts(rest.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    Ok(cons(elt.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynClassPartLst)
}

fn removeExtendsModifiersInElementitems(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inPath: Arc<Absyn::Path>, mut inEnv: GraphicEnvCache, mut keepRedeclares: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outAbsynElementItemLst = 'mc: {
        let __mc_input = (inAbsynElementItemLst.clone(), inPath.clone(), inEnv.clone(), keepRedeclares.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt }, tail: rest }, inherit, env, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut elt_1: Arc<Absyn::Element>;
                    res = removeExtendsModifiersInElementitems(rest.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    elt_1 = removeExtendsModifiersInElement(elt.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt_1.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: elitem, tail: rest }, inherit, env, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    res = removeExtendsModifiersInElementitems(rest.clone(), inherit.clone(), env.clone(), keepRedeclares.clone())?;
                    Ok(cons(elitem.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynElementItemLst)
}

fn removeExtendsModifiersInElement(mut inElement: Arc<Absyn::Element>, mut inPath: Arc<Absyn::Path>, mut inEnv: GraphicEnvCache, mut keepRedeclares: bool) -> Result<Arc<Absyn::Element>> {
    let mut outElement: Arc<Absyn::Element>;
    outElement = 'mc: {
        let __mc_input = (inElement.clone(), inPath.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Element::ELEMENT { constrainClass: constr, info, specification: Deref @ Absyn::ElementSpec::EXTENDS { annotationOpt: annOpt, elementArg: eargs, path }, innerOuter: i, redeclareKeywords: r, finalPrefix: f }, inherit, env) => {
                    let mut path_1: Arc<Absyn::Path>;
                    let mut eargs = (*eargs).clone();
                    (_, path_1) = mkFullyQual(env.clone(), path.clone(), false)?;
                    let true = (AbsynUtil::pathEqual(inherit.clone(), path_1.clone())) else { bail!("pattern mismatch") };
                    eargs = if (!(keepRedeclares.clone())) {metamodelica::nil()} else {{
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (eargs.clone()).into_iter().cloned() {
                    if !((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
                    let __x = e.clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }};
                    Ok(Arc::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: r.clone(), innerOuter: i.clone(), specification: Arc::new(Absyn::ElementSpec::EXTENDS { path: path.clone(), elementArg: eargs.clone(), annotationOpt: annOpt.clone() }), info: info.clone(), constrainClass: constr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

pub fn mkFullyQual(mut env: GraphicEnvCache, mut ipath: Arc<Absyn::Path>, mut failOnError: bool) -> Result<(FCore::Cache, Arc<Absyn::Path>)> {
    let mut ocache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut opath: Arc<Absyn::Path>;
    let mut cpath: Arc<Absyn::Path>;
    let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    if Flags::isSet(Flags::NF_API.clone())? {
        ocache = cacheFromGraphicEnvCache(env.clone())?;
        (program, cpath) = cacheProgramAndPath(env.clone())?;
        opath = NFApi::mkFullyQual(program.clone(), cpath.clone(), ipath.clone(), failOnError.clone())?;
    } else {
        (ocache, opath) = Inst::makeFullyQualified(cacheFromGraphicEnvCache(env.clone())?, envFromGraphicEnvCache(env.clone())?, ipath.clone())?;
    }
    Ok((ocache, opath))
}

pub fn getExtendsModifierValue(mut classPath: Arc<Absyn::Path>, mut extendsPath: Arc<Absyn::Path>, mut modifierPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut ext_mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedExtendsInProgram(classPath.clone(), extendsPath.clone(), program.clone())) {
            Some(Deref @ Absyn::ElementSpec::EXTENDS { elementArg: __pa1, .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ext_mod = __pa1.clone();
        result = ValuesMake::makeCodeTypeNameStr((Dump::printExpStr(getModificationValue(ext_mod.clone(), modifierPath.clone()).unwrap()).unwrap()).clone());
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeCodeTypeNameStr((literal!("")).clone());
        }
    }
    result
}

pub fn isExtendsModifierFinal(mut classPath: Arc<Absyn::Path>, mut extendsPath: Arc<Absyn::Path>, mut modifierPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut ext_mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedExtendsInProgram(classPath.clone(), extendsPath.clone(), program.clone())) {
            Some(Deref @ Absyn::ElementSpec::EXTENDS { elementArg: __pa1, .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ext_mod = __pa1.clone();
        result = ValuesMake::makeBoolean(isModifierfinal(ext_mod.clone(), modifierPath.clone()).unwrap());
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isModifierfinal(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inPath: Arc<Absyn::Path>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inAbsynElementArgLst.clone(), inPath.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(_), path: p1, finalPrefix: f, .. }, tail: _ }, p2) if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            f.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: name1 }, .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { path: p2, name: name2 }) if (stringEq((name1.clone()).clone(), (name2.clone()).clone())?) => {
            let mut f: bool = false;
            f = isModifierfinal(args.clone(), p2.clone())?;
            f.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            let mut f: bool = false;
            f = isModifierfinal(rest.clone(), inPath.clone())?;
            f.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn makeExtendsFullyQualified(mut inElementSpec: Arc<Absyn::ElementSpec>, mut inEnv: GraphicEnvCache) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outElementSpec: Arc<Absyn::ElementSpec>;
    outElementSpec = (::match_deref::match_deref! { match &((inElementSpec.clone(), inEnv.clone())) {
        (Deref @ Absyn::ElementSpec::EXTENDS { annotationOpt: annOpt, elementArg: earg, path }, env) => {
            let mut path_1: Arc<Absyn::Path>;
            (_, path_1) = mkFullyQual(env.clone(), path.clone(), false)?;
            Arc::new(Absyn::ElementSpec::EXTENDS { path: path_1.clone(), elementArg: earg.clone(), annotationOpt: annOpt.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementSpec)
}

pub fn removeComponentModifiers(mut path: Arc<Absyn::Path>, mut inComponentName: ArcStr, mut inProgram: Absyn::Program, mut keepRedeclares: bool) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut outResult: bool = false;
    let mut within_: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    match '__try0: {
        within_ = unwrap_break_err!(InteractiveUtil::buildWithin(path.clone()), '__try0);
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), inProgram.clone(), false, false), '__try0);
        cls = unwrap_break_err!(InteractiveUtil::clearComponentModifiersInClass(cls.clone(), (inComponentName.clone()).clone(), keepRedeclares.clone()), '__try0);
        outProgram = unwrap_break_err!(InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, inProgram.clone(), false), '__try0);
        outResult = true;
        Ok::<_, anyhow::Error>((outProgram.clone(), outResult.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outResult = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outResult = false;
        }
    }
    (outProgram, outResult)
}

pub fn getComponentModifierValue(mut classRef: Arc<Absyn::ComponentRef>, mut varRef: Arc<Absyn::ComponentRef>, mut subModRef: Arc<Absyn::ComponentRef>, mut program: Absyn::Program) -> ArcStr {
    let mut valueStr: ArcStr = arcstr::literal!("");
    let mut cls_path: Arc<Absyn::Path>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    match '__try0: {
        cls_path = unwrap_break_err!(AbsynUtil::crefToPath(classRef.clone()), '__try0);
        name = (unwrap_break_err!(AbsynUtil::crefIdent(varRef.clone()), '__try0)).clone();
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(cls_path.clone(), program.clone(), false, false), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getComponentInClass(cls.clone(), (name.clone()).clone()), '__try0)) {
            Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa1, .. }), .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        args = __pa1.clone();
        valueStr = (unwrap_break_err!(Dump::printExpStr(getModificationValue(args.clone(), AbsynUtil::crefToPath(subModRef.clone()).unwrap()).unwrap()), '__try0)).clone();
        Ok::<_, anyhow::Error>((valueStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valueStr = __try0_o0;
        }
        Err(_) => {
            valueStr = (literal!("")).clone();
        }
    }
    valueStr
}

pub fn getModificationValue(mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Exp>> {
    let mut value: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut name: ArcStr = arcstr::literal!("");
    let mut rest_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = args.clone();
    let mut arg: Arc<Absyn::ElementArg>;
    let mut found: bool = false;
    while !(found.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        found = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } if (AbsynUtil::pathEqual(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), path.clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa0, .. }, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            value = __pa0.clone();
            true
        },
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, .. } if (name.clone() == AbsynUtil::pathFirstIdent(path.clone())?) => {
            let __pa0 = ::match_deref::match_deref! { match &(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            rest_args = __pa0.clone();
            value = getModificationValue(rest_args.clone(), AbsynUtil::pathRest(path.clone())?)?;
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(value)
}

pub fn getComponentModifierValues(mut inComponentRef1: Arc<Absyn::ComponentRef>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>, mut inProgram4: Absyn::Program) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inComponentRef1.clone(), inComponentRef2.clone(), inComponentRef3.clone(), inProgram4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, ident, subident, p) => {
                    let mut p_class: Arc<Absyn::Path>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut comps: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>>> = metamodelica::nil();
                    let mut compelts_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut r#mod: Arc<Absyn::Modification> = Arc::new(<Absyn::Modification as ::std::default::Default>::default());
                    let mut elementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    p_class = AbsynUtil::crefToPath(class_.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::crefToPath(ident.clone())?) {
                        Deref @ Absyn::Path::IDENT { name: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    cdef = InteractiveUtil::getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    comps = InteractiveUtil::getComponentsInClass(cdef.clone(), InteractiveUtil::Visibility::ANY.clone());
                    compelts = List::map(comps.clone(), (std::sync::Arc::new(fnptr!(InteractiveUtil::getComponentitemsInElement, Arc<Absyn::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>> + 'static>));
                    compelts_1 = List::flatten(compelts.clone());
                    let metamodelica::List::Cons { head: Absyn::COMPONENTITEM { component: Absyn::COMPONENT { modification: Some(Absyn::CLASSMOD { elementArgLst: __pa1, .. }), .. }, .. }, tail: __t2 } = (List::select1(compelts_1.clone(), (std::sync::Arc::new(InteractiveUtil::componentitemNamed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentItem>, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    let metamodelica::List::Nil = (__t2.clone()) else { bail!("pattern mismatch") };
                    elementArgLst = __pa1.clone();
                    r#mod = getModificationValues(elementArgLst.clone(), AbsynUtil::crefToPath(subident.clone())?)?;
                    res = (Dump::unparseModificationStr(r#mod.clone())?).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Error"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getModificationValues(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Modification>> {
    let mut outModification: Arc<Absyn::Modification> = Arc::new(<Absyn::Modification as ::std::default::Default>::default());
    outModification = (::match_deref::match_deref! { match &((inAbsynElementArgLst.clone(), inPath.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), path: p1, .. }, tail: _ }, p2) if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            r#mod.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: name1 }, .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { path: p2, name: name2 }) if (stringEq((name1.clone()).clone(), (name2.clone()).clone())?) => {
            let mut res: Arc<Absyn::Modification> = Arc::new(<Absyn::Modification as ::std::default::Default>::default());
            res = getModificationValues(args.clone(), p2.clone())?;
            res.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            let mut r#mod: Arc<Absyn::Modification> = Arc::new(<Absyn::Modification as ::std::default::Default>::default());
            r#mod = getModificationValues(rest.clone(), inPath.clone())?;
            r#mod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outModification)
}

pub fn getComponentModifierNames(mut path: Arc<Absyn::Path>, mut inComponentName: ArcStr, mut inProgram3: Absyn::Program) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = (path.clone(), inComponentName.clone(), inProgram3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut comps: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>>> = metamodelica::nil();
                    let mut compelts_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut r#mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    comps = InteractiveUtil::getComponentsInClass(cdef.clone(), InteractiveUtil::Visibility::ANY.clone());
                    compelts = List::map(comps.clone(), (std::sync::Arc::new(fnptr!(InteractiveUtil::getComponentitemsInElement, Arc<Absyn::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>> + 'static>));
                    compelts_1 = List::flatten(compelts.clone());
                    let metamodelica::List::Cons { head: Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: _, arrayDim: _, modification: Some(Absyn::CLASSMOD { elementArgLst: __pa0, eqMod: _ }) }, condition: _, comment: _ }, tail: __t1 } = (List::select1(compelts_1.clone(), (std::sync::Arc::new(InteractiveUtil::componentitemNamed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentItem>, ArcStr) -> Result<bool> + 'static>), (inComponentName.clone()).clone())) else { bail!("pattern mismatch") };
                    let metamodelica::List::Nil = (__t1.clone()) else { bail!("pattern mismatch") };
                    r#mod = __pa0.clone();
                    res = getModificationNames(r#mod.clone())?;
                    Ok(res.clone())
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
    Ok(outList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getModificationNames(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inAbsynElementArgLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: None, path: Deref @ Absyn::Path::IDENT { name }, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    names = getModificationNames(rest.clone())?;
                    Ok(cons((name.clone()).clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::List::Nil, eqMod: _ }), path: p, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    names = getModificationNames(rest.clone())?;
                    Ok(cons((name.clone()).clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { .. } }), path: p, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    names2 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (getModificationNames(args.clone())?).into_iter().cloned() {
                    let __x = stringAppend((stringAppend((name.clone()).clone(), (literal!(".")).clone())).clone(), (n.clone()).clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    names = getModificationNames(rest.clone())?;
                    res = listAppend(names2.clone(), names.clone());
                    Ok(cons((name.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: _ }), path: p, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    names2 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (getModificationNames(args.clone())?).into_iter().cloned() {
                    let __x = stringAppend((stringAppend((name.clone()).clone(), (literal!(".")).clone())).clone(), (n.clone()).clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    names = getModificationNames(rest.clone())?;
                    res = listAppend(names2.clone(), names.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    names = getModificationNames(rest.clone())?;
                    Ok(names.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

pub fn getComponentBinding(mut path: Arc<Absyn::Path>, mut parameterName: ArcStr, mut program: Absyn::Program) -> ArcStr {
    let mut bindingStr: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut component: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0);
        component = unwrap_break_err!(InteractiveUtil::getComponentInClass(cls.clone(), (parameterName.clone()).clone()), '__try0);
        bindingStr = (unwrap_break_err!(Dump::printExpStr(InteractiveUtil::getVariableBindingInComponentitem(component.clone()).unwrap()), '__try0)).clone();
        Ok::<_, anyhow::Error>((bindingStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bindingStr = __try0_o0;
        }
        Err(_) => {
            bindingStr = (literal!("")).clone();
        }
    }
    bindingStr
}

fn getComponentitemName(mut inComponentItem: Arc<Absyn::ComponentItem>) -> Result<ArcStr> {
    let mut outIdent: ArcStr = arcstr::literal!("");
    outIdent = ((::match_deref::match_deref! { match &(inComponentItem.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id, .. }, .. } => {
            id.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

pub fn renameClass(mut oldName: Arc<Absyn::Path>, mut newName: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<(Absyn::Program, Arc<Values::Value>)> {
    let mut program: Absyn::Program = program;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut env: FCore::Graph;
    let mut new_name: Arc<Absyn::Path>;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    if AbsynUtil::pathIsQual(newName.clone()) {
        result = ValuesMake::makeBoolean(false);
    }
    if AbsynUtil::pathIsQual(oldName.clone()) {
        new_name = AbsynUtil::joinPaths(AbsynUtil::stripLast(oldName.clone())?, newName.clone())?;
    } else {
        new_name = newName.clone();
    }
    (_, env) = Inst::makeEnvFromProgram(SymbolTable::getSCode()?)?;
    let (__pa0, _, (_, _, _, __pa1, _)) = AbsynUtil::traverseClasses(program.clone(), None, (std::sync::Arc::new(renameClassVisitor) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, (Arc<Absyn::Path>, Arc<Absyn::Path>, Absyn::Program, Arc<metamodelica::List<Arc<Absyn::Path>>>, FCore::Graph))) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, (Arc<Absyn::Path>, Arc<Absyn::Path>, Absyn::Program, Arc<metamodelica::List<Arc<Absyn::Path>>>, FCore::Graph))> + 'static>), (oldName.clone(), new_name.clone(), program.clone(), metamodelica::nil(), env.clone()), true)?;
    program = __pa0.clone();
    paths = __pa1.clone();
    result = ValuesMake::makeCodeTypeNameArray(paths.clone())?;
    Ok((program, result))
}

fn renameClassVisitor(mut tup: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, (Arc<Absyn::Path>, Arc<Absyn::Path>, Absyn::Program, Arc<metamodelica::List<Arc<Absyn::Path>>>, FCore::Graph))) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, (Arc<Absyn::Path>, Arc<Absyn::Path>, Absyn::Program, Arc<metamodelica::List<Arc<Absyn::Path>>>, FCore::Graph))> {
    let mut tup: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, (Arc<Absyn::Path>, Arc<Absyn::Path>, Absyn::Program, Arc<metamodelica::List<Arc<Absyn::Path>>>, FCore::Graph)) = tup;
    tup = 'mc: {
        let __mc_input = tup.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { info: file_info, .. }, _, _) => {
                    if !((isReadOnly(file_info.clone())?)) { bail!("guard") }
                    Ok(tup.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, Some(pa), (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: Arc<Absyn::Path>;
                    let mut new_name: ArcStr = arcstr::literal!("");
                    let mut class_ = (*class_).clone();
                    path_1 = AbsynUtil::joinPaths(pa.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                    let true = (AbsynUtil::pathEqual(old_class_path.clone(), path_1.clone())) else { bail!("pattern mismatch") };
                    new_name = (AbsynUtil::pathLastIdent(new_class_path.clone())?).clone();
                    assign_field!(class_.name = new_name.clone());
                    Ok((class_.clone(), Some(pa.clone()), (old_class_path.clone(), new_class_path.clone(), p.clone(), cons(new_class_path.clone(), path_lst.clone()), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, None, (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: Arc<Absyn::Path>;
                    let mut new_name: ArcStr = arcstr::literal!("");
                    let mut class_ = (*class_).clone();
                    path_1 = Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() });
                    let true = (AbsynUtil::pathEqual(old_class_path.clone(), path_1.clone())) else { bail!("pattern mismatch") };
                    new_name = (AbsynUtil::pathLastIdent(new_class_path.clone())?).clone();
                    assign_field!(class_.name = new_name.clone());
                    Ok((class_.clone(), None, (old_class_path.clone(), new_class_path.clone(), p.clone(), cons(new_class_path.clone(), path_lst.clone()), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, Some(pa), (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: Arc<Absyn::Path>;
                    let mut changed: bool = false;
                    let mut cenv: FCore::Graph;
                    let mut class_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut path_lst = (*path_lst).clone();
                    path_1 = AbsynUtil::joinPaths(pa.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                    cenv = getClassEnvNoElaboration(p.clone(), path_1.clone(), env.clone())?;
                    (class_1, changed) = renameClassInClass(class_.clone(), old_class_path.clone(), new_class_path.clone(), cenv.clone())?;
                    if changed.clone() {
                        path_lst = cons(path_1.clone(), path_lst.clone());
                    }
                    Ok((class_1.clone(), Some(pa.clone()), (old_class_path.clone(), new_class_path.clone(), p.clone(), path_lst.clone(), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, None, (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: Arc<Absyn::Path>;
                    let mut changed: bool = false;
                    let mut cenv: FCore::Graph;
                    let mut class_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut path_lst = (*path_lst).clone();
                    path_1 = Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() });
                    cenv = getClassEnvNoElaboration(p.clone(), path_1.clone(), env.clone())?;
                    (class_1, changed) = renameClassInClass(class_.clone(), old_class_path.clone(), new_class_path.clone(), cenv.clone())?;
                    if changed.clone() {
                        path_lst = cons(path_1.clone(), path_lst.clone());
                    }
                    Ok((class_1.clone(), None, (old_class_path.clone(), new_class_path.clone(), p.clone(), path_lst.clone(), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tup.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tup)
}

fn renameClassInClass(mut cls: Arc<Absyn::Class>, mut oldName: Arc<Absyn::Path>, mut newName: Arc<Absyn::Path>, mut env: FCore::Graph) -> Result<(Arc<Absyn::Class>, bool)> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut changed: bool = false;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut path: Arc<Absyn::Path>;
    let mut cenv: FCore::Graph;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut body: Arc<Absyn::ClassDef>;
    let mut ty: Arc<Absyn::TypeSpec>;
    body = cls.body.clone();
    changed = 'mc: {
        let __mc_input = body.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. } => {
                    let mut parts = (*parts).clone();
                    let mut body: Arc<Absyn::ClassDef> = body.clone();
                    let mut changed: bool = changed.clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    (parts, changed) = renameClassInParts(parts.clone(), oldName.clone(), newName.clone(), env.clone())?;
                    assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    assign_field!(cls.body = body.clone());
                    Ok(changed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. } => {
                    let mut parts = (*parts).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let mut body: Arc<Absyn::ClassDef> = body.clone();
                    let mut changed: bool = changed.clone();
                    (parts, changed) = renameClassInParts(parts.clone(), oldName.clone(), newName.clone(), env.clone())?;
                    assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    assign_field!(cls.body = body.clone());
                    Ok(changed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::DERIVED { typeSpec: ty @ Deref @ Absyn::TypeSpec::TPATH { .. }, .. } => {
                    let mut ty = (*ty).clone();
                    let mut cenv: FCore::Graph;
                    let mut path: Arc<Absyn::Path>;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut name: ArcStr = name.clone();
                    let mut body: Arc<Absyn::ClassDef> = body.clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), env.clone(), var_field!((*ty).path, Absyn::TypeSpec::TPATH).clone(), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    name = __pa1.clone();
                    cenv = __pa2.clone();
                    path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
                    (_, path) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), path.clone())?;
                    let true = (AbsynUtil::pathEqual(path.clone(), oldName.clone())) else { bail!("pattern mismatch") };
                    assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = changeLastIdent(path.clone(), newName.clone())?);
                    assign_variant_field!(body => Absyn::ClassDef::DERIVED; typeSpec = ty.clone());
                    assign_field!(cls.body = body.clone());
                    Ok(true)
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
    Ok((cls, changed))
}

fn renameClassInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut oldName: Arc<Absyn::Path>, mut newName: Arc<Absyn::Path>, mut env: FCore::Graph) -> Result<(Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, bool)> {
    let mut outParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut changed: bool = false;
    let mut c: bool = false;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        part = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            (elems, c) = renameClassInElements(var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(), oldName.clone(), newName.clone(), env.clone())?;
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = elems.clone());
            changed = changed.clone() || c.clone();
            part.clone()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            (elems, c) = renameClassInElements(var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone(), oldName.clone(), newName.clone(), env.clone())?;
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = elems.clone());
            changed = changed.clone() || c.clone();
            part.clone()
        },
        _ => part.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outParts = cons(part.clone(), outParts.clone());
    }
    outParts = Dangerous::listReverseInPlace(outParts.clone());
    Ok((outParts, changed))
}

fn renameClassInElements(mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut oldName: Arc<Absyn::Path>, mut newName: Arc<Absyn::Path>, mut env: FCore::Graph) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, bool)> {
    let mut outItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut changed: bool = false;
    let mut elem: Arc<Absyn::Element>;
    let mut spec: Arc<Absyn::ElementSpec>;
    let mut c: bool = false;
    for mut item in &*items.clone() {
        let mut item = item.clone();
        (outItems, changed) = 'mc: {
        let __mc_input = item.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: elem @ Deref @ Absyn::Element::ELEMENT { .. } } => {
                    let mut elem = (*elem).clone();
                    let mut spec: Arc<Absyn::ElementSpec>;
                    let mut c: bool = c.clone();
                    let mut item: Arc<Absyn::ElementItem>;
                    (spec, c) = renameClassInElementSpec(var_field!((*elem).specification, Absyn::Element::ELEMENT).clone(), oldName.clone(), newName.clone(), env.clone())?;
                    assign_variant_field!(elem => Absyn::Element::ELEMENT; specification = spec.clone());
                    assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = elem.clone());
                    Ok((cons(item.clone(), outItems.clone()), changed.clone() || c.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cons(item.clone(), outItems.clone()), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    }
    outItems = Dangerous::listReverseInPlace(outItems.clone());
    Ok((outItems, changed))
}

fn renameClassInElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut oldName: Arc<Absyn::Path>, mut newName: Arc<Absyn::Path>, mut env: FCore::Graph) -> Result<(Arc<Absyn::ElementSpec>, bool)> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let mut changed: bool = false;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut id: ArcStr = arcstr::literal!("");
    let mut cenv: FCore::Graph;
    let mut path: Arc<Absyn::Path>;
    let mut qpath: Arc<Absyn::Path>;
    let mut imp: Absyn::Import;
    changed = 'mc: {
        let __mc_input = spec.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: ty @ Deref @ Absyn::TypeSpec::TPATH { .. }, .. } => {
                    let mut ty = (*ty).clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut cenv: FCore::Graph;
                    let mut qpath: Arc<Absyn::Path>;
                    let mut changed: bool = changed.clone();
                    let mut spec: Arc<Absyn::ElementSpec> = spec.clone();
                    let mut id: ArcStr = id.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), env.clone(), var_field!((*ty).path, Absyn::TypeSpec::TPATH).clone(), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    id = __pa1.clone();
                    cenv = __pa2.clone();
                    (_, qpath) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                    if AbsynUtil::pathEqual(qpath.clone(), oldName.clone()) {
                        assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = changeLastIdent(qpath.clone(), newName.clone())?);
                        assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; typeSpec = ty.clone());
                        changed = true;
                    }
                    Ok(changed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
                    let mut qpath: Arc<Absyn::Path>;
                    let mut spec: Arc<Absyn::ElementSpec> = spec.clone();
                    let mut cenv: FCore::Graph;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut changed: bool = changed.clone();
                    (cache, _, cenv) = Lookup::lookupClass(FCore::emptyCache(), env.clone(), var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone(), None)?;
                    (_, qpath) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone())?;
                    if AbsynUtil::pathEqual(qpath.clone(), oldName.clone()) {
                        assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS; path = changeLastIdent(var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone(), newName.clone())?);
                        changed = true;
                    }
                    Ok(changed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::IMPORT { .. } => {
                    let mut qpath: Arc<Absyn::Path>;
                    let mut cenv: FCore::Graph;
                    let mut changed: bool = changed.clone();
                    let mut spec: Arc<Absyn::ElementSpec> = spec.clone();
                    let mut path: Arc<Absyn::Path>;
                    let mut cache: FCore::Cache = cache.clone();
                    path = AbsynUtil::importPath(var_field!((*spec).import_, Absyn::ElementSpec::IMPORT).clone())?;
                    (cache, _, cenv) = Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), None)?;
                    (_, qpath) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), path.clone())?;
                    if AbsynUtil::pathEqual(qpath.clone(), oldName.clone()) {
                        path = changeLastIdent(path.clone(), newName.clone())?;
                        assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; import_ = AbsynUtil::setImportPath(var_field!((*spec).import_, Absyn::ElementSpec::IMPORT).clone(), path.clone())?);
                        changed = true;
                    }
                    Ok(changed.clone())
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
    Ok((spec, changed))
}

pub fn refactorClass(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
        let mut r#str: ArcStr = arcstr::literal!("");
        cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        cls = Refactor::refactorGraphicalAnnotation(program.clone(), cls.clone())?;
        p = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, program.clone(), false)?;
        SymbolTable::setAbsyn(p.clone())?;
        r#str = (Dump::unparseStr(Absyn::Program { classes: list![cls.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, false, Dump::defaultDumpOptions.clone())?).clone();
        result = ValuesMake::makeString((r#str.clone()).clone());
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, true, Access::icon.clone())?;
    Ok(result)
}

fn changeLastIdent(mut inPath1: Arc<Absyn::Path>, mut inPath2: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath1.clone(), inPath2.clone())) {
        (Deref @ Absyn::Path::IDENT { .. }, Deref @ Absyn::Path::IDENT { name: b }) => {
            Arc::new(Absyn::Path::IDENT { name: (b.clone()).clone() })
        },
        (Deref @ Absyn::Path::IDENT { .. }, p2 @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
            let mut b_1: ArcStr = arcstr::literal!("");
            b_1 = (AbsynUtil::pathLastIdent(p2.clone())?).clone();
            Arc::new(Absyn::Path::IDENT { name: (b_1.clone()).clone() })
        },
        (p1 @ Deref @ Absyn::Path::QUALIFIED { .. }, p2 @ Deref @ Absyn::Path::IDENT { .. }) => {
            let mut a_1: Arc<Absyn::Path>;
            let mut res: Arc<Absyn::Path>;
            a_1 = AbsynUtil::stripLast(p1.clone())?;
            res = AbsynUtil::joinPaths(a_1.clone(), p2.clone())?;
            res.clone()
        },
        (p1 @ Deref @ Absyn::Path::QUALIFIED { .. }, p2 @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
            let mut b_1: ArcStr = arcstr::literal!("");
            let mut a_1: Arc<Absyn::Path>;
            let mut res: Arc<Absyn::Path>;
            a_1 = AbsynUtil::stripLast(p1.clone())?;
            b_1 = (AbsynUtil::pathLastIdent(p2.clone())?).clone();
            res = AbsynUtil::joinPaths(a_1.clone(), Arc::new(Absyn::Path::IDENT { name: (b_1.clone()).clone() }))?;
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn isPrimitive(mut className: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(className.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "Real" } => {
            true
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" } => {
            true
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "String" } => {
            true
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" } => {
            true
        },
        _ => {
            let mut class_: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            class_ = InteractiveUtil::getPathedClassInProgram(className.clone(), inProgram.clone(), false, false)?;
            isPrimitiveClass(class_.clone(), inProgram.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn createModel(mut className: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut name: ArcStr = arcstr::literal!("");
    let mut w: Absyn::Within = Absyn::Within::TOP;
    let mut wp: Arc<Absyn::Path>;
    if AbsynUtil::pathIsIdent(className.clone()) {
        name = (AbsynUtil::pathFirstIdent(className.clone())?).clone();
        w = openmodelica_ast::Absyn::Within::TOP;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(className.clone())?) {
            (__pa0, Deref @ Absyn::Path::IDENT { name: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        wp = __pa0.clone();
        name = __pa1.clone();
        w = Absyn::Within::WITHIN { path: wp.clone() };
    }
    outProgram = InteractiveUtil::updateProgram(Absyn::Program { classes: list![Arc::new(Absyn::Class { name: (name.clone()).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_MODEL, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() })], within_: w.clone() }, inProgram.clone(), false)?;
    Ok(outProgram)
}

pub fn newModel(mut className: Arc<Absyn::Path>, mut withinPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    program = createModel(AbsynUtil::joinPaths(withinPath.clone(), className.clone())?, program.clone())?;
    Ok(program)
}

pub fn deleteClass(mut classPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut success: bool = false;
    let mut outProgram: Absyn::Program = inProgram.clone();
    (success, outProgram) = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut parentcpath: Arc<Absyn::Path>;
            let mut parentparentcpath: Arc<Absyn::Path>;
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut parentcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut parentcdef_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut outProgram: Absyn::Program = outProgram.clone();
            parentcpath = AbsynUtil::stripLast(classPath.clone())?;
            parentparentcpath = AbsynUtil::stripLast(parentcpath.clone())?;
            cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), inProgram.clone(), false, false)?;
            parentcdef = InteractiveUtil::getPathedClassInProgram(parentcpath.clone(), inProgram.clone(), false, false)?;
            parentcdef_1 = InteractiveUtil::removeInnerClass(cdef.clone(), parentcdef.clone())?;
            outProgram = InteractiveUtil::updateProgram(Absyn::Program { classes: list![parentcdef_1.clone()], within_: Absyn::Within::WITHIN { path: parentparentcpath.clone() } }, inProgram.clone(), false)?;
            Ok((true, outProgram.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut parentcpath: Arc<Absyn::Path>;
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut parentcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut parentcdef_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut outProgram: Absyn::Program = outProgram.clone();
            parentcpath = AbsynUtil::stripLast(classPath.clone())?;
            cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), inProgram.clone(), false, false)?;
            parentcdef = InteractiveUtil::getPathedClassInProgram(parentcpath.clone(), inProgram.clone(), false, false)?;
            parentcdef_1 = InteractiveUtil::removeInnerClass(cdef.clone(), parentcdef.clone())?;
            outProgram = InteractiveUtil::updateProgram(Absyn::Program { classes: list![parentcdef_1.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, inProgram.clone(), false)?;
            Ok((true, outProgram.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut outProgram: Absyn::Program = outProgram.clone();
            cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), inProgram.clone(), false, false)?;
            outProgram.classes = List::deleteMemberOnTrue((AbsynUtil::className(cdef.clone())?).clone(), outProgram.classes.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isClassNamed, ArcStr, Arc<Absyn::Class>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::Class>) -> Result<bool> + 'static>))?.0;
            Ok((true, outProgram.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((false, inProgram.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((success, outProgram))
}

pub fn setClassComment(mut path: Arc<Absyn::Path>, mut inString: ArcStr, mut inProgram: Absyn::Program) -> Result<(Absyn::Program, bool)> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut success: bool = false;
    (outProgram, success) = 'mc: {
        let __mc_input = (path.clone(), inString.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, r#str, p @ Absyn::Program { .. }) => {
                    let mut within_: Absyn::Within = Absyn::Within::TOP;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut cdef_1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    within_ = InteractiveUtil::buildWithin(p_class.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    cdef_1 = setClassCommentInClass(cdef.clone(), (r#str.clone()).clone())?;
                    newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cdef_1.clone()], within_: within_.clone() }, p.clone(), false)?;
                    Ok((newp.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inProgram.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outProgram, success))
}

fn setClassCommentInClass(mut cls: Arc<Absyn::Class>, mut commentString: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    assign_field!(cls.body = setClassCommentInClassdef(cls.body.clone(), (commentString.clone()).clone())?);
    Ok(cls)
}

fn setClassCommentInClassdef(mut classDef: Arc<Absyn::ClassDef>, mut commentString: ArcStr) -> Result<Arc<Absyn::ClassDef>> {
    let mut classDef: Arc<Absyn::ClassDef> = classDef;
    let mut cmt_str: Option<ArcStr> = None;
    cmt_str = if (stringEmpty((commentString.clone()).clone())) {None} else {Some((commentString.clone()).clone())};
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::PARTS; comment = cmt_str.clone());
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::DERIVED; comment = AbsynUtil::setCommentString(var_field!((*classDef).comment, Absyn::ClassDef::DERIVED).clone(), cmt_str.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::ENUMERATION; comment = AbsynUtil::setCommentString(var_field!((*classDef).comment, Absyn::ClassDef::ENUMERATION).clone(), cmt_str.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::OVERLOAD; comment = AbsynUtil::setCommentString(var_field!((*classDef).comment, Absyn::ClassDef::OVERLOAD).clone(), cmt_str.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::CLASS_EXTENDS; comment = cmt_str.clone());
            ()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::PDER; comment = AbsynUtil::setCommentString(var_field!((*classDef).comment, Absyn::ClassDef::PDER).clone(), cmt_str.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(classDef)
}

pub fn getShortDefinitionBaseClassInformation(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut attr: Absyn::ElementAttributes = <Absyn::ElementAttributes as ::std::default::Default>::default();
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { attributes: __pa1 @ Absyn::ElementAttributes { .. }, typeSpec: __pa2, .. }, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        attr = __pa1.clone();
        ty = __pa2.clone();
        vals = cons(unwrap_break_err!(ValuesMake::makeArray(InteractiveUtil::dimensionListValues(AbsynUtil::typeSpecDimensions(ty.clone()))?), '__try0), vals.clone());
        vals = cons(ValuesMake::makeString((InteractiveUtil::attrDirectionStr(attr.clone())?).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((InteractiveUtil::attrVariabilityStr(attr.clone())?).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((if (attr.streamPrefix.clone()) {literal!("stream")} else {literal!("")}).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((if (attr.flowPrefix.clone()) {literal!("flow")} else {literal!("")}).clone()), vals.clone());
        vals = cons(ValuesMake::makeCodeTypeName(AbsynUtil::typeSpecPath(ty.clone())?), vals.clone());
        Ok::<_, anyhow::Error>((vals.clone(),))
    } {
        Ok((__try0_o0,)) => {
            vals = __try0_o0;
        }
        Err(_) => {
            vals = metamodelica::nil();
        }
    }
    result = ValuesMake::makeArray(vals.clone())?;
    Ok(result)
}

pub fn getExternalFunctionSpecification(mut functionName: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut ext_decl: Arc<Absyn::ExternalDecl> = Arc::new(<Absyn::ExternalDecl as ::std::default::Default>::default());
    let mut ann: Option<Arc<Absyn::Annotation>> = None;
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(functionName.clone(), program.clone(), false, false), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::getExternalDecl(cls.clone()), '__try0)) {
            Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: __pa1, annotation_: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ext_decl = __pa1.clone();
        ann = __pa2.clone();
        vals = cons(ValuesMake::makeString((Dump::unparseAnnotationOption(ann.clone())?).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((Dump::unparseAnnotationOption(ext_decl.annotation_.clone())?).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((Dump::printExpLstStr(ext_decl.args.clone())).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((Util::getOptionOrDefault(ext_decl.funcName.clone(), (literal!("")).clone())).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((Util::applyOptionOrDefault(ext_decl.output_.clone(), (std::sync::Arc::new(Dump::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone())).clone()), vals.clone());
        vals = cons(ValuesMake::makeString((Util::getOptionOrDefault(ext_decl.lang.clone(), (literal!("")).clone())).clone()), vals.clone());
        Ok::<_, anyhow::Error>((vals.clone(),))
    } {
        Ok((__try0_o0,)) => {
            vals = __try0_o0;
        }
        Err(_) => {
            vals = metamodelica::nil();
        }
    }
    result = ValuesMake::makeArray(vals.clone())?;
    Ok(result)
}

fn getClassDimensions(mut cdef: Arc<Absyn::ClassDef>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(ad), .. }, .. } => List::toString(ad.clone(), (std::sync::Arc::new(Dump::printSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(",")).clone(), (literal!("}")).clone(), true, 0)?,
        _ => literal!("{}"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn getClassRestriction(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> ArcStr {
    let mut outRestriction: ArcStr = arcstr::literal!("");
    let mut restr: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { restriction: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        restr = __pa1.clone();
        outRestriction = (unwrap_break_err!(Dump::unparseRestrictionStr(restr.clone()), '__try0)).clone();
        Ok::<_, anyhow::Error>((outRestriction.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outRestriction = __try0_o0;
        }
        Err(_) => {
            outRestriction = (literal!("")).clone();
        }
    }
    outRestriction
}

pub fn isType(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_TYPE => true,
        _ => false,
    });
    res
}

pub fn isConnector(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_CONNECTOR => true,
        Absyn::Restriction::R_EXP_CONNECTOR => true,
        _ => false,
    });
    res
}

pub fn isModel(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_MODEL => true,
        _ => false,
    });
    res
}

pub fn isOperator(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_OPERATOR => true,
        _ => false,
    });
    res
}

pub fn isOperatorRecord(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_OPERATOR_RECORD => true,
        _ => false,
    });
    res
}

pub fn isOperatorFunction(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION } => true,
        _ => false,
    });
    res
}

pub fn isRecord(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_RECORD => true,
        _ => false,
    });
    res
}

pub fn isBlock(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_BLOCK => true,
        _ => false,
    });
    res
}

pub fn isOptimization(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_OPTIMIZATION => true,
        _ => false,
    });
    res
}

pub fn isFunction(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { .. } } => true,
        _ => false,
    });
    res
}

pub fn isPackage(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_PACKAGE => true,
        _ => false,
    });
    res
}

pub fn isClass(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    res = (match InteractiveUtil::getPathedClassRestriction(path.clone(), program.clone()) {
        Absyn::Restriction::R_CLASS => true,
        _ => false,
    });
    res
}

pub fn isPartial(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    match '__try0: {
        ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { partialPrefix: true, .. } => (),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        res = true;
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isReplaceable(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    match '__try0: {
        res = AbsynUtil::isElementReplaceable(InteractiveUtil::getPathedElementInProgram(path.clone(), program.clone()).unwrap());
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isRedeclare(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    match '__try0: {
        res = AbsynUtil::isElementRedeclare(InteractiveUtil::getPathedElementInProgram(path.clone(), program.clone()).unwrap());
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isParameter(mut componentName: Arc<Absyn::Path>, mut className: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    let mut path: Arc<Absyn::Path>;
    let mut elem: Arc<Absyn::Element>;
    match '__try0: {
        path = unwrap_break_err!(AbsynUtil::joinPaths(className.clone(), componentName.clone()), '__try0);
        ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedElementInProgram(path.clone(), program.clone()), '__try0)) {
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::PARAM, .. }, .. }, .. } => (),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        res = true;
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isConstant(mut componentName: Arc<Absyn::Path>, mut className: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    let mut path: Arc<Absyn::Path>;
    let mut elem: Arc<Absyn::Element>;
    match '__try0: {
        path = unwrap_break_err!(AbsynUtil::joinPaths(className.clone(), componentName.clone()), '__try0);
        ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedElementInProgram(path.clone(), program.clone()), '__try0)) {
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::CONST, .. }, .. }, .. } => (),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        res = true;
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isProtected(mut componentName: Arc<Absyn::Path>, mut className: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    match '__try0: {
        parts = AbsynUtil::getClassPartsInClass(InteractiveUtil::getPathedClassInProgram(className.clone(), program.clone(), false, false).unwrap());
        items = InteractiveUtil::getProtectedList(parts.clone());
        unwrap_break_err!(getComponentsContainsName(AbsynUtil::pathToCref(componentName.clone()).unwrap(), items.clone()), '__try0);
        res = true;
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isEnumeration(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    match '__try0: {
        ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, restriction: Absyn::Restriction::R_TYPE, .. } => (),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        res = true;
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub fn isProtectedClass(mut path: Arc<Absyn::Path>, mut className: ArcStr, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    match '__try0: {
        parts = AbsynUtil::getClassPartsInClass(InteractiveUtil::getPathedClassInProgram(path.clone(), program.clone(), false, false).unwrap());
        items = InteractiveUtil::getProtectedList(parts.clone());
        res = isProtectedClassInElements(items.clone(), (className.clone()).clone());
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

fn isProtectedClassInElements(mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut className: ArcStr) -> bool {
    let mut res: bool = false;
    let mut name: ArcStr = arcstr::literal!("");
    for mut item in &*items.clone() {
        let mut item = item.clone();
        res = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, .. }, .. }, .. } } => name.clone() == className.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if res.clone() {
            break;
        }
    }
    res
}

pub fn getEnumerationLiterals(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut literals: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: Deref @ Absyn::EnumDef::ENUMLITERALS { enumLiterals: __pa1 }, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        literals = __pa1.clone();
        names = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut l in (literals.clone()).into_iter().cloned() {
            let __x = AbsynUtil::enumLiteralName(l.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        Ok::<_, anyhow::Error>((names.clone(),))
    } {
        Ok((__try0_o0,)) => {
            names = __try0_o0;
        }
        Err(_) => {
            names = metamodelica::nil();
        }
    }
    result = ValuesMake::makeStringArray(names.clone())?;
    Ok(result)
}

pub fn getDerivedClassModifierNames(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outString = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { arguments: args, .. }, restriction: Absyn::Restriction::R_TYPE, .. } => getModificationNames(args.clone())?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn getDerivedClassModifierValue(mut cls: Arc<Absyn::Class>, mut path: Arc<Absyn::Path>) -> ArcStr {
    let mut value: ArcStr = arcstr::literal!("");
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(cls.clone()) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { arguments: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        args = __pa1.clone();
        value = (unwrap_break_err!(Dump::printExpStr(getModificationValue(args.clone(), path.clone()).unwrap()), '__try0)).clone();
        Ok::<_, anyhow::Error>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            value = (literal!("")).clone();
        }
    }
    value
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getElementitemContainsName(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<Absyn::ElementItem>> {
    let mut outElementItem: Arc<Absyn::ElementItem>;
    outElementItem = 'mc: {
        let __mc_input = (inComponentRef.clone(), inAbsynElementItemLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: elt, tail: _ }) => {
                    getComponentsContainsName(cr.clone(), list![elt.clone()])?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut res: Arc<Absyn::ElementItem>;
                    res = getElementitemContainsName(cr.clone(), rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElementItem)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getComponentsContainsName(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outElementSpec: Arc<Absyn::ElementSpec>;
    outElementSpec = 'mc: {
        let __mc_input = (inComponentRef.clone(), inAbsynElementItemLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: res @ Deref @ Absyn::ElementSpec::COMPONENTS { components: ellst, .. }, .. } }, tail: _ }) => {
                    getCompitemNamed(cr.clone(), ellst.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: _, tail: xs }) => {
                    let mut res: Arc<Absyn::ElementSpec>;
                    res = getComponentsContainsName(cr.clone(), xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElementSpec)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getElementContainsName(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<Absyn::Element>> {
    let mut outElement: Arc<Absyn::Element>;
    outElement = 'mc: {
        let __mc_input = (inComponentRef.clone(), inAbsynElementItemLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: res @ Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: ellst, .. }, .. } }, tail: _ }) => {
                    getCompitemNamed(cr.clone(), ellst.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: _, tail: xs }) => {
                    let mut res: Arc<Absyn::Element>;
                    res = getElementContainsName(cr.clone(), xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCompitemNamed(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Arc<Absyn::ComponentItem>> {
    let mut outComponentItem: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
    outComponentItem = 'mc: {
        let __mc_input = (inComponentRef.clone(), inAbsynComponentItemLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_IDENT { name: id1, .. }, Deref @ metamodelica::List::Cons { head: x @ Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id2, .. }, .. }, tail: _ }) => {
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())?) else { bail!("pattern mismatch") };
                    Ok(x.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::List::Cons { head: _, tail: xs }) => {
                    let mut res: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
                    res = getCompitemNamed(cr.clone(), xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComponentItem)
}

pub fn existClass(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    match '__try0: {
        unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        res = true;
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isPrimitiveClass(mut inClass: Arc<Absyn::Class>, mut inProgram: Absyn::Program) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inClass.clone(), inProgram.clone())) {
        (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_INTEGER, .. }, _) => {
            true
        },
        (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_REAL, .. }, _) => {
            true
        },
        (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_STRING, .. }, _) => {
            true
        },
        (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_BOOLEAN, .. }, _) => {
            true
        },
        (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_CLOCK, .. }, _) => {
            true
        },
        (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_TYPE, .. }, _) => {
            true
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: _ }, .. }, restriction: Absyn::Restriction::R_CLASS, name: cname, .. }, p) => {
            let mut inmodel: Arc<Absyn::Path>;
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut res: bool = false;
            inmodel = AbsynUtil::crefToPath(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (cname.clone()).clone(), subscripts: metamodelica::nil() }))?;
            (cdef, _) = lookupClassdef(path.clone(), inmodel.clone(), p.clone())?;
            res = isPrimitiveClass(cdef.clone(), p.clone())?;
            res.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

fn lookupClassdef(mut inPath1: Arc<Absyn::Path>, mut inPath2: Arc<Absyn::Path>, mut inProgram3: Absyn::Program) -> Result<(Arc<Absyn::Class>, Arc<Absyn::Path>)> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut outPath: Arc<Absyn::Path>;
    (outClass, outPath) = 'mc: {
        let __mc_input = (inPath1.clone(), inPath2.clone(), inProgram3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, inmodel, p @ Absyn::Program { .. }) => {
                    let mut inmodeldef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newpath: Arc<Absyn::Path>;
                    let mut path = (*path).clone();
                    path = InstUtil::removeSelfReference((AbsynUtil::pathLastIdent(inmodel.clone())?).clone(), path.clone())?;
                    inmodeldef = InteractiveUtil::getPathedClassInProgram(inmodel.clone(), p.clone(), false, false)?;
                    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), Absyn::Program { classes: list![inmodeldef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, false, false)?;
                    newpath = AbsynUtil::joinPaths(inmodel.clone(), path.clone())?;
                    Ok((cdef.clone(), newpath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, inmodel, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut innewpath: Arc<Absyn::Path>;
                    let mut respath: Arc<Absyn::Path>;
                    innewpath = AbsynUtil::stripLast(inmodel.clone())?;
                    (cdef, respath) = lookupClassdef(path.clone(), innewpath.clone(), p.clone())?;
                    Ok((cdef.clone(), respath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, _, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    Ok((cdef.clone(), path.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Real" }, _, _) => {
                    Ok((Arc::new(Absyn::Class { name: (literal!("Real")).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_REAL, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, _, _) => {
                    Ok((Arc::new(Absyn::Class { name: (literal!("Integer")).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_INTEGER, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, _, _) => {
                    Ok((Arc::new(Absyn::Class { name: (literal!("String")).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_STRING, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("String")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" }, _, _) => {
                    Ok((Arc::new(Absyn::Class { name: (literal!("Boolean")).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_BOOLEAN, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Boolean")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Clock" }, _, _) => {
                    let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(Absyn::Class { name: (literal!("Clock")).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_CLOCK, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Clock")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, inmodel, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s2 = (AbsynUtil::pathString(inmodel.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outClass, outPath))
}

fn deleteOrUpdateComponent(mut componentName: ArcStr, mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut item: Option<(Arc<Absyn::Path>, Arc<Absyn::ComponentItem>)>) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    let mut w: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    w = if (AbsynUtil::pathIsIdent(classPath.clone())) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(classPath.clone())? }};
    cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
    cls = deleteOrUpdateComponentFromClass((componentName.clone()).clone(), cls.clone(), item.clone())?;
    program = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: w.clone() }, program.clone(), false)?;
    Ok(program)
}

fn deleteOrUpdateComponentFromClass(mut inString: ArcStr, mut inClass: Arc<Absyn::Class>, mut item: Option<(Arc<Absyn::Path>, Arc<Absyn::ComponentItem>)>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &((inString.clone(), inClass.clone())) {
        (name, outClass @ Deref @ Absyn::Class { info: _, body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }) => {
            let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut protlst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut protlst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut l2: i32 = 0;
            let mut l1: i32 = 0;
            let mut l1_1: i32 = 0;
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut success: bool = false;
            let mut outClass = (*outClass).clone();
            publst = InteractiveUtil::getPublicList(parts.clone());
            (publst2, success) = deleteOrUpdateComponentFromElementitems((name.clone()).clone(), publst.clone(), item.clone())?;
            l2 = (publst2.clone().len() as i32);
            l1 = (publst.clone().len() as i32);
            l1_1 = l1.clone() - 1;
            if intEq(l1_1.clone(), l2.clone()) && isNone(item.clone()) && success.clone() || boolNot(intEq(l1_1.clone(), l2.clone())) && isSome(item.clone()) && success.clone() {
                parts2 = InteractiveUtil::replacePublicList(parts.clone(), publst2.clone())?;
            } else {
                protlst = InteractiveUtil::getProtectedList(parts.clone());
                (protlst2, _) = deleteOrUpdateComponentFromElementitems((name.clone()).clone(), protlst.clone(), item.clone())?;
                parts2 = InteractiveUtil::replaceProtectedList(parts.clone(), protlst2.clone())?;
            }
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        (name, outClass @ Deref @ Absyn::Class { info: _, body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: r#mod, baseClassName: bcpath }, .. }) => {
            let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut protlst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut protlst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut l2: i32 = 0;
            let mut l1: i32 = 0;
            let mut l1_1: i32 = 0;
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut success: bool = false;
            let mut outClass = (*outClass).clone();
            publst = InteractiveUtil::getPublicList(parts.clone());
            (publst2, success) = deleteOrUpdateComponentFromElementitems((name.clone()).clone(), publst.clone(), item.clone())?;
            l2 = (publst2.clone().len() as i32);
            l1 = (publst.clone().len() as i32);
            l1_1 = l1.clone() - 1;
            if intEq(l1_1.clone(), l2.clone()) && isNone(item.clone()) && success.clone() || boolNot(intEq(l1_1.clone(), l2.clone())) && isSome(item.clone()) && success.clone() {
                parts2 = InteractiveUtil::replacePublicList(parts.clone(), publst2.clone())?;
            } else {
                protlst = InteractiveUtil::getProtectedList(parts.clone());
                (protlst2, _) = deleteOrUpdateComponentFromElementitems((name.clone()).clone(), protlst.clone(), item.clone())?;
                parts2 = InteractiveUtil::replaceProtectedList(parts.clone(), protlst2.clone())?;
            }
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcpath.clone()).clone(), modifications: r#mod.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn deleteOrUpdateComponentFromElementitems(mut inString: ArcStr, mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut item: Option<(Arc<Absyn::Path>, Arc<Absyn::ComponentItem>)>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, bool)> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut success: bool = false;
    (outAbsynElementItemLst, success) = (::match_deref::match_deref! { match &((inString.clone(), inAbsynElementItemLst.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            (metamodelica::nil(), false)
        },
        (name, Deref @ metamodelica::List::Cons { head: x @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt @ Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, typeSpec: typeSpec @ Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. } }, tail: xs }) => {
            let mut name2: ArcStr = arcstr::literal!("");
            let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut eltold: Arc<Absyn::Element>;
            let mut compitem: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
            let mut tppath: Arc<Absyn::Path>;
            let mut hasOtherComponents: bool = false;
            let mut successResult: bool = false;
            let mut elt = (*elt).clone();
            let mut spec = (*spec).clone();
            let mut typeSpec = (*typeSpec).clone();
            if {
        let mut __acc: Option<bool> = None;
        for mut c in (comps.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. } if (stringEq((name.clone()).clone(), (name2.clone()).clone())?) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } {
                (res, successResult) = (::match_deref::match_deref! { match &(item.clone()) {
        Some((tppath, compitem)) => {
            if AbsynUtil::pathEqual(tppath.clone(), AbsynUtil::typeSpecPath(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone())?) {
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. } if (stringEq((name.clone()).clone(), (name2.clone()).clone())?) => compitem.clone(),
        _ => c.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                res = cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), xs.clone());
                successResult = true;
            } else {
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. } => !(stringEq((name.clone()).clone(), (name2.clone()).clone())?),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                hasOtherComponents = !(var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone().is_empty());
                if hasOtherComponents.clone() {
                    assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                    eltold = elt.clone();
                }
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = list![compitem.clone()]);
                assign_variant_field!(typeSpec => Absyn::TypeSpec::TPATH; path = tppath.clone());
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; typeSpec = typeSpec.clone());
                assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                res = cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), xs.clone());
                if hasOtherComponents.clone() {
                    res = cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: eltold.clone() }), res.clone());
                }
                successResult = true;
            }
            (res.clone(), successResult.clone())
        },
        _ => {
            if (comps.clone().len() as i32) == 1 {
                res = xs.clone();
                successResult = true;
            } else {
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. } => !(stringEq((name.clone()).clone(), (name2.clone()).clone())?),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                res = cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), xs.clone());
                successResult = true;
            }
            (res.clone(), successResult.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            } else {
                (res, successResult) = deleteOrUpdateComponentFromElementitems((name.clone()).clone(), xs.clone(), item.clone())?;
                res = cons(x.clone(), res.clone());
            }
            (res.clone(), successResult.clone())
        },
        (name, Deref @ metamodelica::List::Cons { head: x, tail: xs }) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut successResult: bool = false;
            (res, successResult) = deleteOrUpdateComponentFromElementitems((name.clone()).clone(), xs.clone(), item.clone())?;
            (cons(x.clone(), res.clone()), successResult.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outAbsynElementItemLst, success))
}

pub fn addComponent(mut componentName: ArcStr, mut typeName: Arc<Absyn::Path>, mut classPath: Arc<Absyn::Path>, mut bindingExp: Arc<Absyn::Exp>, mut modifier: Arc<Absyn::Modification>, mut commentExp: Arc<Absyn::Exp>, mut annotationExp: Arc<Absyn::Exp>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut name: ArcStr = arcstr::literal!("");
    let mut filename: ArcStr = arcstr::literal!("");
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut annotation_: Option<Arc<Absyn::Comment>> = None;
    let mut modification: Option<Arc<Absyn::Modification>> = None;
    let mut w: Absyn::Within = Absyn::Within::TOP;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut redecl: Option<Absyn::RedeclareKeywords> = None;
    let mut attr: Absyn::ElementAttributes = <Absyn::ElementAttributes as ::std::default::Default>::default();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut ty_path: Arc<Absyn::Path>;
    match '__try0: {
        w = (::match_deref::match_deref! { match &(classPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => openmodelica_ast::Absyn::Within::TOP,
        _ => Absyn::Within::WITHIN { path: unwrap_break_err!(AbsynUtil::stripLast(classPath.clone()), '__try0) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        let (__pa2, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0)) {
            __pa2 @ Deref @ Absyn::Class { info: SourceInfo { fileName: __pa1, .. }, .. } => (__pa2.clone(), __pa1.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        filename = __pa1.clone();
        cdef = __pa2.clone();
        info = SourceInfo { fileName: (filename.clone()).clone(), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };
        annotation_ = unwrap_break_err!(InteractiveUtil::makeCommentFromArgs(commentExp.clone(), annotationExp.clone(), None), '__try0);
        modification = unwrap_break_err!(InteractiveUtil::makeModifierFromArgs(bindingExp.clone(), modifier.clone(), info.clone(), None), '__try0);
        (io, redecl, attr) = unwrap_break_err!(getDefaultPrefixes(program.clone(), typeName.clone()), '__try0);
        let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::pathStripSamePrefix(typeName.clone(), classPath.clone()), '__try0)) {
            Some(__pa3) => __pa3.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ty_path = __pa3.clone();
        if unwrap_break_err!(AbsynUtil::pathContains(classPath.clone(), (AbsynUtil::pathFirstIdent(ty_path.clone()).unwrap()).clone()), '__try0) {
            ty_path = typeName.clone();
        }
        cdef = unwrap_break_err!(InteractiveUtil::addToPublic(cdef.clone(), Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: redecl.clone(), innerOuter: io.clone(), specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: ty_path.clone(), arrayDim: None }), components: list![Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (componentName.clone()).clone(), arrayDim: metamodelica::nil(), modification: modification.clone() }, condition: None, comment: annotation_.clone() })] }), info: info.clone(), constrainClass: None }) })), '__try0);
        program = unwrap_break_err!(InteractiveUtil::updateProgram(Absyn::Program { classes: list![cdef.clone()], within_: w.clone() }, program.clone(), false), '__try0);
        Ok::<_, anyhow::Error>((annotation_.clone(), attr.clone(), cdef.clone(), filename.clone(), info.clone(), io.clone(), modification.clone(), program.clone(), redecl.clone(), ty_path.clone(), w.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10)) => {
            annotation_ = __try0_o0;
            attr = __try0_o1;
            cdef = __try0_o2;
            filename = __try0_o3;
            info = __try0_o4;
            io = __try0_o5;
            modification = __try0_o6;
            program = __try0_o7;
            redecl = __try0_o8;
            ty_path = __try0_o9;
            w = __try0_o10;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

fn getDefaultPrefixes(mut p: Absyn::Program, mut className: Arc<Absyn::Path>) -> Result<(Absyn::InnerOuter, Option<Absyn::RedeclareKeywords>, Absyn::ElementAttributes)> {
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut redecl: Option<Absyn::RedeclareKeywords> = None;
    let mut attr: Absyn::ElementAttributes = <Absyn::ElementAttributes as ::std::default::Default>::default();
    (io, redecl, attr) = (::match_deref::match_deref! { match &((p.clone(), className.clone())) {
        (_, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNamedAnnotationExp(className.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("defaultComponentPrefixes")).clone() }), Some((literal!("{}")).clone()), (std::sync::Arc::new(getDefaultComponentPrefixesModStr) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<ArcStr> + 'static>))?).clone();
            io = getDefaultInnerOuter((r#str.clone()).clone())?;
            redecl = getDefaultReplaceable((r#str.clone()).clone())?;
            redecl = makeReplaceableIfPartial(p.clone(), className.clone(), redecl.clone())?;
            attr = getDefaultAttr((r#str.clone()).clone())?;
            (io.clone(), redecl.clone(), attr.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((io, redecl, attr))
}

fn makeReplaceableIfPartial(mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut redecl: Option<Absyn::RedeclareKeywords>) -> Result<Option<Absyn::RedeclareKeywords>> {
    let mut new_redecl: Option<Absyn::RedeclareKeywords> = None;
    new_redecl = (::match_deref::match_deref! { match &((p.clone(), className.clone(), redecl.clone())) {
        (_, _, None) if (isPartial(className.clone(), p.clone())) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE),
        (_, _, None) => redecl.clone(),
        (_, _, Some(Absyn::RedeclareKeywords::REPLACEABLE)) => redecl.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(new_redecl)
}

fn getDefaultInnerOuter(mut r#str: ArcStr) -> Result<Absyn::InnerOuter> {
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    io = 'mc: {
        let __mc_input = r#str.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let (-1) = (System::stringFind((r#str.clone()).clone(), (literal!("inner")).clone())?) else { bail!("pattern mismatch") };
            let (-1) = (System::stringFind((r#str.clone()).clone(), (literal!("outer")).clone())?) else { bail!("pattern mismatch") };
            Ok(openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let (-1) = (System::stringFind((r#str.clone()).clone(), (literal!("outer")).clone())?) else { bail!("pattern mismatch") };
            Ok(openmodelica_ast::Absyn::InnerOuter::INNER)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let (-1) = (System::stringFind((r#str.clone()).clone(), (literal!("inner")).clone())?) else { bail!("pattern mismatch") };
            Ok(openmodelica_ast::Absyn::InnerOuter::OUTER)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(io)
}

fn getDefaultReplaceable(mut r#str: ArcStr) -> Result<Option<Absyn::RedeclareKeywords>> {
    let mut repl: Option<Absyn::RedeclareKeywords> = None;
    repl = 'mc: {
        let __mc_input = r#str.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let (-1) = (System::stringFind((r#str.clone()).clone(), (literal!("replaceable")).clone())?) else { bail!("pattern mismatch") };
            Ok(None)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind((r#str.clone()).clone(), (literal!("replaceable")).clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(repl)
}

fn getDefaultAttr(mut r#str: ArcStr) -> Result<Absyn::ElementAttributes> {
    let mut attr: Absyn::ElementAttributes = <Absyn::ElementAttributes as ::std::default::Default>::default();
    attr = 'mc: {
        let __mc_input = r#str.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind((r#str.clone()).clone(), (literal!("parameter")).clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind((r#str.clone()).clone(), (literal!("constant")).clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind((r#str.clone()).clone(), (literal!("discrete")).clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::DISCRETE, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() })
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(attr)
}

pub fn getDefaultComponentPrefixesModStr(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut docStr: ArcStr = arcstr::literal!("");
    docStr = ('mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut docStr: ArcStr = docStr.clone();
                    docStr = (Dump::printExpStr(e.clone())?).clone();
                    Ok(docStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(docStr)
}

pub fn updateComponent(mut componentName: ArcStr, mut typeName: Arc<Absyn::Path>, mut classPath: Arc<Absyn::Path>, mut bindingExp: Arc<Absyn::Exp>, mut modifier: Arc<Absyn::Modification>, mut commentExp: Arc<Absyn::Exp>, mut annotationExp: Arc<Absyn::Exp>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut protlst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut arrayDimensions: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut r#mod: Option<Arc<Absyn::Modification>> = None;
    let mut modification: Option<Arc<Absyn::Modification>> = None;
    let mut cond: Option<Arc<Absyn::Exp>> = None;
    let mut ann: Option<Arc<Absyn::Comment>> = None;
    let mut annotation_: Option<Arc<Absyn::Comment>> = None;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        parts = __pa1.clone();
        publst = InteractiveUtil::getPublicList(parts.clone());
        protlst = InteractiveUtil::getProtectedList(parts.clone());
        let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(getElementContainsName(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (componentName.clone()).clone(), subscripts: metamodelica::nil() }), listAppend(publst.clone(), protlst.clone())), '__try0)) {
            Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, components: __pa3 }, info: _, constrainClass: _ } => __pa3.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        items = __pa3.clone();
        let (__pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(unwrap_break_err!(getCompitemNamed(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (componentName.clone()).clone(), subscripts: metamodelica::nil() }), items.clone()), '__try0)) {
            Deref @ Absyn::ComponentItem { component: Absyn::Component { name: _, arrayDim: __pa5, modification: __pa6 }, condition: __pa7, comment: __pa8 } => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        arrayDimensions = __pa5.clone();
        r#mod = __pa6.clone();
        cond = __pa7.clone();
        ann = __pa8.clone();
        annotation_ = unwrap_break_err!(InteractiveUtil::makeCommentFromArgs(commentExp.clone(), annotationExp.clone(), ann.clone()), '__try0);
        modification = unwrap_break_err!(InteractiveUtil::makeModifierFromArgs(bindingExp.clone(), modifier.clone(), Absyn::dummyInfo.clone(), r#mod.clone()), '__try0);
        program = unwrap_break_err!(deleteOrUpdateComponent((componentName.clone()).clone(), classPath.clone(), program.clone(), Some((typeName.clone(), Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (componentName.clone()).clone(), arrayDim: arrayDimensions.clone(), modification: modification.clone() }, condition: cond.clone(), comment: annotation_.clone() })))), '__try0);
        Ok::<_, anyhow::Error>((ann.clone(), annotation_.clone(), arrayDimensions.clone(), cond.clone(), items.clone(), r#mod.clone(), modification.clone(), parts.clone(), program.clone(), protlst.clone(), publst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10)) => {
            ann = __try0_o0;
            annotation_ = __try0_o1;
            arrayDimensions = __try0_o2;
            cond = __try0_o3;
            items = __try0_o4;
            r#mod = __try0_o5;
            modification = __try0_o6;
            parts = __try0_o7;
            program = __try0_o8;
            protlst = __try0_o9;
            publst = __try0_o10;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

pub fn deleteComponent(mut componentName: ArcStr, mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    match '__try0: {
        program = unwrap_break_err!(deleteOrUpdateComponent((componentName.clone()).clone(), classPath.clone(), program.clone(), None), '__try0);
        Ok::<_, anyhow::Error>((program.clone(),))
    } {
        Ok((__try0_o0,)) => {
            program = __try0_o0;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

pub fn addClassAnnotation(mut inClass: Arc<Absyn::ComponentRef>, mut inAnnotation: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut class_path: Arc<Absyn::Path>;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut class_within: Absyn::Within = Absyn::Within::TOP;
    class_path = AbsynUtil::crefToPath(inClass.clone())?;
    cls = InteractiveUtil::getPathedClassInProgram(class_path.clone(), inProgram.clone(), false, false)?;
    cls = addClassAnnotationToClass(cls.clone(), InteractiveUtil::annotationListToAbsyn(inAnnotation.clone())?)?;
    class_within = if (AbsynUtil::pathIsIdent(class_path.clone())) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(class_path.clone())? }};
    outProgram = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: class_within.clone() }, inProgram.clone(), false)?;
    Ok(outProgram)
}

pub fn addClassAnnotationToClass(mut inClass: Arc<Absyn::Class>, mut inAnnotation: Arc<Absyn::Annotation>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut body: Arc<Absyn::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    body = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; ann = list![AbsynUtil::mergeAnnotationsList(inAnnotation.clone(), var_field!((*body).ann, Absyn::ClassDef::PARTS).clone())?]);
            body.clone()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::DERIVED; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation.clone(), var_field!((*body).comment, Absyn::ClassDef::DERIVED).clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::ENUMERATION; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation.clone(), var_field!((*body).comment, Absyn::ClassDef::ENUMERATION).clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::OVERLOAD; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation.clone(), var_field!((*body).comment, Absyn::ClassDef::OVERLOAD).clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; ann = list![AbsynUtil::mergeAnnotationsList(inAnnotation.clone(), var_field!((*body).ann, Absyn::ClassDef::CLASS_EXTENDS).clone())?]);
            body.clone()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PDER; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation.clone(), var_field!((*body).comment, Absyn::ClassDef::PDER).clone())?);
            body.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    outClass = AbsynUtil::setClassBody(inClass.clone(), body.clone())?;
    Ok(outClass)
}

fn getInheritedClassesHelper(mut inClass1: Arc<SCode::Element>, mut inClass2: Arc<Absyn::Class>, mut inEnv4: FCore::Graph) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = (inClass1.clone(), inClass2.clone(), inEnv4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c @ Deref @ SCode::Element::CLASS { restriction: restr, encapsulatedPrefix: encflag, name: id, .. }, cdef, env) => {
                    let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut env2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    ErrorExt::setCheckpoint((literal!("getInheritedClassesHelper")).clone());
                    if SCodeUtil::isDerivedClass(c.clone()) {
                        env_2 = env.clone();
                    } else {
                        env2 = FGraph::openScope(env.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
                        ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env2.clone())?)?;
                        (_, env_2, _, _, _) = Inst::partialInstClassIn(FCore::emptyCache(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    }
                    lst = getBaseClasses(cdef.clone(), env_2.clone())?;
                    ErrorExt::rollBack((literal!("getInheritedClassesHelper")).clone());
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    ErrorExt::rollBack((literal!("getInheritedClassesHelper")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynComponentRefLst)
}

pub fn getInheritedClasses(mut inPath: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    if Flags::isSet(Flags::NF_API.clone())? {
        outPaths = NFApi::getInheritedClasses(inPath.clone(), SymbolTable::getAbsyn())?;
        return Ok(outPaths);
    }
    match '__try0: {
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::setCheckpoint((literal!("getInheritedClasses")).clone());
        }
        outPaths = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                modelpath => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut p_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    cdef = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(modelpath.clone(), SymbolTable::getAbsyn(), false, false), '__try0);
                    p_1 = unwrap_break_err!(SymbolTable::getSCode(), '__try0);
                    (cache, env) = unwrap_break_err!(Inst::makeEnvFromProgram(p_1.clone()), '__try0);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(Lookup::lookupClass(cache.clone(), env.clone(), modelpath.clone(), None), '__try0)) {
                        (_, __pa0 @ Deref @ SCode::Element::CLASS { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    c = __pa0.clone();
                    env_1 = __pa1.clone();
                    paths = unwrap_break_err!(getInheritedClassesHelper(c.clone(), cdef.clone(), env_1.clone()), '__try0);
                    if '__try2: {
                        ::match_deref::match_deref! { match &(paths.clone()) {
                            Deref @ metamodelica::List::Nil => (),
                            _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(paths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                modelpath => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut extendsLst: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    cdef = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(modelpath.clone(), SymbolTable::getAbsyn(), false, false), '__try0);
                    extendsLst = getExtendsInClass(cdef.clone());
                    paths = List::map(extendsLst.clone(), (std::sync::Arc::new(AbsynUtil::elementSpecToPath) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementSpec>) -> Result<Arc<Absyn::Path>> + 'static>));
                    Ok(paths.clone())
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
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::rollBack((literal!("getInheritedClasses")).clone());
        }
        Ok::<_, anyhow::Error>((outPaths.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outPaths = __try0_o0;
        }
        Err(_) => {
            if !(Flags::isSet(Flags::NF_API_NOISE.clone())?) {
                ErrorExt::rollBack((literal!("getInheritedClasses")).clone());
            }
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(outPaths)
}

pub fn getInheritanceCount(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut count: i32 = 0;
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        result = ValuesMake::makeInteger(countBaseClasses(cls.clone()));
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeInteger(0);
        }
    }
    result
}

fn getNthInheritedClassAnnotationOpt(mut inModelPath: Arc<Absyn::Path>, mut inInteger: i32, mut inClass: Arc<Absyn::Class>, mut inProgram: Absyn::Program) -> Result<(ArcStr, Option<Arc<Absyn::Annotation>>)> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut annotationOpt: Option<Arc<Absyn::Annotation>> = None;
    (outString, annotationOpt) = 'mc: {
        let __mc_input = (inModelPath.clone(), inInteger.clone(), inClass.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, n, _, _) => {
                    let mut path: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut extends_: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut annOpt: Option<Arc<Absyn::Annotation>> = None;
                    cdef = inClass.clone();
                    extends_ = getExtendsInClass(cdef.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((extends_.clone()).get(n.clone())?) {
                        Deref @ Absyn::ElementSpec::EXTENDS { path: __pa0, elementArg: _, annotationOpt: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    annOpt = __pa1.clone();
                    s = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Ok((s.clone(), annOpt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((literal!("Error"), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outString, annotationOpt))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getMapAnnotationStr(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inMapType: ArcStr, mut inClass: Arc<Absyn::Class>, mut inFullProgram: Absyn::Program, mut inModelPath: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inAbsynElementArgLst.clone(), inMapType.clone(), inClass.clone(), inFullProgram.clone(), inModelPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(literal!("{}"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ann @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: mapType }, .. }, tail: _ }, _, _, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (stringEqual((mapType.clone()).clone(), (inMapType.clone()).clone())) else { bail!("pattern mismatch") };
                    r#str = (getAnnotationString(Arc::new(Absyn::Annotation { elementArgs: list![ann.clone()] }), inClass.clone(), inFullProgram.clone(), inModelPath.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _, _, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getMapAnnotationStr(xs.clone(), (inMapType.clone()).clone(), inClass.clone(), inFullProgram.clone(), inModelPath.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn getNthInheritedClassIconMapAnnotation(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        result = getNthInheritedClassMapAnnotation(classPath.clone(), n.clone(), program.clone(), (literal!("IconMap")).clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = n.clone(); move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }), true, false, Access::icon.clone())?;
    Ok(result)
}

pub fn getNthInheritedClassDiagramMapAnnotation(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        result = getNthInheritedClassMapAnnotation(classPath.clone(), n.clone(), program.clone(), (literal!("DiagramMap")).clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = n.clone(); move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }), true, false, Access::icon.clone())?;
    Ok(result)
}

fn getNthInheritedClassMapAnnotation(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program, mut mapType: ArcStr) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut s: ArcStr = arcstr::literal!("");
    let mut annStr: ArcStr = arcstr::literal!("");
    let mut opt_ann: Option<Arc<Absyn::Annotation>> = None;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
    (s, opt_ann) = getNthInheritedClassAnnotationOpt(classPath.clone(), n.clone(), cls.clone(), program.clone())?;
    annStr = ((::match_deref::match_deref! { match &(opt_ann.clone()) {
        Some(Deref @ Absyn::Annotation { elementArgs: args }) => getMapAnnotationStr(args.clone(), (mapType.clone()).clone(), cls.clone(), program.clone(), classPath.clone())?,
        _ => literal!("{}"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    result = InteractiveUtil::makeAnnotationArrayValue(list![(s.clone()).clone(), (annStr.clone()).clone()])?;
    Ok(result)
}

fn getExtendsInClass(mut inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outExtends = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => getExtendsInParts(parts.clone()),
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => getExtendsInParts(parts.clone()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExtends
}

fn getExtendsInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> {
    let mut outExtends: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    let mut spec: Arc<Absyn::ElementSpec>;
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        for mut el in &*AbsynUtil::getElementItemsInClassPart(part.clone()) {
            let mut el = el.clone();
            outExtends = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::EXTENDS { .. }, .. } } => cons(spec.clone(), outExtends.clone()),
        _ => outExtends.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    outExtends = Dangerous::listReverseInPlace(outExtends.clone());
    outExtends
}

pub fn getComponentCount(mut model_: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<i32> {
    let mut count: i32 = 0;
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    cdef = InteractiveUtil::getPathedClassInProgram(model_.clone(), p.clone(), false, false)?;
    count = countComponents(cdef.clone())?;
    Ok(count)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn countComponents(mut inClass: Arc<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elt }, tail: lst }, .. }, .. } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(elt.clone(), 0);
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elt }, tail: lst }, .. }, .. } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(elt.clone(), 0);
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: Deref @ metamodelica::List::Cons { head: _, tail: lst }, .. }, .. } => {
                    let mut res: i32 = 0;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    res = countComponents(cdef.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::List::Nil, .. }, .. } => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elt }, tail: lst }, .. }, .. } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(elt.clone(), 0);
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elt }, tail: lst }, .. }, .. } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(elt.clone(), 0);
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts: Deref @ metamodelica::List::Cons { head: _, tail: lst }, .. }, .. } => {
                    let mut res: i32 = 0;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    res = countComponents(cdef.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::List::Nil, .. }, .. } => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
                    Ok(-1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn countComponentsInElts(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inInteger: i32) -> i32 {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inAbsynElementItemLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: complst, .. }, .. } }, tail: lst } => {
            countComponentsInElts(lst.clone(), inInteger.clone() + (complst.clone().len() as i32))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: lst } => {
            countComponentsInElts(lst.clone(), inInteger.clone())
        },
        Deref @ metamodelica::List::Nil => {
            inInteger.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInteger
}

pub fn getNthComponent(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut n: i32) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut genv: GraphicEnvCache;
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    match '__try0: {
        genv = unwrap_break_err!(InteractiveUtil::createEnvironment(SymbolTable::getAbsyn(), Some(SymbolTable::getSCode().unwrap()), classPath.clone()), '__try0);
        cdef = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        result = unwrap_break_err!(getNthComponent2(cdef.clone(), n.clone(), genv.clone()), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

fn getNthComponent2(mut inClass: Arc<Absyn::Class>, mut n: i32, mut genv: GraphicEnvCache) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut comp: Arc<Absyn::Element>;
    let mut comp_name: ArcStr = arcstr::literal!("");
    let mut cmt: ArcStr = arcstr::literal!("");
    let mut ty: Arc<Absyn::Path>;
    comp = InteractiveUtil::getNthComponentInClass(inClass.clone(), n.clone());
    (comp_name, ty, cmt) = getComponentInfoOld(comp.clone(), genv.clone())?;
    result = Arc::new(Values::Value::ARRAY { valueLst: list![ValuesMake::makeCodeTypeName(ty.clone()), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (comp_name.clone()).clone(), subscripts: metamodelica::nil() }) }) }), ValuesMake::makeString((cmt.clone()).clone())], dimLst: list![3] });
    Ok(result)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn useQuotes(mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inAbsynNamedArgLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argValue: Deref @ Absyn::Exp::BOOL { value: b }, argName: Deref @ "useQuotes" }, tail: _ } => {
            b.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: al } => {
            let mut res: bool = false;
            res = useQuotes(al.clone());
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn insertQuotesToList(mut inStringList: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringList = (::match_deref::match_deref! { match &(inStringList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: r#str, tail: rest } => {
            let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut str_1: ArcStr = arcstr::literal!("");
            str_1 = stringAppendList(list![(literal!("\"")).clone(), (r#str.clone()).clone(), (literal!("\"")).clone()]);
            res = insertQuotesToList(rest.clone());
            cons((str_1.clone()).clone(), res.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStringList
}

pub fn getComponents(mut classPath: Arc<Absyn::Path>, mut useQuotes: bool, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = getElements(classPath.clone(), useQuotes.clone(), program.clone(), true)?;
    Ok(result)
}

pub fn getElements(mut classPath: Arc<Absyn::Path>, mut useQuotes: bool, mut program: Absyn::Program, mut onlyComponents: bool) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut access: Access = Access::hide;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut env: GraphicEnvCache;
    let mut silent: bool = false;
    let mut infos: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut elems: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    match '__try0: {
        access = checkAccessAnnotationAndEncryption(classPath.clone(), program.clone());
        if access.clone() < Access::icon.clone() {
            unwrap_break_err!(Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil()), '__try0);
            result = unwrap_break_err!(ValuesMake::makeArray(metamodelica::nil()), '__try0);
            return Ok(result);
        }
        silent = !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0));
        if silent.clone() {
            ErrorExt::setCheckpoint((literal!("Interactive.getElements")).clone());
        }
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        env = unwrap_break_err!(InteractiveUtil::createEnvironment(program.clone(), Some(SymbolTable::getSCode()?), classPath.clone()), '__try0);
        if access.clone() >= Access::diagram.clone() {
            elems = InteractiveUtil::getProtectedElementsInClass(cls.clone());
            infos = unwrap_break_err!(InteractiveUtil::getElementsInfo(elems.clone(), false, useQuotes.clone(), onlyComponents.clone(), env.clone(), metamodelica::nil()), '__try0);
        }
        elems = InteractiveUtil::getPublicElementsInClass(cls.clone());
        infos = unwrap_break_err!(InteractiveUtil::getElementsInfo(elems.clone(), true, useQuotes.clone(), onlyComponents.clone(), env.clone(), infos.clone()), '__try0);
        result = unwrap_break_err!(ValuesMake::makeArray(infos.clone()), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeArray(metamodelica::nil())?;
        }
    }
    if silent.clone() {
        ErrorExt::rollBack((literal!("Interactive.getElements")).clone());
    }
    Ok(result)
}

pub fn getComponentAnnotations(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        let mut comps: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
        let mut access: Access = Access::hide;
        cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        if accessLevel.clone() >= Access::diagram.clone() {
            comps = InteractiveUtil::getProtectedComponentsInClass(cdef.clone());
        }
        comps = listAppend(InteractiveUtil::getPublicComponentsInClass(cdef.clone()), comps.clone());
        result = InteractiveUtil::getElementAnnotationsFromElts(comps.clone(), cdef.clone(), program.clone(), classPath.clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, false, Access::icon.clone())?;
    Ok(result)
}

pub fn getElementAnnotations(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        let mut elts: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
        cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        if accessLevel.clone() >= Access::diagram.clone() {
            elts = InteractiveUtil::getProtectedElementsInClass(cdef.clone());
        }
        elts = listAppend(InteractiveUtil::getPublicElementsInClass(cdef.clone()), elts.clone());
        result = InteractiveUtil::getElementAnnotationsFromElts(elts.clone(), cdef.clone(), program.clone(), classPath.clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, false, Access::icon.clone())?;
    Ok(result)
}

pub fn getNthComponentAnnotation(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        let mut comp: Arc<Absyn::Element>;
        cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        comp = InteractiveUtil::getNthComponentInClass(cdef.clone(), n.clone());
        result = InteractiveUtil::getElementAnnotationsFromElts(list![comp.clone()], cdef.clone(), program.clone(), classPath.clone())?;
        if ValuesUtil::isArray(result.clone()) && ValuesUtil::arraySize(result.clone())? == 1 {
            result = ValuesUtil::arrayScalar(result.clone())?;
        }
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = n.clone(); move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }), true, false, Access::icon.clone())?;
    Ok(result)
}

pub fn getNthComponentModification(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut comp: Arc<Absyn::Element>;
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        comp = InteractiveUtil::getNthComponentInClass(cls.clone(), n.clone());
        result = unwrap_break_err!(getComponentModification(comp.clone()), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub fn getNthComponentCondition(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut comp: Arc<Absyn::Element>;
    let mut r#str: ArcStr = arcstr::literal!("");
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        comp = InteractiveUtil::getNthComponentInClass(cls.clone(), n.clone());
        r#str = (unwrap_break_err!(getComponentCondition(comp.clone()), '__try0)).clone();
        r#str = (System::trim((r#str.clone()).clone(), (literal!(" ")).clone())).clone();
        result = ValuesMake::makeString((r#str.clone()).clone());
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

fn getComponentCondition(mut inElement: Arc<Absyn::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: lst, .. }, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getComponentitemsCondition(lst.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getComponentitemsCondition(mut inAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAbsynComponentItemLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { condition: cond, .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (Dump::unparseComponentCondition(cond.clone())?).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn getNthConnection(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inProgram: Absyn::Program, mut inInteger: i32) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValue: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValue = 'mc: {
        let __mc_input = (inComponentRef.clone(), inProgram.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (model_, p, n) => {
                    let mut modelpath: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut eq: Arc<Absyn::Equation>;
                    let mut cmt: Option<Arc<Absyn::Comment>> = None;
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    modelpath = AbsynUtil::crefToPath(model_.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((getConnections(cdef.clone())?).get(n.clone())?) {
                        Deref @ Absyn::EquationItem::EQUATIONITEM { comment: __pa0, equation_: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cmt = __pa0.clone();
                    eq = __pa1.clone();
                    r#str = (getStringComment(cmt.clone())).clone();
                    (s1, s2) = getConnectionStr(eq.clone())?;
                    vals = list![Arc::new(Values::Value::STRING { string: (s1.clone()).clone() }), Arc::new(Values::Value::STRING { string: (s2.clone()).clone() }), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() })];
                    Ok(vals.clone())
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
    Ok(outValue)
}

pub fn getStringComment(mut inAbsynCommentOption: Option<Arc<Absyn::Comment>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAbsynCommentOption.clone()) {
        Some(Deref @ Absyn::Comment { annotation_: _, comment: Some(r#str) }) => {
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn addConnection(mut classPath: Arc<Absyn::Path>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut commentExp: Arc<Absyn::Exp>, mut annotationExp: Arc<Absyn::Exp>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = false;
    let mut eq: Arc<Absyn::EquationItem>;
    let mut cmt: Option<Arc<Absyn::Comment>> = None;
    match '__try0: {
        cmt = unwrap_break_err!(InteractiveUtil::makeCommentFromArgs(commentExp.clone(), annotationExp.clone(), None), '__try0);
        eq = Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_CONNECT { connector1: connector1.clone(), connector2: connector2.clone() }), comment: cmt.clone(), info: Absyn::dummyInfo.clone() });
        program = unwrap_break_err!(transformPathedClassInProgram(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = eq.clone(); move |__pe_a0| InteractiveUtil::addToEquation(__pe_a0, __pe_b1.clone()) })), '__try0);
        success = true;
        Ok::<_, anyhow::Error>((cmt.clone(), eq.clone(), program.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cmt = __try0_o0;
            eq = __try0_o1;
            program = __try0_o2;
            success = __try0_o3;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

pub fn deleteConnection(mut classPath: Arc<Absyn::Path>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = false;
    match '__try0: {
        program = unwrap_break_err!(transformPathedClassInProgram(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); move |__pe_a0| deleteConnectionInClass(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })), '__try0);
        success = true;
        Ok::<_, anyhow::Error>((program.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            program = __try0_o0;
            success = __try0_o1;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

fn deleteConnectionInClass(mut cls: Arc<Absyn::Class>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::ClassDef>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            let mut cdef = (*cdef).clone();
            eqlst = InteractiveUtil::getEquationList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone())?;
            eqlst = deleteEquationInEqlist(eqlst.clone(), connector1.clone(), connector2.clone())?;
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = InteractiveUtil::replaceEquationList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), eqlst.clone())?);
            assign_field!(cls.body = cdef.clone());
            ()
        },
        Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            let mut cdef = (*cdef).clone();
            eqlst = InteractiveUtil::getEquationList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?;
            eqlst = deleteEquationInEqlist(eqlst.clone(), connector1.clone(), connector2.clone())?;
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = InteractiveUtil::replaceEquationList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), eqlst.clone())?);
            assign_field!(cls.body = cdef.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deleteEquationInEqlist(mut inAbsynEquationItemLst1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outAbsynEquationItemLst = (::match_deref::match_deref! { match &((inAbsynEquationItemLst1.clone(), inComponentRef2.clone(), inComponentRef3.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector2: cn2, connector1: cn1 }, .. }, tail: xs }, c1, c2) if (AbsynUtil::crefEqual(c1.clone(), cn1.clone()) && AbsynUtil::crefEqual(c2.clone(), cn2.clone())) => {
            deleteEquationInEqlist(xs.clone(), c1.clone(), c2.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_FOR { iterators: forIterator, forEquations: forEqList }, .. }, tail: xs }, c1, c2) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut loopRes: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            res = deleteEquationInEqlist(xs.clone(), c1.clone(), c2.clone())?;
            loopRes = deleteEquationInEqlist(forEqList.clone(), c1.clone(), c2.clone())?;
            if !(loopRes.clone().is_empty()) {
                loopRes = list![Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_FOR { iterators: forIterator.clone(), forEquations: loopRes.clone() }), comment: None, info: Absyn::dummyInfo.clone() })];
            }
            listAppend(loopRes.clone(), res.clone())
        },
        (Deref @ metamodelica::List::Cons { head: x, tail: xs }, c1, c2) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            res = deleteEquationInEqlist(xs.clone(), c1.clone(), c2.clone())?;
            cons(x.clone(), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynEquationItemLst)
}

pub fn addTransition(mut inComponentRef: Arc<Absyn::ComponentRef>, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32, mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut b: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (b, outProgram) = addTransitionWithAnnotation(inComponentRef.clone(), (from.clone()).clone(), (to.clone()).clone(), (condition.clone()).clone(), immediate.clone(), reset.clone(), synchronize.clone(), priority.clone(), InteractiveUtil::annotationListToAbsyn(inAbsynNamedArgLst.clone())?, inProgram.clone())?;
    Ok((b, outProgram))
}

pub fn addTransitionWithAnnotation(mut inComponentRef: Arc<Absyn::ComponentRef>, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32, mut inAnnotation: Arc<Absyn::Annotation>, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut b: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (b, outProgram) = (::match_deref::match_deref! { match &((inComponentRef.clone(), from.clone(), to.clone(), condition.clone(), immediate.clone(), reset.clone(), synchronize.clone(), priority.clone(), inAnnotation.clone(), inProgram.clone())) {
        (model_ @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_, ann, p @ Absyn::Program { .. }) => {
            let mut modelpath: Arc<Absyn::Path>;
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut newcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut cmt: Option<Arc<Absyn::Comment>> = None;
            let mut conditionExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            modelpath = AbsynUtil::crefToPath(model_.clone())?;
            cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
            cmt = Some(Arc::new(Absyn::Comment { annotation_: Some(ann.clone()), comment: None }));
            let GlobalScript::ISTMTS { interactiveStmtLst: metamodelica::List::Cons { head: GlobalScript::IEXP { exp: __pa0, info: _ }, tail: __t1 }, semicolon: _ } = (Parser::parsestringexp((condition_.clone()).clone(), (literal!("<interactive>")).clone())?) else { bail!("pattern mismatch") };
            ::match_deref::match_deref! { match &(__t1.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            conditionExp = __pa0.clone();
            newcdef = InteractiveUtil::addToEquation(cdef.clone(), Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("transition")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (from_.clone()).clone(), subscripts: metamodelica::nil() }) }), Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (to_.clone()).clone(), subscripts: metamodelica::nil() }) }), conditionExp.clone()], argNames: list![Arc::new(Absyn::NamedArg { argName: (literal!("immediate")).clone(), argValue: Arc::new(Absyn::Exp::BOOL { value: immediate_.clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("reset")).clone(), argValue: Arc::new(Absyn::Exp::BOOL { value: reset_.clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("synchronize")).clone(), argValue: Arc::new(Absyn::Exp::BOOL { value: synchronize_.clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("priority")).clone(), argValue: Arc::new(Absyn::Exp::INTEGER { value: priority_.clone() }) })] }) }), comment: cmt.clone(), info: Absyn::dummyInfo.clone() }))?;
            newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: p.within_.clone() }, p.clone(), false)?;
            (true, newp.clone())
        },
        (model_ @ Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_, ann, p @ Absyn::Program { .. }) => {
            let mut modelpath: Arc<Absyn::Path>;
            let mut package_: Arc<Absyn::Path>;
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut newcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut cmt: Option<Arc<Absyn::Comment>> = None;
            let mut conditionExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            modelpath = AbsynUtil::crefToPath(model_.clone())?;
            cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
            package_ = AbsynUtil::stripLast(modelpath.clone())?;
            cmt = Some(Arc::new(Absyn::Comment { annotation_: Some(ann.clone()), comment: None }));
            let GlobalScript::ISTMTS { interactiveStmtLst: metamodelica::List::Cons { head: GlobalScript::IEXP { exp: __pa0, info: _ }, tail: __t1 }, semicolon: _ } = (Parser::parsestringexp((condition_.clone()).clone(), (literal!("<interactive>")).clone())?) else { bail!("pattern mismatch") };
            ::match_deref::match_deref! { match &(__t1.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            conditionExp = __pa0.clone();
            newcdef = InteractiveUtil::addToEquation(cdef.clone(), Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("transition")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (from_.clone()).clone(), subscripts: metamodelica::nil() }) }), Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (to_.clone()).clone(), subscripts: metamodelica::nil() }) }), conditionExp.clone()], argNames: list![Arc::new(Absyn::NamedArg { argName: (literal!("immediate")).clone(), argValue: Arc::new(Absyn::Exp::BOOL { value: immediate_.clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("reset")).clone(), argValue: Arc::new(Absyn::Exp::BOOL { value: reset_.clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("synchronize")).clone(), argValue: Arc::new(Absyn::Exp::BOOL { value: synchronize_.clone() }) }), Arc::new(Absyn::NamedArg { argName: (literal!("priority")).clone(), argValue: Arc::new(Absyn::Exp::INTEGER { value: priority_.clone() }) })] }) }), comment: cmt.clone(), info: Absyn::dummyInfo.clone() }))?;
            newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: package_.clone() } }, p.clone(), false)?;
            (true, newp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((b, outProgram))
}

pub fn deleteTransition(mut inComponentRef1: Arc<Absyn::ComponentRef>, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut b: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (b, outProgram) = 'mc: {
        let __mc_input = (inComponentRef1.clone(), from.clone(), to.clone(), condition.clone(), immediate.clone(), reset.clone(), synchronize.clone(), priority.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (model_, from_, to_, condition_, immediate_, reset_, synchronize_, priority_, p @ Absyn::Program { .. }) => {
                    let mut modelpath: Arc<Absyn::Path>;
                    let mut modelwithin: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    modelpath = AbsynUtil::crefToPath(model_.clone())?;
                    modelwithin = AbsynUtil::stripLast(modelpath.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    newcdef = deleteTransitionInClass(cdef.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
                    newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: modelwithin.clone() } }, p.clone(), false)?;
                    Ok((true, newp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (model_, from_, to_, condition_, immediate_, reset_, synchronize_, priority_, p @ Absyn::Program { .. }) => {
                    let mut modelpath: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    modelpath = AbsynUtil::crefToPath(model_.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    newcdef = deleteTransitionInClass(cdef.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
                    newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), false)?;
                    Ok((true, newp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, p @ Absyn::Program { .. }) => {
                    Ok((false, p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((b, outProgram))
}

fn deleteTransitionInClass(mut inClass: Arc<Absyn::Class>, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &((inClass.clone(), from.clone(), to.clone(), condition.clone(), immediate.clone(), reset.clone(), synchronize.clone(), priority.clone())) {
        (outClass @ Deref @ Absyn::Class { info: _, body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_) => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = InteractiveUtil::getEquationList(parts.clone())?;
            eqlst_1 = deleteTransitionInEqlist(eqlst.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
            parts2 = InteractiveUtil::replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_) => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = InteractiveUtil::getEquationList(parts.clone())?;
            eqlst_1 = deleteTransitionInEqlist(eqlst.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
            parts2 = InteractiveUtil::replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deleteTransitionInEqlist(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outAbsynEquationItemLst = 'mc: {
        let __mc_input = (inAbsynEquationItemLst.clone(), from.clone(), to.clone(), condition.clone(), immediate.clone(), reset.clone(), synchronize.clone(), priority.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_NORETCALL { functionName: name, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expArgs, argNames: namedArgs } }, .. }, tail: xs }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_) => {
                    if !((AbsynUtil::crefEqual(name.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("transition")).clone(), subscripts: metamodelica::nil() })))) { bail!("guard") }
                    let mut args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut conditionExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut condition_ = (*condition_).clone();
                    args = List::map(expArgs.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>));
                    args = addOrUpdateNamedArg(namedArgs.clone(), (literal!("immediate")).clone(), (literal!("true")).clone(), args.clone(), 4)?;
                    args = addOrUpdateNamedArg(namedArgs.clone(), (literal!("reset")).clone(), (literal!("true")).clone(), args.clone(), 5)?;
                    args = addOrUpdateNamedArg(namedArgs.clone(), (literal!("synchronize")).clone(), (literal!("false")).clone(), args.clone(), 6)?;
                    args = addOrUpdateNamedArg(namedArgs.clone(), (literal!("priority")).clone(), (literal!("1")).clone(), args.clone(), 7)?;
                    let GlobalScript::ISTMTS { interactiveStmtLst: metamodelica::List::Cons { head: GlobalScript::IEXP { exp: __pa0, info: _ }, tail: __t1 }, semicolon: _ } = (Parser::parsestringexp((condition_.clone()).clone(), (literal!("<interactive>")).clone())?) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(__t1.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    conditionExp = __pa0.clone();
                    condition_ = (Dump::printExpStr(conditionExp.clone())?).clone();
                    let true = (compareTransitionFuncArgs(args.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?) else { bail!("pattern mismatch") };
                    Ok(deleteTransitionInEqlist(xs.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: x, tail: xs }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    res = deleteTransitionInEqlist(xs.clone(), (from_.clone()).clone(), (to_.clone()).clone(), (condition_.clone()).clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
                    Ok(cons(x.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynEquationItemLst)
}

pub fn addOrUpdateNamedArg(mut inNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut namedArg: ArcStr, mut defaultValue: ArcStr, mut inTransition: Arc<metamodelica::List<ArcStr>>, mut position: i32) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outTransition: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut namedArgValue: ArcStr = arcstr::literal!("");
    let mut isDefault: bool = false;
    (namedArgValue, isDefault) = namedArgValueAsString(inNamedArgLst.clone(), (namedArg.clone()).clone(), (defaultValue.clone()).clone())?;
    if (inTransition.clone().len() as i32) < position.clone() {
        outTransition = List::insert(inTransition.clone(), position.clone(), (namedArgValue.clone()).clone())?;
    } else if boolAnd((inTransition.clone().len() as i32) >= position.clone(), boolNot(isDefault.clone())) {
        outTransition = List::replaceAt((namedArgValue.clone()).clone(), position.clone(), inTransition.clone())?;
    } else {
        outTransition = inTransition.clone();
    }
    Ok(outTransition)
}

fn namedArgValueAsString(mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inNamedArg: ArcStr, mut inDefaultValue: ArcStr) -> Result<(ArcStr, bool)> {
    let mut outNamedArg: ArcStr = arcstr::literal!("");
    let mut outDefault: bool = false;
    (outNamedArg, outDefault) = (::match_deref::match_deref! { match &(inAbsynNamedArgLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inDefaultValue.clone(), true)
        },
        Deref @ metamodelica::List::Cons { head: namedArg @ Deref @ Absyn::NamedArg { argName: namedArgName, .. }, tail: _ } if (stringEq((namedArgName.clone()).clone(), (inNamedArg.clone()).clone())?) => {
            (Dump::printNamedArgValueStr(namedArg.clone())?, false)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: al } => {
            namedArgValueAsString(al.clone(), (inNamedArg.clone()).clone(), (inDefaultValue.clone()).clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outNamedArg, outDefault))
}

fn compareTransitionFuncArgs(mut args: Arc<metamodelica::List<ArcStr>>, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = (args.clone(), from.clone(), to.clone(), condition.clone(), immediate.clone(), reset.clone(), synchronize.clone(), priority.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: from1, tail: Deref @ metamodelica::List::Cons { head: to1, tail: Deref @ metamodelica::List::Cons { head: condition1, tail: Deref @ metamodelica::List::Nil } } }, from2, to2, condition2, _, _, _, _) => {
                    if !((stringEq((from1.clone()).clone(), (from2.clone()).clone())? && stringEq((to1.clone()).clone(), (to2.clone()).clone())? && stringEq((condition1.clone()).clone(), (condition2.clone()).clone())?)) { bail!("guard") }
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: from1, tail: Deref @ metamodelica::List::Cons { head: to1, tail: Deref @ metamodelica::List::Cons { head: condition1, tail: Deref @ metamodelica::List::Cons { head: immediate1, tail: Deref @ metamodelica::List::Nil } } } }, from2, to2, condition2, immediate2, _, _, _) => {
                    if !((stringEq((from1.clone()).clone(), (from2.clone()).clone())? && stringEq((to1.clone()).clone(), (to2.clone()).clone())? && stringEq((condition1.clone()).clone(), (condition2.clone()).clone())? && stringEq((immediate1.clone()).clone(), (boolString(immediate2.clone())).clone())?)) { bail!("guard") }
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: from1, tail: Deref @ metamodelica::List::Cons { head: to1, tail: Deref @ metamodelica::List::Cons { head: condition1, tail: Deref @ metamodelica::List::Cons { head: immediate1, tail: Deref @ metamodelica::List::Cons { head: reset1, tail: Deref @ metamodelica::List::Nil } } } } }, from2, to2, condition2, immediate2, reset2, _, _) => {
                    if !((stringEq((from1.clone()).clone(), (from2.clone()).clone())? && stringEq((to1.clone()).clone(), (to2.clone()).clone())? && stringEq((condition1.clone()).clone(), (condition2.clone()).clone())? && stringEq((immediate1.clone()).clone(), (boolString(immediate2.clone())).clone())? && stringEq((reset1.clone()).clone(), (boolString(reset2.clone())).clone())?)) { bail!("guard") }
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: from1, tail: Deref @ metamodelica::List::Cons { head: to1, tail: Deref @ metamodelica::List::Cons { head: condition1, tail: Deref @ metamodelica::List::Cons { head: immediate1, tail: Deref @ metamodelica::List::Cons { head: reset1, tail: Deref @ metamodelica::List::Cons { head: synchronize1, tail: Deref @ metamodelica::List::Nil } } } } } }, from2, to2, condition2, immediate2, reset2, synchronize2, _) => {
                    if !((stringEq((from1.clone()).clone(), (from2.clone()).clone())? && stringEq((to1.clone()).clone(), (to2.clone()).clone())? && stringEq((condition1.clone()).clone(), (condition2.clone()).clone())? && stringEq((immediate1.clone()).clone(), (boolString(immediate2.clone())).clone())? && stringEq((reset1.clone()).clone(), (boolString(reset2.clone())).clone())? && stringEq((synchronize1.clone()).clone(), (boolString(synchronize2.clone())).clone())?)) { bail!("guard") }
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: from1, tail: Deref @ metamodelica::List::Cons { head: to1, tail: Deref @ metamodelica::List::Cons { head: condition1, tail: Deref @ metamodelica::List::Cons { head: immediate1, tail: Deref @ metamodelica::List::Cons { head: reset1, tail: Deref @ metamodelica::List::Cons { head: synchronize1, tail: Deref @ metamodelica::List::Cons { head: priority1, tail: Deref @ metamodelica::List::Nil } } } } } } }, from2, to2, condition2, immediate2, reset2, synchronize2, priority2) => {
                    if !((stringEq((from1.clone()).clone(), (from2.clone()).clone())? && stringEq((to1.clone()).clone(), (to2.clone()).clone())? && stringEq((condition1.clone()).clone(), (condition2.clone()).clone())? && stringEq((immediate1.clone()).clone(), (boolString(immediate2.clone())).clone())? && stringEq((reset1.clone()).clone(), (boolString(reset2.clone())).clone())? && stringEq((synchronize1.clone()).clone(), (boolString(synchronize2.clone())).clone())? && stringEq((priority1.clone()).clone(), (intString(priority2.clone())).clone())?)) { bail!("guard") }
                    Ok(true)
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

pub fn getComponentComment(mut classPath: Arc<Absyn::Path>, mut componentName: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut comment: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut path: Arc<Absyn::Path>;
    let mut elem: Arc<Absyn::Element>;
    let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut comp: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
    let mut cmt: ArcStr = arcstr::literal!("");
    let mut comp_name: ArcStr = arcstr::literal!("");
    path = AbsynUtil::joinPaths(classPath.clone(), componentName.clone())?;
    comp_name = (AbsynUtil::pathLastIdent(componentName.clone())?).clone();
    elem = InteractiveUtil::getPathedElementInProgram(path.clone(), program.clone())?;
    comps = AbsynUtil::getComponentItemsFromElement(elem.clone());
    comp = List::find(comps.clone(), Arc::new({ let __pe_b0 = (comp_name.clone()).clone(); move |__pe_a1| Ok(AbsynUtil::isComponentItemNamed(__pe_b0.clone(), __pe_a1)) }))?;
    cmt = (InteractiveUtil::getClassCommentInCommentOpt(comp.comment.clone())).clone();
    comment = ValuesMake::makeString((cmt.clone()).clone());
    Ok(comment)
}

pub fn setComponentComment(mut classPath: Arc<Absyn::Path>, mut componentName: Arc<Absyn::Path>, mut comment: ArcStr, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = false;
    let mut path: Arc<Absyn::Path>;
    let mut comp_name: ArcStr = arcstr::literal!("");
    match '__try0: {
        path = unwrap_break_err!(AbsynUtil::joinPaths(classPath.clone(), componentName.clone()), '__try0);
        comp_name = (unwrap_break_err!(AbsynUtil::pathLastIdent(componentName.clone()), '__try0)).clone();
        (program, _, success) = unwrap_break_err!(InteractiveUtil::transformPathedElementInProgram(path.clone(), Arc::new({ let __pe_b1 = (comp_name.clone()).clone(); let __pe_b2 = (comment.clone()).clone(); move |__pe_a0| setComponentCommentInElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), program.clone()), '__try0);
        Ok::<_, anyhow::Error>((comp_name.clone(), path.clone(), program.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            comp_name = __try0_o0;
            path = __try0_o1;
            program = __try0_o2;
            success = __try0_o3;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

fn setComponentCommentInElement(mut element: Arc<Absyn::Element>, mut componentName: ArcStr, mut comment: ArcStr) -> Result<Arc<Absyn::Element>> {
    fn set_comment(mut item: Arc<Absyn::ComponentItem>, mut comment: ArcStr) -> Result<Arc<Absyn::ComponentItem>> {
        let mut item: Arc<Absyn::ComponentItem> = item;
        assign_field!(item.comment = AbsynUtil::setCommentString(item.comment.clone(), if (stringEmpty((comment.clone()).clone())) {None} else {Some((comment.clone()).clone())})?);
        Ok(item)
    }

    let mut element: Arc<Absyn::Element> = element;
    let mut spec: Arc<Absyn::ElementSpec>;
    let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => {
            let mut spec = (*spec).clone();
            let __pa0 = ::match_deref::match_deref! { match &(List::findAndMap(var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone(), Arc::new({ let __pe_b0 = (componentName.clone()).clone(); move |__pe_a1| Ok(AbsynUtil::isComponentItemNamed(__pe_b0.clone(), __pe_a1)) }), Arc::new({ let __pe_b1 = (comment.clone()).clone(); move |__pe_a0| set_comment(__pe_a0, __pe_b1.clone()) }))?) {
                (__pa0, true) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            comps = __pa0.clone();
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = comps.clone());
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setConnectionComment(mut classPath: Arc<Absyn::Path>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut comment: ArcStr, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = false;
    match '__try0: {
        (program, _, success) = unwrap_break_err!(InteractiveUtil::transformPathedElementInProgram(classPath.clone(), Arc::new({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); let __pe_b3 = (comment.clone()).clone(); move |__pe_a0| setConnectionCommentInElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }), program.clone()), '__try0);
        Ok::<_, anyhow::Error>((program.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            program = __try0_o0;
            success = __try0_o1;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

fn setConnectionCommentInElement(mut element: Arc<Absyn::Element>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut comment: ArcStr) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let mut spec: Arc<Absyn::ElementSpec>;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } => {
            let mut spec = (*spec).clone();
            cls = setConnectionCommentInClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), connector1.clone(), connector2.clone(), (comment.clone()).clone())?;
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cls.clone());
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

fn setConnectionCommentInClass(mut cls: Arc<Absyn::Class>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut comment: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut cdef = (*cdef).clone();
            parts = setConnectionCommentInParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), connector1.clone(), connector2.clone(), (comment.clone()).clone())?;
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
            assign_field!(cls.body = cdef.clone());
            ()
        },
        Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut cdef = (*cdef).clone();
            parts = setConnectionCommentInParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), connector1.clone(), connector2.clone(), (comment.clone()).clone())?;
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
            assign_field!(cls.body = cdef.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

fn setConnectionCommentInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut comment: ArcStr) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = parts;
    let __pa0 = ::match_deref::match_deref! { match &(List::findMap(parts.clone(), Arc::new({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); let __pe_b3 = (comment.clone()).clone(); move |__pe_a0| setConnectionCommentInEquationsPart(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?) {
        (__pa0, true) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    parts = __pa0.clone();
    Ok(parts)
}

fn setConnectionCommentInEquationsPart(mut part: Arc<Absyn::ClassPart>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut comment: ArcStr) -> Result<(Arc<Absyn::ClassPart>, bool)> {
    let mut part: Arc<Absyn::ClassPart> = part;
    let mut found: bool = false;
    let mut eql: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    (part, found) = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            (eql, found) = List::findMap(var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone(), Arc::new({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); let __pe_b3 = (comment.clone()).clone(); move |__pe_a0| setConnectionCommentInEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = eql.clone());
            (part.clone(), found.clone())
        },
        _ => (part.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((part, found))
}

fn setConnectionCommentInEquation(mut eq: Arc<Absyn::EquationItem>, mut connector1: Arc<Absyn::ComponentRef>, mut connector2: Arc<Absyn::ComponentRef>, mut comment: ArcStr) -> Result<(Arc<Absyn::EquationItem>, bool)> {
    let mut eq: Arc<Absyn::EquationItem> = eq;
    let mut success: bool = false;
    let mut c1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut c2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    success = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector2: c2, connector1: c1 }, .. } if (AbsynUtil::crefEqual(connector1.clone(), c1.clone()) && AbsynUtil::crefEqual(connector2.clone(), c2.clone())) => {
            assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; comment = AbsynUtil::setCommentString(var_field!((*eq).comment, Absyn::EquationItem::EQUATIONITEM).clone(), if (stringEmpty((comment.clone()).clone())) {None} else {Some((comment.clone()).clone())})?);
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, success))
}

pub fn getNthConnectionAnnotation(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        let mut conn: Arc<Absyn::EquationItem>;
        cdef = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        conn = (getConnections(cdef.clone())?).get(n.clone())?;
        result = getConnectionAnnotationStr(conn.clone(), cdef.clone(), program.clone(), classPath.clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = n.clone(); move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }), true, false, Access::diagram.clone())?;
    Ok(result)
}

pub fn getConnectorCount(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    match '__try0: {
        cdef = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        result = ValuesMake::makeInteger(countPublicConnectors(classPath.clone(), program.clone(), cdef.clone()).unwrap());
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub fn getNthConnector(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    let mut ty: Arc<Absyn::Path>;
    match '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(getNthPublicConnectorStr(classPath.clone(), cls.clone(), program.clone(), n.clone()), '__try0)) {
            (Some((__pa1, __pa2)), _) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        name = __pa1.clone();
        ty = __pa2.clone();
        result = unwrap_break_err!(ValuesMake::makeCodeTypeNameArray(list![Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), ty.clone()]), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub fn getNthConnectorIconAnnotation(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut n: i32, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        let mut comp: Arc<Absyn::Element>;
        let mut ty: Arc<Absyn::Path>;
        let mut r#str: ArcStr = arcstr::literal!("");
        cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        let __pa0 = ::match_deref::match_deref! { match &(getNthPublicConnectorStr(classPath.clone(), cls.clone(), program.clone(), n.clone())?) {
            (Some((_, __pa0)), _) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa0.clone();
        result = getIconAnnotation(ty.clone(), program.clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = n.clone(); move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }), true, false, Access::icon.clone())?;
    Ok(result)
}

pub fn getIconAnnotation(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        result = getNamedAnnotationValue(classPath.clone(), program.clone(), (literal!("Icon")).clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, true, Access::icon.clone())?;
    Ok(result)
}

pub fn refactorIconAnnotation(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        cls = Refactor::refactorGraphicalAnnotation(program.clone(), cls.clone())?;
        result = getNamedAnnotationValue(classPath.clone(), program.clone(), (literal!("Icon")).clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, true, Access::icon.clone())?;
    Ok(result)
}

pub fn getDiagramAnnotation(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        result = getNamedAnnotationValue(classPath.clone(), program.clone(), (literal!("Diagram")).clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, true, Access::icon.clone())?;
    Ok(result)
}

pub fn refactorDiagramAnnotation(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
        cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
        cls = Refactor::refactorGraphicalAnnotation(program.clone(), cls.clone())?;
        result = getNamedAnnotationValue(classPath.clone(), program.clone(), (literal!("Diagram")).clone())?;
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), (std::sync::Arc::new(r#impl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>), true, true, Access::icon.clone())?;
    Ok(result)
}

pub fn getNamedAnnotation(mut classPath: Arc<Absyn::Path>, mut annotationPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn r#impl(mut classPath: Arc<Absyn::Path>, mut annotationPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
        let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (getNamedAnnotationExp(classPath.clone(), program.clone(), annotationPath.clone(), Some((literal!("{}")).clone()), (std::sync::Arc::new(getAnnotationValue) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<ArcStr> + 'static>))?).clone();
        result = ValuesMake::makeCodeTypeNameStr((r#str.clone()).clone());
        Ok(result)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = InteractiveUtil::accessClass(classPath.clone(), program.clone(), Arc::new({ let __pe_b1 = annotationPath.clone(); move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }), true, false, Access::icon.clone())?;
    Ok(result)
}

pub fn getNamedAnnotationExp<T: Clone + 'static>(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut id: Arc<Absyn::Path>, mut default: Option<T>, mut f: Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<T> + 'static>) -> Result<T> {
    pub type ModFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<T> + 'static>;

    let mut outString: T;
    outString = 'mc: {
        let __mc_input = (inPath.clone(), inProgram.clone(), id.clone(), default.clone(), f.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p, _, _, _) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut r#str: T;
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    let Some(__pa0) = (AbsynUtil::getNamedAnnotationInClass(cdef.clone(), id.clone(), f.clone())?) else { bail!("pattern mismatch") };
                    r#str = __pa0.clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Some(r#str), _) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outString)
}

pub fn getStringNamedAnnotation(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut id: Arc<Absyn::Path>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    match '__try0: {
        let Absyn::STRING { value: __pa1 } = (unwrap_break_err!(getNamedAnnotationExp(inPath.clone(), inProgram.clone(), id.clone(), Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), (std::sync::Arc::new(getAnnotationExp) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<Absyn::Exp>> + 'static>)), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        outString = __pa1.clone();
        Ok::<_, anyhow::Error>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            outString = (literal!("")).clone();
        }
    }
    outString
}

pub fn getIntegerNamedAnnotation(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut id: Arc<Absyn::Path>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut exp: Option<Arc<Absyn::Exp>> = None;
    let mut ann: i32 = 0;
    match '__try0: {
        cdef = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(inPath.clone(), inProgram.clone(), false, false), '__try0);
        exp = unwrap_break_err!(AbsynUtil::getNamedAnnotationInClass(cdef.clone(), id.clone(), (std::sync::Arc::new(getAnnotationExp) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<Absyn::Exp>> + 'static>)), '__try0);
        if isSome(exp.clone()) {
            let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
                Some(Deref @ Absyn::Exp::INTEGER { value: __pa1 }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            ann = __pa1.clone();
            outString = (intString(ann.clone())).clone();
        } else {
            outString = (literal!("")).clone();
        }
        Ok::<_, anyhow::Error>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            outString = (literal!("")).clone();
        }
    }
    outString
}

pub fn getNamedAnnotationValue(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut name: ArcStr) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
    result = getNamedAnnotationValueInClass(classPath.clone(), cls.clone(), program.clone(), (name.clone()).clone())?;
    Ok(result)
}

pub fn getNamedAnnotationValueInClass(mut classPath: Arc<Absyn::Path>, mut cls: Arc<Absyn::Class>, mut program: Absyn::Program, mut name: ArcStr) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#mod: Option<Arc<Absyn::Modification>> = None;
    let mut arg: Arc<Absyn::ElementArg>;
    let mut r#str: ArcStr = arcstr::literal!("");
    r#mod = AbsynUtil::lookupClassAnnotation(cls.clone(), (name.clone()).clone())?;
    result = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { .. }) => {
            arg = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), modification: r#mod.clone(), comment: None, info: Absyn::dummyInfo.clone() });
            r#str = (getAnnotationString(Arc::new(Absyn::Annotation { elementArgs: list![arg.clone()] }), cls.clone(), program.clone(), classPath.clone())?).clone();
            InteractiveUtil::makeAnnotationArrayValue(list![(r#str.clone()).clone()])?
        },
        _ => ValuesMake::makeEmptyArray(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub static USES_PATH: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::IDENT { name: (literal!("uses")).clone() }) });

pub fn getUsesAnnotation(mut program: Absyn::Program) -> Result<Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>>> {
    let mut outUses: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    let mut opt_uses: Option<Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>>> = None;
    let mut uses: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let Absyn::PROGRAM { classes: __pa0, .. } = (program.clone()) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    for mut cls in &*classes.clone() {
        let mut cls = cls.clone();
        opt_uses = AbsynUtil::getNamedAnnotationInClass(cls.clone(), USES_PATH.clone(), Arc::new({ let __pe_b1 = (cls.name.clone()).clone(); move |__pe_a0| getUsesAnnotationString(__pe_a0, __pe_b1.clone()) }))?;
        if isSome(opt_uses.clone()) {
            let __pa1 = ::match_deref::match_deref! { match &(opt_uses.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            uses = __pa1.clone();
            outUses = listAppend(uses.clone(), outUses.clone());
        }
    }
    Ok(outUses)
}

pub fn getUsesAnnotationOrDefault(mut p: Absyn::Program, mut requireExactVersion: bool) -> Result<Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>>> {
    let mut usesStr: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut strs: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut fromVersions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    usesStr = getUsesAnnotation(p.clone())?;
    paths = List::map(usesStr.clone(), std::sync::Arc::new(fnptr!(Util::tuple41, _)));
    fromVersions = List::map(usesStr.clone(), std::sync::Arc::new(fnptr!(Util::tuple42, _)));
    strs = List::map(usesStr.clone(), std::sync::Arc::new(fnptr!(Util::tuple43, _)));
    usesStr = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
        for ((p, f), s) in (&(paths.clone())).into_iter().zip((&(fromVersions.clone())).into_iter()).zip((&(strs.clone())).into_iter()) {
            let __x = (p.clone(), f.clone(), s.clone(), false);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(usesStr)
}

fn getUsesAnnotationString(mut r#mod: Option<Arc<Absyn::Modification>>, mut classOrigin: ArcStr) -> Result<Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>>> {
    let mut usesStr: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    usesStr = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            getUsesAnnotationString2(arglst.clone(), (classOrigin.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(usesStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getUsesAnnotationString2(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut classOrigin: ArcStr) -> Result<Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>>> {
    let mut strs: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    strs = (::match_deref::match_deref! { match &(eltArgs.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: omod, path: Deref @ Absyn::Path::IDENT { name: Deref @ "version" }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }), path: Deref @ Absyn::Path::IDENT { name }, .. }, tail: xs } => {
            let mut version: ArcStr = arcstr::literal!("");
            let mut ss: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
            version = ((::match_deref::match_deref! { match &(omod.clone()) {
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: Deref @ Absyn::Exp::STRING { value: version }, .. }, .. }, .. }) => version.clone(),
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::STRING { value: version }, .. }, .. }) => version.clone(),
        _ => {
            Error::addSourceMessage(Error::USES_MISSING_VERSION.clone(), list![(name.clone()).clone()], info.clone())?;
            literal!("default")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
            ss = getUsesAnnotationString2(xs.clone(), (classOrigin.clone()).clone())?;
            cons((Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), classOrigin.clone(), list![(version.clone()).clone()], false), ss.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut ss: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
            ss = getUsesAnnotationString2(xs.clone(), (classOrigin.clone()).clone())?;
            ss.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(strs)
}

pub fn getUsedVersion(mut cls: Arc<Absyn::Class>, mut library: Arc<Absyn::Path>) -> Result<Option<ArcStr>> {
    let mut version: Option<ArcStr> = None;
    let mut uses: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    let mut lib: Arc<Absyn::Path>;
    let mut versions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    uses = getUsesAnnotationOrDefault(Absyn::Program { classes: list![cls.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, true)?;
    for mut u in &*uses.clone() {
        let mut u = u.clone();
        (lib, _, versions, _) = u.clone();
        if AbsynUtil::pathEqual(library.clone(), lib.clone()) {
            if !(versions.clone().is_empty()) {
                version = Some((listHead(versions.clone())?).clone());
                return Ok(version);
            }
        }
    }
    Ok(version)
}

pub fn updateUsedVersion(mut cls: Arc<Absyn::Class>, mut library: Arc<Absyn::Path>, mut newVersion: ArcStr) -> Result<Arc<Absyn::Class>> {
    fn make_version_exp(mut exp: Arc<Absyn::Exp>, mut version: ArcStr) -> Arc<Absyn::Exp> {
        let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::STRING { value: (version.clone()).clone() });
        outExp
    }

    let mut cls: Arc<Absyn::Class> = cls;
    let mut opt_ann: Option<Arc<Absyn::Annotation>> = None;
    let mut ann: Arc<Absyn::Annotation> = Arc::new(<Absyn::Annotation as ::std::default::Default>::default());
    let mut found: bool = false;
    opt_ann = AbsynUtil::getClassAnnotation(cls.clone())?;
    if isNone(opt_ann.clone()) {
        return Ok(cls);
    }
    let __pa0 = ::match_deref::match_deref! { match &(opt_ann.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ann = __pa0.clone();
    (ann, found) = AbsynUtil::mapAnnotationBinding(ann.clone(), AbsynUtil::prefixPath((literal!("uses")).clone(), AbsynUtil::joinPaths(library.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("version")).clone() }))?), Arc::new({ let __pe_b1 = (newVersion.clone()).clone(); move |__pe_a0| Ok(make_version_exp(__pe_a0, __pe_b1.clone())) }))?;
    if found.clone() {
        cls = AbsynUtil::setClassAnnotation(cls.clone(), Some(ann.clone()))?;
    }
    Ok(cls)
}

pub fn getConversionAnnotation(mut cls: Arc<Absyn::Class>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut withoutConversion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut withConversion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut opt_conversion: Option<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> = None;
    opt_conversion = AbsynUtil::getNamedAnnotationInClass(cls.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("conversion")).clone() }), (std::sync::Arc::new(getConversionAnnotationString) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> + 'static>))?;
    (withoutConversion, withConversion) = (::match_deref::match_deref! { match &(opt_conversion.clone()) {
        Some((withoutConversion, withConversion)) => (withoutConversion.clone(), withConversion.clone()),
        _ => (metamodelica::nil(), metamodelica::nil()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((withoutConversion, withConversion))
}

fn getConversionAnnotationString(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut result: (Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>);
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut without: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut with: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut from: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut script: Option<ArcStr> = None;
    let __pa0 = ::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        (from, _, script) = parseConversionAnnotationElement(arg.clone())?;
        if isNone(script.clone()) {
            without = List::append_reverse(from.clone(), without.clone());
        } else {
            with = List::append_reverse(from.clone(), with.clone());
        }
    }
    result = (without.clone().reverse(), with.clone().reverse());
    Ok(result)
}

pub fn getConversionsInClass(mut cls: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>> {
    let mut result: Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>> = metamodelica::nil();
    let mut res: Option<Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>> = None;
    res = AbsynUtil::getNamedAnnotationInClass(cls.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("conversion")).clone() }), (std::sync::Arc::new(getConversionsInClassMod) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>> + 'static>))?;
    result = Util::getOptionOrDefault(res.clone(), metamodelica::nil());
    Ok(result)
}

fn getConversionsInClassMod(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>> {
    let mut res: Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut from: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut to: Option<ArcStr> = None;
    let mut script: Option<ArcStr> = None;
    let __pa0 = ::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        (from, to, script) = parseConversionAnnotationElement(arg.clone())?;
        for mut v in &*from.clone() {
            let mut v = v.clone();
            res = cons((v.clone(), to.clone(), script.clone()), res.clone());
        }
    }
    Ok(res)
}

fn parseConversionAnnotationElement(mut r#mod: Arc<Absyn::ElementArg>) -> Result<(Arc<metamodelica::List<ArcStr>>, Option<ArcStr>, Option<ArcStr>)> {
    let mut fromVersion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut toVersion: Option<ArcStr> = None;
    let mut scriptFilename: Option<ArcStr> = None;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut arg_mod: Option<Arc<Absyn::Modification>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noneFromVersion" }, .. } => {
            fromVersion = list![(AbsynUtil::expString(AbsynUtil::stripCommentExpressions(getAnnotationExp(var_field!((*r#mod).modification, Absyn::ElementArg::MODIFICATION).clone())?, false)?)?).clone()];
            ()
        },
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "from" }, .. } => {
            for mut arg in &*args.clone() {
                let mut arg = arg.clone();
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(arg.clone()) {
                    Deref @ Absyn::ElementArg::MODIFICATION { info: __pa0, modification: __pa1, path: Deref @ Absyn::Path::IDENT { name: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                info = __pa0.clone();
                arg_mod = __pa1.clone();
                name = __pa2.clone();
                let () = 'mc: {
        let __mc_input = name.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "version" => {
                    let mut fromVersion: Arc<metamodelica::List<ArcStr>> = fromVersion.clone();
                    let mut exp: Arc<Absyn::Exp> = exp.clone();
                    exp = AbsynUtil::stripCommentExpressions(getAnnotationExp(arg_mod.clone())?, false)?;
                    fromVersion = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::STRING { .. } => list![(var_field!((*exp).value, Absyn::Exp::STRING).clone()).clone()],
        Deref @ Absyn::Exp::ARRAY { .. } => {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone()).into_iter().cloned() {
                    let __x = AbsynUtil::expString(e.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => bail!("match: no arm matched"),
    } });
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "to" => {
                    let mut toVersion: Option<ArcStr> = toVersion.clone();
                    toVersion = Some((AbsynUtil::expString(AbsynUtil::stripCommentExpressions(getAnnotationExp(arg_mod.clone())?, false)?)?).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "script" => {
                    let mut scriptFilename: Option<ArcStr> = scriptFilename.clone();
                    scriptFilename = Some((AbsynUtil::expString(AbsynUtil::stripCommentExpressions(getAnnotationExp(arg_mod.clone())?, false)?)?).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !(StringUtil::startsWith((name.clone()).clone(), (literal!("__")).clone())) {
                        Error::addSourceMessage(Error::CONVERSION_UNKNOWN_ANNOTATION.clone(), list![(name.clone()).clone()], info.clone())?;
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                if fromVersion.clone().is_empty() {
                    let __pa4 = ::match_deref::match_deref! { match &(r#mod.clone()) {
                        Deref @ Absyn::ElementArg::MODIFICATION { info: __pa4, .. } => __pa4.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa4.clone();
                    Error::addSourceMessage(Error::CONVERSION_MISSING_FROM_VERSION.clone(), list![(Dump::unparseElementArgStr(r#mod.clone())?).clone()], info.clone())?;
                }
            }
            ()
        },
        Deref @ Absyn::ElementArg::MODIFICATION { info, path: Deref @ Absyn::Path::IDENT { name }, .. } => {
            if !(StringUtil::startsWith((name.clone()).clone(), (literal!("__")).clone())) {
                Error::addSourceMessage(Error::CONVERSION_UNKNOWN_ANNOTATION.clone(), list![(name.clone()).clone()], info.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((fromVersion, toVersion, scriptFilename))
}

pub fn getPackagesInPath(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = 'mc: {
        let __mc_input = (inPath.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    Ok(getPackagesInClass(modelpath.clone(), p.clone(), cdef.clone())?)
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
    Ok(paths)
}

pub fn getTopPackages(mut p: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = List::map(getTopPackagesInProgram(p.clone())?, (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>));
    Ok(paths)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTopPackagesInProgram(mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::List::Nil, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { within_: w, classes: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PACKAGE, name: id, .. }, tail: rest } } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getTopPackagesInProgram(Absyn::Program { classes: rest.clone(), within_: w.clone() })?;
                    Ok(cons((id.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { within_: w, classes: Deref @ metamodelica::List::Cons { head: _, tail: rest } } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getTopPackagesInProgram(Absyn::Program { classes: rest.clone(), within_: w.clone() })?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

fn getPackagesInClass(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outString: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outString = (::match_deref::match_deref! { match &((inPath.clone(), inProgram.clone(), inClass.clone())) {
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getPackagesInParts(parts.clone())?;
            List::map(strlist.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>))
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getPackagesInParts(parts.clone())?;
            List::map(strlist.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>))
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, .. }) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPackagesInParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    l1 = getPackagesInElts(elts.clone())?;
                    l2 = getPackagesInParts(rest.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    l1 = getPackagesInElts(elts.clone())?;
                    l2 = getPackagesInParts(rest.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getPackagesInParts(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPackagesInElts(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inAbsynElementItemLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PACKAGE, name: id, .. }, .. }, .. } }, tail: rest } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getPackagesInElts(rest.clone())?;
                    Ok(cons((id.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getPackagesInElts(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

pub fn getClassnamesInPath(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = 'mc: {
        let __mc_input = (inPath.clone(), inProgram.clone(), inShowProtected.clone(), includeConstants.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p, b, c) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    Ok(InteractiveUtil::getClassnamesInClass(modelpath.clone(), p.clone(), cdef.clone(), b.clone(), c.clone())?)
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
    Ok(paths)
}

pub fn getTopClassnames(mut p: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = List::map(getTopClassnamesInProgram(p.clone())?, (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>));
    Ok(paths)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getTopClassnamesInProgram(mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::List::Nil, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { within_: w, classes: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Class { name: id, .. }, tail: rest } } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getTopClassnamesInProgram(Absyn::Program { classes: rest.clone(), within_: w.clone() })?;
                    Ok(cons((id.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { within_: w, classes: Deref @ metamodelica::List::Cons { head: _, tail: rest } } => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getTopClassnamesInProgram(Absyn::Program { classes: rest.clone(), within_: w.clone() })?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTopQualifiedClassnames(mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outStringLst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::List::Nil, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { within_: w, classes: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Class { name: id, .. }, tail: rest } } => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut p: Arc<Absyn::Path>;
                    p = AbsynUtil::joinWithinPath(w.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                    res = getTopQualifiedClassnames(Absyn::Program { classes: rest.clone(), within_: w.clone() })?;
                    Ok(cons(p.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { within_: w, classes: Deref @ metamodelica::List::Cons { head: _, tail: rest } } => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    res = getTopQualifiedClassnames(Absyn::Program { classes: rest.clone(), within_: w.clone() })?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

fn getBaseClasses(mut cls: Arc<Absyn::Class>, mut env: FCore::Graph) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut baseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut base_class_name: ArcStr = arcstr::literal!("");
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut cenv: FCore::Graph;
    let mut env_path_opt: Option<Arc<Absyn::Path>> = None;
    let mut env_path: Arc<Absyn::Path>;
    let mut path: Arc<Absyn::Path>;
    baseClasses = 'mc: {
        let __mc_input = cls.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
                    Ok(getBaseClassesFromParts(parts.clone(), env.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, baseClassName: base_class_name, .. }, .. } => {
                    let mut path: Arc<Absyn::Path>;
                    let mut cenv: FCore::Graph;
                    let mut env_path: Arc<Absyn::Path>;
                    (_, _, cenv) = Lookup::lookupClassIdent(FCore::emptyCache(), env.clone(), (base_class_name.clone()).clone(), Some(cls.info.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(cenv.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    env_path = __pa0.clone();
                    path = AbsynUtil::suffixPath(env_path.clone(), (base_class_name.clone()).clone())?;
                    Ok(cons(path.clone(), getBaseClassesFromParts(parts.clone(), env.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut path = (*path).clone();
                    let mut env_path_opt: Option<Arc<Absyn::Path>> = env_path_opt.clone();
                    let mut cenv: FCore::Graph;
                    (_, _, cenv) = Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), Some(cls.info.clone()))?;
                    env_path_opt = FGraph::getScopePath(cenv.clone())?;
                    if isSome(env_path_opt.clone()) {
                        path = AbsynUtil::suffixPath(Util::getOption(env_path_opt.clone())?, (AbsynUtil::pathLastIdent(path.clone())?).clone())?;
                    }
                    Ok(list![path.clone()])
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
    Ok(baseClasses)
}

fn getBaseClassesFromParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut env: FCore::Graph) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut baseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        for mut el in &*AbsynUtil::getElementItemsInClassPart(part.clone()) {
            let mut el = el.clone();
            baseClasses = getBaseClassesFromElt(el.clone(), env.clone(), baseClasses.clone())?;
        }
    }
    baseClasses = Dangerous::listReverseInPlace(baseClasses.clone());
    Ok(baseClasses)
}

fn getBaseClassesFromElt(mut element: Arc<Absyn::ElementItem>, mut env: FCore::Graph, mut baseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut baseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>> = baseClasses;
    let mut path: Arc<Absyn::Path>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut cenv: FCore::Graph;
    let mut env_path_opt: Option<Arc<Absyn::Path>> = None;
    baseClasses = 'mc: {
        let __mc_input = element.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { info, specification: Deref @ Absyn::ElementSpec::EXTENDS { path, .. }, .. } } => {
                    let mut path = (*path).clone();
                    let mut env_path_opt: Option<Arc<Absyn::Path>> = env_path_opt.clone();
                    let mut cenv: FCore::Graph;
                    (_, _, cenv) = Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), Some(info.clone()))?;
                    env_path_opt = FGraph::getScopePath(cenv.clone())?;
                    if isSome(env_path_opt.clone()) {
                        path = AbsynUtil::suffixPath(Util::getOption(env_path_opt.clone())?, (AbsynUtil::pathLastIdent(path.clone())?).clone())?;
                    }
                    Ok(cons(path.clone(), baseClasses.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(baseClasses.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(baseClasses)
}

fn countBaseClasses(mut inClass: Arc<Absyn::Class>) -> i32 {
    let mut count: i32 = 0;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    count = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => countBaseClassesFromParts(parts.clone()),
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => countBaseClassesFromParts(parts.clone()),
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    count
}

fn countBaseClassesFromParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> i32 {
    let mut count: i32 = 0;
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        for mut el in &*AbsynUtil::getElementItemsInClassPart(part.clone()) {
            let mut el = el.clone();
            if AbsynUtil::isElementItemExtends(el.clone()) {
                count = count.clone() + 1;
            }
        }
    }
    count
}

pub fn getDocumentationClassAnnotation(mut className: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<bool> {
    let mut isDocClass: bool = false;
    isDocClass = (::match_deref::match_deref! { match &((className.clone(), p.clone())) {
        (_, _) => {
            let mut docStr: ArcStr = arcstr::literal!("");
            docStr = (getNamedAnnotationExp(className.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("DocumentationClass")).clone() }), Some((literal!("false")).clone()), (std::sync::Arc::new(getDocumentationClassAnnotationModStr) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<ArcStr> + 'static>))?).clone();
            stringEq((docStr.clone()).clone(), (literal!("true")).clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isDocClass)
}

fn getDocumentationClassAnnotationModStr(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut docStr: ArcStr = arcstr::literal!("");
    docStr = ('mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut docStr: ArcStr = docStr.clone();
                    docStr = (Dump::printExpStr(e.clone())?).clone();
                    Ok(docStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("false"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(docStr)
}

pub fn getDefaultComponentName(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (getStringNamedAnnotation(classPath.clone(), program.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("defaultComponentName")).clone() }))).clone();
    result = ValuesMake::makeString((r#str.clone()).clone());
    result
}

pub fn getDefaultComponentPrefixes(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (getStringNamedAnnotation(classPath.clone(), program.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("defaultComponentPrefixes")).clone() }))).clone();
    result = ValuesMake::makeString((r#str.clone()).clone());
    result
}

fn getAnnotationValue(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    r#str = ('mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, elementArgLst: Deref @ metamodelica::List::Nil }) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("{}"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn getAnnotationExp(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let __pa0 = ::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa0, .. }, elementArgLst: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    Ok(exp)
}

pub fn getAnnotationStringValueOrFail(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, elementArgLst: Deref @ metamodelica::List::Nil }) => {
            AbsynUtil::getString(exp.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn getExperimentAnnotationString(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut experimentStr: ArcStr = arcstr::literal!("");
    experimentStr = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            strs = getExperimentAnnotationString2(arglst.clone())?;
            s = stringDelimitList(strs.clone(), (literal!(",")).clone());
            s = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(experimentStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getExperimentAnnotationString2(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strs = 'mc: {
        let __mc_input = eltArgs.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name }, .. }, tail: xs } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut ss: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }).clone();
                    ss = getExperimentAnnotationString2(xs.clone())?;
                    Ok(cons((s.clone()).clone(), ss.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut ss: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    ss = getExperimentAnnotationString2(xs.clone())?;
                    Ok(ss.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(strs)
}

pub fn getDocumentationAnnotationString(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<(ArcStr, ArcStr, ArcStr)> {
    let mut docStr: (ArcStr, ArcStr, ArcStr);
    docStr = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            let mut info: ArcStr = arcstr::literal!("");
            let mut revisions: ArcStr = arcstr::literal!("");
            let mut infoHeader: ArcStr = arcstr::literal!("");
            let mut partialInst: bool = false;
            partialInst = System::getPartialInstantiation();
            System::setPartialInstantiation(true);
            info = (getDocumentationAnnotationInfo(arglst.clone())?).clone();
            revisions = (getDocumentationAnnotationRevision(arglst.clone())?).clone();
            infoHeader = (getDocumentationAnnotationInfoHeader(arglst.clone())?).clone();
            System::setPartialInstantiation(partialInst.clone());
            (info.clone(), revisions.clone(), infoHeader.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(docStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDocumentationAnnotationInfo(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = eltArgs.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "info" }, .. }, tail: _ } => {
                    let mut dexp: Arc<DAE::Exp>;
                    let mut s: ArcStr = arcstr::literal!("");
                    (_, dexp, _) = StaticScript::elabGraphicsExp(FCore::emptyCache(), FGraph::empty(), exp.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(dexp.clone())?) {
                        (Deref @ DAE::Exp::SCONST { string: __pa0 }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    s = __pa0.clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut ss: ArcStr = arcstr::literal!("");
                    ss = (getDocumentationAnnotationInfo(xs.clone())?).clone();
                    Ok(ss.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDocumentationAnnotationRevision(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = eltArgs.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "revisions" }, .. }, tail: _ } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut dexp: Arc<DAE::Exp>;
                    (_, dexp, _) = StaticScript::elabGraphicsExp(FCore::emptyCache(), FGraph::empty(), exp.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(dexp.clone())?) {
                        (Deref @ DAE::Exp::SCONST { string: __pa0 }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    s = __pa0.clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut ss: ArcStr = arcstr::literal!("");
                    ss = (getDocumentationAnnotationRevision(xs.clone())?).clone();
                    Ok(ss.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDocumentationAnnotationInfoHeader(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = eltArgs.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "__OpenModelica_infoHeader" }, .. }, tail: _ } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut dexp: Arc<DAE::Exp>;
                    (_, dexp, _) = StaticScript::elabGraphicsExp(FCore::emptyCache(), FGraph::empty(), exp.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(dexp.clone())?) {
                        (Deref @ DAE::Exp::SCONST { string: __pa0 }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    s = __pa0.clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut ss: ArcStr = arcstr::literal!("");
                    ss = (getDocumentationAnnotationInfoHeader(xs.clone())?).clone();
                    Ok(ss.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn getNthPublicConnectorStr(mut classPath: Arc<Absyn::Path>, mut cls: Arc<Absyn::Class>, mut program: Absyn::Program, mut n: i32) -> Result<(Option<(ArcStr, Arc<Absyn::Path>)>, i32)> {
    let mut conn: Option<(ArcStr, Arc<Absyn::Path>)> = None;
    let mut n: i32 = n;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    parts = AbsynUtil::getClassPartsInClass(cls.clone());
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        (conn, n) = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => getNthConnectorInfo(program.clone(), classPath.clone(), var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(), n.clone())?,
        _ => (conn.clone(), n.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if n.clone() <= 0 {
            break;
        }
    }
    Ok((conn, n))
}

fn getNthConnectorInfo(mut program: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut n: i32) -> Result<(Option<(ArcStr, Arc<Absyn::Path>)>, i32)> {
    let mut conn: Option<(ArcStr, Arc<Absyn::Path>)> = None;
    let mut n: i32 = n;
    let mut tp: Arc<Absyn::Path>;
    let mut cls_path: Arc<Absyn::Path>;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut comp_count: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    for mut item in &*items.clone() {
        let mut item = item.clone();
        (conn, n) = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path: tp, .. }, .. } } => {
            (cls, cls_path) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
            getNthPublicConnectorStr(cls_path.clone(), cls.clone(), program.clone(), n.clone())?
        },
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, .. }, .. }, .. } } => {
            (cls, _) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
            if AbsynUtil::isConnector(cls.clone()) || AbsynUtil::isExpandableConnector(cls.clone()) {
                comp_count = (comps.clone().len() as i32);
                if n.clone() <= comp_count.clone() {
                    name = (AbsynUtil::componentName((comps.clone()).get(n.clone())?)?).clone();
                    conn = Some((name.clone(), tp.clone()));
                }
                n = n.clone() - comp_count.clone();
            }
            (conn.clone(), n.clone())
        },
        _ => (conn.clone(), n.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if n.clone() <= 0 {
            break;
        }
    }
    Ok((conn, n))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn countPublicConnectors(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut cls: Arc<Absyn::Class>) -> Result<i32> {
    let mut count: i32 = 0;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut cls_name: Arc<Absyn::Path>;
    count = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => countPublicConnectorsInParts(parts.clone(), classPath.clone(), program.clone())?,
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => countPublicConnectorsInParts(parts.clone(), classPath.clone(), program.clone())?,
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cls_name, .. }, .. }, .. } => {
            (cdef, _) = lookupClassdef(cls_name.clone(), classPath.clone(), program.clone())?;
            countPublicConnectors(classPath.clone(), program.clone(), cdef.clone())?
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(count)
}

fn countPublicConnectorsInParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<i32> {
    let mut count: i32 = 0;
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        count = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => count.clone() + countConnectors(classPath.clone(), program.clone(), var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone())?,
        _ => count.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(count)
}

fn countConnectors(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<i32> {
    let mut count: i32 = 0;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut tp: Arc<Absyn::Path>;
    let mut cls_path: Arc<Absyn::Path>;
    let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut c: i32 = 0;
    for mut item in &*items.clone() {
        let mut item = item.clone();
        c = 'mc: {
        let __mc_input = item.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path: tp, .. }, .. } } => {
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let mut cls_path: Arc<Absyn::Path>;
                    (cls, cls_path) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
                    Ok(countPublicConnectors(cls_path.clone(), program.clone(), cls.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, .. }, .. }, .. } } => {
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    (cls, _) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
                    Ok(if (AbsynUtil::isConnector(cls.clone()) || AbsynUtil::isExpandableConnector(cls.clone())) {(comps.clone().len() as i32)} else {0})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        count = count.clone() + c.clone();
    }
    Ok(count)
}

fn getConnectionAnnotationStrElArgs(mut inElArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut info: SourceInfo, mut inClass: Arc<Absyn::Class>, mut inFullProgram: Absyn::Program, mut inModelPath: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = (inElArgLst.clone(), info.clone(), inClass.clone(), inFullProgram.clone(), inModelPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: _ }), path: Deref @ Absyn::Path::IDENT { name: annName }, .. }, tail: rest }, _, _, _, _) => {
                    let mut fargs: Arc<Absyn::FunctionArgs>;
                    let mut p_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut env: FCore::Graph;
                    let mut newexp: Arc<DAE::Exp>;
                    let mut gexpstr: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut prop: DAE::Properties;
                    let mut lineProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    lineProgram = InteractiveUtil::modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?;
                    fargs = createFuncargsFromElementargs(r#mod.clone())?;
                    p_1 = AbsynToSCode::translateAbsyn2SCode(lineProgram.clone())?;
                    (cache, env) = Inst::makeEnvFromProgram(p_1.clone())?;
                    (_, newexp, prop) = StaticScript::elabGraphicsExp(cache.clone(), env.clone(), Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (annName.clone()).clone(), subscripts: metamodelica::nil() }), functionArgs: fargs.clone(), typeVars: metamodelica::nil() }), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    (cache, newexp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), newexp.clone(), prop.clone(), false, info.clone())?;
                    Print::clearErrorBuf();
                    gexpstr = (ExpressionBasics::printExpStr(newexp.clone())?).clone();
                    res = getConnectionAnnotationStrElArgs(rest.clone(), info.clone(), inClass.clone(), inFullProgram.clone(), inModelPath.clone())?;
                    Ok(cons((gexpstr.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: _, eqMod: Deref @ Absyn::EqMod::NOMOD }), path: Deref @ Absyn::Path::IDENT { name: annName }, .. }, tail: rest }, _, _, _, _) => {
                    let mut gexpstr_1: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gexpstr_1 = stringAppendList(list![(annName.clone()).clone(), (literal!("(error)")).clone()]);
                    res = getConnectionAnnotationStrElArgs(rest.clone(), info.clone(), inClass.clone(), inFullProgram.clone(), inModelPath.clone())?;
                    Ok(cons((gexpstr_1.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

fn getConnectionAnnotationStr(mut inEquationItem: Arc<Absyn::EquationItem>, mut inClass: Arc<Absyn::Class>, mut inFullProgram: Absyn::Program, mut inModelPath: Arc<Absyn::Path>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut gexpstr: ArcStr = arcstr::literal!("");
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut annotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    result = (::match_deref::match_deref! { match &(inEquationItem.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annotations }), .. }), equation_: Deref @ Absyn::Equation::EQ_CONNECT { .. }, info } => {
            res = getConnectionAnnotationStrElArgs(annotations.clone(), info.clone(), inClass.clone(), inFullProgram.clone(), inModelPath.clone())?;
            InteractiveUtil::makeAnnotationArrayValue(res.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

pub fn createFuncargsFromElementargs(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<Absyn::FunctionArgs>> {
    let mut outFunctionArgs: Arc<Absyn::FunctionArgs>;
    outFunctionArgs = 'mc: {
        let __mc_input = inAbsynElementArgLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: xs } => {
                    let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut narg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createFuncargsFromElementargs(xs.clone())?) {
                        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __pa0, argNames: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa0.clone();
                    narg = __pa1.clone();
                    Ok(Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: expl.clone(), argNames: cons(Arc::new(Absyn::NamedArg { argName: (id.clone()).clone(), argValue: exp.clone() }), narg.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut narg: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createFuncargsFromElementargs(xs.clone())?) {
                        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __pa0, argNames: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa0.clone();
                    narg = __pa1.clone();
                    Ok(Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: expl.clone(), argNames: narg.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFunctionArgs)
}

fn getConnectionStr(mut inEquation: Arc<Absyn::Equation>) -> Result<(ArcStr, ArcStr)> {
    let mut outFromString: ArcStr = arcstr::literal!("");
    let mut outToString: ArcStr = arcstr::literal!("");
    (outFromString, outToString) = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ Absyn::Equation::EQ_CONNECT { connector2: cr2, connector1: cr1 } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (Dump::printComponentRefStr(cr1.clone())?).clone();
            s2 = (Dump::printComponentRefStr(cr2.clone())?).clone();
            (s1.clone(), s2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outFromString, outToString))
}

pub fn getConnections(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut connections: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut body: Arc<Absyn::ClassDef> = inClass.body.clone();
    connections = getConnectionsInClassparts(AbsynUtil::getClassPartsInClass(inClass.clone()))?;
    Ok(connections)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getConnectionsInClassparts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqlist1 }, tail: xs } => {
                    let mut eqlist2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    let mut eqlist1 = (*eqlist1).clone();
                    eqlist1 = getConnectionsInEquations(eqlist1.clone());
                    eqlist2 = getConnectionsInClassparts(xs.clone())?;
                    Ok(listAppend(eqlist1.clone(), eqlist2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut eqlist1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    eqlist1 = getConnectionsInClassparts(xs.clone())?;
                    Ok(eqlist1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getConnectionsInEquations(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Arc<metamodelica::List<Arc<Absyn::EquationItem>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inAbsynEquationItemLst.clone()) {
        Deref @ metamodelica::List::Cons { head: eq @ Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { .. }, .. }, tail: xs } => {
            let mut eqlist1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            eqlist1 = getConnectionsInEquations(xs.clone());
            cons(eq.clone(), eqlist1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_FOR { forEquations: forEqList, .. }, .. }, tail: xs } => {
            let mut eqlist1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlist2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            eqlist1 = getConnectionsInEquations(forEqList.clone());
            eqlist2 = getConnectionsInEquations(xs.clone());
            listAppend(eqlist1.clone(), eqlist2.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut eqlist1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            eqlist1 = getConnectionsInEquations(xs.clone());
            eqlist1.clone()
        },
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

pub fn getComponentModification(mut element: Arc<Absyn::Element>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut opt_mod: Option<Arc<Absyn::Modification>> = None;
    let mut r#mod: Arc<Absyn::Modification> = Arc::new(<Absyn::Modification as ::std::default::Default>::default());
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, .. }, .. } => {
            for mut c in &*comps.clone() {
                let mut c = c.clone();
                opt_mod = c.component.modification.clone();
                r#mod = if (isSome(opt_mod.clone())) {Util::getOption(opt_mod.clone())?} else {Absyn::emptyMod.clone()};
                vals = cons(Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_MODIFICATION { modification: r#mod.clone() }) }), vals.clone());
            }
            vals = Dangerous::listReverseInPlace(vals.clone());
            ValuesMake::makeArray(vals.clone())?
        },
        _ => ValuesMake::makeEmptyArray(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn cacheProgramAndPath(mut inCache: GraphicEnvCache) -> Result<(Absyn::Program, Arc<Absyn::Path>)> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut outPath: Arc<Absyn::Path>;
    (outProgram, outPath) = (match inCache.clone() {
        GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { .. } => (var_field!(inCache.program, GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(), var_field!(inCache.modelPath, GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone()),
        GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE { .. } => (var_field!(inCache.program, GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(), var_field!(inCache.modelPath, GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone()),
        GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { .. } => (var_field!(inCache.program, GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(), var_field!(inCache.modelPath, GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone()),
        _ => bail!("match: no arm matched"),
    });
    Ok((outProgram, outPath))
}

pub fn envFromGraphicEnvCache(mut inEnvCache: GraphicEnvCache) -> Result<FCore::Graph> {
    let mut env: FCore::Graph;
    let GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { env: __pa0, .. } = (inEnvCache.clone()) else { bail!("pattern mismatch") };
    env = __pa0.clone();
    Ok(env)
}

fn cacheFromGraphicEnvCache(mut inEnvCache: GraphicEnvCache) -> Result<FCore::Cache> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { cache: __pa0, .. } = (inEnvCache.clone()) else { bail!("pattern mismatch") };
    cache = __pa0.clone();
    Ok(cache)
}

fn getAnnotationString(mut inAnnotation: Arc<Absyn::Annotation>, mut inClass: Arc<Absyn::Class>, mut inFullProgram: Absyn::Program, mut inModelPath: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut el: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut ann: Arc<Absyn::Annotation> = Arc::new(<Absyn::Annotation as ::std::default::Default>::default());
    if Flags::isSet(Flags::NF_API.clone())? {
        match '__try0: {
            outString = (unwrap_break_err!(NFApi::evaluateAnnotation(inFullProgram.clone(), inModelPath.clone(), inAnnotation.clone()), '__try0)).clone();
            Ok::<_, anyhow::Error>((outString.clone(),))
        } {
            Ok((__try0_o0,)) => {
                outString = __try0_o0;
            }
            Err(_) => {
                outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseAnnotation(inAnnotation.clone())?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            }
        }
        return Ok(outString);
    }
    outString = ('mc: {
        let __mc_input = inAnnotation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Annotation { elementArgs: el } => {
                    let mut outString: ArcStr = outString.clone();
                    outString = listHead(InteractiveUtil::getElementitemsAnnotationsElArgs(el.clone(), FGraph::emptyGraph().clone(), inClass.clone(), GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { program: inFullProgram.clone(), modelPath: inModelPath.clone() }, false)?)?;
                    Ok(outString.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseAnnotation(inAnnotation.clone())?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn keywordReplaceable(mut inAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inAbsynRedeclareKeywordsOption.clone() {
        Some(Absyn::RedeclareKeywords::REPLACEABLE) => true,
        Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE) => true,
        _ => false,
    });
    outBoolean
}

fn getComponentInfoOld(mut inElement: Arc<Absyn::Element>, mut inEnv: GraphicEnvCache) -> Result<(ArcStr, Arc<Absyn::Path>, ArcStr)> {
    let mut componentName: ArcStr = arcstr::literal!("");
    let mut typeName: Arc<Absyn::Path>;
    let mut comment: ArcStr = arcstr::literal!("");
    let mut comp: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
    (componentName, typeName, comment) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::List::Cons { head: comp, tail: _ }, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: typeName, arrayDim: _ }, .. }, .. } => {
            let mut typeName = (*typeName).clone();
            componentName = (comp.component.name.clone()).clone();
            typeName = InteractiveUtil::qualifyPath(inEnv.clone(), typeName.clone(), false)?;
            comment = (InteractiveUtil::getComponentComment(comp.clone(), inElement.clone())).clone();
            (componentName.clone(), typeName.clone(), comment.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((componentName, typeName, comment))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn transformPathedClassInProgram(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>) -> Result<Absyn::Program> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    outProgram = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => transformClassInProgram((var_field!((*inPath).name, Absyn::Path::IDENT).clone()).clone(), inProgram.clone(), inFunc.clone())?,
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => transformPathedClassInProgram(var_field!((*inPath).path, Absyn::Path::FULLYQUALIFIED).clone(), inProgram.clone(), inFunc.clone())?,
        Deref @ Absyn::Path::QUALIFIED { .. } => transformClassInProgram((var_field!((*inPath).name, Absyn::Path::QUALIFIED).clone()).clone(), inProgram.clone(), Arc::new({ let __pe_b0 = var_field!((*inPath).path, Absyn::Path::QUALIFIED).clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static> = inFunc.clone(); move |__pe_a1| transformPathedClassInClass(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }))?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outProgram)
}

pub fn transformClassInProgram(mut inName: ArcStr, mut inProgram: Absyn::Program, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>) -> Result<Absyn::Program> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut wi: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    let Absyn::PROGRAM { classes: __pa0, within_: __pa1 } = (inProgram.clone()) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    wi = __pa1.clone();
    loop {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(classes.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cls = __pa2.clone();
        classes = __pa3.clone();
        let __pa4 = ::match_deref::match_deref! { match &(cls.clone()) {
            Deref @ Absyn::Class { name: __pa4, .. } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa4.clone();
        if name.clone() == inName.clone() {
            cls = inFunc(cls.clone())?;
            classes = List::append_reverse(acc.clone(), cons(cls.clone(), classes.clone()));
            outProgram = Absyn::Program { classes: classes.clone(), within_: wi.clone() };
            break;
        }
        acc = cons(cls.clone(), acc.clone());
    }
    Ok(outProgram)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn transformPathedClassInClass(mut inPath: Arc<Absyn::Path>, mut inClass: Arc<Absyn::Class>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>) -> Result<Arc<Absyn::Class>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => transformClassInClass((var_field!((*inPath).name, Absyn::Path::IDENT).clone()).clone(), inFunc.clone(), inClass.clone())?,
        Deref @ Absyn::Path::QUALIFIED { .. } => transformClassInClass((var_field!((*inPath).name, Absyn::Path::QUALIFIED).clone()).clone(), Arc::new({ let __pe_b0 = var_field!((*inPath).path, Absyn::Path::QUALIFIED).clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static> = inFunc.clone(); move |__pe_a1| transformPathedClassInClass(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }), inClass.clone())?,
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => transformPathedClassInClass(var_field!((*inPath).path, Absyn::Path::FULLYQUALIFIED).clone(), inClass.clone(), inFunc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn transformClassInClass(mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>, mut cls: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut cls: Arc<Absyn::Class> = cls;
    let mut body: Arc<Absyn::ClassDef> = cls.body.clone();
    let () = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = List::findMap(var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static> = func.clone(); move |__pe_a2| transformClassInClassPart(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?.0);
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = List::findMap(var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static> = func.clone(); move |__pe_a2| transformClassInClassPart(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?.0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assign_field!(cls.body = body.clone());
    Ok(cls)
}

fn transformClassInClassPart(mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>, mut part: Arc<Absyn::ClassPart>) -> Result<(Arc<Absyn::ClassPart>, bool)> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut part: Arc<Absyn::ClassPart> = part;
    let mut found: bool = false;
    found = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            (items, found) = List::findMap(var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static> = func.clone(); move |__pe_a2| Ok(transformClassInElementItem(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)) }))?;
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = items.clone());
            found.clone()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            (items, found) = List::findMap(var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static> = func.clone(); move |__pe_a2| Ok(transformClassInElementItem(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)) }))?;
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = items.clone());
            found.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((part, found))
}

fn transformClassInElementItem(mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>, mut item: Arc<Absyn::ElementItem>) -> (Arc<Absyn::ElementItem>, bool) {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut item: Arc<Absyn::ElementItem> = item;
    let mut found: bool = false;
    found = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => {
            let mut e: Arc<Absyn::Element>;
            (e, found) = transformClassInElement((name.clone()).clone(), func.clone(), var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone());
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = e.clone());
            found.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (item, found)
}

fn transformClassInElement(mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>, mut element: Arc<Absyn::Element>) -> (Arc<Absyn::Element>, bool) {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut element: Arc<Absyn::Element> = element;
    let mut found: bool = false;
    found = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            let mut spec: Arc<Absyn::ElementSpec>;
            (spec, found) = transformClassInElementSpec((name.clone()).clone(), func.clone(), var_field!((*element).specification, Absyn::Element::ELEMENT).clone());
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            found.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (element, found)
}

fn transformClassInElementSpec(mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>, mut spec: Arc<Absyn::ElementSpec>) -> (Arc<Absyn::ElementSpec>, bool) {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>;

    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let mut found: bool = false;
    found = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls, .. } if (cls.name.clone() == name.clone()) => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = func(cls.clone()).unwrap());
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (spec, found)
}

pub fn getContainedClassAndFile(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<(Absyn::Program, ArcStr)> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut outString: ArcStr = arcstr::literal!("");
    (outProgram, outString) = (::match_deref::match_deref! { match &((inPath.clone(), inProgram.clone())) {
        (classname, p) => {
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut filename: ArcStr = arcstr::literal!("");
            let mut p_1: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut p_2: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            cdef = InteractiveUtil::getPathedClassInProgram(classname.clone(), p.clone(), false, false)?;
            filename = (AbsynUtil::classFilename(cdef.clone())?).clone();
            p_1 = getSurroundingPackage(classname.clone(), p.clone())?;
            p_2 = removeInnerDiffFiledClasses(p_1.clone())?;
            (p_2.clone(), filename.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outProgram, outString))
}

fn removeInnerDiffFiledClasses(mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut p: Absyn::Program = inProgram.clone();
    p = (match p.clone() {
        Absyn::Program { .. } => {
            p.classes = List::map(p.classes.clone(), (std::sync::Arc::new(removeInnerDiffFiledClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> + 'static>));
            p.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(p)
}

fn removeInnerDiffFiledClass(mut inClass: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &(inClass.clone()) {
        outClass @ Deref @ Absyn::Class { info: SourceInfo { fileName: file, .. }, body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. } => {
            let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            publst = InteractiveUtil::getPublicList(parts.clone());
            publst2 = removeClassDiffFiledInElementitemlist(publst.clone(), (file.clone()).clone());
            parts2 = InteractiveUtil::replacePublicList(parts.clone(), publst2.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        outClass @ Deref @ Absyn::Class { info: SourceInfo { fileName: file, .. }, body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications, baseClassName }, .. } => {
            let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            publst = InteractiveUtil::getPublicList(parts.clone());
            publst2 = removeClassDiffFiledInElementitemlist(publst.clone(), (file.clone()).clone());
            parts2 = InteractiveUtil::replacePublicList(parts.clone(), publst2.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (baseClassName.clone()).clone(), modifications: modifications.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => {
            inClass.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outClass)
}

fn classIsInFile(mut inFilename: ArcStr, mut inElement: Arc<Absyn::ElementItem>) -> Result<bool> {
    let mut outInFile: bool = false;
    outInFile = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { info: SourceInfo { fileName: filename, .. }, .. }, .. }, .. } } => {
            stringEq((inFilename.clone()).clone(), (filename.clone()).clone())?
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInFile)
}

fn removeClassDiffFiledInElementitemlist(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inFilename: ArcStr) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outElements = List::filterOnTrue(inElements.clone(), Arc::new({ let __pe_b0 = (inFilename.clone()).clone(); move |__pe_a1| classIsInFile(__pe_b0.clone(), __pe_a1) }));
    outElements
}

fn getSurroundingPackage(mut classpath: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut p: Absyn::Program = inProgram.clone();
    p = 'mc: {
        let __mc_input = p.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut pdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut filename1: ArcStr = arcstr::literal!("");
            let mut filename2: ArcStr = arcstr::literal!("");
            let mut ppath: Arc<Absyn::Path>;
            let mut res: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            cdef = InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?;
            filename1 = (AbsynUtil::classFilename(cdef.clone())?).clone();
            ppath = AbsynUtil::stripLast(classpath.clone())?;
            pdef = InteractiveUtil::getPathedClassInProgram(ppath.clone(), p.clone(), false, false)?;
            filename2 = (AbsynUtil::classFilename(pdef.clone())?).clone();
            let true = (stringEq((filename1.clone()).clone(), (filename2.clone()).clone())?) else { bail!("pattern mismatch") };
            res = getSurroundingPackage(ppath.clone(), p.clone())?;
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let Absyn::Program { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Absyn::Program = p.clone();
            p.classes = list![InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?];
            p.within_ = InteractiveUtil::buildWithin(classpath.clone())?;
            Ok(p.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(p)
}

pub fn transformFlatProgram(mut p: Absyn::Program) -> Result<Absyn::Program> {
    let mut newP: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    newP = (match p.clone() {
        _ => {
            (newP, _, _) = AbsynUtil::traverseClasses(p.clone(), None, (std::sync::Arc::new(transformFlatClass) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32)> + 'static>), 0, true)?;
            newP.clone()
        },
    });
    Ok(newP)
}

fn transformFlatClass(mut inTuple: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32)> {
    let mut outTuple: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, i32);
    outTuple = 'mc: {
        let __mc_input = inTuple.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cl @ Deref @ Absyn::Class { body: cdef, .. }, pa, i) => {
                    let mut cdef1: Arc<Absyn::ClassDef>;
                    let mut cl = (*cl).clone();
                    cdef1 = transformFlatClassDef(cdef.clone())?;
                    assign_field!(cl.body = cdef1.clone());
                    Ok((cl.clone(), pa.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("Interactive.transformFlatClass failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTuple)
}

fn transformFlatClassDef(mut cdef: Arc<Absyn::ClassDef>) -> Result<Arc<Absyn::ClassDef>> {
    let mut outCdef: Arc<Absyn::ClassDef>;
    outCdef = 'mc: {
        let __mc_input = cdef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::DERIVED { .. } => {
                    Ok(cdef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
                    Ok(cdef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
                    Ok(cdef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::PDER { .. } => {
                    Ok(cdef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt } => {
                    let mut partsTransformed: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    partsTransformed = List::map(parts.clone(), (std::sync::Arc::new(transformFlatPart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>) -> Result<Arc<Absyn::ClassPart>> + 'static>));
                    Ok(Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: partsTransformed.clone(), ann: ann.clone(), comment: cmt.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, ann, comment: cmt, modifications, baseClassName } => {
                    let mut partsTransformed: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    partsTransformed = List::map(parts.clone(), (std::sync::Arc::new(transformFlatPart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>) -> Result<Arc<Absyn::ClassPart>> + 'static>));
                    Ok(Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (baseClassName.clone()).clone(), modifications: modifications.clone(), comment: cmt.clone(), parts: partsTransformed.clone(), ann: ann.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("Interactive.transformFlatClassDef failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCdef)
}

pub fn transformFlatPart(mut part: Arc<Absyn::ClassPart>) -> Result<Arc<Absyn::ClassPart>> {
    let mut outPart: Arc<Absyn::ClassPart>;
    outPart = 'mc: {
        let __mc_input = part.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PUBLIC { contents: eitems } => {
                    let mut eitems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    eitems1 = List::map(eitems.clone(), (std::sync::Arc::new(transformFlatElementItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::ElementItem>> + 'static>));
                    Ok(Arc::new(Absyn::ClassPart::PUBLIC { contents: eitems1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PROTECTED { contents: eitems } => {
                    let mut eitems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    eitems1 = List::map(eitems.clone(), (std::sync::Arc::new(transformFlatElementItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::ElementItem>> + 'static>));
                    Ok(Arc::new(Absyn::ClassPart::PROTECTED { contents: eitems1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::EQUATIONS { contents: eqnitems } => {
                    let mut eqnitems1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    eqnitems1 = List::map(eqnitems.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
                    Ok(Arc::new(Absyn::ClassPart::EQUATIONS { contents: eqnitems1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqnitems } => {
                    let mut eqnitems1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    eqnitems1 = List::map(eqnitems.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
                    Ok(Arc::new(Absyn::ClassPart::INITIALEQUATIONS { contents: eqnitems1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::ALGORITHMS { contents: algitems } => {
                    let mut algitems1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    algitems1 = List::map(algitems.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
                    Ok(Arc::new(Absyn::ClassPart::ALGORITHMS { contents: algitems1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algitems } => {
                    let mut algitems1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    algitems1 = List::map(algitems.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
                    Ok(Arc::new(Absyn::ClassPart::INITIALALGORITHMS { contents: algitems1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: _, annotation_: _ } => {
                    Ok(part.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("Interactive.transformFlatPart failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPart)
}

fn transformFlatElementItem(mut eitem: Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::ElementItem>> {
    let mut outEitem: Arc<Absyn::ElementItem>;
    outEitem = (::match_deref::match_deref! { match &(eitem.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt } => {
            let mut elt1: Arc<Absyn::Element>;
            elt1 = transformFlatElement(elt.clone())?;
            Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elt1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEitem)
}

fn transformFlatElement(mut elt: Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> {
    let mut outElt: Arc<Absyn::Element>;
    outElt = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::Element::TEXT { .. } => {
            elt.clone()
        },
        Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: r, innerOuter: io, specification: spec, info, constrainClass: constr } => {
            let mut spec1: Arc<Absyn::ElementSpec>;
            spec1 = transformFlatElementSpec(spec.clone())?;
            Arc::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: r.clone(), innerOuter: io.clone(), specification: spec1.clone(), info: info.clone(), constrainClass: constr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElt)
}

fn transformFlatElementSpec(mut eltSpec: Arc<Absyn::ElementSpec>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outEltSpec: Arc<Absyn::ElementSpec>;
    outEltSpec = (::match_deref::match_deref! { match &(eltSpec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: r, class_: cl } => {
            let mut cl1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            (cl1, _, _) = transformFlatClass((cl.clone(), None, 0))?;
            Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: r.clone(), class_: cl1.clone() })
        },
        Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg: eargs, annotationOpt: annOpt } => {
            let mut eargs1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            eargs1 = List::map(eargs.clone(), (std::sync::Arc::new(transformFlatElementArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> + 'static>));
            Arc::new(Absyn::ElementSpec::EXTENDS { path: path.clone(), elementArg: eargs1.clone(), annotationOpt: annOpt.clone() })
        },
        Deref @ Absyn::ElementSpec::IMPORT { .. } => {
            eltSpec.clone()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { attributes: attr, typeSpec: tp, components: comps } => {
            let mut comps1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
            comps1 = List::map(comps.clone(), (std::sync::Arc::new(transformFlatComponentItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentItem>) -> Result<Arc<Absyn::ComponentItem>> + 'static>));
            Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: tp.clone(), components: comps1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEltSpec)
}

fn transformFlatComponentItem(mut compitem: Arc<Absyn::ComponentItem>) -> Result<Arc<Absyn::ComponentItem>> {
    let mut outCompitem: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
    outCompitem = (::match_deref::match_deref! { match &(compitem.clone()) {
        Deref @ Absyn::ComponentItem { component: comp, condition: cond, comment: cmt } => {
            let mut compTransformed: Absyn::Component = <Absyn::Component as ::std::default::Default>::default();
            compTransformed = transformFlatComponent(comp.clone())?;
            Arc::new(Absyn::ComponentItem { component: compTransformed.clone(), condition: cond.clone(), comment: cmt.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCompitem)
}

fn transformFlatComponent(mut comp: Absyn::Component) -> Result<Absyn::Component> {
    let mut outComp: Absyn::Component = <Absyn::Component as ::std::default::Default>::default();
    outComp = (match comp.clone() {
        Absyn::Component { name: mut id, arrayDim: ref arraydim, modification: mut r#mod } => {
            let mut arraydimTransformed: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            let mut modTransformed: Option<Arc<Absyn::Modification>> = None;
            modTransformed = transformFlatModificationOption(r#mod.clone())?;
            arraydimTransformed = transformFlatArrayDim(arraydim.clone());
            Absyn::Component { name: (id.clone()).clone(), arrayDim: arraydimTransformed.clone(), modification: modTransformed.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outComp)
}

fn transformFlatArrayDim(mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outAd: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    outAd = (::match_deref::match_deref! { match &(ad.clone()) {
        _ => {
            let mut adTransformed: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            adTransformed = List::map(ad.clone(), (std::sync::Arc::new(transformFlatSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>) -> Result<Arc<Absyn::Subscript>> + 'static>));
            adTransformed.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAd
}

fn transformFlatSubscript(mut s: Arc<Absyn::Subscript>) -> Result<Arc<Absyn::Subscript>> {
    let mut outS: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    outS = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Absyn::Subscript::NOSUB => {
            Arc::new(openmodelica_ast::Absyn::Subscript::NOSUB)
        },
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e } => {
            let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            (e1, _) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: e1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outS)
}

fn transformFlatElementArg(mut eltArg: Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> {
    let mut outEltArg: Arc<Absyn::ElementArg>;
    outEltArg = (::match_deref::match_deref! { match &(eltArg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: f, eachPrefix: e, path: p, modification: r#mod, comment: cmt, info } => {
            let mut mod1: Option<Arc<Absyn::Modification>> = None;
            mod1 = transformFlatModificationOption(r#mod.clone())?;
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: f.clone(), eachPrefix: e.clone(), path: p.clone(), modification: mod1.clone(), comment: cmt.clone(), info: info.clone() })
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            eltArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEltArg)
}

fn transformFlatModificationOption(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>> = None;
    outMod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: eltArgs, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, info } }) => {
            let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut eltArgs1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            eltArgs1 = List::map(eltArgs.clone(), (std::sync::Arc::new(transformFlatElementArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> + 'static>));
            (e1, _) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            Some(Arc::new(Absyn::Modification { elementArgLst: eltArgs1.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: e1.clone(), info: info.clone() }) }))
        },
        Some(Deref @ Absyn::Modification { elementArgLst: eltArgs, eqMod: Deref @ Absyn::EqMod::NOMOD }) => {
            let mut eltArgs1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            eltArgs1 = List::map(eltArgs.clone(), (std::sync::Arc::new(transformFlatElementArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> + 'static>));
            Some(Arc::new(Absyn::Modification { elementArgLst: eltArgs1.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) }))
        },
        None => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn transformFlatComponentRef(mut cr: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCr = (::match_deref::match_deref! { match &(cr.clone()) {
        _ => {
            let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut ss: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            ss = AbsynUtil::crefLastSubs(cr.clone())?;
            cr1 = AbsynUtil::crefStripLastSubs(cr.clone())?;
            s = (Dump::printComponentRefStr(cr1.clone())?).clone();
            Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (s.clone()).clone(), subscripts: ss.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCr)
}

fn transformFlatEquationItem(mut eqnitem: Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> {
    let mut outEqnitem: Arc<Absyn::EquationItem>;
    outEqnitem = (::match_deref::match_deref! { match &(eqnitem.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eqn, comment: cmt, info } => {
            let mut eqn1: Arc<Absyn::Equation>;
            eqn1 = transformFlatEquation(eqn.clone())?;
            Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: eqn1.clone(), comment: cmt.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqnitem)
}

fn transformFlatEquation(mut eqn: Arc<Absyn::Equation>) -> Result<Arc<Absyn::Equation>> {
    let mut outEqn: Arc<Absyn::Equation>;
    outEqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Absyn::Equation::EQ_IF { ifExp: e1, equationTrueItems: thenpart, elseIfBranches: elseifpart, equationElseItems: elsepart } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut thenpart1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut elsepart1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut elseifpart1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            thenpart1 = List::map(thenpart.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
            elsepart1 = List::map(elsepart.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
            elseifpart1 = List::map(elseifpart.clone(), (std::sync::Arc::new(transformFlatElseIfPart) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)> + 'static>));
            Arc::new(Absyn::Equation::EQ_IF { ifExp: e11.clone(), equationTrueItems: thenpart1.clone(), elseIfBranches: elseifpart1.clone(), equationElseItems: elsepart1.clone() })
        },
        Deref @ Absyn::Equation::EQ_EQUALS { leftSide: e1, rightSide: e2 } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e21: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            (e21, _) = AbsynUtil::traverseExp(e2.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            Arc::new(Absyn::Equation::EQ_EQUALS { leftSide: e11.clone(), rightSide: e21.clone() })
        },
        Deref @ Absyn::Equation::EQ_PDE { leftSide: e1, rightSide: e2, domain: cr1 } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e21: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut cr11: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            (e21, _) = AbsynUtil::traverseExp(e2.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            cr11 = transformFlatComponentRef(cr1.clone())?;
            Arc::new(Absyn::Equation::EQ_PDE { leftSide: e11.clone(), rightSide: e21.clone(), domain: cr11.clone() })
        },
        Deref @ Absyn::Equation::EQ_CONNECT { connector1: cr1, connector2: cr2 } => {
            let mut cr11: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut cr21: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            cr11 = transformFlatComponentRef(cr1.clone())?;
            cr21 = transformFlatComponentRef(cr2.clone())?;
            Arc::new(Absyn::Equation::EQ_CONNECT { connector1: cr11.clone(), connector2: cr21.clone() })
        },
        Deref @ Absyn::Equation::EQ_FOR { iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(e1) }, tail: Deref @ metamodelica::List::Nil }, forEquations: forEqns } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut forEqns1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            forEqns1 = List::map(forEqns.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
            Arc::new(Absyn::Equation::EQ_FOR { iterators: list![Arc::new(Absyn::ForIterator { name: (id.clone()).clone(), guardExp: None, range: Some(e11.clone()) })], forEquations: forEqns1.clone() })
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { whenExp: e1, whenEquations: whenEqns, elseWhenEquations: elseWhenEqns } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut whenEqns1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut elseWhenEqns1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            elseWhenEqns1 = List::map(elseWhenEqns.clone(), (std::sync::Arc::new(transformFlatElseIfPart) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)> + 'static>));
            whenEqns1 = List::map(whenEqns.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
            Arc::new(Absyn::Equation::EQ_WHEN_E { whenExp: e11.clone(), whenEquations: whenEqns1.clone(), elseWhenEquations: elseWhenEqns1.clone() })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: name, functionArgs: fargs } => {
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            fargs1 = transformFlatFunctionArgs(fargs.clone())?;
            Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: name.clone(), functionArgs: fargs1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqn)
}

fn transformFlatElseIfPart(mut elseIfPart: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)> {
    let mut outElseIfPart: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>);
    outElseIfPart = (::match_deref::match_deref! { match &(elseIfPart.clone()) {
        (e1, eqnitems) => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut eqnitems1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            eqnitems1 = List::map(eqnitems.clone(), (std::sync::Arc::new(transformFlatEquationItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> + 'static>));
            (e11.clone(), eqnitems1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElseIfPart)
}

fn transformFlatFunctionArgs(mut fargs: Arc<Absyn::FunctionArgs>) -> Result<Arc<Absyn::FunctionArgs>> {
    let mut outFargs: Arc<Absyn::FunctionArgs>;
    outFargs = (::match_deref::match_deref! { match &(fargs.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expl, argNames: namedArgs } => {
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut namedArgs1: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
            expl1 = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = (AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            namedArgs1 = List::map(namedArgs.clone(), (std::sync::Arc::new(transformFlatNamedArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::NamedArg>) -> Result<Arc<Absyn::NamedArg>> + 'static>));
            Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: expl1.clone(), argNames: namedArgs1.clone() })
        },
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            fargs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFargs)
}

fn transformFlatNamedArg(mut namedArg: Arc<Absyn::NamedArg>) -> Result<Arc<Absyn::NamedArg>> {
    let mut outNamedArg: Arc<Absyn::NamedArg> = Arc::new(<Absyn::NamedArg as ::std::default::Default>::default());
    outNamedArg = (::match_deref::match_deref! { match &(namedArg.clone()) {
        Deref @ Absyn::NamedArg { argName: id, argValue: e1 } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            Arc::new(Absyn::NamedArg { argName: (id.clone()).clone(), argValue: e11.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNamedArg)
}

fn transformFlatExp(mut inExp: Arc<Absyn::Exp>, mut inDummy: i32) -> Result<(Arc<Absyn::Exp>, i32)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outDummy: i32 = 0;
    (outExp, outDummy) = 'mc: {
        let __mc_input = (inExp.clone(), inDummy.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, i) => {
                    let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cr1 = transformFlatComponentRef(cr.clone())?;
                    Ok((Arc::new(Absyn::Exp::CREF { componentRef: cr1.clone() }), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inDummy.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outDummy))
}

fn transformFlatAlgorithmItem(mut algitem: Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut outAlgitem: Arc<Absyn::AlgorithmItem>;
    outAlgitem = (::match_deref::match_deref! { match &(algitem.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, comment: cmt, info } => {
            let mut alg1: Arc<Absyn::Algorithm> = Arc::new(Absyn::Algorithm::ALG_BREAK);
            alg1 = transformFlatAlgorithm(alg.clone())?;
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg1.clone(), comment: cmt.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAlgitem)
}

fn transformFlatAlgorithm(mut alg: Arc<Absyn::Algorithm>) -> Result<Arc<Absyn::Algorithm>> {
    let mut outAlg: Arc<Absyn::Algorithm> = Arc::new(Absyn::Algorithm::ALG_BREAK);
    outAlg = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr }, value: e1 } => {
            let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            (_, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            cr1 = transformFlatComponentRef(cr.clone())?;
            Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: Arc::new(Absyn::Exp::CREF { componentRef: cr1.clone() }), value: e1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: e1 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, value: e2 } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e21: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            (e21, _) = AbsynUtil::traverseExp(e2.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: e11.clone(), value: e21.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_IF { ifExp: e1, trueBranch: thenPart, elseIfAlgorithmBranch: elseIfPart, elseBranch: elsePart } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut thenPart1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut elsePart1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut elseIfPart1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
            thenPart1 = List::map(thenPart.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            elseIfPart1 = List::map(elseIfPart.clone(), (std::sync::Arc::new(transformFlatElseIfAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)> + 'static>));
            elsePart1 = List::map(elsePart.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            Arc::new(Absyn::Algorithm::ALG_IF { ifExp: e11.clone(), trueBranch: thenPart1.clone(), elseIfAlgorithmBranch: elseIfPart1.clone(), elseBranch: elsePart1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(e1) }, tail: Deref @ metamodelica::List::Nil }, forBody: body } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut body1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            body1 = List::map(body.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::Algorithm::ALG_FOR { iterators: list![Arc::new(Absyn::ForIterator { name: (id.clone()).clone(), guardExp: None, range: Some(e11.clone()) })], forBody: body1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { boolExpr: e1, whileBody: body } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut body1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            body1 = List::map(body.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::Algorithm::ALG_WHILE { boolExpr: e11.clone(), whileBody: body1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { boolExpr: e1, whenBody: body, elseWhenAlgorithmBranch: whenBranch } => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut body1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut whenBranch1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            body1 = List::map(body.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            whenBranch1 = List::map(whenBranch.clone(), (std::sync::Arc::new(transformFlatElseIfAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)> + 'static>));
            Arc::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: e11.clone(), whenBody: body1.clone(), elseWhenAlgorithmBranch: whenBranch1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: cr, functionArgs: fargs } => {
            let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            cr1 = transformFlatComponentRef(cr.clone())?;
            fargs1 = transformFlatFunctionArgs(fargs.clone())?;
            Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cr1.clone(), functionArgs: fargs1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_BREAK => {
            Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_BREAK)
        },
        Deref @ Absyn::Algorithm::ALG_RETURN => {
            Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_RETURN)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAlg)
}

fn transformFlatElseIfAlgorithm(mut elseIfbranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)> {
    let mut outElseIfbranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>);
    outElseIfbranch = (::match_deref::match_deref! { match &(elseIfbranch.clone()) {
        (e1, algitems) => {
            let mut e11: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut algitems1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(transformFlatExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 0)?;
            algitems1 = List::map(algitems.clone(), (std::sync::Arc::new(transformFlatAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            (e11.clone(), algitems1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElseIfbranch)
}

/* Start getDefinitions */
pub fn getDefinitions(mut ast: Absyn::Program, mut addFunctions: bool) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut toPrint: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut handle: i32 = 0;
    let mut cl: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let Absyn::PROGRAM { classes: __pa0, .. } = (MetaUtil::createMetaClassesInProgram(ast.clone())?) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    handle = Print::saveAndClearBuf()?;
    Print::printBuf((literal!("(\n")).clone())?;
    for mut c in &*classes.clone() {
        let mut c = c.clone();
        Print::printBuf((getDefinitionsClass(c.clone(), addFunctions.clone())?).clone())?;
        Print::printBufNewLine()?;
    }
    cl = InteractiveUtil::getPathedClassInProgram(Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }), ast.clone(), false, false)?;
    Print::printBuf((getDefinitionsClass(cl.clone(), false)?).clone())?;
    Print::printBuf((literal!("\n\n)")).clone())?;
    res = ValuesMake::makeString((Print::getString()?).clone());
    Print::restoreBuf(handle.clone())?;
    Ok(res)
}

fn getDefinitionsClass(mut class_: Arc<Absyn::Class>, mut addFunctions: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ('mc: {
        let __mc_input = (class_.clone(), addFunctions.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PACKAGE, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, .. }, _) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut ident = (*ident).clone();
                    ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(package ")); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone();
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone(), addFunctions.clone())?;
                    strs = cons((ident.clone()).clone(), strs.clone());
                    Ok(stringDelimitList(strs.clone(), (literal!("\n")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE } }, body: Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, partialPrefix: true, .. }, _) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = list![(literal!("(partial impure function")).clone(), (ident.clone()).clone(), (literal!(")")).clone()];
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, body: Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, partialPrefix: true, .. }, _) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = list![(literal!("(partial function")).clone(), (ident.clone()).clone(), (literal!(")")).clone()];
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE } }, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, partialPrefix: false, .. }, true) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone(), true)?;
                    strs = cons((literal!("(impure function")).clone(), cons((ident.clone()).clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { .. } }, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, partialPrefix: false, .. }, true) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone(), true)?;
                    strs = cons((literal!("(function")).clone(), cons((ident.clone()).clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION }, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, partialPrefix: false, .. }, true) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone(), true)?;
                    strs = cons((literal!("(operator function")).clone(), cons((ident.clone()).clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_UNIONTYPE, body: Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, .. }, _) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = list![(literal!("(uniontype")).clone(), (ident.clone()).clone(), (literal!(")")).clone()];
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_RECORD, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, .. }, _) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone(), false)?;
                    strs = cons((literal!("(record")).clone(), cons((ident.clone()).clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_METARECORD { index, name: path, .. }, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, name: ident, .. }, _) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut indexArg: ArcStr = arcstr::literal!("");
                    let mut pathArg: ArcStr = arcstr::literal!("");
                    indexArg = (intString(index.clone())).clone();
                    pathArg = (AbsynUtil::pathLastIdent(path.clone())?).clone();
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone(), false)?;
                    strs = cons((literal!("(metarecord")).clone(), cons((ident.clone()).clone(), cons((indexArg.clone()).clone(), cons((pathArg.clone()).clone(), strs.clone()))));
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { attributes: attr, typeSpec: ts, .. }, name: ident, .. }, _) => {
                    let mut tyStr: ArcStr = arcstr::literal!("");
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut numDim: i32 = 0;
                    numDim = getDefinitionDimensions(ts.clone(), attr.clone());
                    tyStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (numDim.clone() == 0) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(numDim.clone())); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*getDefinitionTypeSpecPathString(ts.clone())?); ArcStr::from(__mm_s) }).clone();
                    strs = list![(literal!("(type")).clone(), (ident.clone()).clone(), (tyStr.clone()).clone(), (literal!(")")).clone()];
                    Ok(stringDelimitList(strs.clone(), (literal!(" ")).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(res)
}

fn getDefinitionsReplaceableClass(mut class_: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ((::match_deref::match_deref! { match &(class_.clone()) {
        Deref @ Absyn::Class { restriction: Absyn::Restriction::R_TYPE, body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, typeSpecs: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Any" }, arrayDim: None }, tail: Deref @ metamodelica::List::Nil }, arrayDim: None }, .. }, name: ident, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(replaceable type ")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(res)
}

fn getDefinitionPathString(mut path: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
    Ok(out)
}

pub fn getDefinitionTypeSpecPathString(mut tp: Arc<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ('mc: {
        let __mc_input = tp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: Deref @ metamodelica::List::Nil, path: p, .. } => {
                    Ok(getDefinitionPathString(p.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: tspecs, path: p, .. } => {
                    let mut tspecsStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    tspecsStr = List::map(tspecs.clone(), (std::sync::Arc::new(getDefinitionTypeSpecPathString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::TypeSpec>) -> Result<ArcStr> + 'static>));
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*getDefinitionPathString(p.clone())?); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*stringDelimitList(tspecsStr.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TPATH { path: p, .. } => {
                    Ok(getDefinitionPathString(p.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(s)
}

fn getDefinitionDimensions(mut ts: Arc<Absyn::TypeSpec>, mut attr: Absyn::ElementAttributes) -> i32 {
    let mut out: i32 = 0;
    out = (::match_deref::match_deref! { match &((ts.clone(), attr.clone())) {
        (Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(l1), .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            (l1.clone().len() as i32) + (l2.clone().len() as i32)
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: Some(l1), .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            (l1.clone().len() as i32) + (l2.clone().len() as i32)
        },
        (Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            (l2.clone().len() as i32)
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: None, .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            (l2.clone().len() as i32)
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDefinitionParts(mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut isFunction: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    res = 'mc: {
        let __mc_input = (parts.clone(), isFunction.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(getDefinitionTypeVars(inTypeVars.clone(), list![(literal!(")")).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents }, tail: rest }, _) => {
                    Ok(listAppend(getDefinitionContent(contents.clone(), isFunction.clone(), true)?, getDefinitionParts(rest.clone(), inTypeVars.clone(), isFunction.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents }, tail: rest }, _) => {
                    Ok(listAppend(getDefinitionContent(contents.clone(), isFunction.clone(), false)?, getDefinitionParts(rest.clone(), inTypeVars.clone(), isFunction.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    Ok(getDefinitionParts(rest.clone(), inTypeVars.clone(), isFunction.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn getDefinitionContent(mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut addFunctions: bool, mut isPublic: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    res = 'mc: {
        let __mc_input = (contents.clone(), addFunctions.clone(), isPublic.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_, replaceable_: false }, .. } }, tail: rest }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = res.clone();
                    res = getDefinitionContent(rest.clone(), addFunctions.clone(), isPublic.clone())?;
                    r#str = (getDefinitionsClass(class_.clone(), addFunctions.clone())?).clone();
                    Ok(cons((r#str.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_, replaceable_: true }, .. } }, tail: rest }, _, _) => {
                    let mut ident: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = res.clone();
                    res = getDefinitionContent(rest.clone(), addFunctions.clone(), isPublic.clone())?;
                    ident = (getDefinitionsReplaceableClass(class_.clone())?).clone();
                    Ok(cons((ident.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: attr @ Absyn::ElementAttributes { variability, direction, .. }, components, typeSpec: ts }, .. } }, tail: rest }, _, true) => {
                    let mut typeStr: ArcStr = arcstr::literal!("");
                    let mut dirStr: ArcStr = arcstr::literal!("");
                    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = res.clone();
                    typeStr = (getDefinitionTypeSpecPathString(ts.clone())?).clone();
                    dirStr = (getDefinitionDirString(direction.clone(), variability.clone(), addFunctions.clone())?).clone();
                    res = getDefinitionComponents((typeStr.clone()).clone(), (dirStr.clone()).clone(), getDefinitionDimensions(ts.clone(), attr.clone()), components.clone())?;
                    res2 = getDefinitionContent(rest.clone(), addFunctions.clone(), isPublic.clone())?;
                    Ok(listAppend(res.clone(), res2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path, .. }, .. } }, tail: rest }, false, true) => {
                    let mut typeStr: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = res.clone();
                    typeStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(extends ")); __mm_s.push_str(&*getDefinitionPathString(path.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    res = getDefinitionContent(rest.clone(), addFunctions.clone(), isPublic.clone())?;
                    Ok(cons((typeStr.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _) => {
                    Ok(getDefinitionContent(rest.clone(), addFunctions.clone(), isPublic.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn getDefinitionDirString(mut dir: Absyn::Direction, mut variability: Absyn::Variability, mut isFunction: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ((match (dir.clone(), isFunction.clone()) {
        (Absyn::Direction::INPUT, true) => literal!("input "),
        (Absyn::Direction::OUTPUT, true) => literal!("output "),
        (_, false) => {
            if '__try0: {
                let Absyn::CONST { .. } = (variability.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(res)
}

fn getDefinitionComponents(mut typeStr: ArcStr, mut dirStr: ArcStr, mut numDim: i32, mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    res = 'mc: {
        let __mc_input = components.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { arrayDim: l, name: ident, .. }, .. }, tail: rest } => {
                    let mut sumDim: i32 = 0;
                    let mut ident = (*ident).clone();
                    let mut res: Arc<metamodelica::List<ArcStr>> = res.clone();
                    sumDim = numDim.clone() + (l.clone().len() as i32);
                    ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dirStr.clone()); __mm_s.push_str(&*if (numDim.clone() == 0) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(sumDim.clone())); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*typeStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone();
                    ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    res = getDefinitionComponents((typeStr.clone()).clone(), (dirStr.clone()).clone(), numDim.clone(), rest.clone())?;
                    Ok(cons((ident.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                rest => {
                    Ok(getDefinitionComponents((typeStr.clone()).clone(), (dirStr.clone()).clone(), numDim.clone(), rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn getDefinitionTypeVars(mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inDefinitions: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outDefinitions: Arc<metamodelica::List<ArcStr>> = inDefinitions.clone();
    for mut ty_var in &*inTypeVars.clone().reverse() {
        let mut ty_var = ty_var.clone();
        outDefinitions = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(replaceable type ")); __mm_s.push_str(&*ty_var.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone(), outDefinitions.clone());
    }
    outDefinitions
}

/* End getDefinitions */
pub fn parseFile(mut fileName: ArcStr, mut encoding: ArcStr, mut updateProgram: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut topClassNamesQualified: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut parsed: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut dir: ArcStr = arcstr::literal!("");
    let mut filename: ArcStr = arcstr::literal!("");
    let mut lveStarted: bool = false;
    let mut lveInstance: Option<i32> = None;
    let mut features: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libraryKey: ArcStr = arcstr::literal!("");
    let mut licenseFile: ArcStr = arcstr::literal!("");
    if !(System::regularFileExists((fileName.clone()).clone())) {
        topClassNamesQualified = metamodelica::nil();
        return Ok(topClassNamesQualified);
    }
    (dir, filename) = Util::getAbsoluteDirectoryAndFile((fileName.clone()).clone())?;
    if filename.clone() == literal!("package.moc") {
        (lveStarted, lveInstance) = Parser::startLibraryVendorExecutable((dir.clone()).clone());
        if !(lveStarted.clone()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unable to start library vendor executable.")).clone()])?;
            topClassNamesQualified = metamodelica::nil();
            return Ok(topClassNamesQualified);
        }
    }
    parsed = Parser::parse((fileName.clone()).clone(), (encoding.clone()).clone(), (dir.clone()).clone(), lveInstance.clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
    parsed = MetaUtil::createMetaClassesInProgram(parsed.clone())?;
    topClassNamesQualified = getTopQualifiedClassnames(parsed.clone())?;
    if lveStarted.clone() {
        Parser::stopLibraryVendorExecutable(lveInstance.clone());
    }
    if updateProgram.clone() {
        SymbolTable::setAbsyn(InteractiveUtil::updateProgram(parsed.clone(), SymbolTable::getAbsyn(), false)?)?;
    }
    Ok(topClassNamesQualified)
}

pub fn getSCodeClassNamesRecursive(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = List::fold1(inProgram.clone(), (std::sync::Arc::new(getSCodeClassNamesRecursiveWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> + 'static>), None, metamodelica::nil());
    paths
}

fn getSCodeClassNamesRecursiveWork(mut inElement: Arc<SCode::Element>, mut inPath: Option<Arc<Absyn::Path>>, mut inAcc: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = (::match_deref::match_deref! { match &((inElement.clone(), inPath.clone(), inAcc.clone())) {
        (Deref @ SCode::Element::CLASS { name, .. }, None, acc) => {
            let mut classes: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut path: Arc<Absyn::Path>;
            let mut acc = (*acc).clone();
            path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
            acc = cons(path.clone(), acc.clone());
            classes = SCodeUtil::getClassElements(inElement.clone());
            acc = List::fold1(classes.clone(), (std::sync::Arc::new(getSCodeClassNamesRecursiveWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> + 'static>), Some(path.clone()), acc.clone());
            acc.clone()
        },
        (Deref @ SCode::Element::CLASS { name, .. }, Some(path), acc) => {
            let mut classes: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut path = (*path).clone();
            let mut acc = (*acc).clone();
            path = AbsynUtil::suffixPath(path.clone(), (name.clone()).clone())?;
            acc = cons(path.clone(), acc.clone());
            classes = SCodeUtil::getClassElements(inElement.clone());
            acc = List::fold1(classes.clone(), (std::sync::Arc::new(getSCodeClassNamesRecursiveWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> + 'static>), Some(path.clone()), acc.clone());
            acc.clone()
        },
        _ => {
            inAcc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(paths)
}

pub fn getAllInheritedClasses(mut inClassName: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outBaseClassNames: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outBaseClassNames = 'mc: {
        let __mc_input = (inClassName.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, p) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut exts: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    cdef = InteractiveUtil::getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    exts = InteractiveUtil::getExtendsElementspecInClass(cdef.clone())?;
                    paths = List::map(exts.clone(), (std::sync::Arc::new(InteractiveUtil::getBaseClassNameFromExtends) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementSpec>) -> Result<Arc<Absyn::Path>> + 'static>));
                    Ok(paths.clone())
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
    Ok(outBaseClassNames)
}

pub fn printIstmtStr(mut inStatements: GlobalScript::Statements) -> ArcStr {
    let mut strIstmt: ArcStr = arcstr::literal!("");
    strIstmt = (GlobalScriptDump::printIstmtsStr(inStatements.clone())).clone();
    strIstmt
}

fn getClassEnvNoElaboration(mut inProgram: Absyn::Program, mut inClassPath: Arc<Absyn::Path>, mut inEnv: FCore::Graph) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    let mut cl: Arc<SCode::Element>;
    let mut id: ArcStr = arcstr::literal!("");
    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut env: FCore::Graph;
    let mut ci_state: ClassInf::State;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), inEnv.clone(), inClassPath.clone(), None)?) {
        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa0.clone();
    restr = __pa1.clone();
    encflag = __pa2.clone();
    id = __pa3.clone();
    cl = __pa4.clone();
    env = __pa5.clone();
    env = FGraph::openScope(env.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
    ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env.clone())?)?;
    match '__try6: {
        (_, outEnv, _, _, _) = unwrap_break_err!(Inst::partialInstClassIn(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), cl.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0), '__try6);
        Ok::<_, anyhow::Error>((outEnv.clone(),))
    } {
        Ok((__try6_o0,)) => {
            outEnv = __try6_o0;
        }
        Err(_) => {
            (_, outEnv, _, _, _, _, _, _, _, _, _, _) = Inst::instClassIn(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), cl.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), None)?;
        }
    }
    Ok(outEnv)
}

pub fn setComponentDimensions(mut inClass: Arc<Absyn::Path>, mut inComponentName: Arc<Absyn::Path>, mut inDimensions: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inProgram: Absyn::Program) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut outResult: bool = false;
    let mut p_class: Arc<Absyn::Path>;
    let mut within_: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    match '__try0: {
        within_ = unwrap_break_err!(InteractiveUtil::buildWithin(inClass.clone()), '__try0);
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(inClass.clone(), inProgram.clone(), false, false), '__try0);
        cls = unwrap_break_err!(setComponentDimensionsInClass(cls.clone(), inComponentName.clone(), inDimensions.clone()), '__try0);
        outProgram = unwrap_break_err!(InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, inProgram.clone(), false), '__try0);
        outResult = true;
        Ok::<_, anyhow::Error>((outProgram.clone(), outResult.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outResult = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outResult = false;
        }
    }
    (outProgram, outResult)
}

fn setComponentDimensionsInClass(mut inClass: Arc<Absyn::Class>, mut inComponentName: Arc<Absyn::Path>, mut inDimensions: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::traverseClassComponents(inClass.clone(), Arc::new({ let __pe_b2 = inComponentName.clone(); let __pe_b3 = inDimensions.clone(); move |__pe_a0, __pe_a1| setComponentDimensionsInCompitems(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), false)?) {
        (__pa0, true) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

fn setComponentDimensionsInCompitems(mut inComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inFound: bool, mut inComponentName: Arc<Absyn::Path>, mut inDimensions: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, bool, bool)> {
    let mut outComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut outFound: bool = false;
    let mut outContinue: bool = false;
    let mut item: Arc<Absyn::ComponentItem> = Arc::new(<Absyn::ComponentItem as ::std::default::Default>::default());
    let mut rest_items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = inComponents.clone();
    let mut comp: Absyn::Component = <Absyn::Component as ::std::default::Default>::default();
    let mut args_old: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut args_new: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut eqmod_old: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut eqmod_new: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut comp_id: ArcStr = arcstr::literal!("");
    comp_id = (AbsynUtil::pathFirstIdent(inComponentName.clone())?).clone();
    while !(rest_items.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_items.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        item = __pa0.clone();
        rest_items = __pa1.clone();
        if AbsynUtil::componentName(item.clone())? == comp_id.clone() {
            let _ = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ComponentItem { component: comp @ Absyn::Component { .. }, .. } => {
            let mut comp = (*comp).clone();
            comp.arrayDim = List::map(inDimensions.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeSubscript, Arc<Absyn::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Subscript>> + 'static>));
            assign_field!(item.component = comp.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            outComponents = List::append_reverse(outComponents.clone(), cons(item.clone(), rest_items.clone()));
            outFound = true;
            outContinue = false;
            return Ok((outComponents, outFound, outContinue));
        }
        outComponents = cons(item.clone(), outComponents.clone());
    }
    outComponents = inComponents.clone();
    outFound = false;
    outContinue = true;
    Ok((outComponents, outFound, outContinue))
}

pub fn getInstantiatedParametersAndValues(mut odae: Option<DAE::DAElist>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut parametersAndValues: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut params: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s: ArcStr = arcstr::literal!("");
    let mut oe: Option<Arc<DAE::Exp>> = None;
    parametersAndValues = (match odae.clone() {
        Some(DAE::DAElist { elementLst: ref els }) => {
            params = DAEUtil::getParameters(els.clone(), metamodelica::nil())?;
            for mut p in &*params.clone() {
                let mut p = p.clone();
                strs = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, .. }, .. } => cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*DAEDump::dumpVarBindingStr(var_field!((*p).binding, DAE::Element::VAR).clone())?); ArcStr::from(__mm_s) }).clone(), strs.clone()),
        _ => strs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            Dangerous::listReverseInPlace(strs.clone())
        },
        _ => strs.clone(),
    });
    Ok(parametersAndValues)
}

pub fn getAccessAnnotation(mut className: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<ArcStr> {
    let mut access: ArcStr = arcstr::literal!("");
    access = ((match p.clone() {
        _ => {
            let mut accessStr: ArcStr = arcstr::literal!("");
            accessStr = (getNamedAnnotationExp(className.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Protection")).clone() }), Some((literal!("")).clone()), (std::sync::Arc::new(getAccessAnnotationString) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<ArcStr> + 'static>))?).clone();
            accessStr.clone()
        },
        _ => {
            literal!("")
        },
    })).clone();
    Ok(access)
}

fn getAccessAnnotationString(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut access: ArcStr = arcstr::literal!("");
    access = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            getAccessAnnotationString2(arglst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(access)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAccessAnnotationString2(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<ArcStr> {
    let mut access: ArcStr = arcstr::literal!("");
    access = ((::match_deref::match_deref! { match &(eltArgs.clone()) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::CREF { componentRef: cref }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "access" }, .. }, tail: _ } => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (Dump::printComponentRefStr(cref.clone())?).clone();
            name.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (getAccessAnnotationString2(xs.clone())?).clone();
            name.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(access)
}

pub fn checkAccessAnnotationAndEncryption(mut path: Arc<Absyn::Path>, mut p: Absyn::Program) -> Access {
    let mut access: Access = Access::hide;
    let mut access_str: ArcStr = arcstr::literal!("");
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut encryptedClass: bool = false;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { info: SourceInfo { fileName: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        fileName = __pa1.clone();
        encryptedClass = StringUtil::endsWith((fileName.clone()).clone(), (literal!(".moc")).clone());
        if encryptedClass.clone() {
            access = (::match_deref::match_deref! { match &(unwrap_break_err!(getAccessAnnotation(path.clone(), p.clone()), '__try0)) {
        Deref @ "Access.hide" => Access::hide.clone(),
        Deref @ "Access.icon" => Access::icon.clone(),
        Deref @ "Access.documentation" => Access::documentation.clone(),
        Deref @ "Access.diagram" => Access::diagram.clone(),
        Deref @ "Access.nonPackageText" => Access::nonPackageText.clone(),
        Deref @ "Access.nonPackageDuplicate" => Access::nonPackageDuplicate.clone(),
        Deref @ "Access.packageText" => Access::packageText.clone(),
        Deref @ "Access.packageDuplicate" => Access::packageDuplicate.clone(),
        _ if (!(AbsynUtil::pathIsIdent(path.clone()))) => checkAccessAnnotationAndEncryption(AbsynUtil::stripLast(path.clone()).unwrap(), p.clone()),
        _ => Access::documentation.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        } else {
            access = Access::all.clone();
        }
        Ok::<_, anyhow::Error>((access.clone(),))
    } {
        Ok((__try0_o0,)) => {
            access = __try0_o0;
        }
        Err(_) => {
            access = Access::all.clone();
        }
    }
    access
}

pub fn astContainsEncryptedClass(mut inProgram: Absyn::Program) -> Result<bool> {
    let mut containsEncryptedClass: bool = false;
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut fileName: ArcStr = arcstr::literal!("");
    let Absyn::PROGRAM { classes: __pa0, .. } = (inProgram.clone()) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    for mut c in &*classes.clone() {
        let mut c = c.clone();
        let __pa1 = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ Absyn::Class { info: SourceInfo { fileName: __pa1, .. }, .. } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        fileName = __pa1.clone();
        containsEncryptedClass = containsEncryptedClass.clone() || StringUtil::endsWith((fileName.clone()).clone(), (literal!(".moc")).clone());
        if containsEncryptedClass.clone() {
            break;
        }
    }
    Ok(containsEncryptedClass)
}

pub fn addEquation(mut clsPath: Arc<Absyn::Path>, mut eqStr: ArcStr, mut isInitial: bool) -> bool {
    let mut success: bool = false;
    let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut eq: Arc<Absyn::EquationItem>;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    if '__try0: {
        eq = unwrap_break_err!(Parser::stringEq((eqStr.clone()).clone(), (literal!("<internal>")).clone()), '__try0);
        program = unwrap_break_err!(transformPathedClassInProgram(clsPath.clone(), SymbolTable::getAbsyn(), Arc::new({ let __pe_b0 = eq.clone(); let __pe_b1 = isInitial.clone(); move |__pe_a2| AbsynUtil::appendEquation(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) })), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(program.clone()), '__try0);
        success = true;
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    success
}

pub fn updateEquation(mut clsPath: Arc<Absyn::Path>, mut oldEq: ArcStr, mut newEq: ArcStr, mut matchAll: bool, mut matchShallow: bool, mut matchDescription: bool, mut mergeDescription: bool) -> bool {
    let mut success: bool = false;
    let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut old_eq: Arc<Absyn::EquationItem>;
    let mut new_eq: Option<Arc<Absyn::EquationItem>> = None;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    match '__try0: {
        old_eq = unwrap_break_err!(Parser::stringEq((oldEq.clone()).clone(), (literal!("<internal>")).clone()), '__try0);
        new_eq = if (stringEmpty((newEq.clone()).clone())) {None} else {Some(unwrap_break_err!(Parser::stringEq((newEq.clone()).clone(), (literal!("<internal>")).clone()), '__try0))};
        program = unwrap_break_err!(transformPathedClassInProgram(clsPath.clone(), SymbolTable::getAbsyn(), Arc::new({ let __pe_b1 = old_eq.clone(); let __pe_b2 = new_eq.clone(); let __pe_b3 = matchAll.clone(); let __pe_b4 = matchShallow.clone(); let __pe_b5 = matchDescription.clone(); let __pe_b6 = mergeDescription.clone(); move |__pe_a0| updateEquation_impl(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) })), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(program.clone()), '__try0);
        success = true;
        Ok::<_, anyhow::Error>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    success
}

fn updateEquation_impl(mut cls: Arc<Absyn::Class>, mut oldEq: Arc<Absyn::EquationItem>, mut newEq: Option<Arc<Absyn::EquationItem>>, mut matchAll: bool, mut matchShallow: bool, mut matchDescription: bool, mut mergeDescription: bool) -> Result<Arc<Absyn::Class>> {
    fn merge_desc(mut oldEq: Arc<Absyn::EquationItem>, mut newEq: Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::EquationItem>> {
        let mut newEq: Arc<Absyn::EquationItem> = newEq;
        let mut cmt: Arc<Absyn::Comment> = Arc::new(<Absyn::Comment as ::std::default::Default>::default());
        let () = (::match_deref::match_deref! { match &((oldEq.clone(), newEq.clone())) {
        (Deref @ Absyn::EquationItem::EQUATIONITEM { .. }, Deref @ Absyn::EquationItem::EQUATIONITEM { .. }) => {
            if isSome(var_field!((*oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone()) {
                if isNone(var_field!((*newEq).comment, Absyn::EquationItem::EQUATIONITEM).clone()) {
                    assign_variant_field!(newEq => Absyn::EquationItem::EQUATIONITEM; comment = var_field!((*oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone());
                } else {
                    let __pa0 = ::match_deref::match_deref! { match &(var_field!((*newEq).comment, Absyn::EquationItem::EQUATIONITEM).clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cmt = __pa0.clone();
                    if isNone(cmt.annotation_.clone()) {
                        assign_field!(cmt.annotation_ = AbsynUtil::getCommentOptAnnotation(var_field!((*oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone())?);
                    } else {
                        assign_field!(cmt.comment = AbsynUtil::getCommentOptComment(var_field!((*oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone())?);
                    }
                    assign_variant_field!(newEq => Absyn::EquationItem::EQUATIONITEM; comment = Some(cmt.clone()));
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(newEq)
    }

    let mut cls: Arc<Absyn::Class> = cls;
    let mut part: Arc<Absyn::ClassPart>;
    let mut rest_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut accum_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut eq: Arc<Absyn::EquationItem>;
    let mut new_eq: Arc<Absyn::EquationItem>;
    let mut rest_eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut accum_eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut found: bool = false;
    let mut found_in_part: bool = false;
    rest_parts = AbsynUtil::getClassPartsInClass(cls.clone());
    while !(rest_parts.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_parts.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa0.clone();
        rest_parts = __pa1.clone();
        rest_eqs = AbsynUtil::getEquationItemsInPart(part.clone());
        accum_eqs = metamodelica::nil();
        found_in_part = false;
        while !(rest_eqs.clone().is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_eqs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eq = __pa2.clone();
            rest_eqs = __pa3.clone();
            if AbsynUtil::equationItemEqual(eq.clone(), oldEq.clone(), matchShallow.clone(), !(matchDescription.clone()))? {
                if isSome(newEq.clone()) {
                    new_eq = Util::getOption(newEq.clone())?;
                    if mergeDescription.clone() {
                        new_eq = merge_desc(eq.clone(), new_eq.clone())?;
                    }
                    accum_eqs = cons(new_eq.clone(), accum_eqs.clone());
                }
                found_in_part = true;
                found = true;
                if !(matchAll.clone()) {
                    accum_eqs = List::append_reverse(rest_eqs.clone(), accum_eqs.clone());
                    break;
                }
            } else {
                accum_eqs = cons(eq.clone(), accum_eqs.clone());
            }
        }
        if found_in_part.clone() {
            part = AbsynUtil::setEquationItemsInPart(Dangerous::listReverseInPlace(accum_eqs.clone()), part.clone())?;
            accum_parts = cons(part.clone(), accum_parts.clone());
            if !(matchAll.clone()) {
                accum_parts = List::append_reverse(rest_parts.clone(), accum_parts.clone());
                break;
            }
        } else {
            accum_parts = cons(part.clone(), accum_parts.clone());
        }
    }
    if !(found.clone()) {
        bail!("fail");
    }
    cls = AbsynUtil::setClassPartsInClass(Dangerous::listReverseInPlace(accum_parts.clone()), cls.clone())?;
    Ok(cls)
}

