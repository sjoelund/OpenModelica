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
use crate::CevalScriptBackend;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::Static;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;

pub type Ident = ArcStr;

pub static VALID_SIMULATE_ARGS: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("startTime")).clone(), (literal!("stopTime")).clone(), (literal!("numberOfIntervals")).clone(), (literal!("stepSize")).clone(), (literal!("tolerance")).clone(), (literal!("method")).clone(), (literal!("fileNamePrefix")).clone(), (literal!("options")).clone(), (literal!("outputFormat")).clone(), (literal!("variableFilter")).clone(), (literal!("cflags")).clone(), (literal!("simflags")).clone()] });

pub fn checkSimulationArguments(mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut callName: ArcStr, mut info: SourceInfo) -> Result<()> {
    let mut valid_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        if !(listMember((arg.argName.clone()).clone(), VALID_SIMULATE_ARGS.clone())) {
            Error::addSourceMessage(Error::NO_SUCH_PARAMETER.clone(), list![(callName.clone()).clone(), (arg.argName.clone()).clone()], info.clone())?;
            bail!("fail");
        }
    }
    Ok(())
}

pub fn elabCallInteractive(mut cache: FCore::Cache, mut env: FCore::Graph, mut r#fn: Arc<Absyn::ComponentRef>, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut r#impl: bool, mut pre: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut e: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut handles: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if Flags::getConfigBool(Flags::BUILDING_MODEL.clone())? {
        ErrorExt::delCheckpoint((literal!("elabCall_InteractiveFunction")).clone());
        bail!("fail");
    }
    handles = ErrorExt::popCheckPoint((literal!("elabCall_InteractiveFunction")).clone());
    match '__try0: {
        ErrorExt::setCheckpoint((literal!("elabCall_InteractiveFunction1")).clone());
        (cache, e, prop) = unwrap_break_err!(elabCallInteractive_work(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone()), '__try0);
        ErrorExt::delCheckpoint((literal!("elabCall_InteractiveFunction1")).clone());
        Ok::<_, anyhow::Error>((cache.clone(), e.clone(), prop.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            cache = __try0_o0;
            e = __try0_o1;
            prop = __try0_o2;
        }
        Err(_) => {
            ErrorExt::rollBack((literal!("elabCall_InteractiveFunction1")).clone());
            ErrorExt::pushMessages(handles.clone());
            bail!("fail");
        }
    }
    ErrorExt::freeMessages(handles.clone());
    Ok((cache, e, prop))
}

