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

use crate::AvlSetInt;
use crate::BackendDAE;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Sorting;
use openmodelica_ast::Absyn;
use openmodelica_frontend::AvlSetCR;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub const BORDER: &'static str = "********************************************************************************";

pub const UNDERLINE: &'static str = "================================================================================";

type selectParameterFunc = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>;

pub fn evaluateParameters(mut DAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut DAE: Arc<BackendDAE::BackendDAE> = DAE;
    let mut selectParameterfunc: selectParameterFunc;
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut initialEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut graph: FCore::Graph;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut ass2: metamodelica::Array<i32>;
    let mut markarr: metamodelica::Array<i32>;
    let mut size: i32 = 0;
    let mut mark: i32 = 0;
    let mut nselect: i32 = 0;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut selectedParameters: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ht: Arc<AvlSetCR::Tree> = Arc::new(AvlSetCR::Tree::EMPTY);
    let mut isInitial: bool = false;
    isInitial = BackendDAEUtil::isInitializationDAE(DAE.shared.clone());
    if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBEGINNING of preOptModule 'evaluateParameters'\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        BackendDump::dumpBackendDAE(DAE.clone(), (literal!("DAE before evaluating parameters")).clone())?;
    }
    if !(Flags::isSet(Flags::EVAL_PARAM.clone())?) {
        selectParameterfunc = (match (Flags::getConfigBool(Flags::EVALUATE_FINAL_PARAMS.clone())?, Flags::getConfigBool(Flags::EVALUATE_PROTECTED_PARAMS.clone())?) {
        (false, false) => {
            if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                println!("{}", (literal!("\nStructural parameters and parameters with annotation(Evaluate=true) will be evaluated.\n")).clone());
            }
            (std::sync::Arc::new(fnptr!(BackendVariable::hasVarEvaluateAnnotationTrue, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)
        },
        (true, false) => {
            if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                println!("{}", (literal!("\nStructural parameters, final parameters and parameters with annotation(Evaluate=true) will be evaluated.\n")).clone());
            }
            (std::sync::Arc::new(fnptr!(BackendVariable::hasVarEvaluateAnnotationTrueOrFinal, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)
        },
        (false, true) => {
            if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                println!("{}", (literal!("\nStructural parameters, protected parameters and parameters with annotation(Evaluate=true) will be evaluated.\n")).clone());
            }
            (std::sync::Arc::new(fnptr!(BackendVariable::hasVarEvaluateAnnotationTrueOrProtected, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)
        },
        (true, true) => {
            if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                println!("{}", (literal!("\nStructural parameters, final parameters, protected parameters and parameters with annotation(Evaluate=true) will be evaluated.\n")).clone());
            }
            (std::sync::Arc::new(fnptr!(BackendVariable::hasVarEvaluateAnnotationTrueOrFinalOrProtected, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)
        },
        _ => bail!("match: no arm matched"),
    });
        let (__pa0, __pa6, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(DAE.clone()) {
            Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa6 @ BackendDAE::Shared { graph: __pa1, cache: __pa2, initialEqs: __pa3, aliasVars: __pa4, globalKnownVars: __pa5, .. }, .. } => (__pa0.clone(), __pa6.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        systs = __pa0.clone();
        graph = __pa1.clone();
        cache = __pa2.clone();
        initialEqs = __pa3.clone();
        aliasVars = __pa4.clone();
        globalKnownVars = __pa5.clone();
        shared = __pa6.clone();
        size = BackendVariable::varsSize(globalKnownVars.clone())?;
        m = arrayCreate(size.clone(), metamodelica::nil());
        mt = arrayCreate(size.clone(), metamodelica::nil());
        ass2 = Array::createIntRange(size.clone());
        ht = FCore::getEvaluatedParams(cache.clone())?;
        (_, _, _, selectedParameters, m, mt, _, _) = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(getParameterAdjacencyMatrix) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, i32, selectParameterFunc, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, i32, selectParameterFunc, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<AvlSetCR::Tree>, bool))> + 'static>), (globalKnownVars.clone(), 1, selectParameterfunc.clone(), metamodelica::nil(), m.clone(), mt.clone(), ht.clone(), isInitial.clone()))?;
        nselect = (selectedParameters.clone().len() as i32);
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSTART evaluating parameters:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of parameters: ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of parameters selected for evaluation: ")); __mm_s.push_str(&*intString(nselect.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Selected parameters for evaluation:\n")); __mm_s.push_str(&*stringDelimitList(List::map(selectedParameters.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            BackendDump::dumpAdjacencyMatrix(m.clone());
            BackendDump::dumpAdjacencyMatrixT(mt.clone());
        }
        markarr = arrayCreate(size.clone(), -1);
        size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size.clone())) * (metamodelica::OrderedFloat(0.7_f64))).0 as i32));
        nselect = intMax(BaseHashTable::defaultBucketSize.clone(), nselect.clone() * 2);
        repl = BackendVarTransform::emptyReplacementsSized(size.clone());
        oRepl = BackendVarTransform::emptyReplacementsSized(nselect.clone());
        (globalKnownVars, cache, repl, oRepl, mark) = evaluateSelectedParameters(selectedParameters.clone(), globalKnownVars.clone(), m.clone(), initialEqs.clone(), cache.clone(), graph.clone(), markarr.clone(), isInitial.clone(), repl.clone(), oRepl.clone(), 1)?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nAfter evaluating the selected parameters:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("\nAll replacements:")).clone());
            BackendVarTransform::dumpReplacements(repl.clone())?;
            println!("{}", (literal!("\nReplacements that will be replaced in the DAE:")).clone());
            BackendVarTransform::dumpReplacements(oRepl.clone())?;
            BackendDump::dumpVariables(globalKnownVars.clone(), (literal!("globalKnownVars")).clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmark: ")); __mm_s.push_str(&*intString(mark.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("markarr: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(markarr.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        comps = Sorting::TarjanTransposed(mt.clone(), ass2.clone())?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nAfter sorting parameters:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\nOrder:\n")); ArcStr::from(__mm_s) }).clone());
            for mut comp in &*comps.clone() {
                let mut comp = comp.clone();
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(comp.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
        (globalKnownVars, repl, oRepl, cache, mark) = traverseParameterSorted(comps.clone(), globalKnownVars.clone(), m.clone(), initialEqs.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), repl.clone(), oRepl.clone(), isInitial.clone())?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nAfter replacing the evaluated parameters in parameter bindings:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("\nAll replacements:")).clone());
            BackendVarTransform::dumpReplacements(repl.clone())?;
            println!("{}", (literal!("\nReplacements that will be replaced in the DAE:")).clone());
            BackendVarTransform::dumpReplacements(oRepl.clone())?;
            BackendDump::dumpVariables(globalKnownVars.clone(), (literal!("globalKnownVars")).clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmark: ")); __mm_s.push_str(&*intString(mark.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("markarr: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(markarr.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        let (__pa7, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, _, __pa15, __pa16)) = List::mapFold(systs.clone(), (std::sync::Arc::new(replaceEvaluatedParametersSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)) -> Result<(Arc<BackendDAE::EqSystem>, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements))> + 'static>), (globalKnownVars.clone(), m.clone(), initialEqs.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), oRepl.clone()));
        systs = __pa7.clone();
        globalKnownVars = __pa8.clone();
        m = __pa9.clone();
        initialEqs = __pa10.clone();
        cache = __pa11.clone();
        graph = __pa12.clone();
        mark = __pa13.clone();
        markarr = __pa14.clone();
        repl = __pa15.clone();
        oRepl = __pa16.clone();
        (aliasVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(aliasVars.clone(), (std::sync::Arc::new(replaceEvaluatedParameterTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements))> + 'static>), (globalKnownVars.clone(), m.clone(), initialEqs.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), oRepl.clone()))?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nAfter replacing the evaluated parameters in variable bindings and start attributes:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("\nAll replacements:")).clone());
            BackendVarTransform::dumpReplacements(repl.clone())?;
            println!("{}", (literal!("\nReplacements that will be replaced in the DAE:")).clone());
            BackendVarTransform::dumpReplacements(oRepl.clone())?;
            BackendDump::dumpVariables(globalKnownVars.clone(), (literal!("globalKnownVars")).clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmark: ")); __mm_s.push_str(&*intString(mark.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("markarr: ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(markarr.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        if Flags::getConfigBool(Flags::REPLACE_EVALUATED_PARAMS.clone())? {
            assign_field!(shared.externalObjects = BackendVariable::listVar1(List::map1(BackendVariable::varList(shared.externalObjects.clone())?, (std::sync::Arc::new(BackendVarTransform::replaceBindingExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<BackendDAE::Var> + 'static>), oRepl.clone())));
        }
        assign_field!(
            shared.globalKnownVars = globalKnownVars.clone(),
            shared.aliasVars = aliasVars.clone(),
            shared.initialEqs = initialEqs.clone(),
            shared.graph = graph.clone(),
            shared.cache = cache.clone()
        );
        DAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
        if Flags::getConfigBool(Flags::REPLACE_EVALUATED_PARAMS.clone())? {
            if !(BackendVarTransform::isReplacementEmpty(oRepl.clone())) {
                DAE = replaceEvaluatedParametersEqns(DAE.clone(), oRepl.clone())?;
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    BackendDump::dumpBackendDAE(DAE.clone(), (literal!("DAE after replacing the evaluated parameters")).clone())?;
                }
            } else {
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    println!("{}", (literal!("\nThere is no evaluated parameter.\n")).clone());
                }
            }
        } else {
            if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                Error::addCompilerNotification((literal!("Evaluated parameters are not replaced in the DAE. Use --replaceEvaluatedParameters=true to replace them in the DAE.")).clone())?;
            }
        }
    } else {
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\nThere is nothing to do. All parameters are already evaluated.\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEND of preOptModule 'evaluateParameters'\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(DAE)
}

fn getParameterAdjacencyMatrix(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, i32, Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, i32, Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<AvlSetCR::Tree>, bool))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (BackendDAE::Variables, i32, selectParameterFunc, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<AvlSetCR::Tree>, bool);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(e), varKind: BackendDAE::VarKind::PARAM, .. }, (globalKnownVars, index, selectParameter, selectedParameters, m, mt, ht, isInitial)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut tree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut select: bool = false;
                    let mut selectedParameters = (*selectedParameters).clone();
                    let mut m = (*m).clone();
                    let mut mt = (*mt).clone();
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(BackendDAEUtil::traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (globalKnownVars.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone()))?;
                    tree = __pa0.clone();
                    ilst = AvlSetInt::listKeys(tree.clone(), metamodelica::nil());
                    cref = BackendVariable::varCref(v.clone())?;
                    select = selectParameter(v.clone())? || AvlSetCR::hasKey(ht.clone(), cref.clone())?;
                    selectedParameters = List::consOnTrue(select.clone(), index.clone(), selectedParameters.clone());
                    m = {let _arr = m.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = ilst.clone(); _arr};
                    mt = List::fold1(cons(index.clone(), ilst.clone()), (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), index.clone(), mt.clone());
                    Ok((v.clone(), (globalKnownVars.clone(), index.clone() + 1, selectParameter.clone(), selectedParameters.clone(), m.clone(), mt.clone(), ht.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, varKind: BackendDAE::VarKind::PARAM, .. }, (globalKnownVars, index, selectParameter, selectedParameters, m, mt, ht, isInitial)) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut tree: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut select: bool = false;
                    let mut selectedParameters = (*selectedParameters).clone();
                    let mut m = (*m).clone();
                    let mut mt = (*mt).clone();
                    e = DAEUtil::getStartAttrFail(attr.clone())?;
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(BackendDAEUtil::traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (globalKnownVars.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone()))?;
                    tree = __pa0.clone();
                    ilst = AvlSetInt::listKeys(tree.clone(), metamodelica::nil());
                    cref = BackendVariable::varCref(v.clone())?;
                    select = selectParameter(v.clone())? || AvlSetCR::hasKey(ht.clone(), cref.clone())?;
                    selectedParameters = List::consOnTrue(select.clone(), index.clone(), selectedParameters.clone());
                    m = {let _arr = m.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = ilst.clone(); _arr};
                    mt = List::fold1(cons(index.clone(), ilst.clone()), (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), index.clone(), mt.clone());
                    Ok((v.clone(), (globalKnownVars.clone(), index.clone() + 1, selectParameter.clone(), selectedParameters.clone(), m.clone(), mt.clone(), ht.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (globalKnownVars, index, selectParameter, selectedParameters, m, mt, ht, isInitial)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut select: bool = false;
                    let mut selectedParameters = (*selectedParameters).clone();
                    let mut mt = (*mt).clone();
                    cref = BackendVariable::varCref(v.clone())?;
                    select = selectParameter(v.clone())? || AvlSetCR::hasKey(ht.clone(), cref.clone())?;
                    selectedParameters = List::consOnTrue(select.clone(), index.clone(), selectedParameters.clone());
                    ilst = list![index.clone()];
                    mt = {let _arr = mt.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = ilst.clone(); _arr};
                    Ok((v.clone(), (globalKnownVars.clone(), index.clone() + 1, selectParameter.clone(), selectedParameters.clone(), m.clone(), mt.clone(), ht.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn evaluateSelectedParameters(mut iSelected: Arc<metamodelica::List<i32>>, mut globalKnownVars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut cache: FCore::Cache, mut graph: FCore::Graph, mut markarr: metamodelica::Array<i32>, mut isInitial: bool, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements, mut mark: i32) -> Result<(BackendDAE::Variables, FCore::Cache, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements, i32)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut mark: i32 = mark;
    for mut i in &*iSelected.clone() {
        let mut i = i.clone();
        (globalKnownVars, cache, repl, replEvaluate, mark) = evaluateSelectedParameters0(i.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone(), mark.clone())?;
    }
    Ok((globalKnownVars, cache, repl, replEvaluate, mark))
}

fn evaluateSelectedParameters0(mut i: i32, mut globalKnownVars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut cache: FCore::Cache, mut graph: FCore::Graph, mut markarr: metamodelica::Array<i32>, mut isInitial: bool, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements, mut mark: i32) -> Result<(BackendDAE::Variables, FCore::Cache, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements, i32)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut mark: i32 = mark;
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    match '__try0: {
        let false = (intGt(markarr.borrow()[(i.clone()-1) as usize].clone(), 0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        {let _arr = markarr.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = mark.clone(); _arr};
        (globalKnownVars, cache, mark, repl, replEvaluate) = unwrap_break_err!(evaluateSelectedParameters1(m.borrow()[(i.clone()-1) as usize].clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone()), '__try0);
        v = unwrap_break_err!(BackendVariable::getVarAt(globalKnownVars.clone(), i.clone()), '__try0);
        (v, globalKnownVars, cache, mark, repl) = unwrap_break_err!(evaluateFixedAttribute(v.clone(), true, globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone()), '__try0);
        (globalKnownVars, repl, replEvaluate, cache) = unwrap_break_err!(evaluateSelectedParameter(v.clone(), i.clone(), globalKnownVars.clone(), inIEqns.clone(), repl.clone(), replEvaluate.clone(), cache.clone(), graph.clone()), '__try0);
        Ok::<_, anyhow::Error>((cache.clone(), globalKnownVars.clone(), mark.clone(), repl.clone(), replEvaluate.clone(), v.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            cache = __try0_o0;
            globalKnownVars = __try0_o1;
            mark = __try0_o2;
            repl = __try0_o3;
            replEvaluate = __try0_o4;
            v = __try0_o5;
        }
        Err(_) => {
            v = BackendVariable::getVarAt(globalKnownVars.clone(), i.clone())?;
            (globalKnownVars, repl, replEvaluate, cache) = evaluateSelectedParameter(v.clone(), i.clone(), globalKnownVars.clone(), inIEqns.clone(), repl.clone(), replEvaluate.clone(), cache.clone(), graph.clone())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((globalKnownVars, cache, repl, replEvaluate, mark))
}

fn evaluateSelectedParameters1(mut iUsed: Arc<metamodelica::List<i32>>, mut globalKnownVars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut cache: FCore::Cache, mut graph: FCore::Graph, mut mark: i32, mut markarr: metamodelica::Array<i32>, mut isInitial: bool, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, FCore::Cache, i32, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut mark: i32 = mark;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    (globalKnownVars, cache, mark, repl, replEvaluate) = 'mc: {
        let __mc_input = iUsed.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: i, tail: rest } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut mark: i32 = mark.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let false = (intGt(markarr.borrow()[(i.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    {let _arr = markarr.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = mark.clone(); _arr};
                    (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(m.borrow()[(i.clone()-1) as usize].clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())?;
                    v = BackendVariable::getVarAt(globalKnownVars.clone(), i.clone())?;
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), true, globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    (globalKnownVars, cache, repl, replEvaluate) = evaluateParameter(v.clone(), i.clone(), globalKnownVars.clone(), inIEqns.clone(), cache.clone(), graph.clone(), repl.clone(), replEvaluate.clone())?;
                    (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(rest.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())?;
                    Ok((globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut mark: i32 = mark.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(rest.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())?;
                    Ok((globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((globalKnownVars, cache, mark, repl, replEvaluate))
}

fn evaluateSelectedParameter(mut var: BackendDAE::Var, mut index: i32, mut globalKnownVars: BackendDAE::Variables, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements, mut cache: FCore::Cache, mut graph: FCore::Graph) -> Result<(BackendDAE::Variables, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements, FCore::Cache)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut cache: FCore::Cache = cache;
    let () = 'mc: {
        let __mc_input = var.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { bindExp: Some(e), varKind: BackendDAE::VarKind::CONST, varName: cr, .. } => {
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let true = (Expression::isConst(e.clone())?) else { bail!("pattern mismatch") };
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e.clone(), None)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { bindExp: Some(e), varKind: BackendDAE::VarKind::CONST, varName: cr, .. } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e1: Arc<DAE::Exp>;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache: FCore::Cache = cache.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    (e1, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
                    (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    v = BackendVariable::setBindExp(var.clone(), Some(e1.clone()));
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { bindExp: Some(e), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let true = (Expression::isConst(e.clone())?) else { bail!("pattern mismatch") };
                    v = BackendVariable::setVarFinal(var.clone(), true)?;
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    if BackendVariable::varFixed(v.clone()) {
                        repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e.clone(), None)?;
                        replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e.clone(), None)?;
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { bindExp: Some(e), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e1: Arc<DAE::Exp>;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    (e1, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
                    (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    v = BackendVariable::setBindExp(var.clone(), Some(e1.clone()));
                    v = BackendVariable::setVarFinal(v.clone(), true)?;
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { values: attr, varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varHasBindExp(var.clone())) else { bail!("pattern mismatch") };
                    e = DAEUtil::getStartAttrFail(attr.clone())?;
                    (e1, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
                    (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    v = BackendVariable::setVarStartValue(var.clone(), e1.clone())?;
                    v = BackendVariable::setVarFinal(v.clone(), true)?;
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                        info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                        Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot evaluate Variable \"")); __mm_s.push_str(&*BackendDump::varString(var.clone())?); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((globalKnownVars, repl, replEvaluate, cache))
}

fn evaluateParameter(mut var: BackendDAE::Var, mut index: i32, mut globalKnownVars: BackendDAE::Variables, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut cache: FCore::Cache, mut graph: FCore::Graph, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, FCore::Cache, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let () = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { bindExp: Some(e), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } => {
            let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut e1: Arc<DAE::Exp>;
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut e = (*e).clone();
            (e, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
            (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
            e1 = ValuesUtil::valueExp(value.clone(), None)?;
            v = BackendVariable::setVarFinal(var.clone(), true)?;
            globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
            repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
            replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
            ()
        },
        BackendDAE::Var { values: attr, varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } if (BackendVariable::varFixed(var.clone())) => {
            let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            e = DAEUtil::getStartAttrFail(attr.clone())?;
            (e, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
            (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
            e1 = ValuesUtil::valueExp(value.clone(), None)?;
            v = BackendVariable::setVarFinal(var.clone(), true)?;
            globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
            repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
            replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((globalKnownVars, cache, repl, replEvaluate))
}

fn evaluateFixedAttribute(mut var: BackendDAE::Var, mut addVar: bool, mut globalKnownVars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut cache: FCore::Cache, mut graph: FCore::Graph, mut mark: i32, mut markarr: metamodelica::Array<i32>, mut isInitial: bool, mut repl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendDAE::Variables, FCore::Cache, i32, BackendVarTransform::VariableReplacements)> {
    let mut var: BackendDAE::Var = var;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut mark: i32 = mark;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    (var, globalKnownVars, cache, mark, repl) = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { values: None, .. } => {
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { source, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: Some(e), .. }), varName: cr, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(cr.clone(), e.clone(), attr.clone(), source.clone(), var.clone(), addVar.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { source, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: Some(e), .. }), varName: cr, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(cr.clone(), e.clone(), attr.clone(), source.clone(), var.clone(), addVar.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { source, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(e), .. }), varName: cr, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(cr.clone(), e.clone(), attr.clone(), source.clone(), var.clone(), addVar.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        BackendDAE::Var { source, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(e), .. }), varName: cr, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(cr.clone(), e.clone(), attr.clone(), source.clone(), var.clone(), addVar.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        _ => {
            (var.clone(), globalKnownVars.clone(), cache.clone(), mark.clone(), repl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((var, globalKnownVars, cache, mark, repl))
}

fn evaluateFixedAttribute1(mut cr: Arc<DAE::ComponentRef>, mut e: Arc<DAE::Exp>, mut attr: Option<Arc<DAE::VariableAttributes>>, mut source: Arc<DAE::ElementSource>, mut var: BackendDAE::Var, mut addVar: bool, mut globalKnownVars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut cache: FCore::Cache, mut graph: FCore::Graph, mut mark: i32, mut markarr: metamodelica::Array<i32>, mut isInitial: bool, mut repl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendDAE::Variables, FCore::Cache, i32, BackendVarTransform::VariableReplacements)> {
    let mut var: BackendDAE::Var = var;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut mark: i32 = mark;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut e1: Arc<DAE::Exp>;
    let mut b: bool = false;
    let mut ilst: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    let mut attr1: Option<Arc<DAE::VariableAttributes>> = None;
    (e1, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?;
    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(BackendDAEUtil::traversingadjacencyRowExpFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool)) -> Result<(Arc<DAE::Exp>, bool, (BackendDAE::Variables, Arc<AvlSetInt::Tree>, bool))> + 'static>), (globalKnownVars.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), isInitial.clone()))?;
    ilst = __pa0.clone();
    (globalKnownVars, cache, mark, repl, _) = evaluateSelectedParameters1(AvlSetInt::listKeys(ilst.clone(), metamodelica::nil()), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), BackendVarTransform::emptyReplacements())?;
    (e1, _) = BackendVarTransform::replaceExp(e1.clone(), repl.clone(), None)?;
    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
    b = Expression::isConst(e1.clone())?;
    e1 = evaluateFixedAttributeReportWarning(b.clone(), cr.clone(), e.clone(), e1.clone(), source.clone(), globalKnownVars.clone())?;
    attr1 = DAEUtil::setFixedAttr(attr.clone(), Some(e1.clone()))?;
    var = BackendVariable::setVarAttributes(var.clone(), attr1.clone());
    globalKnownVars = if (addVar.clone()) {BackendVariable::addVar(var.clone(), globalKnownVars.clone())?} else {globalKnownVars.clone()};
    Ok((var, globalKnownVars, cache, mark, repl))
}

fn evaluateFixedAttributeReportWarning(mut b: bool, mut cr: Arc<DAE::ComponentRef>, mut e: Arc<DAE::Exp>, mut e1: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut globalKnownVars: BackendDAE::Variables) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut msg: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    if b.clone() {
        outExp = e1.clone();
    } else {
        info = ElementSource::getElementSourceFileInfo(source.clone());
        (outExp, _) = Expression::traverseExpBottomUp(e1.clone(), (std::sync::Arc::new(replaceCrefWithBindStartExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> + 'static>), (globalKnownVars.clone(), false, HashSet::emptyHashSet()))?;
        msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(" has unevaluateable fixed attribute value \"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\" use values from start attribute(s) \"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
        Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![(msg.clone()).clone()], info.clone())?;
    }
    Ok(outExp)
}

fn replaceCrefWithBindStartExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, b, hs)) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut b = (*b).clone();
                    let mut hs = (*hs).clone();
                    let false = (BaseHashSet::has(cr.clone(), hs.clone())?) else { bail!("pattern mismatch") };
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    e = BackendVariable::varStartValueType(v.clone())?;
                    hs = BaseHashSet::add(cr.clone(), hs.clone())?;
                    let (__pa0, (_, __pa1, __pa2)) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(replaceCrefWithBindStartExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> + 'static>), (vars.clone(), b.clone(), hs.clone()))?;
                    e = __pa0.clone();
                    b = __pa1.clone();
                    hs = __pa2.clone();
                    Ok((e.clone(), (vars.clone(), b.clone(), hs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { .. }, (vars, _, hs)) => {
                    Ok((e.clone(), (vars.clone(), true, hs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

fn traverseParameterSorted(mut inComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inGlobalKnownVars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inIEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iCache: FCore::Cache, mut graph: FCore::Graph, mut iMark: i32, mut markarr: metamodelica::Array<i32>, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements, mut isInitial: bool) -> Result<(BackendDAE::Variables, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements, FCore::Cache, i32)> {
    let mut oKnVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut oReplEvaluate: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut oCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut oMark: i32 = 0;
    (oKnVars, oRepl, oReplEvaluate, oCache, oMark) = (::match_deref::match_deref! { match &(inComps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inGlobalKnownVars.clone(), repl.clone(), replEvaluate.clone(), iCache.clone(), iMark.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, tail: rest } => {
            let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut repl1: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut evrepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut mark: i32 = 0;
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            v = BackendVariable::getVarAt(inGlobalKnownVars.clone(), i.clone())?;
            (v, globalKnownVars, cache, mark, repl1) = evaluateFixedAttribute(v.clone(), true, inGlobalKnownVars.clone(), m.clone(), inIEqns.clone(), iCache.clone(), graph.clone(), iMark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
            (globalKnownVars, repl1, evrepl) = evaluateParameterBindings(v.clone(), i.clone(), globalKnownVars.clone(), cache.clone(), graph.clone(), repl1.clone(), replEvaluate.clone())?;
            (globalKnownVars, repl1, evrepl, cache, mark) = traverseParameterSorted(rest.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), repl1.clone(), evrepl.clone(), isInitial.clone())?;
            (globalKnownVars.clone(), repl1.clone(), evrepl.clone(), cache.clone(), mark.clone())
        },
        Deref @ metamodelica::List::Cons { head: ilst, tail: rest } => {
            let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut repl1: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut evrepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut mark: i32 = 0;
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            (globalKnownVars, repl1, evrepl, cache, mark) = traverseParameterSorted(List::map(ilst.clone(), std::sync::Arc::new(fnptr!(List::create, _))), inGlobalKnownVars.clone(), m.clone(), inIEqns.clone(), iCache.clone(), graph.clone(), iMark.clone(), markarr.clone(), repl.clone(), replEvaluate.clone(), isInitial.clone())?;
            (globalKnownVars, repl1, evrepl, cache, mark) = traverseParameterSorted(rest.clone(), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), repl1.clone(), evrepl.clone(), isInitial.clone())?;
            (globalKnownVars.clone(), repl1.clone(), evrepl.clone(), cache.clone(), mark.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oKnVars, oRepl, oReplEvaluate, oCache, oMark))
}

fn evaluateParameterBindings(mut var: BackendDAE::Var, mut index: i32, mut globalKnownVars: BackendDAE::Variables, mut cache: FCore::Cache, mut graph: FCore::Graph, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let () = 'mc: {
        let __mc_input = var.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ BackendDAE::Var { hideResult: hideResultOpt, bindExp: Some(e), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut hideResultExp: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut v = (*v).clone();
                    let mut e = (*e).clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    if Expression::isConst(e.clone())? && BackendVariable::isFinalVar(v.clone()) && BackendVariable::varFixed(v.clone()) {
                        (repl, replEvaluate) = addConstExpReplacement(e.clone(), cr.clone(), repl.clone(), replEvaluate.clone())?;
                    } else {
                        (e, b) = BackendVarTransform::replaceExp(e.clone(), replEvaluate.clone(), None)?;
                        if b.clone() {
                            (e, _) = ExpressionSimplify::simplify(e.clone())?;
                            e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { expLst: exps, .. } if (Expression::isConstWorkList(exps.clone())?) => {
                    let mut e1: Arc<DAE::Exp>;
                    (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    e1.clone()
        },
        Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { expLst: exps, .. }, sub: _ } if (Expression::isConstWorkList(exps.clone())?) => {
                    let mut e1: Arc<DAE::Exp>;
                    (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    e1.clone()
        },
        _ => {
                    e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                            v = BackendVariable::setBindExp(v.clone(), Some(e.clone()));
                            if !(BackendVariable::hasVarEvaluateAnnotationFalse(v.clone())) {
                                        (repl, replEvaluate) = addConstExpReplacement(e.clone(), cr.clone(), repl.clone(), replEvaluate.clone())?;
                                        v = if (Expression::isConst(e.clone())?) {BackendVariable::setVarFinal(v.clone(), true)?} else {v.clone()};
                            }
                        }
                    }
                    let (__pa0, (__pa1, _)) = BackendDAEUtil::traverseBackendDAEVarAttr(v.values.clone(), (std::sync::Arc::new(traverseExpVisitorWrapper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> + 'static>), (replEvaluate.clone(), false))?;
                    attr = __pa0.clone();
                    replEvaluate = __pa1.clone();
                    v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                    v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
        Some(hideResultExp) => {
                    let mut hideResultExp = (*hideResultExp).clone();
                    (hideResultExp, b) = BackendVarTransform::replaceExp(hideResultExp.clone(), replEvaluate.clone(), None)?;
                    if b.clone() {
                        (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                    }
                    Some(hideResultExp.clone())
        },
        _ => v.hideResult.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ BackendDAE::Var { hideResult: hideResultOpt, values: attr, varKind: BackendDAE::VarKind::PARAM, varName: cr, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut hideResultExp: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut v = (*v).clone();
                    let mut attr = (*attr).clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    e = DAEUtil::getStartAttrFail(attr.clone())?;
                    (e, b) = BackendVarTransform::replaceExp(e.clone(), replEvaluate.clone(), None)?;
                    if b.clone() {
                        (e, _) = ExpressionSimplify::simplify(e.clone())?;
                        e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { expLst: exps, .. } if (Expression::isConstWorkList(exps.clone())?) => {
                    let mut e1: Arc<DAE::Exp>;
                    (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    e1.clone()
        },
        Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { expLst: exps, .. }, sub: _ } if (Expression::isConstWorkList(exps.clone())?) => {
                    let mut e1: Arc<DAE::Exp>;
                    (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    e1.clone()
        },
        _ => {
                    e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        v = BackendVariable::setVarStartValue(var.clone(), e.clone())?;
                        (repl, replEvaluate) = addConstExpReplacement(e.clone(), cr.clone(), repl.clone(), replEvaluate.clone())?;
                        v = if (Expression::isConst(e.clone())?) {BackendVariable::setVarFinal(v.clone(), true)?} else {v.clone()};
                    }
                    let (__pa0, (__pa1, _)) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(traverseExpVisitorWrapper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> + 'static>), (replEvaluate.clone(), false))?;
                    attr = __pa0.clone();
                    replEvaluate = __pa1.clone();
                    v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                    v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
        Some(hideResultExp) => {
                    let mut hideResultExp = (*hideResultExp).clone();
                    (hideResultExp, b) = BackendVarTransform::replaceExp(hideResultExp.clone(), replEvaluate.clone(), None)?;
                    if b.clone() {
                        (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                    }
                    Some(hideResultExp.clone())
        },
        _ => v.hideResult.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ BackendDAE::Var { hideResult: hideResultOpt, bindExp: Some(e), .. } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut hideResultExp: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut v = (*v).clone();
                    let mut e = (*e).clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    (e, b) = BackendVarTransform::replaceExp(e.clone(), replEvaluate.clone(), None)?;
                    if b.clone() {
                        (e, _) = ExpressionSimplify::simplify(e.clone())?;
                        e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { expLst: exps, .. } if (Expression::isConstWorkList(exps.clone())?) => {
                    let mut e1: Arc<DAE::Exp>;
                    (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    e1.clone()
        },
        Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { expLst: exps, .. }, sub: _ } if (Expression::isConstWorkList(exps.clone())?) => {
                    let mut e1: Arc<DAE::Exp>;
                    (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    e1.clone()
        },
        _ => {
                    e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        v = BackendVariable::setBindExp(var.clone(), Some(e.clone()));
                    }
                    let (__pa0, (__pa1, _)) = BackendDAEUtil::traverseBackendDAEVarAttr(v.values.clone(), (std::sync::Arc::new(traverseExpVisitorWrapper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> + 'static>), (replEvaluate.clone(), false))?;
                    attr = __pa0.clone();
                    replEvaluate = __pa1.clone();
                    v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                    v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
        Some(hideResultExp) => {
                    let mut hideResultExp = (*hideResultExp).clone();
                    (hideResultExp, b) = BackendVarTransform::replaceExp(hideResultExp.clone(), replEvaluate.clone(), None)?;
                    if b.clone() {
                        (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                    }
                    Some(hideResultExp.clone())
        },
        _ => v.hideResult.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                BackendDAE::Var { hideResult: hideResultOpt, values: attr, .. } => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut hideResultExp: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut attr = (*attr).clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(traverseExpVisitorWrapper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> + 'static>), (replEvaluate.clone(), false))?) {
                        (__pa0, (__pa1, true)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    attr = __pa0.clone();
                    replEvaluate = __pa1.clone();
                    v = BackendVariable::setVarAttributes(var.clone(), attr.clone());
                    v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
        Some(hideResultExp) => {
                    let mut hideResultExp = (*hideResultExp).clone();
                    (hideResultExp, b) = BackendVarTransform::replaceExp(hideResultExp.clone(), replEvaluate.clone(), None)?;
                    if b.clone() {
                        (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                    }
                    Some(hideResultExp.clone())
        },
        _ => v.hideResult.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index.clone(), v.clone())?;
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
    Ok((globalKnownVars, repl, replEvaluate))
}

fn addConstExpReplacement(mut inExp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>, mut repl: BackendVarTransform::VariableReplacements, mut replEvaluate: BackendVarTransform::VariableReplacements) -> Result<(BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)> {
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    if Expression::isConst(inExp.clone())? {
        repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), inExp.clone(), None)?;
        replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), inExp.clone(), None)?;
    }
    Ok((repl, replEvaluate))
}

fn traverseExpVisitorWrapper(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendVarTransform::VariableReplacements, bool);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (exp @ Deref @ DAE::Exp::CREF { .. }, (repl, b)) => {
            let mut b1: bool = false;
            let mut exp = (*exp).clone();
            (exp, b1) = BackendVarTransform::replaceExp(exp.clone(), repl.clone(), None)?;
            (exp.clone(), (repl.clone(), b.clone() || b1.clone()))
        },
        _ => {
            (inExp.clone(), inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn replaceEvaluatedParametersSystem(mut isyst: Arc<BackendDAE::EqSystem>, mut inTypeA: (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)) -> Result<(Arc<BackendDAE::EqSystem>, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements))> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outTypeA: (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements);
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    (vars, outTypeA) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(replaceEvaluatedParameterTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements))> + 'static>), inTypeA.clone())?;
    osyst = BackendDAEUtil::setEqSystVars(isyst.clone(), vars.clone())?;
    Ok((osyst, outTypeA))
}

fn replaceEvaluatedParameterTraverser(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, FCore::Cache, FCore::Graph, i32, metamodelica::Array<i32>, bool, BackendVarTransform::VariableReplacements, BackendVarTransform::VariableReplacements);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, bindExp: Some(e), .. }, (globalKnownVars, m, ieqns, cache, graph, mark, markarr, isInitial, repl, replEvaluate)) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut v = (*v).clone();
                    let mut attr = (*attr).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut cache = (*cache).clone();
                    let mut mark = (*mark).clone();
                    let mut repl = (*repl).clone();
                    let mut replEvaluate = (*replEvaluate).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), replEvaluate.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    v = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    let (__pa1, (__pa2, __pa3)) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(traverseExpVisitorWrapper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> + 'static>), (replEvaluate.clone(), false))?;
                    attr = __pa1.clone();
                    replEvaluate = __pa2.clone();
                    b = __pa3.clone();
                    v = if (b.clone()) {BackendVariable::setVarAttributes(v.clone(), attr.clone())} else {v.clone()};
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), false, globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, .. }, (globalKnownVars, m, ieqns, cache, graph, mark, markarr, isInitial, repl, replEvaluate)) => {
                    let mut v = (*v).clone();
                    let mut attr = (*attr).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut cache = (*cache).clone();
                    let mut mark = (*mark).clone();
                    let mut repl = (*repl).clone();
                    let mut replEvaluate = (*replEvaluate).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(traverseExpVisitorWrapper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)) -> Result<(Arc<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool))> + 'static>), (replEvaluate.clone(), false))?) {
                        (__pa0, (__pa1, true)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    attr = __pa0.clone();
                    replEvaluate = __pa1.clone();
                    v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), false, globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (globalKnownVars, m, ieqns, cache, graph, mark, markarr, isInitial, repl, replEvaluate)) => {
                    let mut v = (*v).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut cache = (*cache).clone();
                    let mut mark = (*mark).clone();
                    let mut repl = (*repl).clone();
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), false, globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn replaceEvaluatedParametersEqns(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut b: bool = false;
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    lsteqns = BackendEquation::equationList(shared.initialEqs.clone());
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), inRepl.clone(), None)?;
    if b.clone() {
        assign_field!(shared.initialEqs = BackendEquation::listEquation(lsteqns.clone())?);
    }
    lsteqns = BackendEquation::equationList(shared.removedEqs.clone());
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), inRepl.clone(), None)?;
    if b.clone() {
        assign_field!(shared.removedEqs = BackendEquation::listEquation(lsteqns.clone())?);
    }
    systs = List::map1(systs.clone(), (std::sync::Arc::new(replaceEvaluatedParametersSystemEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), inRepl.clone());
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn replaceEvaluatedParametersSystemEqns(mut isyst: Arc<BackendDAE::EqSystem>, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut b: bool = false;
    lsteqns = BackendEquation::equationList(osyst.orderedEqs.clone());
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), inRepl.clone(), None)?;
    if b.clone() {
        assign_field!(osyst.orderedEqs = BackendEquation::listEquation(lsteqns.clone())?);
        osyst = BackendDAEUtil::clearEqSyst(osyst.clone())?;
    }
    lsteqns = BackendEquation::equationList(osyst.removedEqs.clone());
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), inRepl.clone(), None)?;
    if b.clone() {
        assign_field!(osyst.removedEqs = BackendEquation::listEquation(lsteqns.clone())?);
    }
    Ok(osyst)
}

