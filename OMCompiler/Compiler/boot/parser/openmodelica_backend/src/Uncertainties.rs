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

use crate::AdjacencyMatrix;
use crate::BackendDAE;
use crate::BackendDAECreate;
use crate::BackendDAEEXT;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::Matching;
use crate::MathematicaDump;
use crate::SimCode;
use crate::Sorting;
use crate::SymbolTable;
use crate::SymbolicJacobian;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTable2;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type ExtAdjacencyMatrixRow = (i32, Arc<metamodelica::List<i32>>);

pub type ExtAdjacencyMatrix = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

pub type mapBlocks = Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, bool, bool)>>;

// {blocks,blocks.visited,blocks.square}
pub const UNDERLINE: &'static str = "==========================================================================";

#[derive(Clone)]
pub struct AliasSet {
    pub symbols: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)),
    pub expl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)),
    pub signs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)),
    pub source: Option<Arc<DAE::ElementSource>>,
}

impl PartialEq for AliasSet {
    fn eq(&self, other: &Self) -> bool {
        (match ((&self.symbols), (&other.symbols)) { ((__lt0, __lt1, __lt2, __lt3, __lt4), (__rt0, __rt1, __rt2, __rt3, __rt4)) => (__lt0 == __rt0) && (__lt1 == __rt1) && (__lt2 == __rt2) && (__lt3 == __rt3) && (match (__lt4, __rt4) { ((__lt0, __lt1, __lt2), (__rt0, __rt1, __rt2)) => std::sync::Arc::ptr_eq(__lt0, __rt0) && std::sync::Arc::ptr_eq(__lt1, __rt1) && std::sync::Arc::ptr_eq(__lt2, __rt2) }) }) && (match ((&self.expl), (&other.expl)) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (__lt0 == __rt0) && (__lt1 == __rt1) && (__lt2 == __rt2) && (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => std::sync::Arc::ptr_eq(__lt0, __rt0) && std::sync::Arc::ptr_eq(__lt1, __rt1) && std::sync::Arc::ptr_eq(__lt2, __rt2) && std::sync::Arc::ptr_eq(__lt3, __rt3) }) }) && (match ((&self.signs), (&other.signs)) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (__lt0 == __rt0) && (__lt1 == __rt1) && (__lt2 == __rt2) && (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => std::sync::Arc::ptr_eq(__lt0, __rt0) && std::sync::Arc::ptr_eq(__lt1, __rt1) && std::sync::Arc::ptr_eq(__lt2, __rt2) && std::sync::Arc::ptr_eq(__lt3, __rt3) }) }) && self.source == other.source
    }
}
impl Eq for AliasSet {}
impl PartialOrd for AliasSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for AliasSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (match ((&self.symbols), (&other.symbols)) { ((__lt0, __lt1, __lt2, __lt3, __lt4), (__rt0, __rt1, __rt2, __rt3, __rt4)) => __lt0.cmp(__rt0).then_with(|| __lt1.cmp(__rt1).then_with(|| __lt2.cmp(__rt2).then_with(|| __lt3.cmp(__rt3).then_with(|| (match (__lt4, __rt4) { ((__lt0, __lt1, __lt2), (__rt0, __rt1, __rt2)) => (std::sync::Arc::as_ptr(__lt0) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt1) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt2) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ())))) }))))) }).then_with(|| (match ((&self.expl), (&other.expl)) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0.cmp(__rt0).then_with(|| __lt1.cmp(__rt1).then_with(|| __lt2.cmp(__rt2).then_with(|| (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (std::sync::Arc::as_ptr(__lt0) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt1) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt2) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt3) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))))) })))) }).then_with(|| (match ((&self.signs), (&other.signs)) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0.cmp(__rt0).then_with(|| __lt1.cmp(__rt1).then_with(|| __lt2.cmp(__rt2).then_with(|| (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (std::sync::Arc::as_ptr(__lt0) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt1) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt2) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt3) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))))) })))) }).then_with(|| self.source.cmp(&other.source))))
    }
}
impl std::fmt::Debug for AliasSet {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("AliasSet");
        __ds.field("symbols", &format_args!("<dyn-fn-container@{:p}>", (&self.symbols) as *const _));
        __ds.field("expl", &format_args!("<dyn-fn-container@{:p}>", (&self.expl) as *const _));
        __ds.field("signs", &format_args!("<dyn-fn-container@{:p}>", (&self.signs) as *const _));
        __ds.field("source", &self.source);
        __ds.finish()
    }
}

impl Default for AliasSet {
    fn default() -> Self {
        Self {
            symbols: (Default::default(), Default::default(), Default::default(), Default::default(), ({ let __placeholder: HashSet::FuncHashCref = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashSet::FuncCrefEqual = std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashSet::FuncCrefStr = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder })),
            expl: (Default::default(), Default::default(), Default::default(), ({ let __placeholder: HashTable2::FuncHashCref = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashTable2::FuncCrefEqual = std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashTable2::FuncCrefStr = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashTable2::FuncExpStr = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder })),
            signs: (Default::default(), Default::default(), Default::default(), ({ let __placeholder: HashTable::FuncHashCref = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashTable::FuncCrefEqual = std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashTable::FuncCrefStr = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder }, { let __placeholder: HashTable::FuncExpStr = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder })),
            source: Default::default(),
        }
    }
}

pub type ALIASSET = AliasSet;