fn elabCallInteractive_work(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inNamedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplInst: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), inExps.clone(), inNamedArgs.clone(), inImplInst.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, _, _, r#impl, _, _) => {
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut exp_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    ErrorExt::setCheckpoint((literal!("Scripting")).clone());
                    cr = AbsynUtil::joinCrefs(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("OpenModelica")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("Scripting")).clone(), subscripts: metamodelica::nil() }) }), cr2.clone())?;
                    (cache, exp_1, prop) = Static::elabExp(cache.clone(), env.clone(), Arc::new(Absyn::Exp::CALL { function_: cr.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: inExps.clone(), argNames: inNamedArgs.clone() }), typeVars: metamodelica::nil() }), r#impl.clone(), false, inPrefix.clone(), info.clone())?;
                    ErrorExt::delCheckpoint((literal!("Scripting")).clone());
                    Ok((cache.clone(), exp_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, _, _, _, _, _) => {
                    ErrorExt::rollBack((literal!("Scripting")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "translateModel", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("translateModel")).clone(), info.clone(), None)?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("translateModel")).clone(), simulationArgs.clone(), DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "modelEquationsUC", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, args, r#impl, pre, _) => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut outputFile: Arc<DAE::Exp>;
                    let mut dumpExtractionSteps: Arc<DAE::Exp>;
                    let mut className: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    (cache, cr_1) = Static::elabUntypedCref(cache.clone(), env.clone(), cr.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    className = ComponentReference::crefToPathIgnoreSubs(cr_1.clone())?;
                    (cache, outputFile) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), (literal!("outputFile")).clone(), DAE::T_STRING_DEFAULT.clone(), args.clone(), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), pre.clone(), info.clone())?;
                    (cache, dumpExtractionSteps) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), (literal!("dumpSteps")).clone(), DAE::T_BOOL_DEFAULT.clone(), args.clone(), Arc::new(DAE::Exp::BCONST { bool: false }), pre.clone(), info.clone())?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("modelEquationsUC")).clone(), list![Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT.clone() }), outputFile.clone(), dumpExtractionSteps.clone()], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "translateModelCPP", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, args, r#impl, pre, _) => {
                    let mut cname_str: Ident = arcstr::literal!("");
                    let mut filenameprefix: Arc<DAE::Exp>;
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut className: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    className = AbsynUtil::crefToPath(cr.clone())?;
                    cname_str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    (cache, filenameprefix) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), (literal!("fileNamePrefix")).clone(), DAE::T_STRING_DEFAULT.clone(), args.clone(), Arc::new(DAE::Exp::SCONST { string: (cname_str.clone()).clone() }), pre.clone(), info.clone())?;
                    recordtype = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationObject")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("flatClass")).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("exeFile")).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false });
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("translateModelCPP")).clone(), list![Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT.clone() }), filenameprefix.clone()], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "translateModelXML", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, args, r#impl, pre, _) => {
                    let mut cname_str: Ident = arcstr::literal!("");
                    let mut filenameprefix: Arc<DAE::Exp>;
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut className: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    className = AbsynUtil::crefToPath(cr.clone())?;
                    cname_str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    (cache, filenameprefix) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), (literal!("fileNamePrefix")).clone(), DAE::T_STRING_DEFAULT.clone(), args.clone(), Arc::new(DAE::Exp::SCONST { string: (cname_str.clone()).clone() }), pre.clone(), info.clone())?;
                    recordtype = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationObject")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("flatClass")).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("exeFile")).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false });
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("translateModelXML")).clone(), list![Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT.clone() }), filenameprefix.clone()], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "exportDAEtoMatlab", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, args, r#impl, pre, _) => {
                    let mut cname_str: Ident = arcstr::literal!("");
                    let mut filenameprefix: Arc<DAE::Exp>;
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut className: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    className = AbsynUtil::crefToPath(cr.clone())?;
                    cname_str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    (cache, filenameprefix) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), (literal!("fileNamePrefix")).clone(), DAE::T_STRING_DEFAULT.clone(), args.clone(), Arc::new(DAE::Exp::SCONST { string: (cname_str.clone()).clone() }), pre.clone(), info.clone())?;
                    recordtype = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationObject")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("flatClass")).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("exeFile")).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false });
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("exportDAEtoMatlab")).clone(), list![Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT.clone() }), filenameprefix.clone()], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "buildModel", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("buildModel")).clone(), info.clone(), None)?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("buildModel")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] }), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "buildModelBeast", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("buildModelBeast")).clone(), info.clone(), None)?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("buildModelBeast")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] }), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "simulate", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("simulate")).clone(), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("simulate")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "simulation", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("simulation")).clone(), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getDrModelicaSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("simulation")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "linearize", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("linearize")).clone(), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("linearize")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "optimize", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("optimize")).clone(), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("optimize")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "moo", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut recordtype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut simulationArgs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), inExps.clone(), args.clone(), inImplInst.clone(), inPrefix.clone(), (literal!("moo")).clone(), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("moo")).clone(), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT.clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "jacobian", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, _, r#impl, pre, _) => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, cr_1) = Static::elabUntypedCref(cache.clone(), env.clone(), cr.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    crefExp = Expression::crefExp(cr_1.clone())?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("jacobian")).clone(), list![crefExp.clone()], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "timing", .. }, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, r#impl, pre, _) => {
                    let mut exp_1: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, exp_1, _) = elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("timing")).clone(), list![exp_1.clone()], DAE::T_REAL_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::List::Nil, args, _, _, _) => {
                    let mut excludeList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut excludeListSize: i32 = 0;
                    excludeList = Static::getOptionalNamedArgExpList((literal!("exclude")).clone(), args.clone())?;
                    excludeListSize = (excludeList.clone().len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("checkExamplePackages")).clone(), list![Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize.clone() })] }), scalar: false, array: excludeList.clone() })], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: r#str }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut excludeList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut excludeListSize: i32 = 0;
                    excludeList = Static::getOptionalNamedArgExpList((literal!("exclude")).clone(), args.clone())?;
                    excludeListSize = (excludeList.clone().len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("checkExamplePackages")).clone(), list![Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize.clone() })] }), scalar: false, array: excludeList.clone() }), Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, args, _, _, _) => {
                    let mut excludeList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut excludeListSize: i32 = 0;
                    let mut className: Arc<Absyn::Path>;
                    className = AbsynUtil::crefToPath(cr.clone())?;
                    excludeList = Static::getOptionalNamedArgExpList((literal!("exclude")).clone(), args.clone())?;
                    excludeListSize = (excludeList.clone().len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("checkExamplePackages")).clone(), list![Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize.clone() })] }), scalar: false, array: excludeList.clone() }), Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT.clone() })], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: r#str }, tail: Deref @ metamodelica::List::Nil } }, args, _, _, _) => {
                    let mut excludeList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut excludeListSize: i32 = 0;
                    let mut className: Arc<Absyn::Path>;
                    className = AbsynUtil::crefToPath(cr.clone())?;
                    excludeList = Static::getOptionalNamedArgExpList((literal!("exclude")).clone(), args.clone())?;
                    excludeListSize = (excludeList.clone().len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall((literal!("checkExamplePackages")).clone(), list![Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize.clone() })] }), scalar: false, array: excludeList.clone() }), Arc::new(DAE::Exp::CODE { code: Arc::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT.clone() }), Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })], DAE::T_STRING_DEFAULT.clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

pub fn elabExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabExp2(inCache.clone(), inEnv.clone(), inExp.clone(), inImplicit.clone(), performVectorization.clone(), inPrefix.clone(), info.clone(), Error::getNumErrorMessages())?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplicit: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut numErrorMessages: i32) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inImplicit.clone(), performVectorization.clone(), inPrefix.clone(), info.clone(), numErrorMessages.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args }, function_: r#fn, .. }, r#impl, _, pre, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = elabCall(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone(), Error::getNumErrorMessages())?;
                    (e_1, _) = ExpressionSimplify::simplify1(e_1.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp, r#impl, doVect, pre, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

fn elabCall(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inImplInst: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut numErrorMessages: i32) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inComponentRef.clone(), inAbsynExpLst.clone(), inAbsynNamedArgLst.clone(), inImplInst.clone(), inPrefix.clone(), info.clone(), numErrorMessages.clone())) {
        (cache, env, r#fn, args, nargs, r#impl, pre, _, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            (cache, e, prop) = elabCallInteractive_work(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache.clone(), e.clone(), prop.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, outProperties))
}

pub fn elabGraphicsExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inImplInst: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inImplInst.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args }, function_: r#fn, .. }, _, pre, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = elabCall(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), true, pre.clone(), info.clone(), Error::getNumErrorMessages())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, r#impl, pre, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