pub fn modelEquationsUC(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut className: Arc<Absyn::Path>, mut outputFileIn: ArcStr, mut dumpSteps: bool) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = ({
        let mut forceOrdering: bool = Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?;
        'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), className.clone(), outputFileIn.clone(), dumpSteps.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, graph, _, outputFile, _) => {
                    let mut resstr: ArcStr = arcstr::literal!("");
                    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut dlow: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
                    let mut dlow_1: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut approximatedEquations_one: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut setC_eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut setS_eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut eqsyslist: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
                    let mut allVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut knownVariables: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut unknownVariables: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut variables: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unknowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut directlyLinked: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut indirectlyLinked: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut outputvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut mExt: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unknownsVarsMatch: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut remainingEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut removed_equations_squared: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut outStringA: ArcStr = arcstr::literal!("");
                    let mut outStringB: ArcStr = arcstr::literal!("");
                    let mut outString: ArcStr = arcstr::literal!("");
                    let mut description: ArcStr = arcstr::literal!("");
                    let mut distributions: Arc<metamodelica::List<Option<Arc<DAE::Distribution>>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    Print::clearBuf();
                    p = SymbolTable::getAbsyn();
                    (dae, cache, graph) = flattenModel(className.clone(), p.clone(), cache.clone())?;
                    description = (DAEUtil::daeDescription(dae.clone())).clone();
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), graph.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (outputFile.clone()).clone(), simSettingsOption: None })?;
                    FlagsUtil::setConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone(), false)?;
                    (dlow_1, _, _, _, _) = BackendDAEUtil::getSolvedSystem(dlow.clone(), (literal!("")).clone(), Some(list![(literal!("removeSimpleEquations")).clone(), (literal!("removeUnusedVariables")).clone(), (literal!("removeEqualRHS")).clone(), (literal!("expandDerOperator")).clone()]), None, None, Some(metamodelica::nil()))?;
                    FlagsUtil::setConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone(), forceOrdering.clone())?;
                    dlow_1 = removeSimpleEquationsUC(dlow_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(dlow_1.clone()) {
                        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: __pa1 }, derivativeNamePrefix: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    currentSystem = __pa0.clone();
                    eqsyslist = __pa1.clone();
                    shared = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(currentSystem.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedEqs: __pa3, orderedVars: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    allEqs = __pa3.clone();
                    allVars = __pa4.clone();
                    let __pa5 = ::match_deref::match_deref! { match &(shared.clone()) {
                        Deref @ BackendDAE::Shared { globalKnownVars: __pa5, .. } => __pa5.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    globalKnownVars = __pa5.clone();
                    (m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
                    let true = (eqsyslist.clone().is_empty()) else { bail!("pattern mismatch") };
                    mExt = getExtAdjacencyMatrix(m.clone())?;
                    variables = List::intRange(BackendVariable::varsSize(allVars.clone())?);
                    (knowns, _) = getUncertainRefineVariableIndexes(allVars.clone(), variables.clone())?;
                    directlyLinked = getRelatedVariables(mExt.clone(), knowns.clone())?;
                    indirectlyLinked = List::setDifference(getRelatedVariables(mExt.clone(), directlyLinked.clone())?, knowns.clone())?;
                    unknowns = listAppend(directlyLinked.clone(), indirectlyLinked.clone());
                    outputvars = List::setDifference(List::intRange(BackendVariable::varsSize(allVars.clone())?), listAppend(unknowns.clone(), knowns.clone()))?;
                    dlow_1 = eliminateVariablesDAE(unknowns.clone(), dlow_1.clone())?;
                    printSep((getMathematicaText((literal!("== Initial system ==")).clone())).clone())?;
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(dlow_1.clone()) {
                        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa6, tail: _ }, derivativeNamePrefix: __pa7, .. } => (__pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    currentSystem = __pa6.clone();
                    shared = __pa7.clone();
                    let (__pa8, __pa9) = ::match_deref::match_deref! { match &(currentSystem.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedEqs: __pa8, orderedVars: __pa9, .. } => (__pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    allEqs = __pa8.clone();
                    allVars = __pa9.clone();
                    let __pa10 = ::match_deref::match_deref! { match &(shared.clone()) {
                        Deref @ BackendDAE::Shared { globalKnownVars: __pa10, .. } => __pa10.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    globalKnownVars = __pa10.clone();
                    (m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
                    printSep((getMathematicaText((literal!("After Symbolic Elimination")).clone())).clone())?;
                    printSep((getMathematicaText((literal!("Equations (Function calls represent more than one equation)")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(List::intRange(BackendEquation::equationArraySize(allEqs.clone())?), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    printSep((getMathematicaText((literal!("Variables")).clone())).clone())?;
                    printSep((variablesToMathematicaGrid(List::intRange(BackendVariable::varsSize(allVars.clone())?), allVars.clone())?).clone())?;
                    mExt = getExtAdjacencyMatrix(m.clone())?;
                    approximatedEquations_one = getEquationsWithApproximatedAnnotation(dlow_1.clone())?;
                    approximatedEquations = List::flatten(List::map1r(approximatedEquations_one.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
                    mExt = removeEquations(mExt.clone(), approximatedEquations.clone())?;
                    printSep((getMathematicaText((literal!("Approximated equations to be removed")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(approximatedEquations.clone(), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    printSep((getMathematicaText((literal!("After eliminating approximated equations")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(getEquationsNumber(mExt.clone()), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    variables = List::intRange(BackendVariable::varsSize(allVars.clone())?);
                    (knowns, distributions) = getUncertainRefineVariableIndexes(allVars.clone(), variables.clone())?;
                    directlyLinked = getRelatedVariables(mExt.clone(), knowns.clone())?;
                    indirectlyLinked = List::setDifference(getRelatedVariables(mExt.clone(), directlyLinked.clone())?, knowns.clone())?;
                    unknowns = listAppend(directlyLinked.clone(), indirectlyLinked.clone());
                    outputvars = List::setDifference(List::intRange(BackendVariable::varsSize(allVars.clone())?), listAppend(unknowns.clone(), knowns.clone()))?;
                    printSep((getMathematicaText((literal!("Known variables")).clone())).clone())?;
                    printSep((variablesToMathematicaGrid(knowns.clone(), allVars.clone())?).clone())?;
                    printSep((getMathematicaText((literal!("Directly linked variables")).clone())).clone())?;
                    printSep((variablesToMathematicaGrid(directlyLinked.clone(), allVars.clone())?).clone())?;
                    printSep((getMathematicaText((literal!("Indirectly linked variables")).clone())).clone())?;
                    printSep((variablesToMathematicaGrid(indirectlyLinked.clone(), allVars.clone())?).clone())?;
                    printSep((getMathematicaText((literal!("Output variables")).clone())).clone())?;
                    printSep((variablesToMathematicaGrid(outputvars.clone(), allVars.clone())?).clone())?;
                    mExt = eliminateOutputVariables(mExt.clone(), outputvars.clone())?;
                    printSep((getMathematicaText((literal!("After eliminating output variables")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(getEquationsNumber(mExt.clone()), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    (setS, unknownsVarsMatch) = getEquationsForUnknownsSystem(mExt.clone(), knowns.clone(), unknowns.clone())?;
                    printSep((getMathematicaText((literal!("Matching performed after step 5 (Set S)")).clone())).clone())?;
                    printSep((unknowsMatchingToMathematicaGrid(unknownsVarsMatch.clone(), setS.clone(), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    remainingEquations = List::setDifference(getEquationsNumber(mExt.clone()), setS.clone())?;
                    printSep((getMathematicaText((literal!("Remaining equations")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(remainingEquations.clone(), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    (setC, removed_equations_squared) = getEquationsForKnownsSystem(mExt.clone(), knowns.clone(), unknowns.clone(), setS.clone(), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?;
                    if !(removed_equations_squared.clone().is_empty()) {
                        println!("{}", (literal!("Warning: the system is ill-posed. One or more equations have been removed from squared system of knowns.\n")).clone());
                    }
                    printSep((getMathematicaText((literal!("Equations removed from squared blocks (with more than one equation)")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(removed_equations_squared.clone(), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    printSep((getMathematicaText((literal!("Final Equations")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(setC.clone(), allEqs.clone(), allVars.clone(), globalKnownVars.clone(), mapIncRowEqn.clone())?).clone())?;
                    setC = List::map1r(setC.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
                    setC = List::unique(setC.clone());
                    setS = List::map1r(setS.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
                    setS = List::unique(setS.clone());
                    setC_eq = List::map1r(setC.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
                    setS_eq = List::map1r(setS.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
                    knownVariables = BackendVariable::listVar(List::map1r(knowns.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()));
                    unknownVariables = BackendVariable::listVar(List::map1r(unknowns.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()));
                    outStringB = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*getMathematicaVarStr(knownVariables.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*getMathematicaEqStr(setC_eq.clone(), allVars.clone(), globalKnownVars.clone())); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*getMathematicaVarStr(unknownVariables.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*getMathematicaEqStr(setS_eq.clone(), allVars.clone(), globalKnownVars.clone())); __mm_s.push_str(&*literal!("},")); __mm_s.push_str(&*dumpVarsDistributionInfo(distributions.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*getMathematicaText((literal!("Extraction finished")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone())?;
                    outStringA = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[{")); __mm_s.push_str(&*Print::getString()?); __mm_s.push_str(&*literal!("}]")); ArcStr::from(__mm_s) }).clone();
                    outString = (if (dumpSteps.clone()) {outStringA.clone()} else {outStringB.clone()}).clone();
                    resstr = (writeFileIfNonEmpty((outputFile.clone()).clone(), (outString.clone()).clone())?).clone();
                    Ok((cache.clone(), Arc::new(Values::Value::STRING { string: (resstr.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, outputFile, _) => {
                    let mut resstr: ArcStr = arcstr::literal!("");
                    let mut outStringA: ArcStr = arcstr::literal!("");
                    Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*getMathematicaText((literal!("Extraction failed")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone())?;
                    outStringA = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[{")); __mm_s.push_str(&*Print::getString()?); __mm_s.push_str(&*literal!("}]")); ArcStr::from(__mm_s) }).clone();
                    let _ = writeFileIfNonEmpty((outputFile.clone()).clone(), (outStringA.clone()).clone())?;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    resstr = AbsynUtil::pathStringNoQual(className.clone(), (literal!(".")).clone(), true, false)?;
                    resstr = stringAppendList(list![(literal!("modelEquationsUC: The model equations in model")).clone(), (resstr.clone()).clone(), (literal!(" could not be extracted")).clone()]);
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(resstr.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok((outCache, outValue))
}

/*
Function which runs the Extraction Algorithm for DataReconcilaiton Procedure
*/
pub fn dataReconciliation(mut inDae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    outDae = (::match_deref::match_deref! { match &(inDae.clone()) {
        dae => {
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut approximatedEquations_one: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut constantvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut extractedvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut setC_eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut setS_eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqsyslist: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
            let mut allVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut tmpglobalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut resVarsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut variables: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut unknowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut directlyLinked: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut indirectlyLinked: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut outputvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut finalvarlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut mExt: ExtAdjacencyMatrix = metamodelica::nil();
            let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tempsetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tempsetC: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut inputvarlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut match1: metamodelica::Array<i32>;
            let mut bltblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut blockstofind: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut blockranks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
            let mut blockstatus: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            let mut var: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut tempvar: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut tmpparamvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut blocktargetinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>> = metamodelica::nil();
            let mut predecessorblocktargetinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
            let mut initblocks: mapBlocks = metamodelica::nil();
            let mut modelname: ArcStr = arcstr::literal!("");
            let mut einfo: BackendDAE::ExtraInfo = <BackendDAE::ExtraInfo as ::std::default::Default>::default();
            let mut simcodejacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
            let mut outDiffVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut outResidualVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut outOtherVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut outResidualEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut outOtherEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(dae.clone()) {
                Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: __pa1 }, derivativeNamePrefix: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            currentSystem = __pa0.clone();
            eqsyslist = __pa1.clone();
            shared = __pa2.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(currentSystem.clone()) {
                Deref @ BackendDAE::EqSystem { orderedEqs: __pa3, orderedVars: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            allEqs = __pa3.clone();
            allVars = __pa4.clone();
            let (__pa5, __pa6) = ::match_deref::match_deref! { match &(shared.clone()) {
                Deref @ BackendDAE::Shared { info: __pa5, globalKnownVars: __pa6, .. } => (__pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            einfo = __pa5.clone();
            globalKnownVars = __pa6.clone();
            let BackendDAE::EXTRA_INFO { fileNamePrefix: __pa7, .. } = (einfo.clone()) else { bail!("pattern mismatch") };
            modelname = __pa7.clone();
            (m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nModelInfo: ")); __mm_s.push_str(&*modelname.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            BackendDump::dumpEquationArray(allEqs.clone(), (literal!("orderedEquation")).clone())?;
            BackendDump::dumpVariables(allVars.clone(), (literal!("orderedVariables")).clone())?;
            (match1, _) = Matching::PerfectMatching(m.clone())?;
            var = dumpMatching(match1.clone());
            BackendDump::dumpMatching(match1.clone())?;
            bltblocks = Sorting::Tarjan(m.clone(), match1.clone(), (match1.clone().borrow().len() as i32))?;
            dumpListList(bltblocks.clone(), (literal!("BLT_BLOCKS")).clone());
            let true = (eqsyslist.clone().is_empty()) else { bail!("pattern mismatch") };
            mExt = getExtAdjacencyMatrix(m.clone())?;
            variables = List::intRange(BackendVariable::varsSize(allVars.clone())?);
            (knowns, _) = getUncertainRefineVariableIndexes(allVars.clone(), variables.clone())?;
            directlyLinked = getRelatedVariables(mExt.clone(), knowns.clone())?;
            indirectlyLinked = List::setDifference(getRelatedVariables(mExt.clone(), directlyLinked.clone())?, knowns.clone())?;
            unknowns = listAppend(directlyLinked.clone(), indirectlyLinked.clone());
            outputvars = List::setDifference(List::intRange(BackendVariable::varsSize(allVars.clone())?), listAppend(unknowns.clone(), knowns.clone()))?;
            unknowns = listAppend(unknowns.clone(), outputvars.clone());
            let _ = listAppend(unknowns.clone(), knowns.clone());
            initblocks = setInitialBlocks(bltblocks.clone());
            constantvars = getConstantVariables(mExt.clone());
            approximatedEquations_one = getEquationsWithApproximatedAnnotation(dae.clone())?;
            approximatedEquations = List::flatten(List::map1r(approximatedEquations_one.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
            let _ = getRemovedEquationSolvedVariables(approximatedEquations.clone(), var.clone());
            (blockstofind, blockstatus) = originalBlocks(bltblocks.clone(), knowns.clone(), unknowns.clone(), outputvars.clone(), var.clone());
            blockranks = List::toListWithPositions(blockstofind.clone());
            blockstatus = checkBlockStatus(blockstofind.clone(), blockstatus.clone());
            blocktargetinfo = findBlockTargets(blockstofind.clone(), blockstatus.clone(), var.clone(), mExt.clone(), initblocks.clone(), blockranks.clone())?;
            predecessorblocktargetinfo = findPredecessorBlocks(blocktargetinfo.clone())?;
            (tempsetC, tempsetS) = ExtractEquationsfromPredecessorBlocks(predecessorblocktargetinfo.clone(), blockranks.clone(), approximatedEquations.clone())?;
            (_, _) = getVariableOccurence(tempsetC.clone(), mExt.clone(), knowns.clone());
            extractedvars = getVariablesAfterExtraction(tempsetC.clone(), tempsetS.clone(), mExt.clone());
            let _ = getVariablesAfterExtraction(tempsetS.clone(), metamodelica::nil(), mExt.clone());
            finalvarlist = getRemovedEquationSolvedVariables(listAppend(tempsetC.clone(), tempsetS.clone()), var.clone());
            (finalvarlist, inputvarlist, _) = List::intersection1OnTrue(extractedvars.clone(), finalvarlist.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            inputvarlist = List::setDifferenceOnTrue(inputvarlist.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFINAL SET OF EQUATIONS After Reconciliation \n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_C: ")); __mm_s.push_str(&*dumplistInteger(tempsetC.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(tempsetS.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            setC = List::map1r(tempsetC.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            setC = List::unique(setC.clone());
            setS = List::map1r(tempsetS.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            setS = List::unique(setS.clone());
            setC_eq = List::map1r(setC.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
            setS_eq = List::map1r(setS.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
            BackendDump::dumpEquationList(setC_eq.clone(), (literal!("SET_C")).clone());
            BackendDump::dumpEquationList(setS_eq.clone(), (literal!("SET_S")).clone());
            outDiffVars = BackendVariable::listVar(List::map1r(knowns.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()));
            outDiffVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
            (_, reqns) = BackendEquation::traverseEquationArray(BackendEquation::listEquation(setC_eq.clone())?, (std::sync::Arc::new(BackendEquation::traverseEquationToScalarResidualForm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))> + 'static>), (shared.functionTree.clone(), metamodelica::nil()))?;
            (reqns, resVarsLst, _) = BackendEquation::convertResidualsIntoSolvedEquations(reqns.clone().reverse(), (literal!("$res")).clone(), 1, false)?;
            outResidualVars = BackendVariable::listVar(resVarsLst.clone());
            outResidualEqns = BackendEquation::listEquation(reqns.clone())?;
            outOtherEqns = BackendEquation::listEquation(setS_eq.clone())?;
            tmpparamvars = BackendEquation::equationsVars(outOtherEqns.clone(), globalKnownVars.clone())?;
            cr_lst = BackendEquation::getAllCrefFromEquations(BackendEquation::listEquation(setS_eq.clone())?)?;
            outOtherVars = dumpCrefList(cr_lst.clone(), outDiffVars.clone(), tmpparamvars.clone())?;
            BackendDump::dumpVariables(outOtherVars.clone(), (literal!("Unknown variables in SET_S ")).clone())?;
            BackendDump::dumpVariables(BackendVariable::listVar(tmpparamvars.clone()), (literal!("Parameters in SET_S")).clone())?;
            VerifyDataReconciliation(tempsetC.clone(), tempsetS.clone(), knowns.clone(), unknowns.clone(), mExt.clone(), var.clone(), constantvars.clone(), approximatedEquations.clone(), allVars.clone(), allEqs.clone(), mapIncRowEqn.clone(), outOtherVars.clone(), setS_eq.clone())?;
            (simcodejacobian, shared) = SymbolicJacobian::getSymbolicJacobian(outDiffVars.clone(), outResidualEqns.clone(), outResidualVars.clone(), outOtherEqns.clone(), outOtherVars.clone(), shared.clone(), BackendVariable::listVar(List::map1r(extractedvars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone())), (literal!("F")).clone(), false)?;
            let _ = BackendVariable::listVar(List::map1r(getRemovedEquationSolvedVariables(tempsetC.clone(), var.clone()), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()));
            assign_field!(shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData { relatedBoundaryConditions: 0, symbolicJacobianH: None, setBVars: None, datareconinputs: outDiffVars.clone(), setcVars: outResidualVars.clone(), symbolicJacobian: simcodejacobian.clone() }));
            currentSystem = BackendDAEUtil::setEqSystVars(currentSystem.clone(), BackendVariable::mergeVariables(outResidualVars.clone(), outOtherVars.clone(), true)?)?;
            currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), BackendEquation::merge(outResidualEqns.clone(), outOtherEqns.clone())?);
            tempvar = BackendVariable::varList(outDiffVars.clone())?;
            tmpglobalKnownVars = BackendVariable::listVar(List::map1(tempvar.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirection, BackendDAE::Var, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, DAE::VarDirection) -> Result<BackendDAE::Var> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT));
            shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), BackendVariable::mergeVariables(globalKnownVars.clone(), tmpglobalKnownVars.clone(), true)?);
            if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelname.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone())) {
                r#str = (literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval,xi,xk,rx_ik\n")).clone();
                r#str = (dumpToCsv((r#str.clone()).clone(), tempvar.clone())?).clone();
                System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelname.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
            }
            outDae = BackendDAE::DAE(list![currentSystem.clone()], shared.clone())?;
            outDae.clone()
        },
        _ => {
            inDae.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

pub fn dumpCrefList(mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut invar: BackendDAE::Variables, mut paramvars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<BackendDAE::Variables> {
    let mut outvar: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut tmpcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut tmpparamcrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut tmpvar: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut count: i32 = 1;
    tmpcr = List::map(BackendVariable::varList(invar.clone())?, (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    tmpparamcrefs = List::map(paramvars.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    for mut i in &*cr_lst.clone() {
        let mut i = i.clone();
        if !(listMember(i.clone(), tmpcr.clone())) && !(listMember(i.clone(), tmpparamcrefs.clone())) {
            tmpvar = cons(BackendVariable::makeVar(i.clone()), tmpvar.clone());
            count = count.clone() + 1;
        }
    }
    outvar = BackendVariable::listVar(List::unique(tmpvar.clone()));
    Ok(outvar)
}

/* function which dumps the variable names to csv file */
pub fn dumpToCsv(mut instring: ArcStr, mut invar: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    for mut i in &*invar.clone() {
        let mut i = i.clone();
        cr = BackendVariable::varCref(i.clone())?;
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*instring.clone()); __mm_s.push_str(&*outstring.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outstring)
}

/* creates list of equations from SET-S needed for jacobian calculation */
pub fn createInnerEquations(mut tempsets: Arc<metamodelica::List<i32>>, mut solvedeqvar: Arc<metamodelica::List<(i32, i32)>>, mut sets: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut inputlist: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<BackendDAE::InnerEquation>> {
    let mut outequations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut varnumber: i32 = 0;
    let mut count: i32 = 1;
    let mut inpcount: i32 = 1;
    for mut eqnumber in &*tempsets.clone() {
        let mut eqnumber = eqnumber.clone();
        varnumber = getSolvedVariableNumber(eqnumber.clone(), solvedeqvar.clone());
        if !(listMember(varnumber.clone(), knowns.clone())) {
            outequations = cons(BackendDAE::InnerEquation::INNEREQUATION { eqn: (sets.clone()).get(count.clone()).unwrap(), vars: list![varnumber.clone()] }, outequations.clone());
        } else {
            outequations = cons(BackendDAE::InnerEquation::INNEREQUATION { eqn: (sets.clone()).get(count.clone()).unwrap(), vars: list![(inputlist.clone()).get(inpcount.clone()).unwrap()] }, outequations.clone());
            inpcount = inpcount.clone() + 1;
        }
        count = count.clone() + 1;
    }
    outequations = outequations.clone().reverse();
    outequations
}

pub fn dumpDependencyTree(mut invartree: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut ineqtree: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut knowns: Arc<metamodelica::List<i32>>, mut constantvars: Arc<metamodelica::List<i32>>, mut allVars: BackendDAE::Variables, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<()> {
    let mut varnumber: i32 = 0;
    let mut count: i32 = 1;
    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut depeqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut var: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut kn1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut kn2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut kn3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut c1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut c2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut c3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut flag: bool = false;
    for mut i in &*invartree.clone() {
        let mut i = i.clone();
        (varnumber, varlist) = i.clone();
        (_, eqs) = (ineqtree.clone()).get(count.clone())?;
        var = List::map1r(list![varnumber.clone()], (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone());
        depeqs = List::map1r(List::map1r(eqs.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
        (kn1, kn2, kn3) = List::intersection1OnTrue(varlist.clone(), listAppend(knowns.clone(), constantvars.clone()), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        if kn1.clone().is_empty() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n-The intermediate variable: ")); __mm_s.push_str(&*intString(varnumber.clone())); __mm_s.push_str(&*literal!(" does not have any knowns or constants as Leaf")); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 5-Failed : The system is ill-posed.")).clone()])?;
            return Ok(());
        }
        BackendDump::dumpVarList(var.clone(), (literal!("Intermediate_Variable_in_SET_C")).clone());
        BackendDump::dumpEquationList(depeqs.clone(), (literal!("Dependency_tree")).clone());
        count = count.clone() + 1;
    }
    Ok(())
}

pub fn getSolvedDependentEquationAndVars(mut inlist: Arc<metamodelica::List<i32>>, mut solvedvar: Arc<metamodelica::List<(i32, i32)>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut sets_eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets_vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnumber: i32 = 0;
    for mut varnumber in &*inlist.clone() {
        let mut varnumber = varnumber.clone();
        eqnumber = getSolvedEquationNumber(varnumber.clone(), solvedvar.clone());
        sets_eqs = cons(eqnumber.clone(), sets_eqs.clone());
        sets_vars = cons(varnumber.clone(), sets_vars.clone());
    }
    (sets_eqs, sets_vars)
}

pub fn getVariablesAfterExtraction(mut setc: Arc<metamodelica::List<i32>>, mut sets: Arc<metamodelica::List<i32>>, mut mext: ExtAdjacencyMatrix) -> Arc<metamodelica::List<i32>> {
    let mut finalvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut fulleqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    fulleqs = listAppend(setc.clone(), sets.clone());
    for mut i in &*fulleqs.clone() {
        let mut i = i.clone();
        for mut j in &*mext.clone() {
            let mut j = j.clone();
            (eq, vars) = j.clone();
            if intEq(i.clone(), eq.clone()) {
                for mut k in &*vars.clone() {
                    let mut k = k.clone();
                    finalvars = cons(k.clone(), finalvars.clone());
                }
            }
        }
    }
    finalvars = List::unique(finalvars.clone());
    finalvars
}

pub fn getConstantVariables(mut mext: ExtAdjacencyMatrix) -> Arc<metamodelica::List<i32>> {
    let mut constantvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnumber: i32 = 0;
    for mut i in &*mext.clone() {
        let mut i = i.clone();
        (eqnumber, vars) = i.clone();
        if (vars.clone().len() as i32) == 1 {
            for mut j in &*vars.clone() {
                let mut j = j.clone();
                constantvars = cons(j.clone(), constantvars.clone());
            }
        }
    }
    constantvars
}

pub fn VerifyDataReconciliation(mut setc: Arc<metamodelica::List<i32>>, mut sets: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>, mut mExt: ExtAdjacencyMatrix, mut solvedvar: Arc<metamodelica::List<(i32, i32)>>, mut constantvars: Arc<metamodelica::List<i32>>, mut approximatedEquations: Arc<metamodelica::List<i32>>, mut allVars: BackendDAE::Variables, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut outsetS_vars: BackendDAE::Variables, mut outsetS_eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<()> {
    let mut matchedeq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedknownssetc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedunknownssetc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedknownssets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedunknownssets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpunknowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpknowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist1sets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setstmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplistvar1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplistvar2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplistvar3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets_eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets_vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut extractedeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnumber: i32 = 0;
    let mut varnumber: i32 = 0;
    let mut var_dependencytree: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut eq_dependencytree: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut resstr: ArcStr = arcstr::literal!("");
    let mut var: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut convar: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nAutomatic Verification Steps of DataReconciliation Algorithm")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    var = List::map1r(knowns.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone());
    BackendDump::dumpVarList(var.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("knownVariables:")); __mm_s.push_str(&*dumplistInteger(knowns.clone().reverse())); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C:")); __mm_s.push_str(&*dumplistInteger(setc.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_S:")); __mm_s.push_str(&*dumplistInteger(sets.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    matchedeq = List::intersectionOnTrue(setc.clone(), sets.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Condition-1 ")); __mm_s.push_str(&*literal!("\"SET_C and SET_S must not have no equations in common\"")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if matchedeq.clone().is_empty() {
        println!("{}", (literal!("-Passed\n\n")).clone());
    } else {
        println!("{}", (literal!("-Failed\n")).clone());
        BackendDump::dumpEquationList(List::map1r(matchedeq.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Equations Found in SET_C and SET_S:")); __mm_s.push_str(&*dumplistInteger(matchedeq.clone())); ArcStr::from(__mm_s) }).clone());
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 1- Failed : The system is ill-posed.")).clone()])?;
        bail!("fail");
    }
    (matchedknownssetc, matchedunknownssetc) = getVariableOccurence(setc.clone(), mExt.clone(), knowns.clone());
    (matchedknownssets, matchedunknownssets) = getVariableOccurence(sets.clone(), mExt.clone(), knowns.clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Condition-2 ")); __mm_s.push_str(&*literal!("\"All variables of interest must be involved in SET_C or SET_S\"")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    (tmplist1, tmplist2, tmplist3) = List::intersection1OnTrue(matchedknownssetc.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if tmplist3.clone().is_empty() {
        println!("{}", (literal!("-Passed \n")).clone());
        BackendDump::dumpVarList(List::map1r(tmplist1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C has all known variables:")); __mm_s.push_str(&*dumplistInteger(tmplist1.clone())); ArcStr::from(__mm_s) }).clone());
    } else if !(tmplist3.clone().is_empty()) {
        (tmplist1sets, tmplist2, _) = List::intersection1OnTrue(tmplist3.clone(), matchedknownssets.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        if !(tmplist2.clone().is_empty()) {
            r#str = (dumplistInteger(tmplist2.clone())).clone();
            println!("{}", (literal!("-Failed\n")).clone());
            BackendDump::dumpVarList(List::map1r(tmplist2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("knownVariables not Found:")); __mm_s.push_str(&*dumplistInteger(tmplist2.clone())); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 2- Failed : The system is ill-posed.")).clone()])?;
            bail!("fail");
        }
        println!("{}", (literal!("-Passed \n")).clone());
        BackendDump::dumpVarList(List::map1r(tmplist1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C has known variables:")); __mm_s.push_str(&*dumplistInteger(tmplist1.clone())); ArcStr::from(__mm_s) }).clone());
        BackendDump::dumpVarList(List::map1r(tmplist1sets.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_S has known variables:")); __mm_s.push_str(&*dumplistInteger(tmplist1sets.clone())); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Condition-3 ")); __mm_s.push_str(&*literal!("\"SET_C equations must be strictly less than Variable of Interest\"")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if (setc.clone().len() as i32) < (knowns.clone().len() as i32) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_C contains:")); __mm_s.push_str(&*intString((setc.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations < ")); __mm_s.push_str(&*intString((knowns.clone().len() as i32))); __mm_s.push_str(&*literal!(" known variables \n\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        resstr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Failed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_C contains:")); __mm_s.push_str(&*intString((setc.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations  > ")); __mm_s.push_str(&*intString((knowns.clone().len() as i32))); __mm_s.push_str(&*literal!(" known variables \n\n")); ArcStr::from(__mm_s) }).clone();
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 3-Failed : The system is ill-posed.")).clone()])?;
        bail!("fail");
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Condition-4 ")); __mm_s.push_str(&*literal!("\"SET_S should contain all intermediate variables involved in SET_C\"")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    (tmplistvar1, tmplistvar2, tmplistvar3) = List::intersection1OnTrue(matchedunknownssetc.clone(), matchedunknownssets.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if matchedunknownssetc.clone().is_empty() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_C contains No Intermediate Variables \n\n")); ArcStr::from(__mm_s) }).clone());
        return Ok(());
    } else {
        BackendDump::dumpVarList(List::map1r(matchedunknownssetc.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C has intermediate variables:")); __mm_s.push_str(&*dumplistInteger(matchedunknownssetc.clone())); ArcStr::from(__mm_s) }).clone());
        if tmplistvar2.clone().is_empty() {
            BackendDump::dumpVarList(List::map1r(tmplistvar1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_S has intermediate variables involved in SET_C:")); __mm_s.push_str(&*dumplistInteger(tmplistvar1.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("-Passed\n\n")).clone());
        } else {
            BackendDump::dumpVarList(List::map1r(tmplistvar2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_S does not have intermediate variables involved in SET_C:")); __mm_s.push_str(&*dumplistInteger(tmplistvar2.clone())); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 4-Failed : The system is ill-posed.")).clone()])?;
            bail!("fail");
        }
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Condition-5 ")); __mm_s.push_str(&*literal!("\"SET_S should be square \"")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if outsetS_eq.clone().is_empty() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_S contains 0 intermediate variables and 0 equations \n\n")); ArcStr::from(__mm_s) }).clone());
        return Ok(());
    } else {
        if (outsetS_eq.clone().len() as i32) == (BackendVariable::varList(outsetS_vars.clone())?.len() as i32) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n ")); __mm_s.push_str(&*literal!("Set_S has ")); __mm_s.push_str(&*intString((outsetS_eq.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString((BackendVariable::varList(outsetS_vars.clone())?.len() as i32))); __mm_s.push_str(&*literal!(" variables\n\n")); ArcStr::from(__mm_s) }).clone());
        } else {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Failed")); __mm_s.push_str(&*literal!("\n ")); __mm_s.push_str(&*literal!("Set_S has ")); __mm_s.push_str(&*intString((outsetS_eq.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString((BackendVariable::varList(outsetS_vars.clone())?.len() as i32))); __mm_s.push_str(&*literal!(" variables\n\n")); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 5-Failed Set_S is not square: The system is ill-posed.")).clone()])?;
            bail!("fail");
        }
    }
    Ok(())
}

pub fn BuildSquareSubSetHelper(mut invars: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut mExt: ExtAdjacencyMatrix, mut solvedeqvar: Arc<metamodelica::List<(i32, i32)>>, mut solvedvars: Arc<metamodelica::List<i32>>, mut solvedeqs: Arc<metamodelica::List<i32>>, mut constantvars: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outlist1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outlist2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outlist1, outlist2) = ({
        let mut found: bool = false;
        (::match_deref::match_deref! { match &((invars.clone(), knowns.clone(), mExt.clone(), solvedeqvar.clone(), solvedvars.clone(), solvedeqs.clone(), constantvars.clone())) {
        (tmpvars, tmpknowns, tmpExt, tmpsolveeqvar, tempsolvedvars, tempsolvedeqs, tmpconstantvars) => {
            let mut t1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut t2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tempeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tempvars1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tempvars2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut allvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tmp2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut c1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut tempsolvedvars = (*tempsolvedvars).clone();
            let mut tempsolvedeqs = (*tempsolvedeqs).clone();
            (t1, t2, _) = List::intersection1OnTrue(tmpvars.clone(), tmpknowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            (c1, _, _) = List::intersection1OnTrue(tmpvars.clone(), tmpconstantvars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if !(c1.clone().is_empty()) {
                (tempsolvedeqs, _) = BuildSquareSubSetHelper1(c1.clone(), tmpsolveeqvar.clone(), tempsolvedeqs.clone());
                tempsolvedvars = listAppend(tempsolvedvars.clone(), c1.clone());
            }
            if !(t1.clone().is_empty()) {
                (tempsolvedeqs, _) = BuildSquareSubSetHelper1(t1.clone(), tmpsolveeqvar.clone(), tempsolvedeqs.clone());
                tempsolvedvars = listAppend(tempsolvedvars.clone(), t1.clone());
            }
            if found.clone() == false {
                tempsolvedvars = listAppend(tempsolvedvars.clone(), t2.clone());
                (tempsolvedeqs, tempeqs) = BuildSquareSubSetHelper1(t2.clone(), solvedeqvar.clone(), tempsolvedeqs.clone());
                (tempvars1, tempvars2) = getVariableOccurence(tempeqs.clone(), mExt.clone(), knowns.clone());
                allvars = List::unique(listAppend(tempvars1.clone(), tempvars2.clone()));
                (_, tmp2, _) = List::intersection1OnTrue(allvars.clone(), solvedvars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                if !(tmp2.clone().is_empty()) {
                    (tempsolvedvars, tempsolvedeqs) = BuildSquareSubSetHelper(tmp2.clone(), tmpknowns.clone(), tmpExt.clone(), tmpsolveeqvar.clone(), tempsolvedvars.clone(), tempsolvedeqs.clone(), tmpconstantvars.clone())?;
                }
            }
            (tempsolvedvars.clone(), tempsolvedeqs.clone())
        },
        (_, _, _, _, _, _, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((outlist1, outlist2))
}

pub fn BuildSquareSubSetHelper1(mut inlist1: Arc<metamodelica::List<i32>>, mut solvedeqvar: Arc<metamodelica::List<(i32, i32)>>, mut solvedeqs: Arc<metamodelica::List<i32>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut tempsolvedeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnumber: i32 = 0;
    for mut varnumber in &*inlist1.clone() {
        let mut varnumber = varnumber.clone();
        eqnumber = getSolvedEquationNumber(varnumber.clone(), solvedeqvar.clone());
        if !(listMember(eqnumber.clone(), solvedeqs.clone())) {
            tempeqs = cons(eqnumber.clone(), tempeqs.clone());
            tempsolvedeqs = cons(eqnumber.clone(), tempsolvedeqs.clone());
        }
    }
    tempsolvedeqs = listAppend(solvedeqs.clone(), tempsolvedeqs.clone());
    (tempsolvedeqs, tempeqs)
}

pub fn BuildSquareSubSet(mut ineqs: Arc<metamodelica::List<i32>>, mut invars: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut mExt: ExtAdjacencyMatrix, mut solvedeqvar: Arc<metamodelica::List<(i32, i32)>>, mut constantvars: Arc<metamodelica::List<i32>>, mut approximatedEquations: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>)> {
    let mut solvedvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut solvedeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dependency_variables_tree: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut dependency_equation_tree: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut tempvars1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempvars2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut t1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut t2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut t3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpeqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnumber: i32 = 0;
    let mut varnumber: i32 = 0;
    let mut count: i32 = 1;
    for mut i in &*ineqs.clone() {
        let mut i = i.clone();
        (tempvars1, tempvars2) = getVariableOccurence(list![i.clone()], mExt.clone(), knowns.clone());
        varnumber = (invars.clone()).get(count.clone())?;
        allvars = List::unique(listAppend(tempvars1.clone(), tempvars2.clone()));
        (t1, t2, t3) = List::intersection1OnTrue(allvars.clone(), list![varnumber.clone()], (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        (tmpvars, tmpeqs) = BuildSquareSubSetHelper(allvars.clone(), knowns.clone(), mExt.clone(), solvedeqvar.clone(), list![varnumber.clone()], list![i.clone()], constantvars.clone())?;
        solvedvars = listAppend(solvedvars.clone(), tmpvars.clone());
        solvedeqs = listAppend(solvedeqs.clone(), tmpeqs.clone());
        dependency_variables_tree = cons((varnumber.clone(), List::unique(tmpvars.clone())), dependency_variables_tree.clone());
        tmpeqs = List::setDifferenceOnTrue(tmpeqs.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        dependency_equation_tree = cons((i.clone(), List::unique(tmpeqs.clone())), dependency_equation_tree.clone());
        count = count.clone() + 1;
    }
    solvedvars = List::unique(solvedvars.clone());
    solvedeqs = List::unique(solvedeqs.clone());
    Ok((solvedvars, solvedeqs, dependency_variables_tree, dependency_equation_tree))
}

pub fn dumpListList(mut lstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut heading: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(lstLst.clone(), (std::sync::Arc::new(fnptr!(dumplistInteger, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

pub fn dumplistInteger(mut inlist: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut outstring: ArcStr = arcstr::literal!("");
    let mut s: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    s = List::map(inlist.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
    outstring = stringDelimitList(s.clone(), (literal!(", ")).clone());
    outstring = stringAppendList(list![(literal!("{")).clone(), (outstring.clone()).clone(), (literal!("}")).clone()]);
    outstring
}

pub fn getVariableOccurence(mut setc: Arc<metamodelica::List<i32>>, mut mext: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut knownvariables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unknownvariables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    for mut i in &*setc.clone() {
        let mut i = i.clone();
        for mut j in &*mext.clone() {
            let mut j = j.clone();
            (eq, vars) = j.clone();
            if intEq(i.clone(), eq.clone()) {
                for mut var in &*vars.clone() {
                    let mut var = var.clone();
                    if listMember(var.clone(), knowns.clone()) {
                        knownvariables = cons(var.clone(), knownvariables.clone());
                    } else {
                        unknownvariables = cons(var.clone(), unknownvariables.clone());
                    }
                }
            }
        }
    }
    knownvariables = List::unique(knownvariables.clone());
    unknownvariables = List::unique(unknownvariables.clone());
    (knownvariables, unknownvariables)
}

pub fn setInitialBlocks(mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> mapBlocks {
    let mut outlist: mapBlocks = metamodelica::nil();
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        outlist = cons((i.clone(), false, true), outlist.clone());
    }
    outlist = outlist.clone().reverse();
    outlist
}

pub fn updateBlocks(mut blocktoupdate: Arc<metamodelica::List<i32>>, mut inlist: mapBlocks, mut visited: bool, mut square: bool) -> mapBlocks {
    let mut outlist: mapBlocks = metamodelica::nil();
    let mut i1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        (i1, b1, b2) = i.clone();
        b3 = List::setEqualOnTrue(i1.clone(), blocktoupdate.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        if b3.clone() == true {
            b1 = visited.clone();
            b2 = square.clone();
        }
        outlist = cons((i1.clone(), b1.clone(), b2.clone()), outlist.clone());
    }
    outlist = outlist.clone().reverse();
    outlist
}

pub fn sortBlocks(mut sortedranklist: Arc<metamodelica::List<i32>>, mut inlist2: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>) -> Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> {
    let mut outlist: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut e1: i32 = 0;
    let mut e2: i32 = 0;
    let mut blocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*sortedranklist.clone() {
        let mut i = i.clone();
        for mut j in &*inlist2.clone() {
            let mut j = j.clone();
            (blocks, e1) = j.clone();
            if i.clone() == e1.clone() {
                outlist = cons((blocks.clone(), e1.clone()), outlist.clone());
            }
        }
    }
    outlist = outlist.clone().reverse();
    outlist
}

pub fn findBlocksRanks(mut inlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut outlist: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut ranklist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rank: i32 = 0;
    for mut i in &*inlist2.clone() {
        let mut i = i.clone();
        for mut j in &*inlist1.clone() {
            let mut j = j.clone();
            (blocks, rank) = j.clone();
            if i.clone() == blocks.clone() {
                outlist = cons((i.clone(), rank.clone()), outlist.clone());
                ranklist = cons(rank.clone(), ranklist.clone());
            }
        }
    }
    outlist = outlist.clone().reverse();
    ranklist = List::sort(ranklist.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok((outlist, ranklist))
}

pub fn findBlockTargets(mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix, mut map: mapBlocks, mut blockranks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>>> {
    let mut outlist: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>> = metamodelica::nil();
    let mut targetblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut targetvarlist: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut blockvarlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ranklist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blocks1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rank: i32 = 0;
    let mut updatedblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        targetblocks = findBlockTargetsHelper(list![i.clone()], inlist2.clone(), solvedvariables.clone(), mxt.clone(), map.clone(), inlist1.clone())?;
        targetblocks = listAppend(list![i.clone()], targetblocks.clone());
        (updatedblocks, ranklist) = findBlocksRanks(blockranks.clone(), targetblocks.clone())?;
        updatedblocks = sortBlocks(ranklist.clone(), updatedblocks.clone());
        targetvarlist = metamodelica::nil();
        for mut blocks in &*updatedblocks.clone() {
            let mut blocks = blocks.clone();
            (blocks1, rank) = blocks.clone();
            blockvarlst = getBlockVarList(blocks1.clone(), inlist1.clone(), inlist2.clone());
            targetvarlist = cons((blockvarlst.clone(), rank.clone()), targetvarlist.clone());
        }
        outlist = cons((i.clone(), updatedblocks.clone(), targetvarlist.clone().reverse()), outlist.clone());
    }
    outlist = outlist.clone().reverse();
    Ok(outlist)
}

pub fn findBlockTargetsHelper(mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix, mut map: mapBlocks, mut actualblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut outlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    outlist = (::match_deref::match_deref! { match &((inlist1.clone(), inlist2.clone(), solvedvariables.clone(), mxt.clone(), map.clone(), actualblocks.clone())) {
        (Deref @ metamodelica::List::Cons { head: first, tail: rest }, Deref @ metamodelica::List::Cons { head: firstitem, tail: restitem }, solvar, mxt1, map1, originalblocks) => {
            let mut dependencyequation: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut targetblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut targetblocks1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            dependencyequation = findBlockTargetsHelper1(cons(first.clone(), rest.clone()), solvar.clone(), mxt1.clone())?;
            targetblocks = getActualBlocks(dependencyequation.clone(), originalblocks.clone(), first.clone());
            targetblocks1 = findBlockTargetsHelper(targetblocks.clone(), cons(firstitem.clone(), restitem.clone()), solvar.clone(), mxt1.clone(), map1.clone(), originalblocks.clone())?;
            List::unique(listAppend(targetblocks.clone(), targetblocks1.clone()))
        },
        (_, _, _, _, _, _) => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outlist)
}

pub fn findBlockTargetsHelper1(mut inlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dependencyequations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        dependencyequations = getDependencyequation(i.clone(), metamodelica::nil(), solvedvariables.clone(), mxt.clone())?;
        for mut v in &*dependencyequations.clone().reverse() {
            let mut v = v.clone();
            outlist = cons(v.clone(), outlist.clone());
        }
    }
    Ok(outlist)
}

pub fn findPredecessorBlocks(mut blockinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>> {
    let mut outblockinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut dependencyequation: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockstoupdate: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut tmptargetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocksvar: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut blockitem: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitems1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitems2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut foundblockranks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockvarlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockvarlst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockvarlst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut foundblock: i32 = 0;
    let mut count: i32 = 1;
    let mut foundblockrank: i32 = 0;
    let mut tmpcount: i32 = 0;
    let mut visited: bool = false;
    let mut square: bool = false;
    let mut status: bool = false;
    let mut checkknowns: bool = false;
    let mut finalsquarestauts: bool = false;
    let mut exist: bool = false;
    let mut exist1: bool = false;
    let mut targetexist: bool = false;
    let mut outlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, bool, i32)>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Targets of blocks without predecessors\n")); __mm_s.push_str(&*literal!("=====================================\n")); ArcStr::from(__mm_s) }).clone());
    for mut blocks in &*blockinfo.clone() {
        let mut blocks = blocks.clone();
        (blockitems1, targetblocks, targetblocksvar) = blocks.clone();
        tmpcount = 1;
        targetexist = false;
        for mut tmpblocks in &*blockinfo.clone() {
            let mut tmpblocks = tmpblocks.clone();
            (_, tmptargetblocks, _) = tmpblocks.clone();
            if !(intEq(count.clone(), tmpcount.clone())) {
                if listMember(listHead(targetblocks.clone())?, tmptargetblocks.clone()) {
                    targetexist = true;
                }
            }
            tmpcount = tmpcount.clone() + 1;
        }
        if targetexist.clone() == false {
            (exist, dependencyequation, foundblockranks) = findSquareAndNonSquareBlocksHelper1(targetblocks.clone(), targetblocksvar.clone());
            if exist.clone() {
                (targetblocks, targetblocksvar) = EliminatePredecessorBlockTarget(targetblocks.clone(), targetblocksvar.clone())?;
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("target(")); __mm_s.push_str(&*dumplistInteger(blockitems1.clone())); __mm_s.push_str(&*literal!(") : ")); __mm_s.push_str(&*anyString(targetblocks.clone())); __mm_s.push_str(&*literal!(" => Blue_Block_Ranks in target ")); __mm_s.push_str(&*dumplistInteger(foundblockranks.clone())); __mm_s.push_str(&*literal!(" => Blue Block Equations : ")); __mm_s.push_str(&*dumplistInteger(dependencyequation.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                outblockinfo = cons((blockitems1.clone(), targetblocks.clone(), targetblocksvar.clone(), dependencyequation.clone(), foundblockranks.clone()), outblockinfo.clone());
            }
        }
        count = count.clone() + 1;
    }
    outblockinfo = outblockinfo.clone().reverse();
    Ok(outblockinfo)
}

// This function eliminates the Blocks which are not needed for extraction algorithm
pub fn EliminatePredecessorBlockTarget(mut inlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inlist2: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>) -> Result<(Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)> {
    let mut targetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocksvar: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut checkknowns: bool = false;
    let mut blocksvarlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut rank: i32 = 0;
    for mut i in &*inlist2.clone().reverse() {
        let mut i = i.clone();
        (blocksvarlist, rank) = i.clone();
        checkknowns = listMember((literal!("knowns")).clone(), blocksvarlist.clone());
        if listMember((literal!("knowns")).clone(), blocksvarlist.clone()) {
            targetblocks = List::firstN(inlist1.clone(), (inlist1.clone().len() as i32) - count.clone() + 1)?;
            targetblocksvar = List::firstN(inlist2.clone(), (inlist2.clone().len() as i32) - count.clone() + 1)?;
            break;
        }
        count = count.clone() + 1;
    }
    Ok((targetblocks, targetblocksvar))
}

/*
public function findPredecessorBlocksHelper
  input list<tuple<list<Integer>,list<tuple<list<Integer>,Integer>>,list<tuple<list<String>,Integer>>>> blockinfo;
protected
  list<Integer> dependencyequation;
  list<tuple<list<Integer>,Integer>> blockstoupdate,targetblocks;
  list<tuple<list<String>,Integer>> targetblocksvar;
  list<Integer> blockitem,blockitems1,blockitems2;
  list<String> blockvarlst,blockvarlst1,blockvarlst2;
  Integer foundblock,count=1,foundblockrank,tmpcount=1;
  //mapBlocks map1=map;
  Boolean visited,square,status,checkknowns,finalsquarestauts,exist,exist1;
  list<tuple<list<Integer>,list<String>,Boolean,Integer>> outlist1={};
algorithm
  print("\n PredeccesorBlocks\n");
  for blocks in blockinfo loop
    (blockitems1,targetblocks,targetblocksvar):= blocks;
    print(intString(count) + ": " + anyString(targetblocks) + "\n ");
    for tmpblocks in blockinfo loop
      (blockitems1,targetblocks,targetblocksvar):= blocks;
    end for;
    count:=count+1;
  end for;
end findPredecessorBlocksHelper;
*/
pub fn findSquareAndNonSquareBlocks(mut blockinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix, mut map: mapBlocks) -> Result<(Arc<metamodelica::List<bool>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, bool, i32, bool)>>)> {
    let mut outlist: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut outlist2: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, bool, i32, bool)>> = metamodelica::nil();
    let mut dependencyequation: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockstoupdate: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocksvar: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut blockitem: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitems1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitems2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockvarlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockvarlst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockvarlst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut foundblock: i32 = 0;
    let mut count: i32 = 1;
    let mut foundblockrank: i32 = 0;
    let mut map1: mapBlocks = map.clone();
    let mut visited: bool = false;
    let mut square: bool = false;
    let mut status: bool = false;
    let mut checkknowns: bool = false;
    let mut finalsquarestauts: bool = false;
    let mut exist: bool = false;
    let mut exist1: bool = false;
    let mut outlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, bool, i32)>> = metamodelica::nil();
    for mut blocks in &*blockinfo.clone() {
        let mut blocks = blocks.clone();
        (blockitems1, targetblocks, targetblocksvar) = blocks.clone();
        (blockstoupdate, exist, foundblock) = findSquareAndNonSquareBlocksHelper(targetblocks.clone(), targetblocksvar.clone())?;
        (blockvarlst1, _) = listHead(targetblocksvar.clone())?;
        outlist1 = cons((blockitems1.clone(), blockvarlst1.clone(), exist.clone(), foundblock.clone()), outlist1.clone());
        for mut j in &*blockstoupdate.clone() {
            let mut j = j.clone();
            (blockitem, _) = j.clone();
            visited = false;
            map1 = updateBlocks(blockitem.clone(), map1.clone(), visited.clone(), false);
        }
    }
    for mut k in &*map1.clone() {
        let mut k = k.clone();
        (_, _, finalsquarestauts) = k.clone();
        (blockitems1, blockvarlst2, exist1, foundblockrank) = (outlist1.clone().reverse()).get(count.clone())?;
        outlist = cons(finalsquarestauts.clone(), outlist.clone());
        outlist2 = cons((blockitems1.clone(), blockvarlst2.clone(), exist1.clone(), foundblockrank.clone(), finalsquarestauts.clone()), outlist2.clone());
        count = count.clone() + 1;
    }
    outlist = outlist.clone().reverse();
    outlist2 = outlist2.clone().reverse();
    Ok((outlist, outlist2))
}

pub fn findSquareAndNonSquareBlocksHelper(mut inlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inlist2: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>) -> Result<(Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, bool, i32)> {
    let mut targetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut exists: bool = false;
    let mut foundblock: i32 = -1;
    let mut checkknowns: bool = false;
    let mut blocksvarlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut rank: i32 = 0;
    let mut targetblockstest: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    for mut i in &*inlist2.clone() {
        let mut i = i.clone();
        (blocksvarlist, rank) = i.clone();
        checkknowns = listMember((literal!("knowns")).clone(), blocksvarlist.clone());
        if checkknowns.clone() == true {
            targetblocks = List::lastN(inlist1.clone(), (inlist1.clone().len() as i32) - count.clone())?;
            foundblock = rank.clone();
            exists = true;
            break;
        }
        count = count.clone() + 1;
    }
    Ok((targetblocks, exists, foundblock))
}

pub fn findSquareAndNonSquareBlocksHelper1(mut inlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inlist2: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>) -> (bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut exists: bool = false;
    let mut foundknownblocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockranks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut checkknowns: bool = false;
    let mut blocksvarlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut rank: i32 = 0;
    let mut tmpcount: i32 = 0;
    let mut targetblocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*inlist2.clone() {
        let mut i = i.clone();
        (blocksvarlist, rank) = i.clone();
        (targetblocks, _) = (inlist1.clone()).get(count.clone()).unwrap();
        checkknowns = listMember((literal!("knowns")).clone(), blocksvarlist.clone());
        if checkknowns.clone() == true {
            exists = true;
            blockranks = cons(rank.clone(), blockranks.clone());
            tmpcount = 1;
            for mut j in &*blocksvarlist.clone() {
                let mut j = j.clone();
                if (j.clone()).clone() == literal!("knowns") {
                    foundknownblocks = cons((targetblocks.clone()).get(tmpcount.clone()).unwrap(), foundknownblocks.clone());
                }
                tmpcount = tmpcount.clone() + 1;
            }
        }
        count = count.clone() + 1;
    }
    foundknownblocks = foundknownblocks.clone().reverse();
    blockranks = blockranks.clone().reverse();
    (exists, foundknownblocks, blockranks)
}

pub fn getBlockVarList(mut blocktofind: Arc<metamodelica::List<i32>>, mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outstringlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut b3: bool = false;
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        b3 = List::setEqualOnTrue(i.clone(), blocktofind.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        if b3.clone() == true {
            outstringlist = (inlist2.clone()).get(count.clone()).unwrap();
        }
        count = count.clone() + 1;
    }
    outstringlist
}

pub fn getActualBlocks(mut searchblock: Arc<metamodelica::List<i32>>, mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut outlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        if !(List::intersectionOnTrue(searchblock.clone(), i.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)).is_empty()) {
            outlist = cons(i.clone(), outlist.clone());
        }
    }
    outlist = outlist.clone().reverse();
    outlist
}

pub fn ExtractEquationsfromPredecessorBlocks(mut predecessortargetinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>, mut allblockranks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut approximatedEquations: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut setc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dependendblock: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut foundblockranks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut knownblocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut usedblocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetblocktobeinserted: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockspostoberemoved: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockstoupdate: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut tmptargetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocksvar: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut tmptargetblocksvar: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut blockitems: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitems1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitems2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpsetc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpsets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockvarlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockvarlst1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockvarlst2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut foundblock: i32 = 0;
    let mut count: i32 = 0;
    let mut tmpcount: i32 = 0;
    let mut blocksize: i32 = 0;
    let mut visited: bool = false;
    let mut square: bool = false;
    let mut status: bool = false;
    let mut checkknowns: bool = false;
    let mut finalsquarestauts: bool = false;
    let mut exist: bool = false;
    let mut exist1: bool = false;
    let mut targetexist: bool = false;
    let mut outlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, bool, i32)>> = metamodelica::nil();
    let mut tmppredecessortargetinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    tmpcount = 1;
    usedblocks = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nLoop-1\n")); __mm_s.push_str(&*literal!("========\n")); ArcStr::from(__mm_s) }).clone());
    for mut blocks in &*predecessortargetinfo.clone() {
        let mut blocks = blocks.clone();
        (blockitems, targetblocks, targetblocksvar, knownblocks, foundblockranks) = blocks.clone();
        (dependendblock, _) = (allblockranks.clone()).get(listHead(foundblockranks.clone())?)?;
        targetblocktobeinserted = List::setDifferenceOnTrue(knownblocks.clone(), usedblocks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtractEquationsfromNoPredecessorBlocks :")); __mm_s.push_str(&*dumplistInteger(blockitems.clone())); __mm_s.push_str(&*literal!(" => ")); __mm_s.push_str(&*dumplistInteger(dependendblock.clone())); __mm_s.push_str(&*literal!(" => known blocks:")); __mm_s.push_str(&*dumplistInteger(targetblocktobeinserted.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        if !(targetblocktobeinserted.clone().is_empty()) {
            usedblocks = cons(listHead(targetblocktobeinserted.clone())?, usedblocks.clone());
        } else {
            blockspostoberemoved = cons(tmpcount.clone(), blockspostoberemoved.clone());
        }
        if !(List::setEqualOnTrue(blockitems.clone(), dependendblock.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) && !(targetblocktobeinserted.clone().is_empty()) {
            blocksize = (blockitems.clone().len() as i32) - 1;
            sets = listAppend(List::firstN(blockitems.clone(), blocksize.clone())?, sets.clone());
            sets = cons(listHead(targetblocktobeinserted.clone())?, sets.clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nAfterinsertion :")); __mm_s.push_str(&*dumplistInteger(targetblocktobeinserted.clone())); __mm_s.push_str(&*literal!("=> SET_S :")); __mm_s.push_str(&*dumplistInteger(sets.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        } else if !(List::setEqualOnTrue(blockitems.clone(), dependendblock.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) && targetblocktobeinserted.clone().is_empty() {
            println!("{}", (literal!("\nProblem is ill posed because there are two few variables of interest. Boundary condition A is ignored \n")).clone());
            sets = listAppend(blockitems.clone(), sets.clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nAfterinsertion :")); __mm_s.push_str(&*dumplistInteger(targetblocktobeinserted.clone())); __mm_s.push_str(&*literal!("=> SET_S :")); __mm_s.push_str(&*dumplistInteger(sets.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        } else if List::setEqualOnTrue(blockitems.clone(), dependendblock.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)) && (blockitems.clone().len() as i32) == 1 {
            sets = metamodelica::nil();
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Problem is ill posed because a variable of interest is set on boundary condition B")).clone()])?;
        }
        tmpcount = tmpcount.clone() + 1;
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFinal Extraction After Loop-1:\n===================================\n")); __mm_s.push_str(&*literal!("SET_C : ")); __mm_s.push_str(&*dumplistInteger(setc.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S : ")); __mm_s.push_str(&*dumplistInteger(sets.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("\nLoop-2\n===========\n")).clone());
    for mut i in &*predecessortargetinfo.clone() {
        let mut i = i.clone();
        (_, targetblocks, targetblocksvar, _, _) = i.clone();
        tmptargetblocks = List::restOrEmpty(targetblocks.clone())?;
        tmptargetblocksvar = List::restOrEmpty(targetblocksvar.clone())?;
        count = 1;
        for mut j in &*tmptargetblocks.clone() {
            let mut j = j.clone();
            (blockitems1, _) = j.clone();
            (blockvarlst1, _) = (tmptargetblocksvar.clone()).get(count.clone())?;
            (tmpsetc, tmpsets) = extractMixedBlock(blockitems1.clone(), blockvarlst1.clone());
            setc = listAppend(setc.clone(), tmpsetc.clone());
            sets = listAppend(sets.clone(), tmpsets.clone());
            count = count.clone() + 1;
        }
    }
    sets = List::unique(sets.clone());
    setc = List::unique(setc.clone());
    setc = List::setDifferenceOnTrue(setc.clone(), sets.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setc = List::setDifferenceOnTrue(setc.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    sets = List::setDifferenceOnTrue(sets.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFinal Extraction After Loop 2:\n=============================\n")); __mm_s.push_str(&*literal!("SET_C : ")); __mm_s.push_str(&*dumplistInteger(setc.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(sets.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if setc.clone().is_empty() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(" Set-C is Empty! : Problem is ill posed because there are two few variables of interest")).clone()])?;
        bail!("fail");
    }
    Ok((setc, sets))
}

pub fn ExtractEquationsfromBlocks(mut blockdata: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>, bool, i32, bool)>>, mut approximatedEquation: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut setc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut removedeq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitem: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockitem1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setc1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut temp1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut temp2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rmeqlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut usedblocklist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut blockvarlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blockexist: bool = false;
    let mut squarestatus: bool = false;
    let mut used: bool = false;
    let mut checkusedblock: bool = false;
    let mut targetBlockSquareStatus: bool = false;
    let mut blockrank: i32 = 0;
    let mut knownvarcount: i32 = 0;
    let mut blocksize: i32 = 0;
    for mut i in &*blockdata.clone() {
        let mut i = i.clone();
        (blockitem, blockvarlist, blockexist, blockrank, squarestatus) = i.clone();
        if blockexist.clone() == true && squarestatus.clone() == true {
            (blockitem1, _, _, _, targetBlockSquareStatus) = (blockdata.clone()).get(blockrank.clone())?;
            checkusedblock = listMember(blockitem1.clone(), usedblocklist.clone());
            if !(List::setEqualOnTrue(blockitem.clone(), blockitem1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) {
                if targetBlockSquareStatus.clone() == true && checkusedblock.clone() == false {
                    temp1 = List::lastN(blockitem.clone(), (blockitem.clone().len() as i32) - 1)?;
                    if temp1.clone().is_empty() {
                        removedeq = listAppend(blockitem.clone(), removedeq.clone());
                    }
                    sets = listAppend(temp1.clone(), sets.clone());
                    sets = listAppend(List::firstOrEmpty(blockitem1.clone()), sets.clone());
                    usedblocklist = cons(blockitem1.clone(), usedblocklist.clone());
                } else if targetBlockSquareStatus.clone() == false || checkusedblock.clone() == true {
                    sets = listAppend(blockitem.clone(), sets.clone());
                }
            } else {
                (setc1, sets1) = extractMixedBlock(blockitem.clone(), blockvarlist.clone());
                (tmplist1, tmplist2, tmplist3) = List::intersection1OnTrue(setc1.clone(), approximatedEquation.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                setc1 = listAppend(tmplist1.clone(), tmplist2.clone());
                setc = listAppend(List::restOrEmpty(setc1.clone())?, setc.clone());
                sets = listAppend(sets.clone(), sets1.clone());
                removedeq = listAppend(List::firstOrEmpty(setc1.clone()), removedeq.clone());
            }
        } else if blockexist.clone() == true && squarestatus.clone() == false {
            (setc1, sets1) = extractMixedBlock(blockitem.clone(), blockvarlist.clone());
            sets = listAppend(sets.clone(), sets1.clone());
            setc = listAppend(setc.clone(), setc1.clone());
        } else {
            removedeq = listAppend(blockitem.clone(), removedeq.clone());
        }
    }
    setc = List::unique(setc.clone());
    sets = List::unique(sets.clone());
    removedeq = List::unique(removedeq.clone());
    Ok((setc, sets, removedeq))
}

pub fn getRemovedEquationSolvedVariables(mut inlist: Arc<metamodelica::List<i32>>, mut solvedvar: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<i32>> {
    let mut outvarlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varnumber: i32 = 0;
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        varnumber = getSolvedVariableNumber(i.clone(), solvedvar.clone());
        outvarlist = cons(varnumber.clone(), outvarlist.clone());
    }
    outvarlist
}

pub fn countKnownVariables(mut inlist1: Arc<metamodelica::List<ArcStr>>) -> i32 {
    let mut count: i32 = 0;
    let mut value: bool = false;
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        if (i.clone()).clone() == literal!("knowns") {
            count = count.clone() + 1;
        }
    }
    count
}

pub fn checkBlockStatus(mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> {
    let mut instringlist: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut count: i32 = 0;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut setinputs: bool = true;
    let mut setinputs1: bool = true;
    for mut i in &*inlist2.clone() {
        let mut i = i.clone();
        b1 = listMember((literal!("knowns")).clone(), i.clone());
        b2 = listMember((literal!("unknowns")).clone(), i.clone());
        b3 = listMember((literal!("inputs")).clone(), i.clone());
        if setinputs.clone() == true && b2.clone() == true && b1.clone() == false {
            i = List::fill((literal!("inputs")).clone(), (i.clone().len() as i32));
        }
        if b1.clone() == true && b2.clone() == false {
            setinputs = false;
        }
        if b1.clone() == true && b2.clone() == true {
            setinputs = false;
        }
        instringlist = cons(i.clone(), instringlist.clone());
        count = count.clone() + 1;
    }
    instringlist = instringlist.clone().reverse();
    instringlist
}

pub fn originalBlocks(mut inlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>, mut outputs: Arc<metamodelica::List<i32>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) {
    let mut outlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outstringlist: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut blocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockinfo: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        (blocks, blockinfo) = checkBlueOrRedSquareBlocks(i.clone(), knowns.clone(), unknowns.clone(), outputs.clone(), solvedvariables.clone());
        outlist = cons(blocks.clone(), outlist.clone());
        outstringlist = cons(blockinfo.clone(), outstringlist.clone());
    }
    outlist = outlist.clone().reverse();
    outstringlist = outstringlist.clone().reverse();
    (outlist, outstringlist)
}

pub fn extractMixedBlock(mut inlist: Arc<metamodelica::List<i32>>, mut instringList: Arc<metamodelica::List<ArcStr>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut setc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut s: ArcStr = arcstr::literal!("");
    for mut e in &*inlist.clone() {
        let mut e = e.clone();
        s = ((instringList.clone()).get(count.clone()).unwrap()).clone();
        if (s.clone()).clone() == literal!("knowns") {
            setc = cons(e.clone(), setc.clone());
        } else {
            sets = cons(e.clone(), sets.clone());
        }
        count = count.clone() + 1;
    }
    (setc, sets)
}

pub fn getDependencyequation(mut inlist: Arc<metamodelica::List<i32>>, mut inlist1: Arc<metamodelica::List<i32>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut m: ExtAdjacencyMatrix) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outinteger: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut t: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nonsq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varnumber: i32 = 0;
    for mut eqnumber in &*inlist.clone() {
        let mut eqnumber = eqnumber.clone();
        varnumber = getSolvedVariableNumber(eqnumber.clone(), solvedvariables.clone());
        nonsq = getdirectOccurrencesinEquation(m.clone(), eqnumber.clone(), varnumber.clone())?;
        for mut lst in &*nonsq.clone() {
            let mut lst = lst.clone();
            if !(listMember(lst.clone(), inlist.clone())) {
                t = cons(lst.clone(), t.clone());
            }
        }
    }
    outinteger = listAppend(t.clone(), inlist1.clone());
    Ok(outinteger)
}

pub fn getdirectOccurrencesinEquation(mut m: ExtAdjacencyMatrix, mut eqnumber: i32, mut varnumber: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut out: Arc<metamodelica::List<i32>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((m.clone(), eqnumber.clone(), varnumber.clone())) {
        (Deref @ metamodelica::List::Cons { head: (eq, vars), tail: tail }, eqnum, varnum) => {
            let mut ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut matchedeq: Arc<metamodelica::List<i32>> = metamodelica::nil();
            if !(intEq(eq.clone(), eqnum.clone())) {
                if listMember(varnum.clone(), vars.clone()) {
                    matchedeq = list![eq.clone()];
                } else {
                    matchedeq = metamodelica::nil();
                }
            } else {
                matchedeq = metamodelica::nil();
            }
            ret = getdirectOccurrencesinEquation(tail.clone(), eqnum.clone(), varnum.clone())?;
            listAppend(matchedeq.clone(), ret.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

pub fn checkBlueOrRedSquareBlocks(mut inlist: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>, mut outputs: Arc<metamodelica::List<i32>>, mut solvedvar: Arc<metamodelica::List<(i32, i32)>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>) {
    let mut outlist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outstring: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut varnumber: i32 = 0;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut s1: ArcStr = arcstr::literal!("");
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        varnumber = getSolvedVariableNumber(i.clone(), solvedvar.clone());
        b1 = listMember(varnumber.clone(), knowns.clone());
        b2 = listMember(varnumber.clone(), unknowns.clone());
        b3 = listMember(varnumber.clone(), outputs.clone());
        if b1.clone() == false && b2.clone() == true {
            s1 = (literal!("unknowns")).clone();
            outstring = cons((s1.clone()).clone(), outstring.clone());
            outlist = cons(i.clone(), outlist.clone());
        }
        if b1.clone() == true && b2.clone() == false {
            s1 = (literal!("knowns")).clone();
            outstring = cons((s1.clone()).clone(), outstring.clone());
            outlist = cons(i.clone(), outlist.clone());
        }
        if b1.clone() == false && b2.clone() == false {
            s1 = (literal!("unknowns")).clone();
            outstring = cons((s1.clone()).clone(), outstring.clone());
            outlist = cons(i.clone(), outlist.clone());
        }
        count = count.clone() + 1;
    }
    outlist = outlist.clone().reverse();
    outstring = outstring.clone().reverse();
    (outlist, outstring)
}

/* function which gives solvedvars based on the equation */
pub fn getSolvedVariableNumber(mut eqnumber: i32, mut inlist: Arc<metamodelica::List<(i32, i32)>>) -> i32 {
    let mut solvedvar: i32 = 0;
    let mut solvedeq: i32 = 0;
    for mut var in &*inlist.clone() {
        let mut var = var.clone();
        (solvedeq, solvedvar) = var.clone();
        if intEq(eqnumber.clone(), solvedeq.clone()) {
            return solvedvar;
        }
    }
    solvedvar
}

/* function which gives solvedeqs based on the variables */
pub fn getSolvedEquationNumber(mut varnumber: i32, mut inlist: Arc<metamodelica::List<(i32, i32)>>) -> i32 {
    let mut solvedeq: i32 = 0;
    let mut solvedvar: i32 = 0;
    for mut var in &*inlist.clone() {
        let mut var = var.clone();
        (solvedeq, solvedvar) = var.clone();
        if intEq(varnumber.clone(), solvedvar.clone()) {
            return solvedeq;
        }
    }
    solvedeq
}

pub fn dumpMatching(mut v: metamodelica::Array<i32>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut eqvarlist: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = v.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqvarlist = cons((i.clone(), count.clone()), eqvarlist.clone());
        count = count.clone() + 1;
    }
    eqvarlist
}

fn printSep(mut s: ArcStr) -> Result<()> {
    Print::printBuf((literal!("{ ")).clone())?;
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("} , ")).clone())?;
    Ok(())
}

fn wrapInList(mut text: ArcStr) -> ArcStr {
    let mut oText: ArcStr = arcstr::literal!("");
    oText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*text.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    oText
}

fn verticalGrid(mut elems: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[{")); __mm_s.push_str(&*stringDelimitList(List::map(elems.clone(), (std::sync::Arc::new(fnptr!(wrapInList, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}]")); ArcStr::from(__mm_s) }).clone();
    out
}

fn verticalGridBoxed(mut elems: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[{")); __mm_s.push_str(&*stringDelimitList(List::map(elems.clone(), (std::sync::Arc::new(fnptr!(wrapInList, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("},Frame -> All]")); ArcStr::from(__mm_s) }).clone();
    out
}

fn numerateList(mut elems: Arc<metamodelica::List<ArcStr>>, mut index: i32) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &((elems.clone(), index.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut ss: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
            ss = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*numerateList(t.clone(), index.clone() + 1)?); ArcStr::from(__mm_s) }).clone();
            ss.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(out)
}

fn numerateListIndex(mut elems: Arc<metamodelica::List<ArcStr>>, mut indices: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &((elems.clone(), indices.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Nil }) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: t }, Deref @ metamodelica::List::Cons { head: n, tail: tn }) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut ss: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
            ss = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*numerateListIndex(t.clone(), tn.clone())?); ArcStr::from(__mm_s) }).clone();
            ss.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(out)
}

fn equationsToMathematicaGrid(mut equIndices: Arc<metamodelica::List<i32>>, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut variables: BackendDAE::Variables, mut knownVariables: BackendDAE::Variables, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    let mut eqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqsString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqns = List::unique(List::map1r(equIndices.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
    eqList = List::map1r(eqns.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
    eqsString = List::map1(eqList.clone(), (std::sync::Arc::new(MathematicaDump::printMmaEqnStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<ArcStr> + 'static>), (variables.clone(), knownVariables.clone()));
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[{")); __mm_s.push_str(&*numerateListIndex(eqsString.clone(), eqns.clone())?); __mm_s.push_str(&*literal!("}, Frame -> All]")); ArcStr::from(__mm_s) }).clone();
    Ok(out)
}

fn unknowsMatchingToMathematicaGrid2(mut vars: Arc<metamodelica::List<ArcStr>>, mut eqns: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut out: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    out = 'mc: {
        let __mc_input = (vars.clone(), eqns.clone());
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
                (Deref @ metamodelica::List::Nil, _) => {
                    println!("{}", (literal!("Warning: The system is ill-posed. When computing the unknowns, there are more equations than variables.\n")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    println!("{}", (literal!("Warning: The system is ill-posed. When computing the unknowns, there are more variables than equations.\n")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var, tail: var_t }, Deref @ metamodelica::List::Cons { head: eqn, tail: eqn_t }) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*eqn.clone()); ArcStr::from(__mm_s) }).clone();
                    r = unknowsMatchingToMathematicaGrid2(var_t.clone(), eqn_t.clone())?;
                    Ok(cons((s.clone()).clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

fn getEquationStringOrNothing(mut equations: Arc<metamodelica::List<i32>>, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut variables: BackendDAE::Variables, mut knownVariables: BackendDAE::Variables, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut out: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    out = 'mc: {
        let __mc_input = (equations.clone(), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone());
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
                (Deref @ metamodelica::List::Cons { head: eqn, tail: eqn_t }, _, _, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (intEq(eqn.clone(), 0)) else { bail!("pattern mismatch") };
                    r = getEquationStringOrNothing(eqn_t.clone(), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?;
                    s = (literal!("\"-\"")).clone();
                    Ok(cons((s.clone()).clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eqn, tail: eqn_t }, _, _, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    e = BackendEquation::get(allEqs.clone(), eqn.clone());
                    r = getEquationStringOrNothing(eqn_t.clone(), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?;
                    s = (MathematicaDump::printMmaEqnStr(e.clone(), (variables.clone(), knownVariables.clone()))?).clone();
                    Ok(cons((s.clone()).clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

fn unknowsMatchingToMathematicaGrid(mut vars: Arc<metamodelica::List<i32>>, mut equations: Arc<metamodelica::List<i32>>, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut variables: BackendDAE::Variables, mut knownVariables: BackendDAE::Variables, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    let mut eqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqsString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut varString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqns = List::map1r(equations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
    eqsString = getEquationStringOrNothing(eqns.clone(), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?;
    varList = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), variables.clone());
    varString = List::map2(varList.clone(), (std::sync::Arc::new(MathematicaDump::printMmaVarStr) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool, BackendDAE::Variables) -> Result<ArcStr> + 'static>), false, variables.clone());
    out = (verticalGridBoxed(unknowsMatchingToMathematicaGrid2(varString.clone(), eqsString.clone())?)).clone();
    Ok(out)
}

fn variablesToMathematicaGrid(mut varIndices: Arc<metamodelica::List<i32>>, mut variables: BackendDAE::Variables) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqsString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    varList = List::map1r(varIndices.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), variables.clone());
    eqsString = List::map2(varList.clone(), (std::sync::Arc::new(MathematicaDump::printMmaVarStr) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool, BackendDAE::Variables) -> Result<ArcStr> + 'static>), false, variables.clone());
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[{")); __mm_s.push_str(&*numerateListIndex(eqsString.clone(), varIndices.clone())?); __mm_s.push_str(&*literal!("},Frame -> All]")); ArcStr::from(__mm_s) }).clone();
    Ok(out)
}

fn writeFileIfNonEmpty(mut filename: ArcStr, mut content: ArcStr) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ('mc: {
        let __mc_input = (filename.clone(), content.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", _) => {
                    Ok(content.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut directory: ArcStr = arcstr::literal!("");
                    directory = (System::dirname((filename.clone()).clone())).clone();
                    let true = (System::directoryExists((directory.clone()).clone())) else { bail!("pattern mismatch") };
                    System::writeFile((filename.clone()).clone(), (content.clone()).clone())?;
                    Ok(literal!("Done..."))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(content.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(out)
}

fn dumpVarDistributionInfo(mut d: Option<Arc<DAE::Distribution>>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &(d.clone()) {
        Some(Deref @ DAE::Distribution { name, params, paramNames }) => {
            let mut e1: ArcStr = arcstr::literal!("");
            let mut e2: ArcStr = arcstr::literal!("");
            let mut e3: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            e1 = (MathematicaDump::printExpMmaStr(name.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
            e2 = (MathematicaDump::printExpMmaStr(params.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
            e3 = (MathematicaDump::printExpMmaStr(paramNames.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))?).clone();
            s1 = stringDelimitList(list![(e1.clone()).clone(), (e2.clone()).clone(), (e3.clone()).clone()], (literal!(",")).clone());
            s = stringAppendList(list![(literal!("{")).clone(), (s1.clone()).clone(), (literal!("}")).clone()]);
            s.clone()
        },
        None => {
            literal!("\"None\"")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(out)
}

fn dumpVarsDistributionInfo(mut d: Arc<metamodelica::List<Option<Arc<DAE::Distribution>>>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(d.clone(), (std::sync::Arc::new(dumpVarDistributionInfo) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Distribution>>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    s
}

fn getEquationsWithApproximatedAnnotation(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outEqs = (::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: BackendDAE::EqSystem { orderedEqs, .. }, tail: _ }, derivativeNamePrefix: _, .. } => {
            let mut ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
            ret = getEquationsWithApproximatedAnnotation2(BackendEquation::equationList(orderedEqs.clone()), 1)?;
            ret.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getEquationsWithApproximatedAnnotation2(mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut index: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut listOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    listOut = 'mc: {
        let __mc_input = (eqs.clone(), index.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, i) => {
                    let mut inner_ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (isApproximatedEquation(h.clone())?) else { bail!("pattern mismatch") };
                    inner_ret = getEquationsWithApproximatedAnnotation2(t.clone(), i.clone() + 1)?;
                    Ok(cons(i.clone(), inner_ret.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: t }, i) => {
                    let mut inner_ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    inner_ret = getEquationsWithApproximatedAnnotation2(t.clone(), i.clone() + 1)?;
                    Ok(inner_ret.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(listOut)
}

fn isApproximatedEquation(mut eqn: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { source: Deref @ DAE::ElementSource { comment, .. }, .. } => {
            let mut ret: bool = false;
            ret = isApproximatedEquation2(comment.clone())?;
            ret.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isApproximatedEquation2(mut commentIn: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<bool> {
    let mut out: bool = false;
    out = 'mc: {
        let __mc_input = commentIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst, .. } }), .. }, tail: t } => {
                    let mut ret: bool = false;
                    ret = List::any(subModLst.clone(), (std::sync::Arc::new(fnptr!(isApproximatedEquation3, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>)) || isApproximatedEquation2(t.clone())?;
                    Ok(ret.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: t } => {
                    let mut ret: bool = false;
                    ret = isApproximatedEquation2(t.clone())?;
                    Ok(ret.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

fn isApproximatedEquation3(mut m: Arc<SCode::SubMod>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_ApproximatedEquation", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn flattenModel(mut className: Arc<Absyn::Path>, mut p: Absyn::Program, mut icache: FCore::Cache) -> Result<(DAE::DAElist, FCore::Cache, FCore::Graph)> {
    let mut daeOut: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut cacheOut: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut graphOut: FCore::Graph;
    (daeOut, cacheOut, graphOut) = 'mc: {
        let __mc_input = (className.clone(), p.clone(), icache.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut p_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
                    let mut graph: FCore::Graph;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_UNCERTAINTIES.clone())?;
                    p_1 = AbsynToSCode::translateAbsyn2SCode(p.clone())?;
                    (cache, graph, _, dae) = Inst::instantiateClass(icache.clone(), InnerOuter::emptyInstHierarchy().clone(), p_1.clone(), className.clone(), true, true, true)?;
                    let _ = System::realtimeTock(ClockIndexes::RT_CLOCK_UNCERTAINTIES.clone())?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
                    dae = DAEUtil::transformationsBeforeBackend(cache.clone(), graph.clone(), dae.clone())?;
                    Ok((dae.clone(), cache.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut resstr: ArcStr = arcstr::literal!("");
                    resstr = AbsynUtil::pathStringNoQual(className.clone(), (literal!(".")).clone(), true, false)?;
                    resstr = stringAppendList(list![(literal!("modelEquationsUC: The model ")).clone(), (resstr.clone()).clone(), (literal!(" could not be flattened")).clone()]);
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(resstr.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((daeOut, cacheOut, graphOut))
}

fn getMathematicaVarStr(mut vars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    let mut states: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut algs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut inputsStates: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s1: ArcStr = arcstr::literal!("");
    (states, algs, outputs, inputsStates) = MathematicaDump::printMmaVarsStr(vars.clone())?;
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*Util::stringDelimitListNonEmptyElts(listAppend(states.clone(), listAppend(algs.clone(), listAppend(outputs.clone(), inputsStates.clone()))), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    Ok(out)
}

fn getMathematicaEqStr(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut systemVars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = (MathematicaDump::printMmaEqnsStr(eqns.clone(), (systemVars.clone(), globalKnownVars.clone()))).clone();
    out
}

fn getEquationsForUnknownsSystem(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut eqnsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (eqnsOut, varsOut) = 'mc: {
        let __mc_input = (m.clone(), knowns.clone(), unknowns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut unknownsSystem: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut yEqMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut yVarMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nv: i32 = 0;
                    let mut ne: i32 = 0;
                    let mut my: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    unknownsSystem = getSystemForUnknowns(m.clone(), knowns.clone(), unknowns.clone())?;
                    (yEqMap, yVarMap, my) = prepareForMatching(unknownsSystem.clone())?;
                    ne = (yEqMap.clone().len() as i32);
                    nv = (yVarMap.clone().len() as i32);
                    ass1 = arrayCreate(ne.clone(), -1);
                    ass2 = arrayCreate(nv.clone(), -1);
                    let true = (BackendDAEEXT::setAssignment(ne.clone(), nv.clone(), ass1.clone(), ass2.clone())) else { bail!("pattern mismatch") };
                    Matching::matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), my.clone());
                    BackendDAEEXT::matching(nv.clone(), ne.clone(), 1, -1, metamodelica::OrderedFloat(0.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
                    vars = yVarMap.clone();
                    setS = restoreIndicesEquivalence(List::filter1OnTrue(Arc::new(ass2.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), -1), yEqMap.clone())?;
                    Ok((setS.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((eqnsOut, varsOut))
}

fn getEquationsForKnownsSystem(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>, mut setS: Arc<metamodelica::List<i32>>, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut variables: BackendDAE::Variables, mut knownVariables: BackendDAE::Variables, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut setCOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut removed_equations_squaredOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (setCOut, removed_equations_squaredOut) = 'mc: {
        let __mc_input = (m.clone(), knowns.clone(), unknowns.clone(), setS.clone(), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut knownsSystem: ExtAdjacencyMatrix = metamodelica::nil();
                    knownsSystem = removeEquations(m.clone(), setS.clone())?;
                    knownsSystem = removeUnrelatedEquations(knownsSystem.clone(), knowns.clone());
                    let true = (knownsSystem.clone().is_empty()) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("Warning: The system is ill-posed. There are no remaining equations containing the knowns.\n")).clone());
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut knownsSystem: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut knownsSystemComp: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut xEqMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut xVarMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mx: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut comps_fixed: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut removed_equations_squared: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nxVarMap: i32 = 0;
                    let mut nxEqMap: i32 = 0;
                    let mut size: i32 = 0;
                    knownsSystem = removeEquations(m.clone(), setS.clone())?;
                    knownsSystem = removeUnrelatedEquations(knownsSystem.clone(), knowns.clone());
                    printSep((getMathematicaText((literal!("System of knowns after step 7")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(getEquationsNumber(knownsSystem.clone()), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?).clone())?;
                    knownsSystemComp = sortEquations(knownsSystem.clone(), knowns.clone())?;
                    knownsSystemComp = removeVarsNotInSet(knownsSystemComp.clone(), knowns.clone());
                    (xEqMap, xVarMap, mx) = prepareForMatching(knownsSystemComp.clone())?;
                    nxVarMap = (xVarMap.clone().len() as i32);
                    nxEqMap = (xEqMap.clone().len() as i32);
                    size = if (nxEqMap.clone() > nxVarMap.clone()) {nxEqMap.clone()} else {nxVarMap.clone()};
                    Matching::matchingExternalsetAdjacencyMatrix(size.clone(), size.clone(), mx.clone());
                    ass1 = arrayCreate(size.clone(), 0);
                    ass2 = arrayCreate(size.clone(), 0);
                    let true = (BackendDAEEXT::setAssignment(size.clone(), size.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
                    BackendDAEEXT::matching(size.clone(), size.clone(), 1, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
                    mt = AdjacencyMatrix::transposeAdjacencyMatrix(mx.clone(), nxVarMap.clone())?;
                    comps = getComponentsWrapper(mx.clone(), mt.clone(), ass1.clone(), ass2.clone())?;
                    comps = removeDummyEquations(comps.clone(), (xEqMap.clone().len() as i32))?;
                    comps_fixed = List::map1(comps.clone(), (std::sync::Arc::new(restoreIndicesEquivalence) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), xEqMap.clone());
                    (knownsSystem, removed_equations_squared) = removeEquationInSquaredBlock(knownsSystem.clone(), knowns.clone(), unknowns.clone(), comps_fixed.clone())?;
                    comps_fixed = List::map1(comps_fixed.clone(), (std::sync::Arc::new(restoreIndicesEquivalence) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
                    printSep((getMathematicaText((literal!("Blocks (each row is a block)")).clone())).clone())?;
                    printSep(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[")); __mm_s.push_str(&*listString(List::map(comps_fixed.clone(), (std::sync::Arc::new(fnptr!(intListString, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))); __mm_s.push_str(&*literal!(",Frame->All]")); ArcStr::from(__mm_s) }).clone())?;
                    printSep((getMathematicaText((literal!("System of knowns after step 8 and 9")).clone())).clone())?;
                    printSep((equationsToMathematicaGrid(getEquationsNumber(knownsSystem.clone()), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?).clone())?;
                    checkSystemContainsVars(knownsSystem.clone(), knowns.clone(), variables.clone())?;
                    setC = getEquationsNumber(knownsSystem.clone());
                    Ok((setC.clone(), removed_equations_squared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((setCOut, removed_equations_squaredOut))
}

fn printVarReduction(mut elems: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> () {
    println!("{}", (literal!("Reduced variables:\n")).clone());
    println!("{}", stringDelimitList(List::map(elems.clone(), (std::sync::Arc::new(fnptr!(printVarReduction2, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone()));
    ()
}

fn printVarReduction2(mut elem: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    let mut occurrences: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (occurrences, vars) = elem.clone();
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(List::map(vars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(") (")); __mm_s.push_str(&*stringDelimitList(List::map(occurrences.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    out
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn pickReductionCandidates(mut elems: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut elemsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    elemsOut = 'mc: {
        let __mc_input = elems.clone();
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
                Deref @ metamodelica::List::Cons { head: (occurrence, vars), tail: tail } => {
                    let mut newElems: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = ((vars.clone().len() as i32) > 1 && (occurrence.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
                    newElems = pickReductionCandidates(tail.clone())?;
                    Ok(cons(vars.clone(), newElems.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: tail } => {
                    Ok(pickReductionCandidates(tail.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(elemsOut)
}

fn reduceVariables(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    let mut neq: i32 = 0;
    let mut nvar: i32 = 0;
    let mut variables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut occurrences: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut candidates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut reducedVars: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut newM: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = 'mc: {
        let __mc_input = (m.clone(), knowns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut variables: Arc<metamodelica::List<i32>> = variables.clone();
                    let mut nvar: i32 = nvar.clone();
                    let mut neq: i32 = neq.clone();
                    neq = (getEquationsNumber(m.clone()).len() as i32);
                    variables = getVariables(m.clone());
                    nvar = (variables.clone().len() as i32);
                    let true = (neq.clone() >= nvar.clone()) else { bail!("pattern mismatch") };
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut occurrences: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = occurrences.clone();
                    let mut neq: i32 = neq.clone();
                    let mut newM: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = newM.clone();
                    let mut variables: Arc<metamodelica::List<i32>> = variables.clone();
                    let mut reducedVars: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = reducedVars.clone();
                    let mut nvar: i32 = nvar.clone();
                    let mut candidates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = candidates.clone();
                    neq = (getEquationsNumber(m.clone()).len() as i32);
                    variables = getVariables(m.clone());
                    nvar = (variables.clone().len() as i32);
                    let true = (neq.clone() < nvar.clone()) else { bail!("pattern mismatch") };
                    occurrences = List::map1r(knowns.clone(), (std::sync::Arc::new(occurrencesOfVariable) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>), m.clone());
                    reducedVars = findReductionCantidates(variables.clone(), occurrences.clone(), metamodelica::nil())?;
                    candidates = pickReductionCandidates(reducedVars.clone())?;
                    newM = reduceVariablesInMatrix(m.clone(), candidates.clone(), nvar.clone() - neq.clone())?;
                    Ok(newM.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn reduceVariablesInMatrix(mut m: ExtAdjacencyMatrix, mut candidates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut count: i32) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = 'mc: {
        let __mc_input = (m.clone(), candidates.clone(), count.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    let true = (count.clone() > 0) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("Warning: The system of equations is under-determined. The results may be incorrect.\n")).clone());
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (intEq(count.clone(), 0)) else { bail!("pattern mismatch") };
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: candidate, tail: candidatesTail }, _) => {
                    let mut variables: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut temp: i32 = 0;
                    let mut newM: ExtAdjacencyMatrix = metamodelica::nil();
                    let true = (count.clone() > 0) else { bail!("pattern mismatch") };
                    temp = listHead(candidate.clone())?;
                    variables = List::setDifference(getVariables(m.clone()), list![temp.clone()])?;
                    newM = removeVarsNotInSet(m.clone(), variables.clone());
                    newM = reduceVariablesInMatrix(newM.clone(), candidatesTail.clone(), count.clone() - 1)?;
                    Ok(newM.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findReductionCantidates(mut variables: Arc<metamodelica::List<i32>>, mut occurrences: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut acc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>> {
    let mut out: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((variables.clone(), occurrences.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: var, tail: varTail }, Deref @ metamodelica::List::Cons { head: occurrence, tail: occurrenceTail }, _) => {
            let mut newAcc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
            newAcc = findReductionCantidates2(var.clone(), occurrence.clone(), acc.clone())?;
            findReductionCantidates(varTail.clone(), occurrenceTail.clone(), newAcc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn findReductionCantidates2(mut var: i32, mut occurrence: Arc<metamodelica::List<i32>>, mut acc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>> {
    let mut accOut: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    accOut = 'mc: {
        let __mc_input = (var.clone(), occurrence.clone(), acc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    let mut newAcc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
                    newAcc = list![(occurrence.clone(), list![var.clone()])];
                    Ok(newAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: (elemOccurrences, vars), tail: tail }) => {
                    let mut newAcc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
                    let mut elem: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let true = (intEq((occurrence.clone().len() as i32), (elemOccurrences.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let true = (containsAll(occurrence.clone(), elemOccurrences.clone())) else { bail!("pattern mismatch") };
                    elem = (elemOccurrences.clone(), listAppend(vars.clone(), list![var.clone()]));
                    newAcc = cons(elem.clone(), tail.clone());
                    Ok(newAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: (elemOccurrences, vars), tail: tail }) => {
                    let mut newAcc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
                    newAcc = findReductionCantidates2(var.clone(), occurrence.clone(), tail.clone())?;
                    Ok(cons((elemOccurrences.clone(), vars.clone()), newAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(accOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn eliminateOutputVariables(mut mIn: ExtAdjacencyMatrix, mut outputs: Arc<metamodelica::List<i32>>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = 'mc: {
        let __mc_input = (mIn.clone(), outputs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ metamodelica::List::Nil) => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ metamodelica::List::Cons { head: var, tail: tail }) => {
                    let mut o: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut newM: ExtAdjacencyMatrix = metamodelica::nil();
                    o = occurrencesOfVariable(m.clone(), var.clone())?;
                    let true = (intEq((o.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    newM = removeEquations(m.clone(), o.clone())?;
                    newM = eliminateOutputVariables(newM.clone(), tail.clone())?;
                    Ok(newM.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ metamodelica::List::Cons { head: _, tail: tail }) => {
                    let mut newM: ExtAdjacencyMatrix = metamodelica::nil();
                    newM = eliminateOutputVariables(m.clone(), tail.clone())?;
                    Ok(newM.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn occurrencesOfVariable(mut m: ExtAdjacencyMatrix, mut var: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut out: Arc<metamodelica::List<i32>> = metamodelica::nil();
    out = 'mc: {
        let __mc_input = (m.clone(), var.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (eq, vars), tail: tail }, _) => {
                    let mut ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (containsAny(vars.clone(), list![var.clone()])) else { bail!("pattern mismatch") };
                    ret = occurrencesOfVariable(tail.clone(), var.clone())?;
                    Ok(cons(eq.clone(), ret.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (_, _), tail: tail }, _) => {
                    let mut ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    ret = occurrencesOfVariable(tail.clone(), var.clone())?;
                    Ok(ret.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

fn getEquationsNumber(mut m: ExtAdjacencyMatrix) -> Arc<metamodelica::List<i32>> {
    let mut numbers: Arc<metamodelica::List<i32>> = metamodelica::nil();
    numbers = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (eq, _), tail: t } => {
            let mut inner_ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
            inner_ret = getEquationsNumber(t.clone());
            cons(eq.clone(), inner_ret.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    numbers
}

fn getMathematicaText(mut text: ArcStr) -> ArcStr {
    let mut textOut: ArcStr = arcstr::literal!("");
    textOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Text[Style[\"")); __mm_s.push_str(&*text.clone()); __mm_s.push_str(&*literal!("\",Bold,Large]]")); ArcStr::from(__mm_s) }).clone();
    textOut
}

fn getComponentsWrapper(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut compsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    compsOut = 'mc: {
        let __mc_input = (m.clone(), mt.clone(), ass1.clone(), ass2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(0, (m.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            Ok(list![metamodelica::nil()])
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(1, (m.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            Ok(list![list![1]])
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut comp: Arc<metamodelica::List<i32>> = metamodelica::nil();
            if '__try0: {
                let _ = unwrap_break_err!(Sorting::TarjanTransposed(mt.clone(), ass2.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            println!("{}", (literal!("TarjanAlgorithm failed\n")).clone());
            Error::clearMessages();
            comp = List::intRange((m.clone().borrow().len() as i32));
            comps = list![comp.clone()];
            Ok(comps.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            comps = Sorting::TarjanTransposed(mt.clone(), ass2.clone())?;
            Ok(comps.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compsOut)
}

fn getVariables(mut m: ExtAdjacencyMatrix) -> Arc<metamodelica::List<i32>> {
    let mut varsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varsOut = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (_, vars), tail: t } => {
            let mut newVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            newVars = listAppend(vars.clone(), getVariables(t.clone()));
            newVars = List::unique(newVars.clone());
            newVars.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varsOut
}

fn removeEquationInSquaredBlock(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>, mut components: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(ExtAdjacencyMatrix, Arc<metamodelica::List<i32>>)> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    let mut removedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (mOut, removedEquations) = 'mc: {
        let __mc_input = (m.clone(), knowns.clone(), unknowns.clone(), components.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: h, tail: t }) => {
                    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut usedKnowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut compEqns: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut compsSorted: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut tailEquations: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut inner_ret: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut removeEquation: i32 = 0;
                    let mut removed_inner: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    compEqns = getEquations(m.clone(), h.clone());
                    vars = getVariables(compEqns.clone());
                    usedKnowns = List::intersectionOnTrue(vars.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
                    let true = (intEq((h.clone().len() as i32), (usedKnowns.clone().len() as i32))) else { bail!("pattern mismatch") };
                    compsSorted = sortEquations(compEqns.clone(), unknowns.clone())?.reverse();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(compsSorted.clone()) {
                        Deref @ metamodelica::List::Cons { head: (__pa0, _), tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    removeEquation = __pa0.clone();
                    tailEquations = __pa1.clone();
                    (inner_ret, removed_inner) = removeEquationInSquaredBlock(m.clone(), knowns.clone(), unknowns.clone(), t.clone())?;
                    removed_inner = if ((compsSorted.clone().len() as i32) > 1) {cons(removeEquation.clone(), removed_inner.clone())} else {removed_inner.clone()};
                    Ok((listAppend(tailEquations.clone(), inner_ret.clone()), removed_inner.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: h, tail: t }) => {
                    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut usedKnowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut compEqns: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut inner_ret: ExtAdjacencyMatrix = metamodelica::nil();
                    let mut removed_inner: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    compEqns = getEquations(m.clone(), h.clone());
                    vars = getVariables(compEqns.clone());
                    usedKnowns = List::intersectionOnTrue(vars.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
                    let false = (intEq((h.clone().len() as i32), (usedKnowns.clone().len() as i32))) else { bail!("pattern mismatch") };
                    (inner_ret, removed_inner) = removeEquationInSquaredBlock(m.clone(), knowns.clone(), unknowns.clone(), t.clone())?;
                    Ok((listAppend(compEqns.clone(), inner_ret.clone()), removed_inner.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((mOut, removedEquations))
}

fn printIntList(mut l: Arc<metamodelica::List<i32>>) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("List of size = ")); __mm_s.push_str(&*intString((l.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", stringDelimitList(List::map(l.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone()));
    println!("{}", (literal!("\n")).clone());
    ()
}

fn intListString(mut l: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(l.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    out
}

fn listString(mut l: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(l.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    out
}

fn setOfList(mut inList: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outList = List::unique(inList.clone());
    outList
}

fn countKnowns(mut row: ExtAdjacencyMatrixRow, mut knowns: Arc<metamodelica::List<i32>>) -> i32 {
    let mut out: i32 = 0;
    out = (::match_deref::match_deref! { match &((row.clone(), knowns.clone())) {
        ((_, vars), _) => {
            let mut n: i32 = 0;
            n = (List::intersectionOnTrue(vars.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)).len() as i32);
            n.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn sortEquations(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = sortBy1(m.clone(), (std::sync::Arc::new(fnptr!(countKnowns, (i32, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>) -> Result<i32> + 'static>), knowns.clone())?;
    Ok(mOut)
}

fn removeVarsNotInSet_helper(mut var: i32, mut elems: Arc<metamodelica::List<i32>>) -> bool {
    let mut out: bool = false;
    out = containsAny(list![var.clone()], elems.clone());
    out
}

fn removeVarsNotInSet(mut m: ExtAdjacencyMatrix, mut set: Arc<metamodelica::List<i32>>) -> ExtAdjacencyMatrix {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    for mut el in &*m.clone() {
        let mut el = el.clone();
        (eq, vars) = el.clone();
        newVars = List::filter1OnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(removeVarsNotInSet_helper, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), set.clone());
        if !(newVars.clone().is_empty()) {
            mOut = cons((eq.clone(), newVars.clone()), mOut.clone());
        }
    }
    mOut = metamodelica::Dangerous::listReverseInPlace(mOut.clone());
    mOut
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeEquations(mut m: ExtAdjacencyMatrix, mut eqns: Arc<metamodelica::List<i32>>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = 'mc: {
        let __mc_input = (m.clone(), eqns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e @ (eq, _), tail: t }, _) => {
                    let mut inner_ret: ExtAdjacencyMatrix = metamodelica::nil();
                    let false = (containsAny(list![eq.clone()], eqns.clone())) else { bail!("pattern mismatch") };
                    inner_ret = removeEquations(t.clone(), eqns.clone())?;
                    Ok(cons(e.clone(), inner_ret.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (eq, _), tail: t }, _) => {
                    let mut inner_ret: ExtAdjacencyMatrix = metamodelica::nil();
                    let true = (containsAny(list![eq.clone()], eqns.clone())) else { bail!("pattern mismatch") };
                    inner_ret = removeEquations(t.clone(), eqns.clone())?;
                    Ok(inner_ret.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mOut)
}

fn getEquationsHelper(mut m: ExtAdjacencyMatrixRow, mut eqns: Arc<metamodelica::List<i32>>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &((m.clone(), eqns.clone())) {
        ((e, _), _) => {
            List::isMemberOnTrue(e.clone(), eqns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn getEquations(mut m: ExtAdjacencyMatrix, mut eqns: Arc<metamodelica::List<i32>>) -> ExtAdjacencyMatrix {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = List::filter1OnTrue(m.clone(), (std::sync::Arc::new(fnptr!(getEquationsHelper, (i32, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), eqns.clone());
    mOut
}

fn removeUnrelatedEquations2(mut row: ExtAdjacencyMatrixRow, mut knowns: Arc<metamodelica::List<i32>>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &((row.clone(), knowns.clone())) {
        ((_, vars), _) => {
            let mut ret: bool = false;
            ret = containsAny(vars.clone(), knowns.clone());
            ret.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn removeUnrelatedEquations(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>) -> ExtAdjacencyMatrix {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = List::filter1OnTrue(m.clone(), (std::sync::Arc::new(fnptr!(removeUnrelatedEquations2, (i32, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), knowns.clone());
    mOut
}

fn checkSystemContainsVars(mut m: ExtAdjacencyMatrix, mut knows: Arc<metamodelica::List<i32>>, mut variables: BackendDAE::Variables) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (m.clone(), knows.clone(), variables.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
                    let mut not_found_var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (removeUnrelatedEquations(m.clone(), list![h.clone()]).is_empty()) else { bail!("pattern mismatch") };
                    not_found_var = BackendVariable::getVarAt(variables.clone(), h.clone())?;
                    r#str = (ComponentReference::crefStr(BackendVariable::varCref(not_found_var.clone())?)?).clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: The variable '")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("' was not found in the system of knowns\n")); ArcStr::from(__mm_s) }).clone());
                    checkSystemContainsVars(m.clone(), t.clone(), variables.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
                    let false = (removeUnrelatedEquations(m.clone(), list![h.clone()]).is_empty()) else { bail!("pattern mismatch") };
                    checkSystemContainsVars(m.clone(), t.clone(), variables.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getSystemForUnknowns(mut m: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    let mut mTemp: ExtAdjacencyMatrix = metamodelica::nil();
    mTemp = sortEquations(m.clone(), knowns.clone())?;
    mOut = removeVarsNotInSet(mTemp.clone(), unknowns.clone());
    Ok(mOut)
}

fn getRelatedVariables(mut m: ExtAdjacencyMatrix, mut vars: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut varsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varsOut = 'mc: {
        let __mc_input = (m.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (_, eqvars), tail: t }, _) => {
                    let mut eqvars = (*eqvars).clone();
                    let true = (containsAny(eqvars.clone(), vars.clone())) else { bail!("pattern mismatch") };
                    eqvars = listAppend(eqvars.clone(), getRelatedVariables(t.clone(), vars.clone())?);
                    eqvars = List::setDifference(setOfList(eqvars.clone()), vars.clone())?;
                    Ok(eqvars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (_, eqvars), tail: t }, _) => {
                    let mut eqvars = (*eqvars).clone();
                    let false = (containsAny(eqvars.clone(), vars.clone())) else { bail!("pattern mismatch") };
                    eqvars = getRelatedVariables(t.clone(), vars.clone())?;
                    eqvars = List::setDifference(setOfList(eqvars.clone()), vars.clone())?;
                    Ok(eqvars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(varsOut)
}

fn restoreIndicesEquivalence(mut inList: Arc<metamodelica::List<i32>>, mut map: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut out: Arc<metamodelica::List<i32>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((inList.clone(), map.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
            let mut inner_ret: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut v: i32 = 0;
            v = (map.clone()).get(h.clone())?;
            inner_ret = restoreIndicesEquivalence(t.clone(), map.clone())?;
            cons(v.clone(), inner_ret.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn addIndexEquivalence(mut index: i32, mut map: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut indexOut: i32 = 0;
    let mut mapOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (indexOut, mapOut) = 'mc: {
        let __mc_input = (index.clone(), map.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut pos: i32 = 0;
                    let true = (List::isMemberOnTrue(index.clone(), map.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    pos = List::position(index.clone(), map.clone())?;
                    Ok((pos.clone(), map.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut pos: i32 = 0;
                    let mut newMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (List::isMemberOnTrue(index.clone(), map.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    pos = (map.clone().len() as i32) + 1;
                    newMap = listAppend(map.clone(), list![index.clone()]);
                    Ok((pos.clone(), newMap.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((indexOut, mapOut))
}

fn addVarEquivalences(mut vars: Arc<metamodelica::List<i32>>, mut map: Arc<metamodelica::List<i32>>, mut varsFixed: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut varMapOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (varMapOut, varsOut) = (::match_deref::match_deref! { match &((vars.clone(), map.clone(), varsFixed.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            (map.clone(), varsFixed.clone())
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: remaining }, _, _) => {
            let mut v: i32 = 0;
            let mut newMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut innerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut innerMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (v, newMap) = addIndexEquivalence(h.clone(), map.clone())?;
            (innerMap, innerVars) = addVarEquivalences(remaining.clone(), newMap.clone(), cons(v.clone(), varsFixed.clone()))?;
            (innerMap.clone(), innerVars.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((varMapOut, varsOut))
}

fn prepareForMatching2(mut mExt: ExtAdjacencyMatrix, mut eqMap: Arc<metamodelica::List<i32>>, mut varMap: Arc<metamodelica::List<i32>>, mut m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut eqMapOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varMapOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (eqMapOut, varMapOut, mOut) = (::match_deref::match_deref! { match &((mExt.clone(), eqMap.clone(), varMap.clone(), m.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            let mut newM: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            newM = m.clone().reverse();
            (eqMap.clone(), varMap.clone(), newM.clone())
        },
        (Deref @ metamodelica::List::Cons { head: (eq, vars), tail: t }, _, _, _) => {
            let mut newVarMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut newEqMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut newVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut newM: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            (_, newEqMap) = addIndexEquivalence(eq.clone(), eqMap.clone())?;
            (newVarMap, newVars) = addVarEquivalences(vars.clone(), varMap.clone(), metamodelica::nil())?;
            (newEqMap, newVarMap, newM) = prepareForMatching2(t.clone(), newEqMap.clone(), newVarMap.clone(), cons(newVars.clone(), m.clone()))?;
            (newEqMap.clone(), newVarMap.clone(), newM.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((eqMapOut, varMapOut, mOut))
}

fn prepareForMatching(mut mExt: ExtAdjacencyMatrix) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut eqMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (eqMap, varMap, m) = prepareForMatching2(mExt.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
    mOut = metamodelica::arrayFromVec(fixUnderdeterminedSystem(m.clone(), (varMap.clone().len() as i32), (eqMap.clone().len() as i32))?.into_iter().cloned().collect());
    Ok((eqMap, varMap, mOut))
}

fn removeDummyEquations(mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut max_neqs: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut out: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((comps.clone(), max_neqs.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
            let mut ret: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            row = List::removeOnTrue(max_neqs.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), h.clone());
            ret = removeDummyEquations(t.clone(), max_neqs.clone())?;
            cons(row.clone(), ret.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fixUnderdeterminedSystem(mut m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut nvars: i32, mut neqs: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut mOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    mOut = 'mc: {
        let __mc_input = (m.clone(), nvars.clone(), neqs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut dummyEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut new_m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (intGt(nvars.clone(), neqs.clone())) else { bail!("pattern mismatch") };
                    dummyEq = List::intRange(nvars.clone());
                    new_m = fixUnderdeterminedSystem(listAppend(m.clone(), list![dummyEq.clone()]), nvars.clone(), neqs.clone() + 1)?;
                    Ok(new_m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mOut)
}

fn getExtAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = getExtAdjacencyMatrix2(1, Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), metamodelica::nil())?;
    Ok(mOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getExtAdjacencyMatrix2(mut i: i32, mut m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut acc: ExtAdjacencyMatrix) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = (::match_deref::match_deref! { match &((i.clone(), m.clone(), acc.clone())) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            acc.clone().reverse()
        },
        (_, Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
            getExtAdjacencyMatrix2(i.clone() + 1, t.clone(), cons((i.clone(), h.clone()), acc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(mOut)
}

fn dumpExtAdjacencyMatrix(mut m: ExtAdjacencyMatrix) -> () {
    let _ = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (eq, vars), tail: t } => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::map(vars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpExtAdjacencyMatrix(t.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn containsAny(mut m1: Arc<metamodelica::List<i32>>, mut m2: Arc<metamodelica::List<i32>>) -> bool {
    let mut out: bool = false;
    let mut m3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    m3 = List::intersectionOnTrue(m1.clone(), m2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    out = !(m3.clone().is_empty());
    out
}

fn containsAll(mut m1: Arc<metamodelica::List<i32>>, mut m2: Arc<metamodelica::List<i32>>) -> bool {
    let mut out: bool = false;
    let mut m3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    m3 = List::intersectionOnTrue(m1.clone(), m2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    out = intEq((m3.clone().len() as i32), (m2.clone().len() as i32));
    out
}

pub fn getUncertainRefineVariableIndexes(mut allVariables: BackendDAE::Variables, mut variableIndexList: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Option<Arc<DAE::Distribution>>>>)> {
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut distributions: Arc<metamodelica::List<Option<Arc<DAE::Distribution>>>> = metamodelica::nil();
    (indices, distributions) = 'mc: {
        let __mc_input = (allVariables.clone(), variableIndexList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: index, tail: variableIndexListRest }) => {
                    let mut refineVariableIndexList: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut dist: Option<Arc<DAE::Distribution>> = None;
                    let mut distInner: Arc<metamodelica::List<Option<Arc<DAE::Distribution>>>> = metamodelica::nil();
                    var = BackendVariable::getVarAt(allVariables.clone(), index.clone())?;
                    let true = (BackendVariable::varHasUncertainValueRefine(var.clone())) else { bail!("pattern mismatch") };
                    dist = BackendVariable::varTryGetDistribution(var.clone());
                    (refineVariableIndexList, distInner) = getUncertainRefineVariableIndexes(allVariables.clone(), variableIndexListRest.clone())?;
                    Ok((cons(index.clone(), refineVariableIndexList.clone()), cons(dist.clone(), distInner.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: index, tail: variableIndexListRest }) => {
                    let mut refineVariableIndexList: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut distInner: Arc<metamodelica::List<Option<Arc<DAE::Distribution>>>> = metamodelica::nil();
                    var = BackendVariable::getVarAt(allVariables.clone(), index.clone())?;
                    let false = (BackendVariable::varHasUncertainValueRefine(var.clone())) else { bail!("pattern mismatch") };
                    (refineVariableIndexList, distInner) = getUncertainRefineVariableIndexes(allVariables.clone(), variableIndexListRest.clone())?;
                    Ok((refineVariableIndexList.clone(), distInner.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    println!("{}", (literal!("getUncertainRefineVariableIndexes failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((indices, distributions))
}

pub fn eliminateVariablesDAE(mut elimVarIndexList: Arc<metamodelica::List<i32>>, mut indae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    outDae = (::match_deref::match_deref! { match &((elimVarIndexList.clone(), indae.clone())) {
        (_, dae @ Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, tail: _ }, derivativeNamePrefix: shared @ BackendDAE::Shared { initialEqs: ieqns, globalKnownVars, .. }, .. }) => {
            let mut vars_1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut kvars_1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut crefDouble: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut movedvars_1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut dae = (*dae).clone();
            let _ = BackendEquation::equationList(ieqns.clone());
            eqnLst = BackendEquation::equationList(eqns.clone());
            crefDouble = findArraysPartiallyIndexed(eqnLst.clone())?;
            repl = BackendVarTransform::emptyReplacements();
            (m, _, _, _) = BackendDAEUtil::adjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            (eqnLst, _, movedvars_1, repl) = eliminateVariablesDAE2(eqnLst.clone(), 1, vars.clone(), globalKnownVars.clone(), HashTable::emptyHashTable(), repl.clone(), crefDouble.clone(), m.clone(), elimVarIndexList.clone(), false)?;
            dae = setDaeEqns(dae.clone(), BackendEquation::listEquation(eqnLst.clone())?, false)?;
            dae = replaceDAElow(dae.clone(), repl.clone(), None, false)?;
            (vars_1, kvars_1) = moveVariables(BackendVariable::daeVars(syst.clone()), BackendVariable::daeGlobalKnownVars(shared.clone()), movedvars_1.clone())?;
            dae = setDaeVars(dae.clone(), vars_1.clone())?;
            dae = BackendDAEUtil::setDAEGlobalKnownVars(dae.clone(), kvars_1.clone())?;
            dae = BackendDAEUtil::transformBackendDAE(dae.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED)), None, None)?;
            dae = BackendDAEUtil::mapEqSystem1(dae.clone(), (std::sync::Arc::new(BackendDAEUtil::getAdjacencyMatrixfromOptionForMapEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendDAE::IndexType, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>), crate::BackendDAE::IndexType::NORMAL)?;
            dae.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

fn findArraysPartiallyIndexed(mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    ht = findArraysPartiallyIndexed1(inEqs.clone(), HashTable::emptyHashTable())?;
    ht = findArraysPartiallyIndexedRecords(inEqs.clone(), ht.clone())?;
    Ok(ht)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findArraysPartiallyIndexed1(mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    outHt = 'mc: {
        let __mc_input = (inEqs.clone(), inht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ht) => {
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }, tail: eqs }, ht) => {
                    let mut ht = (*ht).clone();
                    let _ = Algorithm::getAllExps(alg.clone())?;
                    ht = findArraysPartiallyIndexed1(eqs.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, tail: eqs }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysPartiallyIndexed2(list![e1.clone(), e2.clone()], ht.clone(), HashTable::emptyHashTable())?;
                    ht = findArrayVariables(list![e1.clone(), e2.clone()], ht.clone())?;
                    ht = findArraysPartiallyIndexed1(eqs.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: eqs }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysPartiallyIndexed1(eqs.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findArraysPartiallyIndexed2(mut inRef: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut indubRef: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut inht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    outHt = (::match_deref::match_deref! { match &((inRef.clone(), indubRef.clone(), inht.clone())) {
        (Deref @ metamodelica::List::Nil, _, ht) => {
            ht.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: c1, ty: _ }, tail: expl1 }, dubRef, ht) => {
            let mut c2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dubRef = (*dubRef).clone();
            let mut ht = (*ht).clone();
            c2 = ComponentReferenceBasics::crefStripLastSubs(c1.clone())?;
            if BaseHashTable::hasKey(c2.clone(), dubRef.clone()) {
                if BaseHashTable::hasKey(c2.clone(), ht.clone()) {
                } else {
                    ht = BaseHashTable::add((c2.clone(), 1), ht.clone())?;
                }
            } else {
                dubRef = BaseHashTable::add((c2.clone(), 1), dubRef.clone())?;
            }
            ht = findArraysPartiallyIndexed2(expl1.clone(), dubRef.clone(), ht.clone())?;
            ht.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: expl1 }, dubRef, ht) => {
            let mut ht = (*ht).clone();
            ht = findArraysPartiallyIndexed2(expl1.clone(), dubRef.clone(), ht.clone())?;
            ht.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outHt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findArrayVariables(mut inRef: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    outHt = 'mc: {
        let __mc_input = (inRef.clone(), inht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ht) => {
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: c1, ty: _ }, tail: expl1 }, ht) => {
                    let mut ht = (*ht).clone();
                    let true = (Expression::isArrayType(ComponentReference::crefTypeConsiderSubs(c1.clone())?)) else { bail!("pattern mismatch") };
                    ht = BaseHashTable::add((c1.clone(), 1), ht.clone())?;
                    ht = findArrayVariables(expl1.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: expl1 }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArrayVariables(expl1.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHt)
}

fn findArraysPartiallyIndexedRecords(mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (_, outHt) = BackendEquation::traverseExpsOfEquationList(inEqs.clone(), (std::sync::Arc::new(findArraysPartiallyIndexedRecordsExpVisitor) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> + 'static>), ht.clone())?;
    Ok(outHt)
}

fn findArraysPartiallyIndexedRecordsExpVisitor(mut inExp: Arc<DAE::Exp>, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)))> {
    let mut e: Arc<DAE::Exp>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    (e, ht) = 'mc: {
        let __mc_input = (inExp.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, ht) => {
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut ht = (*ht).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastType(cr.clone())?) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    ht = findArraysInRecordLst(ht.clone(), cr.clone(), varLst.clone())?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((e, ht))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findArraysInRecordLst(mut inht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut recordCr: Arc<DAE::ComponentRef>, mut invarLst: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    outHt = 'mc: {
        let __mc_input = (inht.clone(), recordCr.clone(), invarLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ht, _, Deref @ metamodelica::List::Nil) => {
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ht, _, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty: tp, name, .. }, tail: varLst }) => {
                    let mut thisCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut ht = (*ht).clone();
                    let true = (Expression::isArrayType(tp.clone())) else { bail!("pattern mismatch") };
                    thisCr = ComponentReference::joinCrefs(recordCr.clone(), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: tp.clone(), subscriptLst: metamodelica::nil() }))?;
                    ht = BaseHashTable::add((thisCr.clone(), 0), ht.clone())?;
                    ht = findArraysInRecordLst(ht.clone(), recordCr.clone(), varLst.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ht, _, Deref @ metamodelica::List::Cons { head: _, tail: varLst }) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysInRecordLst(ht.clone(), recordCr.clone(), varLst.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHt)
}

fn eliminateVariablesDAE2(mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut eqnIndex: i32, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut mvars: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut repl: BackendVarTransform::VariableReplacements, mut inDoubles: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut elimVarIndexList: Arc<metamodelica::List<i32>>, mut failCheck: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), BackendVarTransform::VariableReplacements)> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outSimpleEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outMvars: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut outRepl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (outEqns, outSimpleEqns, outMvars, outRepl) = 'mc: {
        let __mc_input = (ieqns.clone(), eqnIndex.clone(), vars.clone(), globalKnownVars.clone(), mvars.clone(), repl.clone(), inDoubles.clone(), m.clone(), elimVarIndexList.clone(), failCheck.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, false) => {
                    Ok((metamodelica::nil(), metamodelica::nil(), mvars.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: eqns }, _, _, _, _, _, _, _, _, false) => {
                    let mut mvars_1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut mvars_2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut repl_1: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut repl_2: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eqns_1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut seqns_1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut varIndexList: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elimVarIndexList_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elimVarIndex: i32 = 0;
                    let mut e2: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut elimVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![e.clone()], repl.clone(), None)?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    varIndexList = m.borrow()[(eqnIndex.clone()-1) as usize].clone();
                    let __pa2 = ::match_deref::match_deref! { match &(List::intersectionOnTrue(varIndexList.clone(), elimVarIndexList.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    elimVarIndex = __pa2.clone();
                    elimVarIndexList_1 = List::removeOnTrue(elimVarIndex.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), elimVarIndexList.clone());
                    elimVar = BackendVariable::getVarAt(vars.clone(), elimVarIndex.clone())?;
                    let BackendDAE::VAR { varName: __pa3, .. } = (elimVar.clone()) else { bail!("pattern mismatch") };
                    cr1 = __pa3.clone();
                    (e2, source) = solveEqn2(e.clone(), cr1.clone())?;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), cr1.clone(), e2.clone(), None)?;
                    mvars_1 = BaseHashTable::add((cr1.clone(), 0), mvars.clone())?;
                    (eqns_1, seqns_1, mvars_2, repl_2) = eliminateVariablesDAE2(eqns.clone(), eqnIndex.clone() + 1, vars.clone(), globalKnownVars.clone(), mvars_1.clone(), repl_1.clone(), inDoubles.clone(), m.clone(), elimVarIndexList_1.clone(), failCheck.clone())?;
                    Ok((eqns_1.clone(), cons(Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1.clone(), exp: e2.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), seqns_1.clone()), mvars_2.clone(), repl_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: eqns }, _, _, _, _, _, _, _, _, false) => {
                    let mut mvars_1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut repl_1: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut eqns_1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut seqns_1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    (eqns_1, seqns_1, mvars_1, repl_1) = eliminateVariablesDAE2(eqns.clone(), eqnIndex.clone() + 1, vars.clone(), globalKnownVars.clone(), mvars.clone(), repl.clone(), inDoubles.clone(), m.clone(), elimVarIndexList.clone(), false)?;
                    Ok((cons(e.clone(), eqns_1.clone()), seqns_1.clone(), mvars_1.clone(), repl_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqns, outSimpleEqns, outMvars, outRepl))
}

fn solveEqn2(mut eqn: Arc<BackendDAE::Equation>, mut cr: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Exp>, Arc<DAE::ElementSource>)> {
    let mut exp: Arc<DAE::Exp>;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    (exp, source) = (::match_deref::match_deref! { match &((eqn.clone(), cr.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { source, scalar: e2, exp: e1, .. }, _) => {
            (exp, _) = ExpressionSolve::solve(e1.clone(), e2.clone(), Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_REAL_DEFAULT().clone() }), None)?;
            (exp.clone(), source.clone())
        },
        (_, _) => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, source))
}

fn setDaeVars(mut systIn: Arc<BackendDAE::BackendDAE>, mut newVarsIn: BackendDAE::Variables) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut sysOut: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    sysOut = BackendDAEUtil::setVars(systIn.clone(), newVarsIn.clone())?;
    Ok(sysOut)
}

fn setDaeEqns(mut dae: Arc<BackendDAE::BackendDAE>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut initEqs: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    odae = (::match_deref::match_deref! { match &((dae.clone(), initEqs.clone())) {
        (Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst, tail: systList }, derivativeNamePrefix: shared, .. }, false) => {
            let mut syst = (*syst).clone();
            syst = BackendDAEUtil::setEqSystEqs(syst.clone(), eqns.clone());
            BackendDAE::DAE(cons(syst.clone(), systList.clone()), shared.clone())?
        },
        (Deref @ DAE { UNIQUEIO: systList, derivativeNamePrefix: shared, .. }, false) => {
            let mut shared = (*shared).clone();
            shared = BackendDAEUtil::setSharedInitialEqns(shared.clone(), eqns.clone())?;
            BackendDAE::DAE(systList.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(odae)
}

pub fn replaceDAElow(mut idlow: Arc<BackendDAE::BackendDAE>, mut repl: BackendVarTransform::VariableReplacements, mut func: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, mut replaceVariables: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    pub type PredicateFunction = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let mut odae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    odae = (::match_deref::match_deref! { match &(idlow.clone()) {
        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst @ BackendDAE::EqSystem { orderedEqs, orderedVars, .. }, tail: systList }, derivativeNamePrefix: shared, .. } => {
            let mut syst = (*syst).clone();
            let mut orderedEqs = (*orderedEqs).clone();
            let mut orderedVars = (*orderedVars).clone();
            orderedVars = BackendVariable::listVar1(replaceVars(BackendVariable::varList(orderedVars.clone())?, repl.clone(), func.clone(), replaceVariables.clone())?);
            (orderedEqs, _) = BackendVarTransform::replaceEquationsArr(orderedEqs.clone(), repl.clone(), None)?;
            syst = BackendDAEUtil::setEqSystVars(syst.clone(), orderedVars.clone())?;
            syst = BackendDAEUtil::setEqSystEqs(syst.clone(), orderedEqs.clone());
            BackendDAE::DAE(cons(syst.clone(), systList.clone()), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(odae)
}

fn replaceVars(mut invarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut repl: BackendVarTransform::VariableReplacements, mut func: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, mut replaceName: bool) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    pub type PredicateFunction = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = (::match_deref::match_deref! { match &((invarLst.clone(), repl.clone(), func.clone(), replaceName.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: varLst }, _, _, true) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut bindExp: Option<Arc<DAE::Exp>> = None;
            let mut v = (*v).clone();
            let mut varLst = (*varLst).clone();
            cr = BackendVariable::varCref(v.clone())?;
            bindExp = varBindingOpt(v.clone())?;
            bindExp = replaceExpOpt(bindExp.clone(), repl.clone(), func.clone())?;
            bindExp = applyOptionSimplify(bindExp.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_REAL_DEFAULT().clone() }), repl.clone(), func.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            v = setVarCref(v.clone(), cr.clone());
            v = setVarBindingOpt(v.clone(), bindExp.clone())?;
            varLst = replaceVars(varLst.clone(), repl.clone(), func.clone(), replaceName.clone())?;
            cons(v.clone(), varLst.clone())
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: varLst }, _, _, false) => {
            let mut bindExp: Option<Arc<DAE::Exp>> = None;
            let mut v = (*v).clone();
            let mut varLst = (*varLst).clone();
            bindExp = varBindingOpt(v.clone())?;
            bindExp = replaceExpOpt(bindExp.clone(), repl.clone(), func.clone())?;
            bindExp = applyOptionSimplify(bindExp.clone())?;
            v = setVarBindingOpt(v.clone(), bindExp.clone())?;
            varLst = replaceVars(varLst.clone(), repl.clone(), func.clone(), replaceName.clone())?;
            cons(v.clone(), varLst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVarLst)
}

pub fn varBindingOpt(mut v: BackendDAE::Var) -> Result<Option<Arc<DAE::Exp>>> {
    let mut exp: Option<Arc<DAE::Exp>> = None;
    exp = (match v.clone() {
        BackendDAE::Var { bindExp: mut exp, .. } => exp.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(exp)
}

pub fn replaceExpOpt(mut inExp: Option<Arc<DAE::Exp>>, mut repl: BackendVarTransform::VariableReplacements, mut funcOpt: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>) -> Result<Option<Arc<DAE::Exp>>> {
    pub type FuncTypeExp_ExpToBoolean = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExp: Option<Arc<DAE::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), repl.clone(), funcOpt.clone())) {
        (None, _, _) => {
            None
        },
        (Some(e), _, _) => {
            let mut e = (*e).clone();
            (e, _) = BackendVarTransform::replaceExp(e.clone(), repl.clone(), funcOpt.clone())?;
            Some(e.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn applyOptionSimplify(mut bindExpIn: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut bindExpOut: Option<Arc<DAE::Exp>> = None;
    bindExpOut = (::match_deref::match_deref! { match &(bindExpIn.clone()) {
        None => {
            None
        },
        Some(e) => {
            let mut e1: Arc<DAE::Exp>;
            (e1, _) = ExpressionSimplify::simplify1(e.clone())?;
            Some(e1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bindExpOut)
}

pub fn setVarCref(mut inVar: BackendDAE::Var, mut cr: Arc<DAE::ComponentRef>) -> BackendDAE::Var {
    let mut outVar: BackendDAE::Var = inVar.clone();
    outVar.varName = cr.clone();
    outVar.unreplaceable = false;
    outVar
}

pub fn setVarBindingOpt(mut inVar: BackendDAE::Var, mut bindExp: Option<Arc<DAE::Exp>>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut kind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    let mut dir: DAE::VarDirection = DAE::VarDirection::BIDIR;
    let mut prl: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut bind: Option<Arc<DAE::Exp>> = None;
    let mut tplExp: Option<Arc<DAE::Exp>> = None;
    let mut ad: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    let mut ts: Option<BackendDAE::TearingSelect> = None;
    let mut hideResult: Option<Arc<DAE::Exp>> = None;
    let mut cmt: Option<Arc<SCode::Comment>> = None;
    let mut ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    let mut innerOuter: DAE::VarInnerOuter = DAE::VarInnerOuter::INNER;
    let mut encrypted: bool = false;
    let BackendDAE::VAR { varName: __pa0, varKind: __pa1, varDirection: __pa2, varParallelism: __pa3, varType: __pa4, bindExp: __pa5, tplExp: __pa6, arryDim: __pa7, source: __pa8, values: __pa9, tearingSelectOption: __pa10, hideResult: __pa11, comment: __pa12, connectorType: __pa13, innerOuter: __pa14, unreplaceable: _, initNonlinear: _, encrypted: __pa15 } = (inVar.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    kind = __pa1.clone();
    dir = __pa2.clone();
    prl = __pa3.clone();
    tp = __pa4.clone();
    bind = __pa5.clone();
    tplExp = __pa6.clone();
    ad = __pa7.clone();
    source = __pa8.clone();
    attr = __pa9.clone();
    ts = __pa10.clone();
    hideResult = __pa11.clone();
    cmt = __pa12.clone();
    ct = __pa13.clone();
    innerOuter = __pa14.clone();
    encrypted = __pa15.clone();
    outVar = BackendDAE::Var { varName: name.clone(), varKind: kind.clone(), varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bindExp.clone(), tplExp: tplExp.clone(), arryDim: ad.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: cmt.clone(), connectorType: ct.clone(), innerOuter: innerOuter.clone(), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() };
    Ok(outVar)
}

pub fn moveVariables(mut inVariables1: BackendDAE::Variables, mut inVariables2: BackendDAE::Variables, mut hashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(BackendDAE::Variables, BackendDAE::Variables)> {
    let mut outVariables1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outVariables2: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut lst1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut lst2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut lst1_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut lst2_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut v1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut v2: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    lst1 = BackendVariable::varList(inVariables1.clone())?;
    lst2 = BackendVariable::varList(inVariables2.clone())?;
    (lst1_1, lst2_1) = moveVariables2(lst1.clone(), lst2.clone(), hashTable.clone())?;
    v1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    v2 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    outVariables1 = BackendVariable::addVars(lst1_1.clone(), v1.clone());
    outVariables2 = BackendVariable::addVars(lst2_1.clone(), v2.clone());
    Ok((outVariables1, outVariables2))
}

fn moveVariables2(mut inVarLst1: Arc<metamodelica::List<BackendDAE::Var>>, mut inVarLst2: Arc<metamodelica::List<BackendDAE::Var>>, mut hashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outVarLst1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outVarLst2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    (outVarLst1, outVarLst2) = (::match_deref::match_deref! { match &((inVarLst1.clone(), inVarLst2.clone(), hashTable.clone())) {
        (Deref @ metamodelica::List::Nil, globalKnownVars, _) => {
            (metamodelica::nil(), globalKnownVars.clone())
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { varName: cr, .. }, tail: vs }, globalKnownVars, mvars) => {
            let mut vs_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut knvars_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            if BaseHashTable::hasKey(cr.clone(), mvars.clone()) {
                (vs_1, knvars_1) = moveVariables2(vs.clone(), globalKnownVars.clone(), mvars.clone())?;
                knvars_1 = cons(v.clone(), knvars_1.clone());
            } else {
                (vs_1, knvars_1) = moveVariables2(vs.clone(), globalKnownVars.clone(), mvars.clone())?;
                vs_1 = cons(v.clone(), vs_1.clone());
            }
            (vs_1.clone(), knvars_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outVarLst1, outVarLst2))
}

pub fn sortBy1<ElementType: Clone + 'static, ArgType1: Clone + 'static>(mut inList: Arc<metamodelica::List<ElementType>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32> + 'static>, mut inArgument1: ArgType1) -> Result<Arc<metamodelica::List<ElementType>>> {
    pub type CompareFunc<ElementType: Clone + 'static, ArgType1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32> + 'static>;

    let mut outList: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } => {
            list![e.clone()]
        },
        _ => {
            let mut left: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
            let mut right: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
            let mut middle: i32 = 0;
            middle = intDiv((inList.clone().len() as i32), 2);
            (left, right) = List::split(inList.clone(), middle.clone())?;
            left = sortBy1(left.clone(), inCompFunc.clone(), inArgument1.clone())?;
            right = sortBy1(right.clone(), inCompFunc.clone(), inArgument1.clone())?;
            mergeBy1(left.clone(), right.clone(), inCompFunc.clone(), inArgument1.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outList)
}

fn mergeBy1<ElementType: Clone + 'static, ArgType1: Clone + 'static>(mut inLeft: Arc<metamodelica::List<ElementType>>, mut inRight: Arc<metamodelica::List<ElementType>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32> + 'static>, mut inArgument1: ArgType1) -> Result<Arc<metamodelica::List<ElementType>>> {
    pub type CompareFunc<ElementType: Clone + 'static, ArgType1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32> + 'static>;

    let mut outList: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = (inLeft.clone(), inRight.clone(), inCompFunc.clone(), inArgument1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: l, tail: l_rest }, Deref @ metamodelica::List::Cons { head: r, tail: _ }, _, _) => {
                    let mut res: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
                    let mut ri: i32 = 0;
                    let mut li: i32 = 0;
                    ri = inCompFunc(r.clone(), inArgument1.clone())?;
                    li = inCompFunc(l.clone(), inArgument1.clone())?;
                    let true = (intGt(ri.clone(), li.clone())) else { bail!("pattern mismatch") };
                    res = mergeBy1(l_rest.clone(), inRight.clone(), inCompFunc.clone(), inArgument1.clone())?;
                    Ok(cons(l.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: r, tail: r_rest }, _, _) => {
                    let mut res: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
                    res = mergeBy1(inLeft.clone(), r_rest.clone(), inCompFunc.clone(), inArgument1.clone())?;
                    Ok(cons(r.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(inRight.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(inLeft.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

fn removeSimpleEquationsUC(mut daeIn: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut daeOut: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    daeOut = (::match_deref::match_deref! { match &(daeIn.clone()) {
        dae @ Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, tail: _ }, derivativeNamePrefix: BackendDAE::Shared { globalKnownVars, .. }, .. } => {
            let mut sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            let mut other_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut simple_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut set_solutions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut removed_vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut removed_vars_table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut dae = (*dae).clone();
            let mut vars = (*vars).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            removed_vars_table = HashTable::emptyHashTable();
            (sets, other_eqns) = separateAliasSetsAndEquations(BackendEquation::equationList(eqns.clone()), metamodelica::nil(), metamodelica::nil())?;
            set_solutions = List::map2(sets.clone(), (std::sync::Arc::new(solveAliasSet) as std::sync::Arc<dyn ::std::ops::Fn(AliasSet, BackendDAE::Variables, BackendDAE::Variables) -> Result<Arc<DAE::ComponentRef>> + 'static>), vars.clone(), globalKnownVars.clone());
            (repl, simple_eqns, removed_vars) = createReplacementsAndEquations(set_solutions.clone(), sets.clone(), vars.clone(), globalKnownVars.clone(), repl.clone(), metamodelica::nil(), metamodelica::nil())?;
            (other_eqns, _) = BackendVarTransform::replaceEquations(other_eqns.clone(), repl.clone(), None)?;
            removed_vars_table = addCrefsToHashTable(removed_vars.clone(), removed_vars_table.clone())?;
            (vars, globalKnownVars) = moveVariables(vars.clone(), globalKnownVars.clone(), removed_vars_table.clone())?;
            dae = setDaeVars(dae.clone(), vars.clone())?;
            dae = BackendDAEUtil::setDAEGlobalKnownVars(dae.clone(), globalKnownVars.clone())?;
            dae = setDaeEqns(dae.clone(), BackendEquation::listEquation(listAppend(simple_eqns.clone(), other_eqns.clone()))?, false)?;
            dae = BackendDAEUtil::transformBackendDAE(dae.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED)), None, None)?;
            dae = BackendDAEUtil::mapEqSystem1(dae.clone(), (std::sync::Arc::new(BackendDAEUtil::getAdjacencyMatrixfromOptionForMapEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendDAE::IndexType, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>), crate::BackendDAE::IndexType::NORMAL)?;
            dae.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(daeOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addCrefsToHashTable(mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))> {
    let mut out: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    out = (::match_deref::match_deref! { match &((crefs.clone(), table.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            table.clone()
        },
        (Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
            let mut new_table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            new_table = BaseHashTable::add((h.clone(), 0), table.clone())?;
            new_table = addCrefsToHashTable(t.clone(), new_table.clone())?;
            new_table.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn getAllVariablesForCref(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (cr.clone(), vars.clone(), globalKnownVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut out: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (out, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok(out.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut out: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (out, _) = BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?;
                    Ok(out.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

fn rateVariable(mut var: BackendDAE::Var) -> Result<metamodelica::Real> {
    let mut out: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut acc: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    acc = metamodelica::OrderedFloat(0.0_f64);
    let BackendDAE::VAR { varName: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
    cr = __pa0.clone();
    i = metamodelica::OrderedFloat(1.0_f64) / (metamodelica::OrderedFloat(1.0_f64) + intReal(ComponentReference::crefDepth(cr.clone())?));
    acc = acc.clone() + i.clone();
    i = if (BackendVariable::isParam(var.clone())) {metamodelica::OrderedFloat(3.0_f64)} else {metamodelica::OrderedFloat(0.0_f64)};
    acc = acc.clone() + i.clone();
    i = if (BackendVariable::isStateVar(var.clone())) {metamodelica::OrderedFloat(5.0_f64)} else {metamodelica::OrderedFloat(0.0_f64)};
    acc = acc.clone() + i.clone();
    i = if (BackendVariable::varHasUncertainValueRefine(var.clone())) {metamodelica::OrderedFloat(7.0_f64)} else {metamodelica::OrderedFloat(0.0_f64)};
    acc = acc.clone() + i.clone();
    out = acc.clone();
    Ok(out)
}

fn rateVariableList(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<metamodelica::Real> {
    let mut out: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    out = (::match_deref::match_deref! { match &(vars.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::OrderedFloat(0.0_f64)
        },
        Deref @ metamodelica::List::Cons { head: h, tail: t } => {
            let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r1 = rateVariable(h.clone())?;
            r2 = rateVariableList(t.clone())?;
            r = if (realGt(r1.clone(), r2.clone())) {r1.clone()} else {r2.clone()};
            r.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn rateSetElement(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<(Arc<DAE::ComponentRef>, metamodelica::Real)> {
    let mut out: (Arc<DAE::ComponentRef>, metamodelica::Real);
    let mut var: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    var = getAllVariablesForCref(cr.clone(), vars.clone(), globalKnownVars.clone())?;
    out = (cr.clone(), rateVariableList(var.clone())?);
    Ok(out)
}

fn setPairSortFunction(mut a: (Arc<DAE::ComponentRef>, metamodelica::Real), mut b: (Arc<DAE::ComponentRef>, metamodelica::Real)) -> bool {
    let mut out: bool = false;
    let mut av: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut bv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (_, av) = a.clone();
    (_, bv) = b.clone();
    out = realLt(av.clone(), bv.clone());
    out
}

fn solveAliasSet(mut set: AliasSet, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<Arc<DAE::ComponentRef>> {
    let mut out: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut names: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut name_rate_list: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, metamodelica::Real)>> = metamodelica::nil();
    names = getAliasSetSymbolList(set.clone())?;
    name_rate_list = List::map2(names.clone(), (std::sync::Arc::new(rateSetElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, BackendDAE::Variables, BackendDAE::Variables) -> Result<(Arc<DAE::ComponentRef>, metamodelica::Real)> + 'static>), vars.clone(), globalKnownVars.clone());
    name_rate_list = List::sort(name_rate_list.clone(), (std::sync::Arc::new(fnptr!(setPairSortFunction, (Arc<DAE::ComponentRef>, metamodelica::Real), (Arc<DAE::ComponentRef>, metamodelica::Real))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, metamodelica::Real), (Arc<DAE::ComponentRef>, metamodelica::Real)) -> Result<bool> + 'static>))?;
    let __pa0 = ::match_deref::match_deref! { match &(name_rate_list.clone()) {
        Deref @ metamodelica::List::Cons { head: (__pa0, _), tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    out = __pa0.clone();
    Ok(out)
}

fn isRemovableVar(mut var: BackendDAE::Var) -> bool {
    let mut out: bool = false;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    out = !(BackendVariable::isStateVar(var.clone())) && !(BackendVariable::varHasUncertainValueRefine(var.clone()));
    out
}

fn isRemovableVarList(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> bool {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(vars.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: h, tail: t } => {
            let mut r1: bool = false;
            let mut r2: bool = false;
            let mut r: bool = false;
            r1 = isRemovableVar(h.clone());
            r2 = isRemovableVarList(t.clone());
            r = r1.clone() && r2.clone();
            r.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn isRemovableSymbol(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<bool> {
    let mut out: bool = false;
    let mut var: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    var = getAllVariablesForCref(cr.clone(), vars.clone(), globalKnownVars.clone())?;
    out = isRemovableVarList(var.clone());
    Ok(out)
}

fn fixSingOfExp(mut sign: i32, mut eIn: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut out: Arc<DAE::Exp>;
    out = (::match_deref::match_deref! { match &((sign.clone(), eIn.clone())) {
        ((-1), _) => {
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            tp = Expression::r#typeof(eIn.clone())?;
            Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: eIn.clone() })
        },
        (_, _) => {
            eIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn generateEquation(mut cr: Arc<DAE::ComponentRef>, mut e: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>) -> Arc<BackendDAE::Equation> {
    let mut out: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    out = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: e.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
    out
}

fn createReplacementsAndEquationsForSet(mut solution: Arc<DAE::ComponentRef>, mut symbols: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut set: AliasSet, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut repl_acc: BackendVarTransform::VariableReplacements, mut eqns_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut removed_vars_acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut eqnsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut removed_varsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (replOut, eqnsOut, removed_varsOut) = 'mc: {
        let __mc_input = (solution.clone(), symbols.clone(), set.clone(), vars.clone(), globalKnownVars.clone(), repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
                    Ok((repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: h, tail: t }, _, _, _, _, _, _) => {
                    let mut new_removed_vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut new_repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut new_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let true = (ComponentReferenceBasics::crefEqual(solution.clone(), h.clone())?) else { bail!("pattern mismatch") };
                    (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution.clone(), t.clone(), set.clone(), vars.clone(), globalKnownVars.clone(), repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone())?;
                    Ok((new_repl.clone(), new_eqns.clone(), new_removed_vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: h, tail: t }, _, _, _, _, _, _) => {
                    let mut new_removed_vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut sign1: i32 = 0;
                    let mut sign2: i32 = 0;
                    let mut sign: i32 = 0;
                    let mut new_repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut new_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let true = (isRemovableSymbol(h.clone(), vars.clone(), globalKnownVars.clone())?) else { bail!("pattern mismatch") };
                    (sign1, e) = getAliasSetExpressionAndSign(solution.clone(), set.clone())?;
                    (sign2, _) = getAliasSetExpressionAndSign(h.clone(), set.clone())?;
                    sign = if (sign2.clone() < 0) {-(sign1.clone())} else {sign1.clone()};
                    e = fixSingOfExp(sign.clone(), e.clone())?;
                    new_repl = BackendVarTransform::addReplacement(repl_acc.clone(), h.clone(), e.clone(), None)?;
                    new_removed_vars = cons(h.clone(), removed_vars_acc.clone());
                    (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution.clone(), t.clone(), set.clone(), vars.clone(), globalKnownVars.clone(), new_repl.clone(), eqns_acc.clone(), new_removed_vars.clone())?;
                    Ok((new_repl.clone(), new_eqns.clone(), new_removed_vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: h, tail: t }, _, _, _, _, _, _) => {
                    let mut new_removed_vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut sign1: i32 = 0;
                    let mut sign2: i32 = 0;
                    let mut sign: i32 = 0;
                    let mut new_repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut new_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let false = (isRemovableSymbol(h.clone(), vars.clone(), globalKnownVars.clone())?) else { bail!("pattern mismatch") };
                    (sign1, e) = getAliasSetExpressionAndSign(solution.clone(), set.clone())?;
                    (sign2, _) = getAliasSetExpressionAndSign(h.clone(), set.clone())?;
                    sign = if (sign2.clone() < 0) {-(sign1.clone())} else {sign1.clone()};
                    e = fixSingOfExp(sign.clone(), e.clone())?;
                    source = getAliasSetSource(set.clone())?;
                    eqn = generateEquation(h.clone(), e.clone(), source.clone());
                    new_eqns = cons(eqn.clone(), eqns_acc.clone());
                    (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution.clone(), t.clone(), set.clone(), vars.clone(), globalKnownVars.clone(), repl_acc.clone(), new_eqns.clone(), removed_vars_acc.clone())?;
                    Ok((new_repl.clone(), new_eqns.clone(), new_removed_vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((replOut, eqnsOut, removed_varsOut))
}

fn createReplacementsAndEquations(mut solutions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut sets: Arc<metamodelica::List<AliasSet>>, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut repl_acc: BackendVarTransform::VariableReplacements, mut eqns_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut removed_vars_acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut eqnsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut removed_vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (replOut, eqnsOut, removed_vars) = (::match_deref::match_deref! { match &((solutions.clone(), sets.clone(), vars.clone(), globalKnownVars.clone(), repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            (repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone())
        },
        (Deref @ metamodelica::List::Cons { head: solution, tail: solt }, Deref @ metamodelica::List::Cons { head: set, tail: sett }, _, _, _, _, _) => {
            let mut symbols: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut new_removed_vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut new_repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut new_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            symbols = getAliasSetSymbolList(set.clone())?;
            (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution.clone(), symbols.clone(), set.clone(), vars.clone(), globalKnownVars.clone(), repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone())?;
            (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquations(solt.clone(), sett.clone(), vars.clone(), globalKnownVars.clone(), new_repl.clone(), new_eqns.clone(), new_removed_vars.clone())?;
            (new_repl.clone(), new_eqns.clone(), new_removed_vars.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((replOut, eqnsOut, removed_vars))
}

fn separateAliasSetsAndEquations(mut eqnIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut sets: Arc<metamodelica::List<AliasSet>>, mut eqn_accIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<AliasSet>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut setsOut: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
    let mut eqn_accOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (setsOut, eqn_accOut) = (::match_deref::match_deref! { match &((eqnIn.clone(), sets.clone(), eqn_accIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            let mut eqn_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            eqn_acc = eqn_accIn.clone().reverse();
            (sets.clone(), eqn_acc.clone())
        },
        (Deref @ metamodelica::List::Cons { head: eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, tail: t }, _, _) => {
            let mut eqn_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            (new_sets, eqn_acc) = addPairToSet(sets.clone(), eqn_accIn.clone(), eqn.clone(), e1.clone(), e2.clone())?;
            (new_sets, eqn_acc) = separateAliasSetsAndEquations(t.clone(), new_sets.clone(), eqn_acc.clone())?;
            (new_sets.clone(), eqn_acc.clone())
        },
        (Deref @ metamodelica::List::Cons { head: eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, tail: t }, _, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut eqn_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            e1 = Expression::crefExp(cr.clone())?;
            (new_sets, eqn_acc) = addPairToSet(sets.clone(), eqn_accIn.clone(), eqn.clone(), e1.clone(), e2.clone())?;
            (new_sets, eqn_acc) = separateAliasSetsAndEquations(t.clone(), new_sets.clone(), eqn_acc.clone())?;
            (new_sets.clone(), eqn_acc.clone())
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: t }, _, _) => {
            let mut eqn_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            (new_sets, eqn_acc) = separateAliasSetsAndEquations(t.clone(), sets.clone(), cons(eqn.clone(), eqn_accIn.clone()))?;
            (new_sets.clone(), eqn_acc.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((setsOut, eqn_accOut))
}

fn addPairToSet(mut sets: Arc<metamodelica::List<AliasSet>>, mut eqn_acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut eqn: Arc<BackendDAE::Equation>, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<AliasSet>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut out: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
    let mut eqn_acc_out: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (out, eqn_acc_out) = (::match_deref::match_deref! { match &((sets.clone(), eqn_acc.clone(), eqn.clone(), lhs.clone(), rhs.clone())) {
        (_, _, _, e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            let mut source: Option<Arc<DAE::ElementSource>> = None;
            source = getSourceIfApproximated(eqn.clone())?;
            new_sets = pushToSetList(sets.clone(), cr1.clone(), e1.clone(), 1, cr2.clone(), e2.clone(), 1, source.clone())?;
            (new_sets.clone(), eqn_acc.clone())
        },
        (_, _, _, e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            let mut source: Option<Arc<DAE::ElementSource>> = None;
            source = getSourceIfApproximated(eqn.clone())?;
            new_sets = pushToSetList(sets.clone(), cr1.clone(), e1.clone(), 1, cr2.clone(), e2.clone(), -1, source.clone())?;
            (new_sets.clone(), eqn_acc.clone())
        },
        (_, _, _, e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            let mut source: Option<Arc<DAE::ElementSource>> = None;
            source = getSourceIfApproximated(eqn.clone())?;
            new_sets = pushToSetList(sets.clone(), cr1.clone(), e1.clone(), -1, cr2.clone(), e2.clone(), 1, source.clone())?;
            (new_sets.clone(), eqn_acc.clone())
        },
        (_, _, _, e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, e2 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            let mut new_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
            let mut source: Option<Arc<DAE::ElementSource>> = None;
            source = getSourceIfApproximated(eqn.clone())?;
            new_sets = pushToSetList(sets.clone(), cr1.clone(), e1.clone(), -1, cr2.clone(), e2.clone(), -1, source.clone())?;
            (new_sets.clone(), eqn_acc.clone())
        },
        _ => {
            (sets.clone(), cons(eqn.clone(), eqn_acc.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out, eqn_acc_out))
}

fn getSourceIfApproximated(mut eqn: Arc<BackendDAE::Equation>) -> Result<Option<Arc<DAE::ElementSource>>> {
    let mut source: Option<Arc<DAE::ElementSource>> = None;
    let mut temp: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    temp = BackendEquation::equationSource(eqn.clone())?;
    source = if (isApproximatedEquation(eqn.clone())?) {Some(temp.clone())} else {None};
    Ok(source)
}

/*     Set handling functions    */
fn createSet(mut cr1: Arc<DAE::ComponentRef>, mut e1: Arc<DAE::Exp>, mut sign1In: i32, mut cr2: Arc<DAE::ComponentRef>, mut e2: Arc<DAE::Exp>, mut sign2In: i32, mut source: Option<Arc<DAE::ElementSource>>) -> Result<AliasSet> {
    let mut setOut: AliasSet = <AliasSet as ::std::default::Default>::default();
    setOut = (::match_deref::match_deref! { match &((cr1.clone(), e1.clone(), sign1In.clone(), cr2.clone(), e2.clone(), sign2In.clone(), source.clone())) {
        (_, _, sign1, _, _, sign2, _) => {
            let mut new_symbols: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut new_signs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut new_expl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            new_signs = HashTable::emptyHashTable();
            new_symbols = HashSet::emptyHashSet();
            new_expl = HashTable2::emptyHashTable();
            new_signs = BaseHashTable::add((cr1.clone(), sign1.clone()), new_signs.clone())?;
            new_signs = BaseHashTable::add((cr2.clone(), sign2.clone()), new_signs.clone())?;
            new_symbols = BaseHashSet::add(cr1.clone(), new_symbols.clone())?;
            new_symbols = BaseHashSet::add(cr2.clone(), new_symbols.clone())?;
            new_expl = BaseHashTable::add((cr1.clone(), e1.clone()), new_expl.clone())?;
            new_expl = BaseHashTable::add((cr2.clone(), e2.clone()), new_expl.clone())?;
            AliasSet { symbols: new_symbols.clone(), expl: new_expl.clone(), signs: new_signs.clone(), source: source.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(setOut)
}

fn addToSet(mut set: AliasSet, mut cr1: Arc<DAE::ComponentRef>, mut e1: Arc<DAE::Exp>, mut sign1In: i32, mut cr2: Arc<DAE::ComponentRef>, mut e2: Arc<DAE::Exp>, mut sign2In: i32, mut sourceIn: Option<Arc<DAE::ElementSource>>) -> Result<AliasSet> {
    let mut setOut: AliasSet = <AliasSet as ::std::default::Default>::default();
    setOut = (::match_deref::match_deref! { match &((set.clone(), cr1.clone(), e1.clone(), sign1In.clone(), cr2.clone(), e2.clone(), sign2In.clone(), sourceIn.clone())) {
        (AliasSet { symbols, expl, signs, source: source_current }, _, _, sign1, _, _, sign2, _) => {
            let mut current_sign: i32 = 0;
            let mut sign1_temp: i32 = 0;
            let mut new_symbols: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut new_signs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut new_expl: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut source_new: Option<Arc<DAE::ElementSource>> = None;
            let mut sign1 = (*sign1).clone();
            let mut sign2 = (*sign2).clone();
            current_sign = BaseHashTable::get(cr1.clone(), signs.clone())?;
            sign1_temp = sign1.clone();
            sign1 = if (intEq(sign1_temp.clone(), current_sign.clone())) {sign1.clone()} else {-(sign1.clone())};
            sign2 = if (intEq(sign1_temp.clone(), current_sign.clone())) {sign2.clone()} else {-(sign2.clone())};
            new_signs = BaseHashTable::add((cr2.clone(), sign2.clone()), signs.clone())?;
            new_symbols = BaseHashSet::add(cr2.clone(), symbols.clone())?;
            new_expl = BaseHashTable::add((cr2.clone(), e2.clone()), expl.clone())?;
            source_new = updateSource(source_current.clone(), sourceIn.clone())?;
            AliasSet { symbols: new_symbols.clone(), expl: new_expl.clone(), signs: new_signs.clone(), source: source_new.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(setOut)
}

fn updateSource(mut source1: Option<Arc<DAE::ElementSource>>, mut source2: Option<Arc<DAE::ElementSource>>) -> Result<Option<Arc<DAE::ElementSource>>> {
    let mut sourceOut: Option<Arc<DAE::ElementSource>> = None;
    sourceOut = (::match_deref::match_deref! { match &((source1.clone(), source2.clone())) {
        (None, None) => {
            None
        },
        (Some(s), None) => {
            Some(s.clone())
        },
        (None, Some(s)) => {
            Some(s.clone())
        },
        (Some(s), Some(_)) => {
            Some(s.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sourceOut)
}

fn existsInSet(mut set: AliasSet, mut cr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &((set.clone(), cr.clone())) {
        (AliasSet { symbols, expl: _, signs: _, source: _ }, _) => {
            let mut ret: bool = false;
            ret = BaseHashSet::has(cr.clone(), symbols.clone())?;
            ret.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn pushToSetList(mut sets: Arc<metamodelica::List<AliasSet>>, mut cr1: Arc<DAE::ComponentRef>, mut e1: Arc<DAE::Exp>, mut sign1: i32, mut cr2: Arc<DAE::ComponentRef>, mut e2: Arc<DAE::Exp>, mut sign2: i32, mut source: Option<Arc<DAE::ElementSource>>) -> Result<Arc<metamodelica::List<AliasSet>>> {
    let mut setsOut: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
    setsOut = 'mc: {
        let __mc_input = (sets.clone(), cr1.clone(), e1.clone(), sign1.clone(), cr2.clone(), e2.clone(), sign2.clone(), source.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    let mut new_set: AliasSet = <AliasSet as ::std::default::Default>::default();
                    new_set = createSet(cr1.clone(), e1.clone(), sign1.clone(), cr2.clone(), e2.clone(), sign2.clone(), source.clone())?;
                    Ok(list![new_set.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, _, _, _, _, _, _, _) => {
                    let mut new_set: AliasSet = <AliasSet as ::std::default::Default>::default();
                    let true = (existsInSet(h.clone(), cr1.clone())?) else { bail!("pattern mismatch") };
                    new_set = addToSet(h.clone(), cr1.clone(), e1.clone(), sign1.clone(), cr2.clone(), e2.clone(), sign2.clone(), source.clone())?;
                    Ok(cons(new_set.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, _, _, _, _, _, _, _) => {
                    let mut new_set: AliasSet = <AliasSet as ::std::default::Default>::default();
                    let true = (existsInSet(h.clone(), cr2.clone())?) else { bail!("pattern mismatch") };
                    new_set = addToSet(h.clone(), cr2.clone(), e2.clone(), sign2.clone(), cr1.clone(), e1.clone(), sign1.clone(), source.clone())?;
                    Ok(cons(new_set.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, _, _, _, _, _, _, _) => {
                    let mut inner_sets: Arc<metamodelica::List<AliasSet>> = metamodelica::nil();
                    inner_sets = pushToSetList(t.clone(), cr1.clone(), e1.clone(), sign1.clone(), cr2.clone(), e2.clone(), sign2.clone(), source.clone())?;
                    Ok(cons(h.clone(), inner_sets.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(setsOut)
}

fn getAliasSetSymbolList(mut set: AliasSet) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut out: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    out = (match set.clone() {
        AliasSet { symbols: mut symbols, expl: _, signs: _, source: _ } => {
            let mut crl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crl = BaseHashSet::hashSetList(symbols.clone())?;
            crl.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(out)
}

fn getAliasSetSource(mut set: AliasSet) -> Result<Arc<DAE::ElementSource>> {
    let mut out: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    out = (::match_deref::match_deref! { match &(set.clone()) {
        AliasSet { symbols: _, expl: _, signs: _, source: Some(source) } => {
            source.clone()
        },
        AliasSet { symbols: _, expl: _, signs: _, source: None } => {
            DAE::emptyElementSource().clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn getAliasSetExpressionAndSign(mut cr: Arc<DAE::ComponentRef>, mut set: AliasSet) -> Result<(i32, Arc<DAE::Exp>)> {
    let mut signOut: i32 = 0;
    let mut eOut: Arc<DAE::Exp>;
    (signOut, eOut) = (::match_deref::match_deref! { match &((cr.clone(), set.clone())) {
        (_, AliasSet { symbols: _, expl, signs, source: _ }) => {
            let mut sign: i32 = 0;
            let mut e: Arc<DAE::Exp>;
            sign = BaseHashTable::get(cr.clone(), signs.clone())?;
            e = BaseHashTable::get(cr.clone(), expl.clone())?;
            (sign.clone(), e.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((signOut, eOut))
}

fn dumpAliasSets(mut sets: Arc<metamodelica::List<AliasSet>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(sets.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: AliasSet { symbols, expl: _, signs, source }, tail: t } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut sign_values: Arc<metamodelica::List<i32>> = metamodelica::nil();
            crefs = BaseHashSet::hashSetList(symbols.clone())?;
            sign_values = List::map1(crefs.clone(), (std::sync::Arc::new(BaseHashTable::get) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), signs.clone());
            dumpAliasSets2(crefs.clone(), sign_values.clone())?;
            dumpAliasSets3(source.clone())?;
            println!("{}", (literal!("\n")).clone());
            dumpAliasSets(t.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpAliasSets2(mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut sign_values: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((crefs.clone(), sign_values.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: cr_t }, Deref @ metamodelica::List::Cons { head: i, tail: i_t }) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (if (i.clone() > 0) {literal!("+")} else {literal!("-")}).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) }).clone());
            dumpAliasSets2(cr_t.clone(), i_t.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpAliasSets3(mut sourceIn: Option<Arc<DAE::ElementSource>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(sourceIn.clone()) {
        None => {
            println!("{}", (literal!(" *Approximated = false")).clone());
            ()
        },
        Some(Deref @ DAE::ElementSource { comment, .. }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (boolString(isApproximatedEquation2(comment.clone())?)).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" *Approximated = ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

