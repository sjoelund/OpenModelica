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

use crate::BackendDAE;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Matching;
use crate::SimCode;
use crate::Sorting;
use crate::SymbolTable;
use crate::SymbolicJacobian;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type ExtAdjacencyMatrixRow = (i32, Arc<metamodelica::List<i32>>);

pub type ExtAdjacencyMatrix = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

pub const UNDERLINE: &'static str = "==========================================================================";

pub fn newExtractionAlgorithm(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outOtherEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outResidualEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut setC_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut setS_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut residualEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut complexEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut swappedEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut varCount: i32 = 0;
    let mut eqCount: i32 = 0;
    let mut ebltEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bindingEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut residualVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut unMeasuredVariables: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut simCodeJacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut modelicaOutput: ArcStr = arcstr::literal!("");
    let mut modelicaFileName: ArcStr = arcstr::literal!("");
    let mut modelName: ArcStr = arcstr::literal!("");
    let mut auxillaryConditionsFilename: ArcStr = arcstr::literal!("");
    let mut auxillaryEquations: ArcStr = arcstr::literal!("");
    let mut intermediateEquationsFilename: ArcStr = arcstr::literal!("");
    let mut intermediateEquations: ArcStr = arcstr::literal!("");
    let mut csvfileName: ArcStr = arcstr::literal!("");
    let mut mappedEbltSetS: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut allVarsList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exactEquationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut extractedVarsfromSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionTaggedEquationSolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMeasuredVariablesOfInterest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inputVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outDiffVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outOtherVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outResidualVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut procedureCount: i32 = 0;
    let mut measurementcsvData: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut debug: bool = false;
    let mut status: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    currentSystem = __pa0.clone();
    shared = inDAE.shared.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nModelInfo: ")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem.clone(), inDAE.shared.clone(), debug.clone())?;
    procedureCount = 1;
    setBFailedBoundaryConditionEquations = metamodelica::nil();
    while !(status.clone()) {
        BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("OrderedVariables")).clone())?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("OrderedEquation")).clone())?;
        allVarsList = List::intRange(BackendVariable::varsSize(currentSystem.orderedVars.clone())?);
        varCount = currentSystem.orderedVars.numberOfVars.clone();
        eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
        (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
        (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount.clone(), eqCount.clone())?;
        BackendDump::dumpMatching(match1.clone())?;
        (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
        bindingEquations = getBindingEquation(currentSystem.clone(), mapIncRowEqn.clone())?;
        bindingEquations = List::flatten(List::map1r(bindingEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(BackendEquation::equationList(currentSystem.orderedEqs.clone()), 1)?;
        if debug.clone() {
            BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("ApproximatedEquations")).clone());
            BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundaryConditionEquations")).clone());
        }
        approximatedEquations = List::flatten(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        boundaryConditionEquations = List::flatten(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        boundaryConditionTaggedEquationSolvedVars = getBoundaryConditionVariables(boundaryConditionEquations.clone(), solvedEqsAndVarsInfo.clone());
        if debug.clone() {
            println!("{}", (literal!("\nApproximated and BoundaryCondition Equation Indexes :\n===========================================")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :")); __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("\n")).clone());
        }
        (knowns, boundaryConditionVars, exactEquationVars, unMeasuredVariablesOfInterest) = getVariablesBlockCategories(currentSystem.orderedVars.clone(), allVarsList.clone())?;
        boundaryConditionVars = listAppend(boundaryConditionVars.clone(), boundaryConditionTaggedEquationSolvedVars.clone());
        if debug.clone() {
            println!("{}", (literal!("\nVariablesCategories\n=============================")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nknownVars                    :")); __mm_s.push_str(&*dumplistInteger(knowns.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nboundaryConditionVars        :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nexactEquationVars            :")); __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nadjacencyMatrix              :")); __mm_s.push_str(&*anyString(adjacencyMatrix.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        dumpSetSVarsSolvedInfo(matchedEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Standard BLT of the original model")).clone())?;
        BackendDump::dumpVarList(List::map1r(knowns.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("Variables of interest")).clone());
        BackendDump::dumpVarList(List::map1r(boundaryConditionVars.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("Boundary conditions")).clone());
        dumpSetSVarsSolvedInfo(bindingEquations.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Binding equations")).clone())?;
        BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("Approximated equations")).clone());
        BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundary condition equations")).clone());
        ebltEqsLst = getEBLTEquations(knowns.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.clone());
        ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst.clone(), bindingEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        dumpSetSVarsSolvedInfo(ebltEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("E-BLT: equations that compute the variables of interest")).clone())?;
        (currentSystem, tempSetS, mappedEbltSetS, status, setBFailedBoundaryConditionEquations) = traverseEBLTAndExtractSetCAndSetS(currentSystem.clone(), ebltEqsLst.clone(), sBltAdjacencyMatrix.clone(), knowns.clone(), boundaryConditionVars.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), solvedEqsAndVarsInfo.clone(), debug.clone(), setBFailedBoundaryConditionEquations.clone(), bindingEquations.clone())?;
        if !(status.clone()) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtraction procedure failed for iteration count: ")); __mm_s.push_str(&*intString(procedureCount.clone())); __mm_s.push_str(&*literal!(", re-running with modified model\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        procedureCount = procedureCount.clone() + 1;
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtraction procedure is successfully completed in iteration count: ")); __mm_s.push_str(&*intString(procedureCount.clone() - 1)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    tempSetS = List::setDifferenceOnTrue(tempSetS.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (ebltEqsLst, tempSetS, complexEquationList, swappedEquationList) = swapComplexEquationsInSetC(ebltEqsLst.clone(), tempSetS.clone(), mappedEbltSetS.clone(), currentSystem.clone(), mapIncRowEqn.clone())?;
    if debug.clone() {
        dumpSetSVarsSolvedInfo(tempSetS.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Set-S Solved-Variables Information")).clone())?;
    }
    extractedVarsfromSetS = getVariablesAfterExtraction(metamodelica::nil(), tempSetS.clone(), sBltAdjacencyMatrix.clone());
    extractedVarsfromSetS = List::setDifferenceOnTrue(extractedVarsfromSetS.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setC = List::unique(getAbsoluteIndexHelper(ebltEqsLst.clone(), mapIncRowEqn.clone()));
    setS = List::unique(getAbsoluteIndexHelper(tempSetS.clone(), mapIncRowEqn.clone()));
    setC_Eq = getEquationsFromSBLTAndEBLT(setC.clone(), currentSystem.orderedEqs.clone(), metamodelica::nil());
    setS_Eq = getEquationsFromSBLTAndEBLT(setS.clone(), currentSystem.orderedEqs.clone(), metamodelica::nil());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFinal set of equations after extraction algorithm\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_C: ")); __mm_s.push_str(&*dumplistInteger(setC.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(setS.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setC_Eq.clone())?, (literal!("SET_C")).clone())?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setS_Eq.clone())?, (literal!("SET_S")).clone())?;
    unMeasuredVariables = List::map1r(unMeasuredVariablesOfInterest.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone());
    outDiffVars = BackendVariable::listVar(List::map1r(knowns.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    outDiffVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
    (csvfileName, measurementcsvData) = readMeasurementsFromCSV(shared.clone())?;
    outDiffVars = setStartValuesToMeasurements(outDiffVars.clone(), measurementcsvData.clone(), (csvfileName.clone()).clone())?;
    (_, residualEquations) = BackendEquation::traverseEquationArray(BackendEquation::listEquation(setC_Eq.clone())?, (std::sync::Arc::new(BackendEquation::traverseEquationToScalarResidualForm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))> + 'static>), (shared.functionTree.clone(), metamodelica::nil()))?;
    (residualEquations, residualVars, _) = BackendEquation::convertResidualsIntoSolvedEquations(residualEquations.clone().reverse(), (literal!("$res_F_")).clone(), 1, false)?;
    outResidualVars = BackendVariable::listVar(residualVars.clone().reverse());
    outResidualEqns = BackendEquation::listEquation(residualEquations.clone())?;
    outOtherEqns = BackendEquation::listEquation(setS_Eq.clone())?;
    paramVars = BackendEquation::equationsVars(outOtherEqns.clone(), shared.globalKnownVars.clone())?;
    outOtherVars = BackendVariable::listVar(List::map1r(extractedVarsfromSetS.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    dumpSetSVars(outOtherVars.clone(), (literal!("Unknown variables in SET_S")).clone())?;
    BackendDump::dumpVariables(BackendVariable::listVar(paramVars.clone()), (literal!("Parameters in SET_S")).clone())?;
    auxillaryConditionsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_AuxiliaryConditions.html")); ArcStr::from(__mm_s) }).clone();
    auxillaryEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(setC_Eq.clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Auxiliary conditions")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(BackendEquation::listEquation(setC_Eq.clone())?))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(BackendEquation::listEquation(setC_Eq.clone())?)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((auxillaryConditionsFilename.clone()).clone(), (auxillaryEquations.clone()).clone())?;
    intermediateEquationsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_IntermediateEquations.html")); ArcStr::from(__mm_s) }).clone();
    intermediateEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(setS_Eq.clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Intermediate equations for measured variables")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(BackendEquation::listEquation(setS_Eq.clone())?))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(BackendEquation::listEquation(setS_Eq.clone())?)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((intermediateEquationsFilename.clone()).clone(), (intermediateEquations.clone()).clone())?;
    dumpRelatedBoundaryConditionsEquations(setBFailedBoundaryConditionEquations.clone(), (shared.info.fileNamePrefix.clone()).clone())?;
    VerifyDataReconciliation(ebltEqsLst.clone(), tempSetS.clone(), knowns.clone(), boundaryConditionVars.clone(), sBltAdjacencyMatrix.clone(), solvedEqsAndVarsInfo.clone(), exactEquationVars.clone(), approximatedEquations.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), outOtherVars.clone(), setS_Eq.clone(), shared.clone(), setC.clone(), setS.clone(), (unMeasuredVariablesOfInterest.clone().len() as i32))?;
    if debug.clone() {
        BackendDump::dumpVariables(outDiffVars.clone(), (literal!("Jacobian_knownVariables")).clone())?;
        BackendDump::dumpVariables(outResidualVars.clone(), (literal!("Jacobian_outResidualVars")).clone())?;
        BackendDump::dumpVariables(outOtherVars.clone(), (literal!("Jacobian_outOtherVars")).clone())?;
        BackendDump::dumpEquationArray(outResidualEqns.clone(), (literal!("Jacobian_ResidualEquation")).clone())?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), (literal!("Jacobian_other_Equation")).clone())?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(outDiffVars.clone(), outResidualEqns.clone(), outResidualVars.clone(), outOtherEqns.clone(), outOtherVars.clone(), shared.clone(), outOtherVars.clone(), (literal!("F")).clone(), false)?;
    assign_field!(shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData { relatedBoundaryConditions: (setBFailedBoundaryConditionEquations.clone().len() as i32), symbolicJacobianH: None, setBVars: Some(BackendVariable::listVar(unMeasuredVariables.clone())), datareconinputs: outDiffVars.clone(), setcVars: outResidualVars.clone(), symbolicJacobian: simCodeJacobian.clone() }));
    currentSystem = BackendDAEUtil::setEqSystVars(currentSystem.clone(), BackendVariable::mergeVariables(outResidualVars.clone(), outOtherVars.clone(), true)?)?;
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), BackendEquation::merge(outResidualEqns.clone(), outOtherEqns.clone())?);
    inputVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirection, BackendDAE::Var, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, DAE::VarDirection) -> Result<BackendDAE::Var> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT));
    shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), BackendVariable::mergeVariables(shared.globalKnownVars.clone(), inputVars.clone(), true)?);
    if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone())) {
        r#str = (literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval\n")).clone();
        r#str = (dumpToCsv((r#str.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?)?).clone();
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    }
    if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Correlation_Inputs.csv")); ArcStr::from(__mm_s) }).clone())) {
        r#str = (dumpCorrelationVarsToCsv(BackendVariable::varList(outDiffVars.clone())?)?).clone();
        r#str = (dumpToCsv(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), BackendVariable::varList(outDiffVars.clone())?)?).clone();
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Correlation_Inputs.csv")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    }
    modelicaFileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Reconciled_tmp")); ArcStr::from(__mm_s) }).clone();
    modelName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reconciled_")); __mm_s.push_str(&*System::stringReplace((shared.info.fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (literal!("/* This is a Reconciled Model which is generated by the Data Reconciliation extraction algorithm */\n")).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("model ")); __mm_s.push_str(&*modelName.clone()); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?, (literal!("Variables of Interest")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), paramVars.clone(), (literal!("parameters in SET-S")).clone())?).clone();
    modelicaOutput = (dumpResidualVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outResidualVars.clone())?, (literal!("residualVars")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outOtherVars.clone())?, (literal!("remaining variables in setS")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nequation")); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), outResidualEqns.clone(), (literal!("set-C Canonical form")).clone())?).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), outOtherEqns.clone(), (literal!("remaining equations in Set-S")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nend ")); __mm_s.push_str(&*modelName.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaFileName.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone(), (modelicaOutput.clone()).clone())?;
    outDAE = BackendDAE::DAE(list![currentSystem.clone()], shared.clone())?;
    Ok(outDAE)
}

// extract the "-sx =.csv" file path from simflags
pub fn extractSxPath(mut simflags: ArcStr) -> Result<ArcStr> {
    let mut csvFilePath: ArcStr = arcstr::literal!("");
    let mut nummatches: i32 = 0;
    let mut filePath: ArcStr = literal!("");
    if System::stringFind((simflags.clone()).clone(), (literal!("-sx")).clone())? < 0 {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": No -sx flag found in simflags, hence no csv file will be read for setting start values of the variables of interest for data reconciliation initialization.")).clone()])?;
        bail!("fail");
    }
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-sx[ \t]*=[ \t]*(\"[^\"]*\"|[^, \t]+)")).clone(), 2, true, false)) {
            (__pa1, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } }) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nummatches = __pa1.clone();
        filePath = __pa2.clone();
        if nummatches.clone() == 2 {
            csvFilePath = (unwrap_break_err!(System::stringReplace((filePath.clone()).clone(), (literal!(" ")).clone(), (literal!("")).clone()), '__try0)).clone();
            csvFilePath = (unwrap_break_err!(System::stringReplace((csvFilePath.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone()), '__try0)).clone();
            return Ok(csvFilePath.clone());
        }
        Ok::<_, anyhow::Error>((filePath.clone(), nummatches.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            filePath = __try0_o0;
            nummatches = __try0_o1;
        }
        Err(_) => {
            return Ok(csvFilePath.clone());
        }
    }
    Ok(csvFilePath)
}

// read the csv file and extract the measurement data for setting start values for data reconciliation initialization.
fn readMeasurementsFromCSV(mut shared: Arc<BackendDAE::Shared>) -> Result<(ArcStr, Arc<metamodelica::List<(ArcStr, ArcStr)>>)> {
    let mut csvFileName: ArcStr = arcstr::literal!("");
    let mut measurementData: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut content: ArcStr = arcstr::literal!("");
    let mut tokens: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut lines: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut simulationSettings: SimCode::SimulationSettings = <SimCode::SimulationSettings as ::std::default::Default>::default();
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    if isNone(shared.info.simSettingsOption.clone()) {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": SimulationSettings is NONE, expected SimulationSettings to be present in shared.info.simSettingsOption for reading measurements from csv file for data reconciliation initialization.")).clone()])?;
        bail!("fail");
    }
    simulationSettings = Util::getOption(shared.info.simSettingsOption.clone())?;
    csvFileName = (extractSxPath((simulationSettings.simflags.clone()).clone())?).clone();
    if stringEmpty((csvFileName.clone()).clone()) {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": No csv file provided or failed to read file with -sx flag in simflags.")).clone()])?;
        bail!("fail");
    }
    if StringUtil::startsWith((csvFileName.clone()).clone(), (literal!("modelica://")).clone()) || StringUtil::startsWith((csvFileName.clone()).clone(), (literal!("file://")).clone()) {
        p = SymbolTable::getAbsyn();
        csvFileName = (ProgramUtil::getFullPathFromUri(p.clone(), (csvFileName.clone()).clone(), true)?).clone();
    }
    content = (System::readFile((csvFileName.clone()).clone())?).clone();
    if stringEmpty((content.clone()).clone()) {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(": Failed to read csv file content from ")); __mm_s.push_str(&*csvFileName.clone()); __mm_s.push_str(&*literal!(" and hence start values can not be set.")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    lines = System::strtok((content.clone()).clone(), (literal!("\n")).clone());
    for mut line in &*lines.clone() {
        let mut line = line.clone();
        line = (System::stringReplace((line.clone()).clone(), (literal!(";")).clone(), (literal!(",")).clone())?).clone();
        line = (System::trim((line.clone()).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone();
        tokens = Util::stringSplitAtChar((line.clone()).clone(), (literal!(",")).clone())?;
        if !(tokens.clone().is_empty()) && (tokens.clone().len() as i32) >= 2 {
            measurementData = cons(((tokens.clone()).get(1)?, (tokens.clone()).get(2)?), measurementData.clone());
        }
    }
    Ok((csvFileName, measurementData))
}

fn setStartValuesToMeasurements(mut inVariables: BackendDAE::Variables, mut measurementData: Arc<metamodelica::List<(ArcStr, ArcStr)>>, mut csvFileName: ArcStr) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varName: ArcStr = arcstr::literal!("");
    let mut valueStr: ArcStr = arcstr::literal!("");
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut foundMeasurement: bool = false;
    varList = metamodelica::nil();
    for mut var in &*BackendVariable::varList(inVariables.clone())? {
        let mut var = var.clone();
        (valueStr, foundMeasurement) = checkVarExistenceInMeasurementData(var.clone(), measurementData.clone())?;
        if !(foundMeasurement.clone()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(": Entry for variable of interest ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(" not found in the measurement csv file ")); __mm_s.push_str(&*csvFileName.clone()); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        if let Ok(__iflet0) = stringReal((valueStr.clone()).clone()) {
            value = __iflet0;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(": Failed to convert the measurement value \"")); __mm_s.push_str(&*valueStr.clone()); __mm_s.push_str(&*literal!("\" for variable of interest ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(" from csv file ")); __mm_s.push_str(&*csvFileName.clone()); __mm_s.push_str(&*literal!(" to a valid Real number for setting start value for data reconciliation initialization.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        var = BackendVariable::setVarStartValue(var.clone(), Arc::new(DAE::Exp::RCONST { real: value.clone() }))?;
        varList = cons(var.clone(), varList.clone());
    }
    outVariables = BackendVariable::listVar(varList.clone().reverse());
    Ok(outVariables)
}

fn checkVarExistenceInMeasurementData(mut var: BackendDAE::Var, mut measurementData: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<(ArcStr, bool)> {
    let mut valueStr: ArcStr = literal!("");
    let mut exists: bool = false;
    let mut varName: ArcStr = arcstr::literal!("");
    for mut measurement in &*measurementData.clone() {
        let mut measurement = measurement.clone();
        (varName, valueStr) = measurement.clone();
        if varName.clone() == ComponentReference::crefStr(var.varName.clone())? {
            exists = true;
            break;
        }
    }
    Ok((valueStr, exists))
}

fn dumpRelatedBoundaryConditionsEquations(mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, mut fileNamePrefix: ArcStr) -> Result<()> {
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut count: i32 = 0;
    count = 1;
    r#str = (literal!("")).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<html>\n<body>\n<h2> Related boundary conditions")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((setBFailedBoundaryConditionEquations.clone().len() as i32))); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*literal!("</h2>\n<ol>")); ArcStr::from(__mm_s) }).clone();
    if setBFailedBoundaryConditionEquations.clone().is_empty() {
        r#str = (literal!("The set of Related boundary conditions are empty.")).clone();
    } else {
        for mut i in &*setBFailedBoundaryConditionEquations.clone() {
            let mut i = i.clone();
            (_, eq, _) = i.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("  <li>")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(BackendEquation::equationSize(eq.clone())?)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!(" </li>")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n</ol>\n</body>\n</html>")); ArcStr::from(__mm_s) }).clone();
    }
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_relatedBoundaryConditionsEquations.html")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    Ok(())
}

pub fn extractBoundaryCondition(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outOtherEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outBoundaryConditionEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut setS_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut failedboundaryConditionEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut varCount: i32 = 0;
    let mut eqCount: i32 = 0;
    let mut ebltEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bindingEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setSPrime: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut setSVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut failedboundaryConditionVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut extraVarsinSetSPrime: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut unMeasuredVariables: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut simCodeJacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut modelicaOutput: ArcStr = arcstr::literal!("");
    let mut modelicaFileName: ArcStr = arcstr::literal!("");
    let mut modelName: ArcStr = arcstr::literal!("");
    let mut auxillaryConditionsFilename: ArcStr = arcstr::literal!("");
    let mut auxillaryEquations: ArcStr = arcstr::literal!("");
    let mut intermediateEquationsFilename: ArcStr = arcstr::literal!("");
    let mut intermediateEquations: ArcStr = arcstr::literal!("");
    let mut csvfileName: ArcStr = arcstr::literal!("");
    let mut mappedEbltSetS: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut allVarsList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exactEquationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionTaggedEquationSolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMeasuredVariablesOfInterest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inputVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outDiffVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outOtherVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outBoundaryConditionVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut procedureCount: i32 = 0;
    let mut measurementcsvData: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut debug: bool = false;
    let mut status: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    currentSystem = __pa0.clone();
    shared = inDAE.shared.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nModelInfo: ")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem.clone(), inDAE.shared.clone(), debug.clone())?;
    procedureCount = 1;
    setBFailedBoundaryConditionEquations = metamodelica::nil();
    while !(status.clone()) {
        BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("OrderedVariables")).clone())?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("OrderedEquation")).clone())?;
        allVarsList = List::intRange(BackendVariable::varsSize(currentSystem.orderedVars.clone())?);
        varCount = currentSystem.orderedVars.numberOfVars.clone();
        eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
        (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
        (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount.clone(), eqCount.clone())?;
        BackendDump::dumpMatching(match1.clone())?;
        (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
        bindingEquations = getBindingEquation(currentSystem.clone(), mapIncRowEqn.clone())?;
        bindingEquations = List::flatten(List::map1r(bindingEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(BackendEquation::equationList(currentSystem.orderedEqs.clone()), 1)?;
        if debug.clone() {
            BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("ApproximatedEquations")).clone());
            BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundaryConditionEquations")).clone());
        }
        approximatedEquations = List::flatten(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        boundaryConditionEquations = List::flatten(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        boundaryConditionTaggedEquationSolvedVars = getBoundaryConditionVariables(boundaryConditionEquations.clone(), solvedEqsAndVarsInfo.clone());
        if debug.clone() {
            println!("{}", (literal!("\nApproximated and BoundaryCondition Equation Indexes :\n===========================================")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :")); __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("\n")).clone());
        }
        (knowns, boundaryConditionVars, exactEquationVars, unMeasuredVariablesOfInterest) = getVariablesBlockCategories(currentSystem.orderedVars.clone(), allVarsList.clone())?;
        boundaryConditionVars = listAppend(boundaryConditionVars.clone(), boundaryConditionTaggedEquationSolvedVars.clone());
        if debug.clone() {
            println!("{}", (literal!("\nVariablesCategories\n=============================")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nknownVars                    :")); __mm_s.push_str(&*dumplistInteger(knowns.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nboundaryConditionVars        :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nexactEquationVars            :")); __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nadjacencyMatrix              :")); __mm_s.push_str(&*anyString(adjacencyMatrix.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        dumpSetSVarsSolvedInfo(matchedEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Standard BLT of the original model")).clone())?;
        BackendDump::dumpVarList(List::map1r(knowns.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("Variables of interest")).clone());
        BackendDump::dumpVarList(List::map1r(boundaryConditionVars.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("Boundary conditions")).clone());
        dumpSetSVarsSolvedInfo(bindingEquations.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Binding equations")).clone())?;
        BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("Approximated equations")).clone());
        BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundary condition equations")).clone());
        ebltEqsLst = getEBLTEquations(knowns.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.clone());
        ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst.clone(), bindingEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        dumpSetSVarsSolvedInfo(ebltEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("E-BLT: equations that compute the variables of interest")).clone())?;
        (currentSystem, tempSetS, mappedEbltSetS, status, setBFailedBoundaryConditionEquations) = traverseEBLTAndExtractSetCAndSetS(currentSystem.clone(), ebltEqsLst.clone(), sBltAdjacencyMatrix.clone(), knowns.clone(), boundaryConditionVars.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), solvedEqsAndVarsInfo.clone(), debug.clone(), setBFailedBoundaryConditionEquations.clone(), bindingEquations.clone())?;
        if !(status.clone()) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtraction procedure failed for iteration count: ")); __mm_s.push_str(&*intString(procedureCount.clone())); __mm_s.push_str(&*literal!(", re-running with modified model\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        procedureCount = procedureCount.clone() + 1;
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtraction procedure is successfully completed in iteration count: ")); __mm_s.push_str(&*intString(procedureCount.clone() - 1)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpFailedBoundaryConditionEquationAndVars(setBFailedBoundaryConditionEquations.clone(), currentSystem.orderedVars.clone(), metamodelica::nil(), false)?;
    (_, setSPrime, failedboundaryConditionEquations, failedboundaryConditionVars, status) = ExtractSetSPrime(currentSystem.clone(), setBFailedBoundaryConditionEquations.clone(), sBltAdjacencyMatrix.clone(), knowns.clone(), boundaryConditionVars.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), solvedEqsAndVarsInfo.clone(), bindingEquations.clone(), debug.clone())?;
    setSPrime = List::setDifferenceOnTrue(setSPrime.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if debug.clone() {
        dumpSetSVarsSolvedInfo(setSPrime.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Set-S Solved-Variables Information")).clone())?;
    }
    setS = List::unique(getAbsoluteIndexHelper(setSPrime.clone(), mapIncRowEqn.clone()));
    setS_Eq = getEquationsFromSBLTAndEBLT(setS.clone(), currentSystem.orderedEqs.clone(), metamodelica::nil());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFinal set of equations after extraction algorithm\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    BackendDump::dumpEquationArray(BackendEquation::listEquation(failedboundaryConditionEquations.clone())?, (literal!("SET_B")).clone())?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setS_Eq.clone())?, (literal!("SET_S'")).clone())?;
    paramVars = BackendEquation::equationsVars(BackendEquation::listEquation(listAppend(failedboundaryConditionEquations.clone(), setS_Eq.clone()))?, shared.globalKnownVars.clone())?;
    setSVars = BackendEquation::equationsVars(BackendEquation::listEquation(listAppend(failedboundaryConditionEquations.clone(), setS_Eq.clone()))?, currentSystem.orderedVars.clone())?;
    (knownVars, setSVars) = List::extractOnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::varHasUncertainValueRefine, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    (_, setSVars) = List::extract1OnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(isBoundaryConditionVars, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<bool> + 'static>), failedboundaryConditionVars.clone());
    (extraVarsinSetSPrime, _) = List::extract1OnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(isBoundaryConditionVars, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<bool> + 'static>), List::map1r(boundaryConditionVars.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    BackendDump::dumpVarList(failedboundaryConditionVars.clone(), (literal!("Boundary condition Vars'")).clone());
    BackendDump::dumpVarList(setSVars.clone(), (literal!("Intermediate vars in set-S'")).clone());
    BackendDump::dumpVarList(knownVars.clone(), (literal!("Known vars in set-S'")).clone());
    BackendDump::dumpVarList(paramVars.clone(), (literal!("Param vars in set-S'")).clone());
    unMeasuredVariables = List::map1r(unMeasuredVariablesOfInterest.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone());
    outDiffVars = BackendVariable::listVar(List::map1r(knowns.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    outDiffVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
    (csvfileName, measurementcsvData) = readMeasurementsFromCSV(shared.clone())?;
    outDiffVars = setStartValuesToMeasurements(outDiffVars.clone(), measurementcsvData.clone(), (csvfileName.clone()).clone())?;
    outBoundaryConditionVars = BackendVariable::listVar(List::map1(failedboundaryConditionVars.clone().reverse(), (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
    outBoundaryConditionEquations = BackendEquation::listEquation(failedboundaryConditionEquations.clone())?;
    outOtherEqns = BackendEquation::listEquation(setS_Eq.clone())?;
    outOtherVars = BackendVariable::listVar(setSVars.clone());
    auxillaryConditionsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionsEquations.html")); ArcStr::from(__mm_s) }).clone();
    auxillaryEquations = (dumpExtractedEquationsToHTML(outBoundaryConditionEquations.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Boundary conditions")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(outBoundaryConditionEquations.clone()))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(outBoundaryConditionEquations.clone())?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((auxillaryConditionsFilename.clone()).clone(), (auxillaryEquations.clone()).clone())?;
    intermediateEquationsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html")); ArcStr::from(__mm_s) }).clone();
    intermediateEquations = (dumpExtractedEquationsToHTML(outOtherEqns.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Intermediate equations")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(outOtherEqns.clone()))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(outOtherEqns.clone())?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((intermediateEquationsFilename.clone()).clone(), (intermediateEquations.clone()).clone())?;
    VerifySetSPrime(outBoundaryConditionVars.clone(), outOtherVars.clone(), outDiffVars.clone(), extraVarsinSetSPrime.clone(), outBoundaryConditionEquations.clone(), outOtherEqns.clone(), shared.clone(), (ebltEqsLst.clone().len() as i32), (setBFailedBoundaryConditionEquations.clone().len() as i32), false)?;
    if debug.clone() {
        BackendDump::dumpVariables(outDiffVars.clone(), (literal!("Jacobian_knownVariables")).clone())?;
        BackendDump::dumpEquationArray(outBoundaryConditionEquations.clone(), (literal!("Jacobian_ResidualEquation")).clone())?;
        BackendDump::dumpVariables(outBoundaryConditionVars.clone(), (literal!("Jacobian_outResidualVars")).clone())?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), (literal!("Jacobian_outOtherEquations")).clone())?;
        BackendDump::dumpVariables(outOtherVars.clone(), (literal!("Jacobian_outOtherVars")).clone())?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(outDiffVars.clone(), outBoundaryConditionEquations.clone(), outBoundaryConditionVars.clone(), outOtherEqns.clone(), outOtherVars.clone(), shared.clone(), outOtherVars.clone(), (literal!("F")).clone(), false)?;
    assign_field!(shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData { relatedBoundaryConditions: (setBFailedBoundaryConditionEquations.clone().len() as i32), symbolicJacobianH: None, setBVars: Some(BackendVariable::listVar(unMeasuredVariables.clone())), datareconinputs: outDiffVars.clone(), setcVars: outBoundaryConditionVars.clone(), symbolicJacobian: simCodeJacobian.clone() }));
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), BackendEquation::merge(outBoundaryConditionEquations.clone(), outOtherEqns.clone())?);
    currentSystem = BackendDAEUtil::setEqSystVars(currentSystem.clone(), BackendVariable::mergeVariables(outBoundaryConditionVars.clone(), outOtherVars.clone(), true)?)?;
    inputVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirection, BackendDAE::Var, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, DAE::VarDirection) -> Result<BackendDAE::Var> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT));
    shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), BackendVariable::mergeVariables(shared.globalKnownVars.clone(), inputVars.clone(), true)?);
    r#str = (dumpToCsv((literal!("")).clone(), BackendVariable::varList(outBoundaryConditionVars.clone())?)?).clone();
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionVars.txt")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    modelicaFileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Reconciled_tmp")); ArcStr::from(__mm_s) }).clone();
    modelName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reconciled_")); __mm_s.push_str(&*System::stringReplace((shared.info.fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (literal!("/* This is a Reconciled Model which is generated by the Boundary condition extraction algorithm */\n")).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("model ")); __mm_s.push_str(&*modelName.clone()); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?, (literal!("Variables of Interest")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), paramVars.clone(), (literal!("parameters in SET-S")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), failedboundaryConditionVars.clone(), (literal!("boundary condition Vars")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outOtherVars.clone())?, (literal!("remaining variables in setS")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nequation")); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), BackendEquation::listEquation(failedboundaryConditionEquations.clone())?, (literal!("boundary condition equations")).clone())?).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), outOtherEqns.clone(), (literal!("remaining equations in Set-S'")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nend ")); __mm_s.push_str(&*modelName.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaFileName.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone(), (modelicaOutput.clone()).clone())?;
    outDAE = BackendDAE::DAE(list![currentSystem.clone()], shared.clone())?;
    Ok(outDAE)
}

pub fn stateEstimation(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outOtherEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outOtherEqnsSetSPrime: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outResidualEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outBoundaryConditionEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut setC_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut setS_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut setSPrime_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut residualEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut failedboundaryConditionEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut allDaeEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut varCount: i32 = 0;
    let mut eqCount: i32 = 0;
    let mut ebltEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bindingEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setSPrime: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMeasuredEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut setSVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut residualVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut failedboundaryConditionVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut extraVarsinSetSPrime: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut unMeasuredVariables: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut simCodeJacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
    let mut simCodeJacobianH: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut modelicaOutput: ArcStr = arcstr::literal!("");
    let mut modelicaFileName: ArcStr = arcstr::literal!("");
    let mut modelName: ArcStr = arcstr::literal!("");
    let mut auxillaryConditionsFilename: ArcStr = arcstr::literal!("");
    let mut auxillaryEquations: ArcStr = arcstr::literal!("");
    let mut intermediateEquationsFilename: ArcStr = arcstr::literal!("");
    let mut intermediateEquations: ArcStr = arcstr::literal!("");
    let mut csvfileName: ArcStr = arcstr::literal!("");
    let mut mappedEbltSetS: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut allVarsList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMeasuredVariablesOfInterest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut failedboundaryConditionEquationIndex: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exactEquationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut extractedVarsfromSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionTaggedEquationSolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inputVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outDiffVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outOtherVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outResidualVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outBoundaryConditionVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outOtherVarsSetSPrime: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut procedureCount: i32 = 0;
    let mut numRelatedBoundaryConditions: i32 = 0;
    let mut measurementcsvData: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut debug: bool = false;
    let mut status: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    currentSystem = __pa0.clone();
    shared = inDAE.shared.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nModelInfo: ")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem.clone(), inDAE.shared.clone(), debug.clone())?;
    procedureCount = 1;
    setBFailedBoundaryConditionEquations = metamodelica::nil();
    while !(status.clone()) {
        BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("OrderedVariables")).clone())?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("OrderedEquation")).clone())?;
        allVarsList = List::intRange(BackendVariable::varsSize(currentSystem.orderedVars.clone())?);
        varCount = currentSystem.orderedVars.numberOfVars.clone();
        eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
        (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
        (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount.clone(), eqCount.clone())?;
        BackendDump::dumpMatching(match1.clone())?;
        (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
        bindingEquations = getBindingEquation(currentSystem.clone(), mapIncRowEqn.clone())?;
        bindingEquations = List::flatten(List::map1r(bindingEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(BackendEquation::equationList(currentSystem.orderedEqs.clone()), 1)?;
        if debug.clone() {
            BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("ApproximatedEquations")).clone());
            BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundaryConditionEquations")).clone());
        }
        approximatedEquations = List::flatten(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        boundaryConditionEquations = List::flatten(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        boundaryConditionTaggedEquationSolvedVars = getBoundaryConditionVariables(boundaryConditionEquations.clone(), solvedEqsAndVarsInfo.clone());
        if debug.clone() {
            println!("{}", (literal!("\nApproximated and BoundaryCondition Equation Indexes :\n===========================================")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :")); __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("\n")).clone());
        }
        (knowns, boundaryConditionVars, exactEquationVars, unMeasuredVariablesOfInterest) = getVariablesBlockCategories(currentSystem.orderedVars.clone(), allVarsList.clone())?;
        boundaryConditionVars = listAppend(boundaryConditionVars.clone(), boundaryConditionTaggedEquationSolvedVars.clone());
        if debug.clone() {
            println!("{}", (literal!("\nVariablesCategories\n=============================")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nknownVars                    :")); __mm_s.push_str(&*dumplistInteger(knowns.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nunMeasuredVars               :")); __mm_s.push_str(&*dumplistInteger(unMeasuredVariablesOfInterest.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nboundaryConditionVars        :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nexactEquationVars            :")); __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nadjacencyMatrix              :")); __mm_s.push_str(&*anyString(adjacencyMatrix.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        dumpSetSVarsSolvedInfo(matchedEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Standard BLT of the original model")).clone())?;
        BackendDump::dumpVarList(List::map1r(knowns.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("Variables of interest")).clone());
        BackendDump::dumpVarList(List::map1r(unMeasuredVariablesOfInterest.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("unMeasured Variables of interest")).clone());
        BackendDump::dumpVarList(List::map1r(boundaryConditionVars.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("Boundary conditions")).clone());
        dumpSetSVarsSolvedInfo(bindingEquations.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Binding equations")).clone())?;
        BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("Approximated equations")).clone());
        BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundary condition equations")).clone());
        ebltEqsLst = getEBLTEquations(knowns.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.clone());
        ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst.clone(), bindingEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        dumpSetSVarsSolvedInfo(ebltEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("E-BLT: equations that compute the variables of interest")).clone())?;
        (currentSystem, tempSetS, mappedEbltSetS, status, setBFailedBoundaryConditionEquations) = traverseEBLTAndExtractSetCAndSetS(currentSystem.clone(), ebltEqsLst.clone(), sBltAdjacencyMatrix.clone(), knowns.clone(), boundaryConditionVars.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), solvedEqsAndVarsInfo.clone(), debug.clone(), setBFailedBoundaryConditionEquations.clone(), bindingEquations.clone())?;
        if !(status.clone()) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtraction procedure failed for iteration count: ")); __mm_s.push_str(&*intString(procedureCount.clone())); __mm_s.push_str(&*literal!(", re-running with modified model\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        procedureCount = procedureCount.clone() + 1;
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtraction procedure is successfully completed in iteration count: ")); __mm_s.push_str(&*intString(procedureCount.clone() - 1)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    tempSetS = List::setDifferenceOnTrue(tempSetS.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    extractedVarsfromSetS = getVariablesAfterExtraction(metamodelica::nil(), tempSetS.clone(), sBltAdjacencyMatrix.clone());
    extractedVarsfromSetS = List::setDifferenceOnTrue(extractedVarsfromSetS.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setC = List::unique(getAbsoluteIndexHelper(ebltEqsLst.clone(), mapIncRowEqn.clone()));
    setS = List::unique(getAbsoluteIndexHelper(tempSetS.clone(), mapIncRowEqn.clone()));
    setC_Eq = getEquationsFromSBLTAndEBLT(setC.clone(), currentSystem.orderedEqs.clone(), metamodelica::nil());
    setS_Eq = getEquationsFromSBLTAndEBLT(setS.clone(), currentSystem.orderedEqs.clone(), metamodelica::nil());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFinal set of equations after extraction algorithm\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_C: ")); __mm_s.push_str(&*dumplistInteger(setC.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(setS.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setC_Eq.clone())?, (literal!("SET_C")).clone())?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setS_Eq.clone())?, (literal!("SET_S")).clone())?;
    outDiffVars = BackendVariable::listVar(List::map1r(knowns.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    outDiffVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
    (csvfileName, measurementcsvData) = readMeasurementsFromCSV(shared.clone())?;
    outDiffVars = setStartValuesToMeasurements(outDiffVars.clone(), measurementcsvData.clone(), (csvfileName.clone()).clone())?;
    (_, residualEquations) = BackendEquation::traverseEquationArray(BackendEquation::listEquation(setC_Eq.clone())?, (std::sync::Arc::new(BackendEquation::traverseEquationToScalarResidualForm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))> + 'static>), (shared.functionTree.clone(), metamodelica::nil()))?;
    (residualEquations, residualVars, _) = BackendEquation::convertResidualsIntoSolvedEquations(residualEquations.clone().reverse(), (literal!("$res_F_")).clone(), 1, false)?;
    outResidualVars = BackendVariable::listVar(residualVars.clone().reverse());
    outResidualEqns = BackendEquation::listEquation(residualEquations.clone())?;
    outOtherEqns = BackendEquation::listEquation(setS_Eq.clone())?;
    paramVars = BackendEquation::equationsVars(outOtherEqns.clone(), shared.globalKnownVars.clone())?;
    outOtherVars = BackendVariable::listVar(List::map1r(extractedVarsfromSetS.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    dumpSetSVars(outOtherVars.clone(), (literal!("Unknown variables in SET_S")).clone())?;
    BackendDump::dumpVariables(BackendVariable::listVar(paramVars.clone()), (literal!("Parameters in SET_S")).clone())?;
    auxillaryConditionsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_AuxiliaryConditions.html")); ArcStr::from(__mm_s) }).clone();
    auxillaryEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(setC_Eq.clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Auxiliary conditions")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(BackendEquation::listEquation(setC_Eq.clone())?))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(BackendEquation::listEquation(setC_Eq.clone())?)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((auxillaryConditionsFilename.clone()).clone(), (auxillaryEquations.clone()).clone())?;
    intermediateEquationsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_IntermediateEquations.html")); ArcStr::from(__mm_s) }).clone();
    intermediateEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(setS_Eq.clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Intermediate equations for measured variables")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(BackendEquation::listEquation(setS_Eq.clone())?))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(BackendEquation::listEquation(setS_Eq.clone())?)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((intermediateEquationsFilename.clone()).clone(), (intermediateEquations.clone()).clone())?;
    dumpRelatedBoundaryConditionsEquations(setBFailedBoundaryConditionEquations.clone(), (shared.info.fileNamePrefix.clone()).clone())?;
    numRelatedBoundaryConditions = (setBFailedBoundaryConditionEquations.clone().len() as i32);
    VerifyDataReconciliation(ebltEqsLst.clone(), tempSetS.clone(), knowns.clone(), boundaryConditionVars.clone(), sBltAdjacencyMatrix.clone(), solvedEqsAndVarsInfo.clone(), exactEquationVars.clone(), approximatedEquations.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), outOtherVars.clone(), setS_Eq.clone(), shared.clone(), setC.clone(), setS.clone(), (unMeasuredVariablesOfInterest.clone().len() as i32))?;
    unMeasuredEqsLst = getEBLTEquations(unMeasuredVariablesOfInterest.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.clone());
    unMeasuredEqsLst = List::setDifferenceOnTrue(unMeasuredEqsLst.clone(), bindingEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    unMeasuredVariables = List::map1r(unMeasuredVariablesOfInterest.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone());
    dumpFailedBoundaryConditionEquationAndVars(setBFailedBoundaryConditionEquations.clone(), currentSystem.orderedVars.clone(), unMeasuredVariables.clone(), true)?;
    (setBFailedBoundaryConditionEquations, failedboundaryConditionEquationIndex) = prepareUnmeasuredVariablesEquations(unMeasuredEqsLst.clone(), sBltAdjacencyMatrix.clone(), knowns.clone(), solvedEqsAndVarsInfo.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), mapIncRowEqn.clone(), setBFailedBoundaryConditionEquations.clone())?;
    dumpSetSVarsSolvedInfo(unMeasuredEqsLst.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("E-BLT: equations in the BLT that compute the unmeasured variables of interest")).clone())?;
    (_, setSPrime, failedboundaryConditionEquations, failedboundaryConditionVars, status) = ExtractSetSPrime(currentSystem.clone(), setBFailedBoundaryConditionEquations.clone(), sBltAdjacencyMatrix.clone(), knowns.clone(), boundaryConditionVars.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), solvedEqsAndVarsInfo.clone(), bindingEquations.clone(), debug.clone())?;
    setSPrime = List::unique(listAppend(failedboundaryConditionEquationIndex.clone(), setSPrime.clone()));
    setSPrime = List::setDifferenceOnTrue(setSPrime.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setSPrime = List::setDifferenceOnTrue(setSPrime.clone(), unMeasuredEqsLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if debug.clone() {
        dumpSetSVarsSolvedInfo(setSPrime.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Set-SPrime Solved-Variables Information")).clone())?;
    }
    setSPrime = List::unique(getAbsoluteIndexHelper(setSPrime.clone(), mapIncRowEqn.clone()));
    setSPrime_Eq = getEquationsFromSBLTAndEBLT(setSPrime.clone(), currentSystem.orderedEqs.clone(), metamodelica::nil());
    BackendDump::dumpEquationArray(BackendEquation::listEquation(failedboundaryConditionEquations.clone())?, (literal!("SET_B")).clone())?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setSPrime_Eq.clone())?, (literal!("SET_SPrime")).clone())?;
    paramVars = BackendEquation::equationsVars(BackendEquation::listEquation(setSPrime_Eq.clone())?, shared.globalKnownVars.clone())?;
    setSVars = BackendEquation::equationsVars(BackendEquation::listEquation(setSPrime_Eq.clone())?, currentSystem.orderedVars.clone())?;
    (knownVars, setSVars) = List::extractOnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::varHasUncertainValueRefine, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    (_, setSVars) = List::extract1OnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(isBoundaryConditionVars, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<bool> + 'static>), failedboundaryConditionVars.clone());
    (extraVarsinSetSPrime, _) = List::extract1OnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(isBoundaryConditionVars, BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<bool> + 'static>), List::map1r(boundaryConditionVars.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    if debug.clone() {
        BackendDump::dumpVarList(unMeasuredVariables.clone(), (literal!("unmeasured variables")).clone());
        BackendDump::dumpVarList(setSVars.clone(), (literal!("Intermediate vars in set-S'")).clone());
        BackendDump::dumpVarList(knownVars.clone(), (literal!("Known vars in set-S'")).clone());
        BackendDump::dumpVarList(paramVars.clone(), (literal!("Param vars in set-S'")).clone());
        BackendDump::dumpVarList(extraVarsinSetSPrime.clone(), (literal!("extra vars in set-S'")).clone());
    }
    outBoundaryConditionVars = BackendVariable::listVar(List::map1(unMeasuredVariables.clone().reverse(), (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
    outBoundaryConditionEquations = BackendEquation::listEquation(failedboundaryConditionEquations.clone())?;
    outOtherEqnsSetSPrime = BackendEquation::listEquation(setSPrime_Eq.clone())?;
    outOtherVarsSetSPrime = BackendVariable::listVar(setSVars.clone());
    dumpSetSVars(outOtherVarsSetSPrime.clone(), (literal!("Unknown variables in SET_SPrime")).clone())?;
    auxillaryConditionsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionsEquations.html")); ArcStr::from(__mm_s) }).clone();
    auxillaryEquations = (dumpExtractedEquationsToHTML(outBoundaryConditionEquations.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Boundary conditions")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(outBoundaryConditionEquations.clone()))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(outBoundaryConditionEquations.clone())?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((auxillaryConditionsFilename.clone()).clone(), (auxillaryEquations.clone()).clone())?;
    intermediateEquationsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html")); ArcStr::from(__mm_s) }).clone();
    intermediateEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(listAppend(failedboundaryConditionEquations.clone(), setSPrime_Eq.clone()))?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Intermediate equations for unmeasured variables ")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(BackendEquation::listEquation(listAppend(failedboundaryConditionEquations.clone(), setSPrime_Eq.clone()))?))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(BackendEquation::listEquation(listAppend(failedboundaryConditionEquations.clone(), setSPrime_Eq.clone()))?)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
    System::writeFile((intermediateEquationsFilename.clone()).clone(), (intermediateEquations.clone()).clone())?;
    VerifySetSPrime(outBoundaryConditionVars.clone(), outOtherVarsSetSPrime.clone(), outDiffVars.clone(), extraVarsinSetSPrime.clone(), outBoundaryConditionEquations.clone(), outOtherEqnsSetSPrime.clone(), shared.clone(), (setC.clone().len() as i32), numRelatedBoundaryConditions.clone(), true)?;
    if debug.clone() {
        BackendDump::dumpVariables(outDiffVars.clone(), (literal!("Jacobian_knownVariables")).clone())?;
        BackendDump::dumpVariables(outResidualVars.clone(), (literal!("Jacobian_outResidualVars")).clone())?;
        BackendDump::dumpVariables(outOtherVars.clone(), (literal!("Jacobian_outOtherVars")).clone())?;
        BackendDump::dumpEquationArray(outResidualEqns.clone(), (literal!("Jacobian_ResidualEquation")).clone())?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), (literal!("Jacobian_other_Equation")).clone())?;
    }
    if debug.clone() {
        BackendDump::dumpVariables(outDiffVars.clone(), (literal!("Jacobian_knownVariables")).clone())?;
        BackendDump::dumpEquationArray(outBoundaryConditionEquations.clone(), (literal!("Jacobian_ResidualEquation")).clone())?;
        BackendDump::dumpVariables(outBoundaryConditionVars.clone(), (literal!("Jacobian_outResidualVars")).clone())?;
        BackendDump::dumpEquationArray(outOtherEqnsSetSPrime.clone(), (literal!("Jacobian_outOtherEquations")).clone())?;
        BackendDump::dumpVariables(outOtherVarsSetSPrime.clone(), (literal!("Jacobian_outOtherVars")).clone())?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(outDiffVars.clone(), outResidualEqns.clone(), outResidualVars.clone(), outOtherEqns.clone(), outOtherVars.clone(), shared.clone(), outOtherVars.clone(), (literal!("F")).clone(), false)?;
    (simCodeJacobianH, shared) = SymbolicJacobian::getSymbolicJacobian(outDiffVars.clone(), outBoundaryConditionEquations.clone(), outBoundaryConditionVars.clone(), outOtherEqnsSetSPrime.clone(), outOtherVarsSetSPrime.clone(), shared.clone(), outOtherVarsSetSPrime.clone(), (literal!("H")).clone(), false)?;
    assign_field!(shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData { relatedBoundaryConditions: numRelatedBoundaryConditions.clone(), symbolicJacobianH: Some(simCodeJacobianH.clone()), setBVars: Some(outBoundaryConditionVars.clone()), datareconinputs: outDiffVars.clone(), setcVars: outResidualVars.clone(), symbolicJacobian: simCodeJacobian.clone() }));
    setSPrime_Eq = List::unique(listAppend(setSPrime_Eq.clone(), failedboundaryConditionEquations.clone()));
    setSPrime_Eq = List::unique(listAppend(setSPrime_Eq.clone(), setS_Eq.clone()));
    allDaeEqs = List::unique(listAppend(setSPrime_Eq.clone(), residualEquations.clone()));
    BackendDump::dumpEquationArray(BackendEquation::listEquation(allDaeEqs.clone())?, (literal!("Final DAE with set-c, set-S and set-SPrime combined")).clone())?;
    paramVars = BackendEquation::equationsVars(BackendEquation::listEquation(allDaeEqs.clone())?, shared.globalKnownVars.clone())?;
    setSVars = BackendEquation::equationsVars(BackendEquation::listEquation(allDaeEqs.clone())?, currentSystem.orderedVars.clone())?;
    (knownVars, setSVars) = List::extractOnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::varHasUncertainValueRefine, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    (_, setSVars) = List::extractOnTrue(setSVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::varHasUncertainValuePropagate, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    setSVars = listAppend(BackendVariable::varList(outBoundaryConditionVars.clone())?, setSVars.clone());
    BackendDump::dumpVarList(listAppend(setSVars.clone(), residualVars.clone()), (literal!("Intermediate vars in final DAE updated'")).clone());
    BackendDump::dumpVarList(paramVars.clone(), (literal!("parameters in final DAE updated")).clone());
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), BackendEquation::listEquation(allDaeEqs.clone())?);
    currentSystem = BackendDAEUtil::setEqSystVars(currentSystem.clone(), BackendVariable::listVar(listAppend(setSVars.clone(), residualVars.clone())))?;
    inputVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirection, BackendDAE::Var, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, DAE::VarDirection) -> Result<BackendDAE::Var> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT));
    shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), BackendVariable::mergeVariables(shared.globalKnownVars.clone(), inputVars.clone(), true)?);
    if debug.clone() {
        BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("FinalOrderedVariables")).clone())?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("FinalOrderedEquation")).clone())?;
        BackendDump::dumpVariables(shared.globalKnownVars.clone(), (literal!("FinalGlobalKnownVars")).clone())?;
    }
    if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone())) {
        r#str = (literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval\n")).clone();
        r#str = (dumpToCsv((r#str.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?)?).clone();
        r#str = (dumpToCsv((r#str.clone()).clone(), BackendVariable::varList(outBoundaryConditionVars.clone())?)?).clone();
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    }
    if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Correlation_Inputs.csv")); ArcStr::from(__mm_s) }).clone())) {
        r#str = (dumpCorrelationVarsToCsv(BackendVariable::varList(outDiffVars.clone())?)?).clone();
        r#str = (dumpToCsv(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), BackendVariable::varList(outDiffVars.clone())?)?).clone();
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Correlation_Inputs.csv")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    }
    r#str = (dumpToCsv((literal!("")).clone(), BackendVariable::varList(outBoundaryConditionVars.clone())?)?).clone();
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionVars.txt")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    modelicaFileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Reconciled_tmp")); ArcStr::from(__mm_s) }).clone();
    modelName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reconciled_")); __mm_s.push_str(&*System::stringReplace((shared.info.fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (literal!("/* This is a Reconciled Model which is generated by the State Estimation extraction algorithm */\n")).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("model ")); __mm_s.push_str(&*modelName.clone()); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?, (literal!("Variables of Interest")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), paramVars.clone(), (literal!("parameters")).clone())?).clone();
    modelicaOutput = (dumpResidualVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outResidualVars.clone())?, (literal!("residualVars")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), setSVars.clone(), (literal!("intermediate variables")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nequation")); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), BackendEquation::listEquation(residualEquations.clone())?, (literal!("residual equations")).clone())?).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), BackendEquation::listEquation(setSPrime_Eq.clone())?, (literal!("remaining equations in Set-S'")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nend ")); __mm_s.push_str(&*modelName.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaFileName.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone(), (modelicaOutput.clone()).clone())?;
    outDAE = BackendDAE::DAE(list![currentSystem.clone()], shared.clone())?;
    Ok(outDAE)
}

fn isBoundaryConditionVars(mut setSVars: BackendDAE::Var, mut boundaryConditionsVars: Arc<metamodelica::List<BackendDAE::Var>>) -> bool {
    let mut result: bool = false;
    if listMember(setSVars.clone(), boundaryConditionsVars.clone()) {
        result = true;
    }
    result
}

fn dumpFailedBoundaryConditionEquationAndVars(mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, mut orderedVars: BackendDAE::Variables, mut unmeasuredVariables: Arc<metamodelica::List<BackendDAE::Var>>, mut stateEstimation: bool) -> Result<()> {
    let mut failedboundaryConditionEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut count: i32 = 0;
    let mut varIndex: i32 = 0;
    let mut varlist: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    if stateEstimation.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart of extraction procedure for unmeasured variables of interest\nSet of equations that failed the extraction of set S and that contain an unmeasured variable of interest: (")); __mm_s.push_str(&*intString((setBFailedBoundaryConditionEquations.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
    } else {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart of extraction procedure for boundary conditions\nSet of boundary conditions equations that failed the extraction of set S: (")); __mm_s.push_str(&*intString((setBFailedBoundaryConditionEquations.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
    }
    count = 1;
    varlist = metamodelica::nil();
    for mut item in &*setBFailedBoundaryConditionEquations.clone().reverse() {
        let mut item = item.clone();
        (varIndex, failedboundaryConditionEquation, _) = item.clone();
        varlist = cons(BackendVariable::getVarAt(orderedVars.clone(), varIndex.clone())?, varlist.clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(count.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*BackendDump::equationString(failedboundaryConditionEquation.clone())?); ArcStr::from(__mm_s) }).clone());
        count = count.clone() + 1;
    }
    println!("{}", (literal!("\n")).clone());
    if stateEstimation.clone() {
        BackendDump::dumpVarList(unmeasuredVariables.clone(), (literal!("umeasured variables to be computed")).clone());
    } else {
        BackendDump::dumpVarList(varlist.clone().reverse(), (literal!("Boundary conditions to be computed")).clone());
    }
    Ok(())
}

fn prepareUnmeasuredVariablesEquations(mut unMeasuredEqsLst: Arc<metamodelica::List<i32>>, mut sBltAdjacencyMatrix: ExtAdjacencyMatrix, mut knownVars: Arc<metamodelica::List<i32>>, mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut orderedVars: BackendDAE::Variables, mut mapIncRowEqn: metamodelica::Array<i32>, mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>) -> Result<(Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, Arc<metamodelica::List<i32>>)> {
    let mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = setBFailedBoundaryConditionEquations;
    let mut failedboundaryConditionEquationIndex: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIndex: i32 = 0;
    let mut intermediateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unmeasuredEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut unMeasuredVariablesAndEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    for mut eq in &*unMeasuredEqsLst.clone() {
        let mut eq = eq.clone();
        varIndex = getSolvedVariableNumber(eq.clone(), solvedEqsAndVarsInfo.clone());
        intermediateVars = getVariablesAfterExtraction(list![eq.clone()], metamodelica::nil(), sBltAdjacencyMatrix.clone());
        intermediateVars = List::setDifferenceOnTrue(intermediateVars.clone(), knownVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?.reverse();
        unmeasuredEq = BackendEquation::get(orderedEqs.clone(), mapIncRowEqn.borrow()[(eq.clone()-1) as usize].clone());
        setBFailedBoundaryConditionEquations = cons((varIndex.clone(), unmeasuredEq.clone(), intermediateVars.clone()), setBFailedBoundaryConditionEquations.clone());
    }
    unMeasuredVariablesAndEquations = metamodelica::nil();
    for mut item in &*setBFailedBoundaryConditionEquations.clone() {
        let mut item = item.clone();
        (varIndex, _, _) = item.clone();
        if BackendVariable::varHasUncertainValuePropagate(BackendVariable::getVarAt(orderedVars.clone(), varIndex.clone())?) {
            unMeasuredVariablesAndEquations = cons(item.clone(), unMeasuredVariablesAndEquations.clone());
        }
    }
    setBFailedBoundaryConditionEquations = List::unique(unMeasuredVariablesAndEquations.clone());
    Ok((setBFailedBoundaryConditionEquations, failedboundaryConditionEquationIndex))
}

fn addUnmeasuredEquationtoBoundaryConditionEquationAndVars(mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, mut orderedVars: BackendDAE::Variables, mut unMeasuredEqsLst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>> {
    let mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = setBFailedBoundaryConditionEquations;
    let mut failedboundaryConditionEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut count: i32 = 0;
    let mut varIndex: i32 = 0;
    let mut varlist: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart of extraction procedure for boundary conditions\nSet of boundary conditions equations that failed the extraction of set S: (")); __mm_s.push_str(&*intString((setBFailedBoundaryConditionEquations.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
    count = 1;
    varlist = metamodelica::nil();
    for mut item in &*setBFailedBoundaryConditionEquations.clone().reverse() {
        let mut item = item.clone();
        (varIndex, failedboundaryConditionEquation, _) = item.clone();
        varlist = cons(BackendVariable::getVarAt(orderedVars.clone(), varIndex.clone())?, varlist.clone());
    }
    println!("{}", (literal!("\n")).clone());
    Ok(setBFailedBoundaryConditionEquations)
}

fn getEBLTEquations(mut knowns: Arc<metamodelica::List<i32>>, mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut currentSystem: Arc<BackendDAE::EqSystem>) -> Arc<metamodelica::List<i32>> {
    let mut ebltequations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    let mut var: i32 = 0;
    for mut v in &*solvedEqsAndVarsInfo.clone() {
        let mut v = v.clone();
        (eq, var) = v.clone();
        if listMember(var.clone(), knowns.clone()) {
            ebltequations = cons(eq.clone(), ebltequations.clone());
        }
    }
    ebltequations
}

fn getBindingEquation(mut currentSystem: Arc<BackendDAE::EqSystem>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut bindingEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut eq in &*BackendEquation::equationList(currentSystem.orderedEqs.clone()) {
        let mut eq = eq.clone();
        if BackendEquation::isBindingEquation(eq.clone())? {
            bindingEquations = cons(index.clone(), bindingEquations.clone());
        }
        index = index.clone() + 1;
    }
    Ok(bindingEquations)
}

fn swapComplexEquationsInSetC(mut ebltEqsLst: Arc<metamodelica::List<i32>>, mut tempSetS: Arc<metamodelica::List<i32>>, mut mappedEbltSetS: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut currentSystem: Arc<BackendDAE::EqSystem>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut ebltEqsLst: Arc<metamodelica::List<i32>> = ebltEqsLst;
    let mut tempSetS: Arc<metamodelica::List<i32>> = tempSetS;
    let mut complexEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut swappedEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqIndex: i32 = 0;
    let mut matchedEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut swapEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    complexEquationList = metamodelica::nil();
    swappedEquationList = metamodelica::nil();
    for mut item in &*mappedEbltSetS.clone() {
        let mut item = item.clone();
        (eqIndex, matchedEqsLst) = item.clone();
        eq = BackendEquation::get(currentSystem.orderedEqs.clone(), mapIncRowEqn.borrow()[(eqIndex.clone()-1) as usize].clone());
        if BackendEquation::isComplexEquation(eq.clone()) {
            complexEquationList = cons(eq.clone(), complexEquationList.clone());
            for mut index in &*matchedEqsLst.clone() {
                let mut index = index.clone();
                swapEq = BackendEquation::get(currentSystem.orderedEqs.clone(), mapIncRowEqn.borrow()[(index.clone()-1) as usize].clone());
                if !(BackendEquation::isComplexEquation(swapEq.clone())) {
                    ebltEqsLst = List::removeOnTrue(eqIndex.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ebltEqsLst.clone());
                    tempSetS = List::removeOnTrue(index.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), tempSetS.clone());
                    tempSetS = cons(eqIndex.clone(), tempSetS.clone());
                    ebltEqsLst = cons(index.clone(), ebltEqsLst.clone());
                    swappedEquationList = cons(swapEq.clone(), swappedEquationList.clone());
                    break;
                }
            }
        }
    }
    if !(complexEquationList.clone().is_empty()) {
        BackendDump::dumpEquationArray(BackendEquation::listEquation(complexEquationList.clone().reverse())?, (literal!("Warning complex equation detected in Set-C")).clone())?;
        BackendDump::dumpEquationArray(BackendEquation::listEquation(swappedEquationList.clone().reverse())?, (literal!("Swapping Equations from Set-S")).clone())?;
    }
    Ok((ebltEqsLst, tempSetS, complexEquationList, swappedEquationList))
}

fn traverseEBLTAndExtractSetCAndSetS(mut currentSystem: Arc<BackendDAE::EqSystem>, mut ebltEquations: Arc<metamodelica::List<i32>>, mut sBltAdjacencyMatrix: ExtAdjacencyMatrix, mut knownVars: Arc<metamodelica::List<i32>>, mut boundaryConditionVars: Arc<metamodelica::List<i32>>, mut orderedVars: BackendDAE::Variables, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>>, mut debug: bool, mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, mut bindingEquations: Arc<metamodelica::List<i32>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, bool, Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>)> {
    let mut currentSystem: Arc<BackendDAE::EqSystem> = currentSystem;
    let mut finalSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mappedEbltSetS: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut outStatus: bool = false;
    let mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>> = setBFailedBoundaryConditionEquations;
    let mut intermediateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut minimalSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut visitedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqlistToRemove: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intermediateVarsInBoundaryConditionEquation: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut status: bool = false;
    let mut setB: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut varnumber: i32 = 0;
    let mut eqnumber: i32 = 0;
    let mut boundaryConditionVarIndex: i32 = 0;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut failedboundaryConditionEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut newEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtracting SET-C and SET-S from E-BLT\nProcedure is applied on each equation in the E-BLT\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
    setB = metamodelica::nil();
    eqlistToRemove = metamodelica::nil();
    finalSetS = metamodelica::nil();
    mappedEbltSetS = metamodelica::nil();
    for mut eq in &*ebltEquations.clone() {
        let mut eq = eq.clone();
        intermediateVars = getVariablesAfterExtraction(list![eq.clone()], metamodelica::nil(), sBltAdjacencyMatrix.clone());
        intermediateVars = List::setDifferenceOnTrue(intermediateVars.clone(), knownVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?.reverse();
        dumpSetSTargetEquations(eq.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), orderedEqs.clone(), orderedVars.clone(), (literal!(">>>")).clone())?;
        minimalSetS = metamodelica::nil();
        visitedVars = metamodelica::nil();
        status = true;
        (_, minimalSetS, visitedVars, status, boundaryConditionVarIndex) = extractNewMinimalSetS(intermediateVars.clone(), sBltAdjacencyMatrix.clone(), knownVars.clone(), boundaryConditionVars.clone(), orderedVars.clone(), orderedEqs.clone(), mapIncRowEqn.clone(), minimalSetS.clone(), visitedVars.clone(), solvedEqsAndVarsInfo.clone(), status.clone(), bindingEquations.clone(), true, debug.clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nProcedure ")); __mm_s.push_str(&*boolSuccessOrFailed(status.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        mappedEbltSetS = cons((eq.clone(), minimalSetS.clone().reverse()), mappedEbltSetS.clone());
        for mut index in &*minimalSetS.clone() {
            let mut index = index.clone();
            if !(listMember(index.clone(), finalSetS.clone())) {
                finalSetS = cons(index.clone(), finalSetS.clone());
            }
        }
        if !(status.clone()) {
            varnumber = getSolvedVariableNumber(eq.clone(), solvedEqsAndVarsInfo.clone());
            if minimalSetS.clone().is_empty() {
                minimalSetS = list![eq.clone()];
            }
            if !(listMember(listHead(minimalSetS.clone())?, eqlistToRemove.clone())) {
                eqlistToRemove = cons(listHead(minimalSetS.clone())?, eqlistToRemove.clone());
                setB = cons((varnumber.clone(), listHead(minimalSetS.clone())?), setB.clone());
                if !(boundaryConditionVarExist(setBFailedBoundaryConditionEquations.clone(), boundaryConditionVarIndex.clone())) {
                    intermediateVarsInBoundaryConditionEquation = getVariablesAfterExtraction(list![listHead(minimalSetS.clone())?], metamodelica::nil(), sBltAdjacencyMatrix.clone());
                    intermediateVarsInBoundaryConditionEquation = List::setDifferenceOnTrue(intermediateVarsInBoundaryConditionEquation.clone(), knownVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?.reverse();
                    failedboundaryConditionEquation = BackendEquation::get(orderedEqs.clone(), mapIncRowEqn.borrow()[(listHead(minimalSetS.clone())?-1) as usize].clone());
                    setBFailedBoundaryConditionEquations = cons((boundaryConditionVarIndex.clone(), failedboundaryConditionEquation.clone(), intermediateVarsInBoundaryConditionEquation.clone()), setBFailedBoundaryConditionEquations.clone());
                }
            }
        }
    }
    if !(setB.clone().is_empty()) {
        newEqnLst = metamodelica::nil();
        for mut item in &*setB.clone().reverse() {
            let mut item = item.clone();
            (varnumber, eqnumber) = item.clone();
            var = BackendVariable::getVarAt(orderedVars.clone(), varnumber.clone())?;
            lhs = BackendVariable::varExp(var.clone())?;
            rhs = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
            newEqnLst = cons(eqn.clone(), newEqnLst.clone());
        }
        if debug.clone() {
            println!("{}", (literal!("\nGenerate Modified Model, For each failed procedure, the equation involving the boundary condition that failed the procedure is replaced by x = 0 where x is the variable of interest of the procedure.\n")).clone());
            dumpSetSVarsSolvedInfo(eqlistToRemove.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Equations to remove")).clone())?;
            BackendDump::dumpEquationList(newEqnLst.clone(), (literal!("Equations to add")).clone());
        }
        eqlistToRemove = List::unique(List::map1r(eqlistToRemove.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
        currentSystem = deleteEquationsFromEqSyst(currentSystem.clone(), eqlistToRemove.clone())?;
        assign_field!(currentSystem.orderedEqs = BackendEquation::merge(currentSystem.orderedEqs.clone(), BackendEquation::listEquation(newEqnLst.clone().reverse())?)?);
    } else {
        outStatus = true;
        finalSetS = finalSetS.clone().reverse();
        mappedEbltSetS = mappedEbltSetS.clone().reverse();
    }
    Ok((currentSystem, finalSetS, mappedEbltSetS, outStatus, setBFailedBoundaryConditionEquations))
}

fn ExtractSetSPrime(mut currentSystem: Arc<BackendDAE::EqSystem>, mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, mut sBltAdjacencyMatrix: ExtAdjacencyMatrix, mut knownVars: Arc<metamodelica::List<i32>>, mut boundaryConditionVars: Arc<metamodelica::List<i32>>, mut orderedVars: BackendDAE::Variables, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>>, mut bindingEquations: Arc<metamodelica::List<i32>>, mut debug: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, bool)> {
    let mut currentSystem: Arc<BackendDAE::EqSystem> = currentSystem;
    let mut finalSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut failedboundaryConditionEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut failedboundaryConditionVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outStatus: bool = false;
    let mut intermediateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut minimalSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut visitedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut status: bool = false;
    let mut boundaryConditionVarIndex: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nExtract set-S' to compute the boundary conditions\nProcedure is applied on each equation in the failed boundary conditions\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); ArcStr::from(__mm_s) }).clone());
    finalSetS = metamodelica::nil();
    failedboundaryConditionEquations = metamodelica::nil();
    failedboundaryConditionVars = metamodelica::nil();
    for mut items in &*setBFailedBoundaryConditionEquations.clone().reverse() {
        let mut items = items.clone();
        (boundaryConditionVarIndex, eq, intermediateVars) = items.clone();
        failedboundaryConditionEquations = cons(eq.clone(), failedboundaryConditionEquations.clone());
        failedboundaryConditionVars = cons(BackendVariable::getVarAt(orderedVars.clone(), boundaryConditionVarIndex.clone())?, failedboundaryConditionVars.clone());
        intermediateVars = List::setDifferenceOnTrue(intermediateVars.clone(), knownVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?.reverse();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(">>>")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); ArcStr::from(__mm_s) }).clone());
        minimalSetS = metamodelica::nil();
        visitedVars = metamodelica::nil();
        status = true;
        (_, minimalSetS, visitedVars, status, _) = extractNewMinimalSetS(intermediateVars.clone(), sBltAdjacencyMatrix.clone(), knownVars.clone(), boundaryConditionVars.clone(), orderedVars.clone(), orderedEqs.clone(), mapIncRowEqn.clone(), minimalSetS.clone(), visitedVars.clone(), solvedEqsAndVarsInfo.clone(), status.clone(), bindingEquations.clone(), false, debug.clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nProcedure ")); __mm_s.push_str(&*boolSuccessOrFailed(status.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        for mut index in &*minimalSetS.clone() {
            let mut index = index.clone();
            if !(listMember(index.clone(), finalSetS.clone())) {
                finalSetS = cons(index.clone(), finalSetS.clone());
            }
        }
    }
    failedboundaryConditionEquations = failedboundaryConditionEquations.clone().reverse();
    failedboundaryConditionVars = failedboundaryConditionVars.clone().reverse();
    Ok((currentSystem, finalSetS, failedboundaryConditionEquations, failedboundaryConditionVars, outStatus))
}

fn boundaryConditionVarExist(mut setBFailedBoundaryConditionEquations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)>>, mut boundaryConditionVarIndex: i32) -> bool {
    let mut status: bool = false;
    let mut varIndex: i32 = 0;
    for mut item in &*setBFailedBoundaryConditionEquations.clone() {
        let mut item = item.clone();
        (varIndex, _, _) = item.clone();
        if intEq(varIndex.clone(), boundaryConditionVarIndex.clone()) {
            status = true;
            break;
        }
    }
    status
}

fn boolSuccessOrFailed(mut status: bool) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (if (status.clone()) {literal!("success")} else {literal!("failed")}).clone();
    outString
}

fn extractNewMinimalSetS(mut unknownsInSetC: Arc<metamodelica::List<i32>>, mut sBltAdjacencyMatrix: ExtAdjacencyMatrix, mut knownVars: Arc<metamodelica::List<i32>>, mut boundaryConditionVars: Arc<metamodelica::List<i32>>, mut orderedVars: BackendDAE::Variables, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut minimalSetS: Arc<metamodelica::List<i32>>, mut visitedVars: Arc<metamodelica::List<i32>>, mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>>, mut status: bool, mut bindingEquations: Arc<metamodelica::List<i32>>, mut extractSetCAndSetS: bool, mut debug: bool) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, bool, i32)> {
    let mut unknownsInSetC: Arc<metamodelica::List<i32>> = unknownsInSetC;
    let mut minimalSetS: Arc<metamodelica::List<i32>> = minimalSetS;
    let mut visitedVars: Arc<metamodelica::List<i32>> = visitedVars;
    let mut status: bool = status;
    let mut boundaryConditionVarIndex: i32 = -1;
    let mut mappedEq: i32 = 0;
    let mut varIndex: i32 = 0;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intermediateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intermediateVarsInMatchedEquation: Arc<metamodelica::List<i32>> = metamodelica::nil();
    while !(unknownsInSetC.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unknownsInSetC.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        varIndex = __pa0.clone();
        rest = __pa1.clone();
        visitedVars = cons(varIndex.clone(), visitedVars.clone());
        var = BackendVariable::getVarAt(orderedVars.clone(), varIndex.clone())?;
        if listMember(varIndex.clone(), boundaryConditionVars.clone()) && extractSetCAndSetS.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(" is a boundary condition ---> exit procedure")); ArcStr::from(__mm_s) }).clone());
            status = false;
            boundaryConditionVarIndex = varIndex.clone();
            break;
        }
        mappedEq = getSolvedEquationNumber(varIndex.clone(), solvedEqsAndVarsInfo.clone());
        if !(listMember(mappedEq.clone(), bindingEquations.clone())) {
            minimalSetS = cons(mappedEq.clone(), minimalSetS.clone());
            dumpSetSTargetEquations(mappedEq.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), orderedEqs.clone(), orderedVars.clone(), (literal!("")).clone())?;
        }
        vars = getVariablesAfterExtraction(list![mappedEq.clone()], metamodelica::nil(), sBltAdjacencyMatrix.clone());
        intermediateVarsInMatchedEquation = List::setDifferenceOnTrue(vars.clone(), knownVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        intermediateVars = List::setDifferenceOnTrue(intermediateVarsInMatchedEquation.clone(), list![varIndex.clone()], (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        intermediateVars = List::setDifferenceOnTrue(intermediateVars.clone(), visitedVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        rest = List::setDifferenceOnTrue(rest.clone(), visitedVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        unknownsInSetC = List::unique(listAppend(intermediateVars.clone(), rest.clone()));
        if debug.clone() {
            dumpMininimalExtraction(varIndex.clone(), var.clone(), mappedEq.clone(), mapIncRowEqn.clone(), orderedEqs.clone(), minimalSetS.clone(), intermediateVarsInMatchedEquation.clone(), rest.clone(), unknownsInSetC.clone(), false, visitedVars.clone())?;
        }
    }
    Ok((unknownsInSetC, minimalSetS, visitedVars, status, boundaryConditionVarIndex))
}

pub fn extractionAlgorithm(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut currentSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outOtherEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outResidualEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut newEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut setC_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut setS_Eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut residualEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut varCount: i32 = 0;
    let mut eqCount: i32 = 0;
    let mut matchedEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMatchedEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMatchedEqsLstCorrectIndex: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempSetC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tempSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut s_BLTBlocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut e_BLTBlocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut allBlocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut allBlocksStatusVarInfo: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut e_BLT_EquationsWithIndex: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut eBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut setS_BLTAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut e_BLTSolvedEqsAndVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut e_BLTBlockRanks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut s_BLTBlockRanks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut s_BLTBlockTargetInfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>> = metamodelica::nil();
    let mut predecessorBlockTargetInfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut residualVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut simCodeJacobian: Arc<BackendDAE::Jacobian> = Arc::new(BackendDAE::Jacobian::EMPTY_JACOBIAN);
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut modelicaOutput: ArcStr = arcstr::literal!("");
    let mut modelicaFileName: ArcStr = arcstr::literal!("");
    let mut modelName: ArcStr = arcstr::literal!("");
    let mut auxillaryConditionsFilename: ArcStr = arcstr::literal!("");
    let mut auxillaryEquations: ArcStr = arcstr::literal!("");
    let mut intermediateEquationsFilename: ArcStr = arcstr::literal!("");
    let mut intermediateEquations: ArcStr = arcstr::literal!("");
    let mut allVarsList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exactEquationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut extractedVarsfromSetS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut knownVariablesWithEquationBinding: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionTaggedEquationSolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unknownVarsInSetC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inputVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outDiffVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outOtherVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outResidualVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut debug: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    currentSystem = __pa0.clone();
    shared = inDAE.shared.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nModelInfo: ")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("OrderedVariables")).clone())?;
    BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("OrderedEquation")).clone())?;
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem.clone(), inDAE.shared.clone(), debug.clone())?;
    if debug.clone() {
        BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("Updated-OrderedVariables-withBoundaryConditionVars")).clone())?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("Updated-OrderedVariables-withBoundaryConditionEqs")).clone())?;
        BackendDump::dumpVariables(shared.globalKnownVars.clone(), (literal!("Updated-GlobalKnownVars-withBoundaryConditionVarsRemoved")).clone())?;
    }
    allVarsList = List::intRange(BackendVariable::varsSize(currentSystem.orderedVars.clone())?);
    (adjacencyMatrix, _, _, _) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    (knowns, boundaryConditionVars, exactEquationVars, _) = getVariablesBlockCategories(currentSystem.orderedVars.clone(), allVarsList.clone())?;
    if debug.clone() {
        println!("{}", (literal!("\nVariablesCategories\n=============================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nknownVars                    :")); __mm_s.push_str(&*dumplistInteger(knowns.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nboundaryConditionVars        :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nexactEquationVars            :")); __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nadjacencyMatrix              :")); __mm_s.push_str(&*anyString(adjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    allVarsList = List::intRange(BackendVariable::varsSize(currentSystem.orderedVars.clone())?);
    knownVariablesWithEquationBinding = getUncertainRefineVariablesBindedEquations(adjacencyMatrix.clone(), knowns.clone());
    if debug.clone() {
        println!("{}", (literal!("\nEquations with KnownBindings:\n===================================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nAdjacency Matrix                     :")); __mm_s.push_str(&*anyString(adjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nLength of Adjacency Matrix           :")); __mm_s.push_str(&*intString((adjacencyMatrix.clone().borrow().len() as i32))); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nList of known equation with bindings :")); __mm_s.push_str(&*anyString(knownVariablesWithEquationBinding.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    newEqnsLst = inverseModelicaModel(currentSystem.orderedVars.clone(), knownVariablesWithEquationBinding.clone())?;
    assign_field!(currentSystem.orderedEqs = BackendEquation::merge(currentSystem.orderedEqs.clone(), BackendEquation::listEquation(newEqnsLst.clone())?)?);
    BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("OverDetermined-System-Equations")).clone())?;
    (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    varCount = currentSystem.orderedVars.numberOfVars.clone();
    eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
    if debug.clone() {
        println!("{}", (literal!("\nOverDetermined-Systems-Information :\n====================================\n")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nAdjacency Matrix     :")); __mm_s.push_str(&*anyString(adjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nNumber of Vars       :")); __mm_s.push_str(&*intString(varCount.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nNumber of Equations  :")); __mm_s.push_str(&*intString(eqCount.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n\n")).clone());
    }
    (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount.clone(), eqCount.clone())?;
    BackendDump::dumpMatching(match1.clone())?;
    (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
    unMatchedEqsLst = List::setDifference(List::intRange(eqCount.clone()), matchedEqsLst.clone())?;
    unMatchedEqsLstCorrectIndex = List::unique(List::map1r(unMatchedEqsLst.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
    if debug.clone() {
        println!("{}", (literal!("\nFinding unmatched subset of equations :\n=========================================\n")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSolvedEqsAndVarsInfo                   :")); __mm_s.push_str(&*anyString(solvedEqsAndVarsInfo.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nList of Equations                      :")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(currentSystem.orderedEqs.clone()))); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nMatchedEquationsLst                    :")); __mm_s.push_str(&*anyString(List::sort(matchedEqsLst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?)); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSizeofMatchedEquationLST               :")); __mm_s.push_str(&*intString((matchedEqsLst.clone().len() as i32))); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnMatchedSubSetOfEquations             :")); __mm_s.push_str(&*anyString(unMatchedEqsLst.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnMatchedSubSetOfEquationsMappedIndex  :")); __mm_s.push_str(&*anyString(unMatchedEqsLstCorrectIndex.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    (e_BLT_EquationsWithIndex, eBltAdjacencyMatrix, e_BLTSolvedEqsAndVars, e_BLTBlocks, e_BLTBlockRanks) = setEBLTEquationsWithIndexAndRank(unMatchedEqsLst.clone(), unMatchedEqsLstCorrectIndex.clone(), currentSystem.orderedEqs.clone(), adjacencyMatrix.clone())?;
    BackendDump::dumpEquationList(List::map1r(unMatchedEqsLstCorrectIndex.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E-BLT-Equations ")); __mm_s.push_str(&*dumplistInteger(unMatchedEqsLst.clone())); ArcStr::from(__mm_s) }).clone());
    if debug.clone() {
        println!("{}", (literal!("\nE-BLT Information\n================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nE-BLT-Blocks   :")); __mm_s.push_str(&*anyString(e_BLTBlocks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nE-BLT-Blocks-with ranks   :")); __mm_s.push_str(&*anyString(e_BLTBlockRanks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nE-BLT-Adjacency-Matrix    :")); __mm_s.push_str(&*anyString(eBltAdjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nE_BLTSolvedEqsAndVars     :")); __mm_s.push_str(&*anyString(e_BLTSolvedEqsAndVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    currentSystem = deleteEquationsFromEqSyst(currentSystem.clone(), unMatchedEqsLstCorrectIndex.clone())?;
    varCount = currentSystem.orderedVars.numberOfVars.clone();
    eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
    BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), (literal!("reOrdered-Equations-after-removal")).clone())?;
    BackendDump::dumpVariables(currentSystem.orderedVars.clone(), (literal!("reOrderedVariables")).clone())?;
    (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(currentSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount.clone(), eqCount.clone())?;
    BackendDump::dumpMatching(match1.clone())?;
    s_BLTBlocks = Sorting::Tarjan(adjacencyMatrix.clone(), match1.clone(), (match1.clone().borrow().len() as i32))?;
    sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
    (solvedEqsAndVarsInfo, _) = getSolvedEquationAndVarsInfo(match1.clone());
    s_BLTBlockRanks = List::toListWithPositions(s_BLTBlocks.clone());
    if debug.clone() {
        println!("{}", (literal!("\nS-BLT-Information\n================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS-BLT Number of Vars       :")); __mm_s.push_str(&*intString(varCount.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS-BLT Number of Equations  :")); __mm_s.push_str(&*intString(eqCount.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS-BLT-Blocks               :")); __mm_s.push_str(&*anyString(s_BLTBlocks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS-BLT-Blocks-with ranks    :")); __mm_s.push_str(&*anyString(s_BLTBlockRanks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS-BLT Adjacency Matrix     :")); __mm_s.push_str(&*anyString(sBltAdjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS_BLTSolvedEqsAndVars      :")); __mm_s.push_str(&*anyString(solvedEqsAndVarsInfo.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n\n")).clone());
    }
    s_BLTBlocks = listAppend(s_BLTBlocks.clone(), e_BLTBlocks.clone());
    s_BLTBlockRanks = listAppend(s_BLTBlockRanks.clone(), e_BLTBlockRanks.clone());
    sBltAdjacencyMatrix = listAppend(sBltAdjacencyMatrix.clone(), eBltAdjacencyMatrix.clone());
    solvedEqsAndVarsInfo = listAppend(solvedEqsAndVarsInfo.clone(), e_BLTSolvedEqsAndVars.clone());
    if debug.clone() {
        println!("{}", (literal!("\nCombined S-BLT and E-BLT Information\n================================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nCombined S-BLT-Blocks and E-BLT-Blocks                :")); __mm_s.push_str(&*anyString(s_BLTBlocks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nCombined S-BLT-Blocks and E-BLT-Blocks with Ranks     :")); __mm_s.push_str(&*anyString(s_BLTBlockRanks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nCombined Adjacency Matrix with S-BLT and E-BLT        :")); __mm_s.push_str(&*anyString(sBltAdjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nCombined SolvedEquationsVarsInfo with S-BLT and E-BLT :")); __mm_s.push_str(&*anyString(solvedEqsAndVarsInfo.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    dumpListList(s_BLTBlocks.clone(), (literal!("BLT_BLOCKS")).clone());
    (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(BackendEquation::equationList(currentSystem.orderedEqs.clone()), 1)?;
    if debug.clone() {
        BackendDump::dumpEquationList(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("ApproximatedEquations")).clone());
        BackendDump::dumpEquationList(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), currentSystem.orderedEqs.clone()), (literal!("boundaryConditionEquations")).clone());
    }
    approximatedEquations = List::flatten(List::map1r(approximatedEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
    boundaryConditionEquations = List::flatten(List::map1r(boundaryConditionEquations.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), Arc::new(mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())));
    if debug.clone() {
        println!("{}", (literal!("\nApproximated and BoundaryCondition Equation Indexes :\n===========================================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :")); __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    boundaryConditionTaggedEquationSolvedVars = getBoundaryConditionVariables(boundaryConditionEquations.clone(), solvedEqsAndVarsInfo.clone());
    if debug.clone() {
        BackendDump::dumpVarList(List::map1r(boundaryConditionTaggedEquationSolvedVars.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()), (literal!("boundaryConditionTaggedEquationSolvedVars")).clone());
    }
    exactEquationVars = List::setDifferenceOnTrue(exactEquationVars.clone(), boundaryConditionTaggedEquationSolvedVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    boundaryConditionVars = listAppend(boundaryConditionVars.clone(), boundaryConditionTaggedEquationSolvedVars.clone());
    if debug.clone() {
        println!("{}", (literal!("\nUpdatedVariablesCategories\n=============================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nknownVars                    :")); __mm_s.push_str(&*dumplistInteger(knowns.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nboundaryConditionVars        :")); __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nexactEquationVars            :")); __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    (allBlocks, allBlocksStatusVarInfo) = traverseBLTAndUpdateBlockStatus(s_BLTBlocks.clone(), knowns.clone(), boundaryConditionVars.clone(), exactEquationVars.clone(), solvedEqsAndVarsInfo.clone());
    if debug.clone() {
        dumpBlockStatus(allBlocks.clone(), allBlocksStatusVarInfo.clone());
    }
    s_BLTBlockTargetInfo = findBlockTargets(allBlocks.clone(), allBlocksStatusVarInfo.clone(), solvedEqsAndVarsInfo.clone(), sBltAdjacencyMatrix.clone(), s_BLTBlockRanks.clone(), debug.clone())?;
    if debug.clone() {
        dumpBlockTargets(s_BLTBlockTargetInfo.clone());
    }
    predecessorBlockTargetInfo = findPredecessorBlocks(s_BLTBlockTargetInfo.clone())?;
    dumpPredecessorBlocks(predecessorBlockTargetInfo.clone());
    (tempSetC, tempSetS) = ExtractEquationsUsingSetOperations(predecessorBlockTargetInfo.clone(), e_BLTBlockRanks.clone(), approximatedEquations.clone(), debug.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFINAL SET OF EQUATIONS After Reconciliation\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_C: ")); __mm_s.push_str(&*dumplistInteger(tempSetC.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(tempSetS.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    if debug.clone() {
        dumpSetSVarsSolvedInfo(tempSetS.clone(), solvedEqsAndVarsInfo.clone(), mapIncRowEqn.clone(), currentSystem.orderedEqs.clone(), currentSystem.orderedVars.clone(), (literal!("Set-S Solved-Variables Information")).clone())?;
    }
    setC = List::unique(getAbsoluteIndexHelper(tempSetC.clone(), mapIncRowEqn.clone()));
    setS = List::unique(getAbsoluteIndexHelper(tempSetS.clone(), mapIncRowEqn.clone()));
    setC_Eq = getEquationsFromSBLTAndEBLT(setC.clone(), currentSystem.orderedEqs.clone(), e_BLT_EquationsWithIndex.clone());
    setS_Eq = getEquationsFromSBLTAndEBLT(setS.clone(), currentSystem.orderedEqs.clone(), e_BLT_EquationsWithIndex.clone());
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setC_Eq.clone())?, (literal!("SET_C")).clone())?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(setS_Eq.clone())?, (literal!("SET_S")).clone())?;
    unknownVarsInSetC = getVariablesAfterExtraction(tempSetC.clone(), metamodelica::nil(), sBltAdjacencyMatrix.clone());
    unknownVarsInSetC = List::setDifferenceOnTrue(unknownVarsInSetC.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?.reverse();
    setS_BLTAdjacencyMatrix = getSetSAdjacencyMatrix(sBltAdjacencyMatrix.clone(), tempSetS.clone());
    if debug.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart of Extract Minimal Set-S Algorithm\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSet-S Adjacency MAtrix : ")); __mm_s.push_str(&*intString((setS_BLTAdjacencyMatrix.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*anyString(setS_BLTAdjacencyMatrix.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\nS'        : {}")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nV_C       :")); __mm_s.push_str(&*dumplistInteger(unknownVarsInSetC.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (_, tempSetS) = extractMinimalSetS(unknownVarsInSetC.clone(), setS_BLTAdjacencyMatrix.clone(), knowns.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), metamodelica::nil(), debug.clone())?;
    if debug.clone() {
        println!("{}", (literal!("\n****End of Minimal extraction Algorithm****\n")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSet-S after running minimal extraction algorithm\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(tempSetS.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    extractedVarsfromSetS = getVariablesAfterExtraction(metamodelica::nil(), tempSetS.clone(), sBltAdjacencyMatrix.clone());
    extractedVarsfromSetS = List::setDifferenceOnTrue(extractedVarsfromSetS.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setC = List::unique(getAbsoluteIndexHelper(tempSetC.clone(), mapIncRowEqn.clone()));
    setS = List::unique(getAbsoluteIndexHelper(tempSetS.clone(), mapIncRowEqn.clone()));
    setC_Eq = getEquationsFromSBLTAndEBLT(setC.clone(), currentSystem.orderedEqs.clone(), e_BLT_EquationsWithIndex.clone());
    setS_Eq = getEquationsFromSBLTAndEBLT(setS.clone(), currentSystem.orderedEqs.clone(), e_BLT_EquationsWithIndex.clone());
    if !(tempSetS.clone().is_empty()) {
        BackendDump::dumpEquationArray(BackendEquation::listEquation(setS_Eq.clone())?, (literal!("SET_S_After_Minimal_Extraction")).clone())?;
    } else {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSET_S_After_Minimal_Extraction (0, 0)\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    outDiffVars = BackendVariable::listVar(List::map1r(knowns.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    outDiffVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarUnreplaceable, BackendDAE::Var, bool)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<BackendDAE::Var> + 'static>), true));
    (_, residualEquations) = BackendEquation::traverseEquationArray(BackendEquation::listEquation(setC_Eq.clone())?, (std::sync::Arc::new(BackendEquation::traverseEquationToScalarResidualForm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))> + 'static>), (shared.functionTree.clone(), metamodelica::nil()))?;
    (residualEquations, residualVars, _) = BackendEquation::convertResidualsIntoSolvedEquations(residualEquations.clone().reverse(), (literal!("$res_F_")).clone(), 1, false)?;
    outResidualVars = BackendVariable::listVar(residualVars.clone().reverse());
    outResidualEqns = BackendEquation::listEquation(residualEquations.clone())?;
    outOtherEqns = BackendEquation::listEquation(setS_Eq.clone())?;
    paramVars = BackendEquation::equationsVars(outOtherEqns.clone(), shared.globalKnownVars.clone())?;
    outOtherVars = BackendVariable::listVar(List::map1r(extractedVarsfromSetS.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), currentSystem.orderedVars.clone()));
    dumpSetSVars(outOtherVars.clone(), (literal!("Unknown variables in SET_S ")).clone())?;
    BackendDump::dumpVariables(BackendVariable::listVar(paramVars.clone()), (literal!("Parameters in SET_S")).clone())?;
    auxillaryConditionsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_AuxiliaryConditions.html")); ArcStr::from(__mm_s) }).clone();
    auxillaryEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(setC_Eq.clone())?, (literal!("Auxiliary conditions")).clone())?).clone();
    System::writeFile((auxillaryConditionsFilename.clone()).clone(), (auxillaryEquations.clone()).clone())?;
    intermediateEquationsFilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_IntermediateEquations.html")); ArcStr::from(__mm_s) }).clone();
    intermediateEquations = (dumpExtractedEquationsToHTML(BackendEquation::listEquation(setS_Eq.clone())?, (literal!("Intermediate equations")).clone())?).clone();
    System::writeFile((intermediateEquationsFilename.clone()).clone(), (intermediateEquations.clone()).clone())?;
    VerifyDataReconciliation(tempSetC.clone(), tempSetS.clone(), knowns.clone(), boundaryConditionVars.clone(), sBltAdjacencyMatrix.clone(), solvedEqsAndVarsInfo.clone(), exactEquationVars.clone(), approximatedEquations.clone(), currentSystem.orderedVars.clone(), currentSystem.orderedEqs.clone(), mapIncRowEqn.clone(), outOtherVars.clone(), setS_Eq.clone(), shared.clone(), setC.clone(), setS.clone(), 0)?;
    if debug.clone() {
        BackendDump::dumpVariables(outDiffVars.clone(), (literal!("Jacobian_knownVariables")).clone())?;
        BackendDump::dumpVariables(outResidualVars.clone(), (literal!("Jacobian_outResidualVars")).clone())?;
        BackendDump::dumpVariables(outOtherVars.clone(), (literal!("Jacobian_outOtherVars")).clone())?;
        BackendDump::dumpEquationArray(outResidualEqns.clone(), (literal!("Jacobian_ResidualEquation")).clone())?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), (literal!("Jacobian_other_Equation")).clone())?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(outDiffVars.clone(), outResidualEqns.clone(), outResidualVars.clone(), outOtherEqns.clone(), outOtherVars.clone(), shared.clone(), outOtherVars.clone(), (literal!("F")).clone(), false)?;
    assign_field!(shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData { relatedBoundaryConditions: 0, symbolicJacobianH: None, setBVars: None, datareconinputs: outDiffVars.clone(), setcVars: outResidualVars.clone(), symbolicJacobian: simCodeJacobian.clone() }));
    currentSystem = BackendDAEUtil::setEqSystVars(currentSystem.clone(), BackendVariable::mergeVariables(outResidualVars.clone(), outOtherVars.clone(), true)?)?;
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), BackendEquation::merge(outResidualEqns.clone(), outOtherEqns.clone())?);
    inputVars = BackendVariable::listVar(List::map1(BackendVariable::varList(outDiffVars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::setVarDirection, BackendDAE::Var, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, DAE::VarDirection) -> Result<BackendDAE::Var> + 'static>), openmodelica_frontend_types::DAE::VarDirection::INPUT));
    shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), BackendVariable::mergeVariables(shared.globalKnownVars.clone(), inputVars.clone(), true)?);
    if !(System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone())) {
        r#str = (literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval\n")).clone();
        r#str = (dumpToCsv((r#str.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?)?).clone();
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
    }
    modelicaFileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Reconciled_tmp")); ArcStr::from(__mm_s) }).clone();
    modelName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reconciled_")); __mm_s.push_str(&*System::stringReplace((shared.info.fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (literal!("/* This is a Reconciled Model which is generated by the Data Reconciliation extraction algorithm */\n")).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("model ")); __mm_s.push_str(&*modelName.clone()); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outDiffVars.clone())?, (literal!("Variables of Interest")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), paramVars.clone(), (literal!("parameters in SET-S")).clone())?).clone();
    modelicaOutput = (dumpResidualVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outResidualVars.clone())?, (literal!("residualVars")).clone())?).clone();
    modelicaOutput = (dumpExtractedVars((modelicaOutput.clone()).clone(), BackendVariable::varList(outOtherVars.clone())?, (literal!("remaining variables in setS")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nequation")); ArcStr::from(__mm_s) }).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), outResidualEqns.clone(), (literal!("set-C Canonical form")).clone())?).clone();
    modelicaOutput = (dumpExtractedEquations((modelicaOutput.clone()).clone(), outOtherEqns.clone(), (literal!("remaining equations in Set-S")).clone())?).clone();
    modelicaOutput = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaOutput.clone()); __mm_s.push_str(&*literal!("\nend ")); __mm_s.push_str(&*modelName.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
    System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelicaFileName.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone(), (modelicaOutput.clone()).clone())?;
    outDAE = BackendDAE::DAE(list![currentSystem.clone()], shared.clone())?;
    Ok(outDAE)
}

fn getSetSAdjacencyMatrix(mut sBltAdjacencyMatrix: ExtAdjacencyMatrix, mut setS: Arc<metamodelica::List<i32>>) -> ExtAdjacencyMatrix {
    let mut setS_BltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut eq: i32 = 0;
    for mut i in &*sBltAdjacencyMatrix.clone() {
        let mut i = i.clone();
        (eq, _) = i.clone();
        if listMember(eq.clone(), setS.clone()) {
            setS_BltAdjacencyMatrix = cons(i.clone(), setS_BltAdjacencyMatrix.clone());
        }
    }
    setS_BltAdjacencyMatrix
}

fn extractMinimalSetS(mut unknownsInSetC: Arc<metamodelica::List<i32>>, mut sBltAdjacencyMatrix: ExtAdjacencyMatrix, mut knownVars: Arc<metamodelica::List<i32>>, mut orderedVars: BackendDAE::Variables, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut minimalSetS: Arc<metamodelica::List<i32>>, mut debug: bool) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut unknownsInSetC: Arc<metamodelica::List<i32>> = unknownsInSetC;
    let mut minimalSetS: Arc<metamodelica::List<i32>> = minimalSetS;
    let mut firstMatchedEquation: i32 = 0;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intermediateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut V_EQ: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut varIndex in &*unknownsInSetC.clone() {
        let mut varIndex = varIndex.clone();
        if unknownsInSetC.clone().is_empty() {
            break;
        }
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nIntermediate varList : ")); __mm_s.push_str(&*dumplistInteger(unknownsInSetC.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        let __pa0 = ::match_deref::match_deref! { match &(unknownsInSetC.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rest = __pa0.clone();
        (firstMatchedEquation, vars) = getVariableFirstOccurrenceInEquation(sBltAdjacencyMatrix.clone(), varIndex.clone(), minimalSetS.clone());
        var = BackendVariable::getVarAt(orderedVars.clone(), varIndex.clone())?;
        if !(intEq(firstMatchedEquation.clone(), 0)) {
            minimalSetS = cons(firstMatchedEquation.clone(), minimalSetS.clone());
            minimalSetS = List::unique(minimalSetS.clone());
            intermediateVars = List::setDifferenceOnTrue(vars.clone(), knownVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            V_EQ = List::unique(listAppend(intermediateVars.clone(), rest.clone()));
            if debug.clone() {
                dumpMininimalExtraction(varIndex.clone(), var.clone(), firstMatchedEquation.clone(), mapIncRowEqn.clone(), orderedEqs.clone(), minimalSetS.clone(), intermediateVars.clone(), rest.clone(), V_EQ.clone(), false, metamodelica::nil())?;
            }
            (unknownsInSetC, minimalSetS) = extractMinimalSetS(V_EQ.clone(), sBltAdjacencyMatrix.clone(), knownVars.clone(), orderedVars.clone(), orderedEqs.clone(), mapIncRowEqn.clone(), minimalSetS.clone(), debug.clone())?;
        } else {
            if debug.clone() {
                dumpMininimalExtraction(varIndex.clone(), var.clone(), 0, mapIncRowEqn.clone(), orderedEqs.clone(), metamodelica::nil(), metamodelica::nil(), rest.clone(), metamodelica::nil(), true, metamodelica::nil())?;
            }
            unknownsInSetC = rest.clone();
        }
    }
    Ok((unknownsInSetC, minimalSetS))
}

fn dumpMininimalExtraction(mut varIndex: i32, mut var: BackendDAE::Var, mut firstMatchedEquation: i32, mut mapIncRowEqn: metamodelica::Array<i32>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut minimalSetS: Arc<metamodelica::List<i32>>, mut intermediateVars: Arc<metamodelica::List<i32>>, mut rest: Arc<metamodelica::List<i32>>, mut V_EQ: Arc<metamodelica::List<i32>>, mut falseBlock: bool, mut visitedVars: Arc<metamodelica::List<i32>>) -> Result<()> {
    let mut mappedEq: i32 = 0;
    let mut tmpEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    if falseBlock.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nVarIndex           : ")); __mm_s.push_str(&*intString(varIndex.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nVariable Name      : ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEquation Not Exist : ")); __mm_s.push_str(&*literal!("NIL")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nRemainingVars      : ")); __mm_s.push_str(&*dumplistInteger(rest.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        mappedEq = mapIncRowEqn.borrow()[(firstMatchedEquation.clone()-1) as usize].clone();
        tmpEq = BackendEquation::get(orderedEqs.clone(), mappedEq.clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nVarIndex                     : ")); __mm_s.push_str(&*intString(varIndex.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nVariable Name                : ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nEquation Exist               : ")); __mm_s.push_str(&*intString(firstMatchedEquation.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmappedEquation               : ")); __mm_s.push_str(&*intString(mappedEq.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nMatched Equation             : ")); __mm_s.push_str(&*BackendDump::equationString(tmpEq.clone())?); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nS'                           : ")); __mm_s.push_str(&*dumplistInteger(minimalSetS.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnknowns in matchedEquation  : ")); __mm_s.push_str(&*dumplistInteger(intermediateVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nVisited vars                 : ")); __mm_s.push_str(&*dumplistInteger(visitedVars.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nRemaining Vars               : ")); __mm_s.push_str(&*dumplistInteger(rest.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nV_EQ                         : ")); __mm_s.push_str(&*dumplistInteger(V_EQ.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn getVariableFirstOccurrenceInEquation(mut m: ExtAdjacencyMatrix, mut varIndex: i32, mut minimalSetS: Arc<metamodelica::List<i32>>) -> (i32, Arc<metamodelica::List<i32>>) {
    let mut matchedEquation: (i32, Arc<metamodelica::List<i32>>) = (0, metamodelica::nil());
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    for mut i in &*m.clone() {
        let mut i = i.clone();
        (eq, vars) = i.clone();
        if eq.clone() > 0 {
            if !(listMember(eq.clone(), minimalSetS.clone())) {
                if listMember(varIndex.clone(), vars.clone()) {
                    matchedEquation = i.clone();
                    break;
                }
            }
        }
    }
    matchedEquation
}

fn dumpResidualVars(mut instring: ArcStr, mut invar: Arc<metamodelica::List<BackendDAE::Var>>, mut comment: ArcStr) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  //")); __mm_s.push_str(&*comment.clone()); ArcStr::from(__mm_s) }).clone();
    for mut var in &*invar.clone() {
        let mut var = var.clone();
        cr = BackendVariable::varCref(var.clone())?;
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.varType.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::stringReplace((ComponentReference::crefStr(cr.clone())?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
        outstring = (System::stringReplace((outstring.clone()).clone(), (literal!("$")).clone(), (literal!("")).clone())?).clone();
    }
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*instring.clone()); __mm_s.push_str(&*outstring.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outstring)
}

fn dumpExtractedVars(mut instring: ArcStr, mut invar: Arc<metamodelica::List<BackendDAE::Var>>, mut comment: ArcStr) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut creflast: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut isRec: bool = false;
    let mut path: Arc<Absyn::Path>;
    let mut recordvarlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  //")); __mm_s.push_str(&*comment.clone()); ArcStr::from(__mm_s) }).clone();
    recordvarlist = metamodelica::nil();
    for mut var in &*invar.clone() {
        let mut var = var.clone();
        cr = BackendVariable::varCref(var.clone())?;
        (cr1, isRec) = ComponentReference::crefGetFirstRec(cr.clone())?;
        if BackendVariable::varHasUncertainValueRefine(var.clone()) {
            outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n  parameter ")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.varType.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::stringReplace((ComponentReference::crefStr(cr.clone())?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
        } else if BackendVariable::isParam(var.clone()) {
            outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n  parameter ")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.varType.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::stringReplace((ComponentReference::crefStr(cr.clone())?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionDump::printOptExpStr(var.bindExp.clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
        } else if isRec.clone() && !(listMember((ComponentReference::crefStr(cr1.clone())?).clone(), recordvarlist.clone())) {
            creflast = ComponentReferenceBasics::crefLastCref(cr1.clone())?;
            path = Types::getRecordPath(ComponentReference::crefType(creflast.clone())?)?;
            recordvarlist = cons((ComponentReference::crefStr(cr1.clone())?).clone(), recordvarlist.clone());
            outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::stringReplace((ComponentReference::crefStr(cr1.clone())?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
        } else if !(isRec.clone()) {
            outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.varType.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::stringReplace((ComponentReference::crefStr(cr.clone())?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
        }
    }
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*instring.clone()); __mm_s.push_str(&*outstring.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outstring)
}

fn dumpExtractedEquations(mut instring: ArcStr, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut comment: ArcStr) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  //")); __mm_s.push_str(&*comment.clone()); ArcStr::from(__mm_s) }).clone();
    for mut eq in &*BackendEquation::equationList(eqs.clone()) {
        let mut eq = eq.clone();
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*dumpEquationString(eq.clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
    }
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*instring.clone()); __mm_s.push_str(&*outstring.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outstring)
}

fn dumpExtractedEquationsToHTML(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut comment: ArcStr) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    if BackendEquation::equationList(eqs.clone()).is_empty() {
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The set of ")); __mm_s.push_str(&*comment.clone()); __mm_s.push_str(&*literal!(" is empty.")); ArcStr::from(__mm_s) }).clone();
    } else {
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<html>\n<body>\n<h2>")); __mm_s.push_str(&*comment.clone()); __mm_s.push_str(&*literal!("</h2>\n<ol>")); ArcStr::from(__mm_s) }).clone();
        for mut eq in &*BackendEquation::equationList(eqs.clone()) {
            let mut eq = eq.clone();
            outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("  <li>")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(BackendEquation::equationSize(eq.clone())?)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!(" </li>")); ArcStr::from(__mm_s) }).clone();
        }
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*literal!("\n</ol>\n</body>\n</html>")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outstring)
}

pub fn setBoundaryConditionEquationsAndVars(mut currentSystem: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut debug: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut currentSystem: Arc<BackendDAE::EqSystem> = currentSystem;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut daeVarsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut updatedGlobalKnownVarsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut var in &*BackendVariable::varList(shared.globalKnownVars.clone())? {
        let mut var = var.clone();
        if BackendVariable::isRealParam(var.clone()) && (BackendVariable::hasOpenModelicaBoundaryConditionAnnotation(var.clone())? || BackendVariable::varHasUncertainValueRefine(var.clone()) || BackendVariable::varHasUncertainValuePropagate(var.clone())) {
            lhs = BackendVariable::varExp(var.clone())?;
            rhs = BackendVariable::varBindExpStartValueNoFail(var.clone())?;
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
            eqnLst = cons(eqn.clone(), eqnLst.clone());
            var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
            var = BackendVariable::setBindExp(var.clone(), None);
            daeVarsLst = cons(var.clone(), daeVarsLst.clone());
        } else if (BackendVariable::isIntParam(var.clone()) || BackendVariable::isBoolParam(var.clone())) && BackendVariable::hasOpenModelicaBoundaryConditionAnnotation(var.clone())? {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(": Boundary Condition cannot be set on Integer or Boolean parameters: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(" must be Real, The extraction algorithm will fail")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        } else {
            updatedGlobalKnownVarsLst = cons(var.clone(), updatedGlobalKnownVarsLst.clone());
        }
    }
    if debug.clone() {
        BackendDump::dumpVarList(daeVarsLst.clone(), (literal!("boundaryConditionVarsTaggedAsParmeters")).clone());
    }
    currentSystem = BackendVariable::addVarsDAE(daeVarsLst.clone(), currentSystem.clone());
    assign_field!(currentSystem.orderedEqs = BackendEquation::merge(currentSystem.orderedEqs.clone(), BackendEquation::listEquation(eqnLst.clone())?)?);
    shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), BackendVariable::listVar(updatedGlobalKnownVarsLst.clone()));
    Ok((currentSystem, shared))
}

fn deleteEquationsFromEqSyst(mut currentSystem: Arc<BackendDAE::EqSystem>, mut eqIndex: Arc<metamodelica::List<i32>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut currentSystem: Arc<BackendDAE::EqSystem> = currentSystem;
    let mut newOrderedEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    assign_field!(currentSystem.orderedEqs = BackendEquation::deleteList(currentSystem.orderedEqs.clone(), eqIndex.clone())?);
    newOrderedEquationArray = BackendEquation::emptyEqns();
    BackendEquation::addList(BackendEquation::equationList(currentSystem.orderedEqs.clone()), newOrderedEquationArray.clone())?;
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem.clone(), newOrderedEquationArray.clone());
    Ok(currentSystem)
}

fn getBoundaryConditionsEquationIndex(mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut boundaryConditions: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut boundaryConditionsEquationIndexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = adjacencyMatrix.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        for mut j in &*boundaryConditions.clone() {
            let mut j = j.clone();
            if i.clone() == list![j.clone()] {
                boundaryConditionsEquationIndexes = cons(count.clone(), boundaryConditionsEquationIndexes.clone());
                break;
            }
        }
        count = count.clone() + 1;
    }
    boundaryConditionsEquationIndexes
}

fn getUncertainRefineVariablesBindedEquations(mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut knowns: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut knownsWithBindedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __range0 = adjacencyMatrix.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        for mut j in &*knowns.clone() {
            let mut j = j.clone();
            if i.clone() == list![j.clone()] {
                knownsWithBindedEquations = cons(j.clone(), knownsWithBindedEquations.clone());
            }
        }
    }
    knownsWithBindedEquations
}

fn getExactConstantVariables(mut constantEquations: Arc<metamodelica::List<i32>>, mut solvedEqsVarInfo: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<i32>> {
    let mut constantVariables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varNumber: i32 = 0;
    for mut eq in &*constantEquations.clone() {
        let mut eq = eq.clone();
        varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo.clone());
        constantVariables = cons(varNumber.clone(), constantVariables.clone());
    }
    constantVariables
}

fn getBoundaryConditionVariables(mut boundaryConditionEquations: Arc<metamodelica::List<i32>>, mut solvedEqsVarInfo: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<i32>> {
    let mut boundaryConditionVariables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varNumber: i32 = 0;
    for mut eq in &*boundaryConditionEquations.clone() {
        let mut eq = eq.clone();
        varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo.clone());
        boundaryConditionVariables = cons(varNumber.clone(), boundaryConditionVariables.clone());
    }
    boundaryConditionVariables
}

fn getEquationsFromSBLTAndEBLT(mut inList: Arc<metamodelica::List<i32>>, mut sBLT_Equations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut eBLT_Equations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>)>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEquationsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut eqIndex in &*inList.clone() {
        let mut eqIndex = eqIndex.clone();
        if eqIndex.clone() > 0 {
            outEquationsList = cons(BackendEquation::get(sBLT_Equations.clone(), eqIndex.clone()), outEquationsList.clone());
        } else {
            outEquationsList = cons(getEquationsFromEBLT(eqIndex.clone(), eBLT_Equations.clone()), outEquationsList.clone());
        }
    }
    outEquationsList = outEquationsList.clone().reverse();
    outEquationsList
}

fn getEquationsFromEBLT(mut eBLTIndex: i32, mut eBLT_Equations: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>)>>) -> Arc<BackendDAE::Equation> {
    let mut outEquations: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut index: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut eqs in &*eBLT_Equations.clone() {
        let mut eqs = eqs.clone();
        (index, eq) = eqs.clone();
        if intEq(eBLTIndex.clone(), index.clone()) {
            outEquations = eq.clone();
            break;
        }
    }
    outEquations
}

fn getAbsoluteIndexHelper(mut inList: Arc<metamodelica::List<i32>>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*inList.clone() {
        let mut i = i.clone();
        if i.clone() > 0 {
            outList = cons(mapIncRowEqn.borrow()[(i.clone()-1) as usize].clone(), outList.clone());
        } else {
            outList = cons(i.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    outList
}

fn dumpSetSTargetEquations(mut eq: i32, mut solvedEqsVarInfo: Arc<metamodelica::List<(i32, i32)>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut orderedVars: BackendDAE::Variables, mut heading: ArcStr) -> Result<()> {
    let mut count: i32 = 1;
    let mut varNumber: i32 = 0;
    let mut mappedEq: i32 = 0;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut tmpEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo.clone());
    var = BackendVariable::getVarAt(orderedVars.clone(), varNumber.clone())?;
    mappedEq = mapIncRowEqn.borrow()[(eq.clone()-1) as usize].clone();
    tmpEq = BackendEquation::get(orderedEqs.clone(), mappedEq.clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*intString(varNumber.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(mappedEq.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(BackendEquation::equationSize(tmpEq.clone())?)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(tmpEq.clone())?); ArcStr::from(__mm_s) }).clone());
    count = count.clone() + 1;
    Ok(())
}

fn dumpSetSVarsSolvedInfo(mut tempSetS: Arc<metamodelica::List<i32>>, mut solvedEqsVarInfo: Arc<metamodelica::List<(i32, i32)>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut orderedVars: BackendDAE::Variables, mut heading: ArcStr) -> Result<()> {
    let mut count: i32 = 1;
    let mut varNumber: i32 = 0;
    let mut mappedEq: i32 = 0;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut tmpEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    if !(stringEmpty((heading.clone()).clone())) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((tempSetS.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("\n============================================================\n")); ArcStr::from(__mm_s) }).clone());
    }
    for mut eq in &*tempSetS.clone() {
        let mut eq = eq.clone();
        varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo.clone());
        var = BackendVariable::getVarAt(orderedVars.clone(), varNumber.clone())?;
        mappedEq = mapIncRowEqn.borrow()[(eq.clone()-1) as usize].clone();
        tmpEq = BackendEquation::get(orderedEqs.clone(), mappedEq.clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(varNumber.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(mappedEq.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(BackendEquation::equationSize(tmpEq.clone())?)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(tmpEq.clone())?); ArcStr::from(__mm_s) }).clone());
        count = count.clone() + 1;
    }
    println!("{}", (literal!("\n\n")).clone());
    Ok(())
}

fn dumpSetSVars(mut setSVars: BackendDAE::Variables, mut heading: ArcStr) -> Result<()> {
    let mut count: i32 = 1;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendVariable::varsSize(setSVars.clone())?)); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*literal!("========================================")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    for mut var in &*BackendVariable::varList(setSVars.clone())? {
        let mut var = var.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(count.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.varType.clone())?); ArcStr::from(__mm_s) }).clone());
        count = count.clone() + 1;
    }
    println!("{}", (literal!("\n\n")).clone());
    Ok(())
}

fn dumpBlockStatus(mut allBlocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut allBlocksStatusVarInfo: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> () {
    let mut count: i32 = 1;
    println!("{}", (literal!("\nBLT-BLOCK_STATUS\n=================\n")).clone());
    for mut blocks in &*allBlocks.clone() {
        let mut blocks = blocks.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBlock :")); __mm_s.push_str(&*dumplistInteger(blocks.clone())); __mm_s.push_str(&*literal!(" || blockStatusVarInfo :")); __mm_s.push_str(&*anyString((allBlocksStatusVarInfo.clone()).get(count.clone()).unwrap())); ArcStr::from(__mm_s) }).clone());
        count = count.clone() + 1;
    }
    println!("{}", (literal!("\n")).clone());
    ()
}

fn dumpBlockTargets(mut s_BLTBlockTargetInfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>>) -> () {
    let mut mainBlock: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetBlocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetBlocksStatusVarInfo: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    println!("{}", (literal!("\nS-BLTBlocks-TargetInfo\n=======================\n")).clone());
    for mut blocks in &*s_BLTBlockTargetInfo.clone() {
        let mut blocks = blocks.clone();
        (mainBlock, targetBlocks, targetBlocksStatusVarInfo) = blocks.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBlock :")); __mm_s.push_str(&*dumplistInteger(mainBlock.clone())); __mm_s.push_str(&*literal!(" || blockTargetsInfo :")); __mm_s.push_str(&*anyString(targetBlocks.clone())); __mm_s.push_str(&*literal!(" || blockStatusVarInfo :")); __mm_s.push_str(&*anyString(targetBlocksStatusVarInfo.clone())); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("\n")).clone());
    ()
}

fn dumpPredecessorBlocks(mut predecessorBlockInfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> () {
    let mut knownBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constantBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blueBlocksTargets: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut redBlocksTargets: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut constantBlocksTargets: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    println!("{}", (literal!("\nTargets of blocks without predecessors:\n========================================")).clone());
    for mut blocks in &*predecessorBlockInfo.clone() {
        let mut blocks = blocks.clone();
        (_, _, _, knownBlocks, constantBlocks, _) = blocks.clone();
        if !(knownBlocks.clone().is_empty()) {
            blueBlocksTargets = cons(blocks.clone(), blueBlocksTargets.clone());
        } else if !(constantBlocks.clone().is_empty()) {
            constantBlocksTargets = cons(blocks.clone(), constantBlocksTargets.clone());
        } else {
            redBlocksTargets = cons(blocks.clone(), redBlocksTargets.clone());
        }
    }
    println!("{}", (literal!("\n")).clone());
    dumpPredecessorBlocksHelper(blueBlocksTargets.clone(), (literal!("knowns")).clone(), (literal!("Targets of Blue blocks")).clone());
    dumpPredecessorBlocksHelper(redBlocksTargets.clone(), (literal!("unknowns")).clone(), (literal!("Targets of Red blocks")).clone());
    dumpPredecessorBlocksHelper(constantBlocksTargets.clone(), (literal!("constant")).clone(), (literal!("Targets of Brown blocks")).clone());
    ()
}

fn dumpPredecessorBlocksHelper(mut predecessorBlockInfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>, mut blockInfo: ArcStr, mut header: ArcStr) -> () {
    let mut mainBlock: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetBlocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut knownBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constantBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*header.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((predecessorBlockInfo.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("\n==============================\n")); ArcStr::from(__mm_s) }).clone());
    for mut blocks in &*predecessorBlockInfo.clone().reverse() {
        let mut blocks = blocks.clone();
        (mainBlock, targetBlocks, _, knownBlocks, constantBlocks, _) = blocks.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBlock :")); __mm_s.push_str(&*dumplistInteger(mainBlock.clone())); __mm_s.push_str(&*literal!(" || blockTargetsInfo :")); __mm_s.push_str(&*anyString(targetBlocks.clone())); __mm_s.push_str(&*literal!(" || KnownBlocks :")); __mm_s.push_str(&*dumplistInteger(knownBlocks.clone())); __mm_s.push_str(&*literal!(" || constantBlocks :")); __mm_s.push_str(&*dumplistInteger(constantBlocks.clone())); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("\n\n")).clone());
    ()
}

pub fn ExtractEquationsUsingSetOperations(mut predecessorBlockInfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>, mut e_BLTBlockRanks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut approximatedEquations: Arc<metamodelica::List<i32>>, mut debug: bool) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut setC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut setS: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mainBlock: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSetC_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSetC_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSetS_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSetS_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut z1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut z2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetBlocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut knownBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constantBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_BLTBlockRanksWithoutRanks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetBlocksWithKnowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetBlocksWithUnknowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetBlocksWithConstants: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut blocks in &*predecessorBlockInfo.clone() {
        let mut blocks = blocks.clone();
        (mainBlock, targetBlocks, _, knownBlocks, constantBlocks, _) = blocks.clone();
        if !(knownBlocks.clone().is_empty()) {
            targetBlocksWithKnowns = filterTargetBlocksWithoutRanks(listRest(targetBlocks.clone())?, targetBlocksWithKnowns.clone());
        } else if !(constantBlocks.clone().is_empty()) {
            targetBlocksWithConstants = filterTargetBlocksWithoutRanks(targetBlocks.clone(), targetBlocksWithConstants.clone());
        } else {
            targetBlocksWithUnknowns = filterTargetBlocksWithoutRanks(targetBlocks.clone(), targetBlocksWithUnknowns.clone());
        }
    }
    targetBlocksWithKnowns = List::unique(targetBlocksWithKnowns.clone());
    targetBlocksWithUnknowns = List::unique(targetBlocksWithUnknowns.clone());
    targetBlocksWithConstants = List::unique(targetBlocksWithConstants.clone());
    e_BLTBlockRanksWithoutRanks = filterTargetBlocksWithoutRanks(e_BLTBlockRanks.clone(), e_BLTBlockRanksWithoutRanks.clone());
    if debug.clone() {
        println!("{}", (literal!("\nUnion of Blue, Red and Yellow and E-BLT-Blocks\n=====================================================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnion-E-BLT-blocks                                     :")); __mm_s.push_str(&*dumplistInteger(e_BLTBlockRanksWithoutRanks.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnion-Blue-TargetBlockInfo (blocks with Knowns)        :")); __mm_s.push_str(&*dumplistInteger(targetBlocksWithKnowns.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnion-Red-TargetBlockInfo  (blocks with UnKnowns)      :")); __mm_s.push_str(&*dumplistInteger(targetBlocksWithUnknowns.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nUnion-Brown-TargetBlockInfo  (blocks with Exact eqns)  :")); __mm_s.push_str(&*dumplistInteger(targetBlocksWithConstants.clone())); ArcStr::from(__mm_s) }).clone());
    }
    tmpSetC_1 = List::intersectionOnTrue(targetBlocksWithKnowns.clone(), e_BLTBlockRanksWithoutRanks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    tmpSetC_2 = List::intersectionOnTrue(targetBlocksWithUnknowns.clone(), e_BLTBlockRanksWithoutRanks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    setC = List::setDifferenceOnTrue(tmpSetC_1.clone(), tmpSetC_2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setC = List::setDifferenceOnTrue(setC.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if debug.clone() {
        println!("{}", (literal!("\n\nSetC-Operations\n====================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n(BlocksWithKnowns) intersection (e_BLTBlocks)   :")); __mm_s.push_str(&*dumplistInteger(tmpSetC_1.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n(BlocksWithUnknowns) intersection (e_BLTBlocks) :")); __mm_s.push_str(&*dumplistInteger(tmpSetC_2.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSetC                                            :")); __mm_s.push_str(&*dumplistInteger(setC.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    tmpSetS_1 = List::setDifferenceOnTrue(targetBlocksWithKnowns.clone(), targetBlocksWithUnknowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    tmpSetS_2 = List::setDifferenceOnTrue(tmpSetS_1.clone(), e_BLTBlockRanksWithoutRanks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    z1 = List::setDifferenceOnTrue(targetBlocksWithConstants.clone(), targetBlocksWithUnknowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    z2 = List::setDifferenceOnTrue(z1.clone(), e_BLTBlockRanksWithoutRanks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    setS = List::unique(List::union(tmpSetS_2.clone(), z2.clone()));
    setS = List::setDifferenceOnTrue(setS.clone(), approximatedEquations.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if debug.clone() {
        println!("{}", (literal!("\nSetS-Operations\n==================")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n(BlocksWithKnowns - BlocksWithUnknowns)                  :")); __mm_s.push_str(&*dumplistInteger(tmpSetS_1.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n((BlocksWithKnowns - BlocksWithUnknowns) - e_BLTBlocks)) :")); __mm_s.push_str(&*dumplistInteger(tmpSetS_2.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nz1(B) => (ConstantBlocks - UnknownsBlocks)               :")); __mm_s.push_str(&*dumplistInteger(z1.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nz2(B) => (z1(B) - e_BLTBlocks)                           :")); __mm_s.push_str(&*dumplistInteger(z2.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nSetS                                                     :")); __mm_s.push_str(&*dumplistInteger(setS.clone())); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("\n")).clone());
    }
    Ok((setC, setS))
}

pub fn filterTargetBlocksWithoutRanks(mut targetBlocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inBlocks: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mainBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut blocks in &*targetBlocks.clone() {
        let mut blocks = blocks.clone();
        mainBlocks = List::append_reverse(Util::tuple21(blocks.clone()), mainBlocks.clone());
    }
    outBlocks = listAppend(inBlocks.clone(), mainBlocks.clone().reverse());
    outBlocks
}

pub fn setEBLTEquationsWithIndexAndRank(mut unMatchedEqList: Arc<metamodelica::List<i32>>, mut unMatchedEqsLstCorrectIndex: Arc<metamodelica::List<i32>>, mut inEqArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>)>>, ExtAdjacencyMatrix, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>)> {
    let mut eBLT_Equation_WithIndex: Arc<metamodelica::List<(i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut e_BLTAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut e_BLTSolvedEqsAndVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut e_BLTBlocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut e_BLTBlockRanks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut actualIndex: i32 = 0;
    let mut index: i32 = -1;
    let mut varsInfoList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*unMatchedEqList.clone() {
        let mut i = i.clone();
        actualIndex = (unMatchedEqsLstCorrectIndex.clone()).get(count.clone())?;
        eBLT_Equation_WithIndex = cons((index.clone(), BackendEquation::get(inEqArray.clone(), actualIndex.clone())), eBLT_Equation_WithIndex.clone());
        varsInfoList = adjacencyMatrix.clone().borrow()[(i.clone()-1) as usize].clone();
        e_BLTAdjacencyMatrix = cons((index.clone(), varsInfoList.clone()), e_BLTAdjacencyMatrix.clone());
        e_BLTSolvedEqsAndVars = cons((index.clone(), (List::sort(varsInfoList.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).get(1)?), e_BLTSolvedEqsAndVars.clone());
        e_BLTBlocks = cons(list![index.clone()], e_BLTBlocks.clone());
        e_BLTBlockRanks = cons((list![index.clone()], index.clone()), e_BLTBlockRanks.clone());
        index = index.clone() - 1;
        count = count.clone() + 1;
    }
    eBLT_Equation_WithIndex = eBLT_Equation_WithIndex.clone().reverse();
    e_BLTAdjacencyMatrix = e_BLTAdjacencyMatrix.clone().reverse();
    e_BLTSolvedEqsAndVars = e_BLTSolvedEqsAndVars.clone().reverse();
    e_BLTBlocks = e_BLTBlocks.clone().reverse();
    e_BLTBlockRanks = e_BLTBlockRanks.clone().reverse();
    Ok((eBLT_Equation_WithIndex, e_BLTAdjacencyMatrix, e_BLTSolvedEqsAndVars, e_BLTBlocks, e_BLTBlockRanks))
}

pub fn inverseModelicaModel(mut inVar: BackendDAE::Variables, mut knownVariablesWithEquationBinding: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut variablesOfInterest: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut variablesOfInterestIndexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    variablesOfInterest = List::filterOnTrue(BackendVariable::varList(inVar.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::varHasUncertainValueRefine, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    for mut var in &*variablesOfInterest.clone() {
        let mut var = var.clone();
        variablesOfInterestIndexes = BackendVariable::getVarIndexFromVars(list![var.clone()], inVar.clone());
        if List::intersectionOnTrue(variablesOfInterestIndexes.clone(), knownVariablesWithEquationBinding.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)).is_empty() {
            eq = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(var.varName.clone())?, scalar: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            eqnlst = cons(eq.clone(), eqnlst.clone());
        }
    }
    Ok(eqnlst)
}

pub fn dumplistInteger(mut inlist: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut outstring: ArcStr = arcstr::literal!("");
    let mut s: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    s = List::map(inlist.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
    outstring = stringDelimitList(s.clone(), (literal!(", ")).clone());
    outstring = stringAppendList(list![(literal!("{")).clone(), (outstring.clone()).clone(), (literal!("}")).clone()]);
    outstring
}

pub fn traverseBLTAndUpdateBlockStatus(mut inlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut knowns: Arc<metamodelica::List<i32>>, mut boundaryConditionVars: Arc<metamodelica::List<i32>>, mut exactEquationVars: Arc<metamodelica::List<i32>>, mut solvedVariables: Arc<metamodelica::List<(i32, i32)>>) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) {
    let mut outlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outstringlist: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut blocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockinfo: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        (blocks, blockinfo) = checkBlueOrRedOrBrownBlocks(i.clone(), knowns.clone(), boundaryConditionVars.clone(), exactEquationVars.clone(), solvedVariables.clone());
        outlist = cons(blocks.clone(), outlist.clone());
        outstringlist = cons(blockinfo.clone(), outstringlist.clone());
    }
    outlist = outlist.clone().reverse();
    outstringlist = outstringlist.clone().reverse();
    (outlist, outstringlist)
}

pub fn checkBlueOrRedOrBrownBlocks(mut inlist: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut boundaryConditionVars: Arc<metamodelica::List<i32>>, mut exactEquationVars: Arc<metamodelica::List<i32>>, mut solvedVar: Arc<metamodelica::List<(i32, i32)>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<ArcStr>>) {
    let mut outIntegerList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut varNumber: i32 = 0;
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        varNumber = getSolvedVariableNumber(i.clone(), solvedVar.clone());
        if listMember(varNumber.clone(), knowns.clone()) {
            outStringList = cons((literal!("knowns")).clone(), outStringList.clone());
            outIntegerList = cons(i.clone(), outIntegerList.clone());
        } else if listMember(varNumber.clone(), exactEquationVars.clone()) {
            outStringList = cons((literal!("constants")).clone(), outStringList.clone());
            outIntegerList = cons(i.clone(), outIntegerList.clone());
        } else {
            outStringList = cons((literal!("unknowns")).clone(), outStringList.clone());
            outIntegerList = cons(i.clone(), outIntegerList.clone());
        }
    }
    outIntegerList = outIntegerList.clone().reverse();
    outStringList = outStringList.clone().reverse();
    (outIntegerList, outStringList)
}

pub fn getSolvedVariableNumber(mut eqnumber: i32, mut inlist: Arc<metamodelica::List<(i32, i32)>>) -> i32 {
    let mut solvedvar: i32 = 0;
    let mut solvedeq: i32 = 0;
    for mut var in &*inlist.clone() {
        let mut var = var.clone();
        (solvedeq, solvedvar) = var.clone();
        if intEq(eqnumber.clone(), solvedeq.clone()) {
            return solvedvar.clone();
        }
    }
    solvedvar
}

pub fn getSolvedEquationNumber(mut varnumber: i32, mut inlist: Arc<metamodelica::List<(i32, i32)>>) -> i32 {
    let mut solvedeq: i32 = 0;
    let mut solvedvar: i32 = 0;
    for mut var in &*inlist.clone() {
        let mut var = var.clone();
        (solvedeq, solvedvar) = var.clone();
        if intEq(varnumber.clone(), solvedvar.clone()) {
            return solvedeq.clone();
        }
    }
    solvedeq
}

pub fn getSolvedEquationAndVarsInfo(mut v: metamodelica::Array<i32>) -> (Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>) {
    let mut eqvarlist: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut solvedEqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = v.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqvarlist = cons((i.clone(), count.clone()), eqvarlist.clone());
        solvedEqLst = cons(i.clone(), solvedEqLst.clone());
        count = count.clone() + 1;
    }
    (eqvarlist, solvedEqLst)
}

fn getVariablesBlockCategories(mut allVariables: BackendDAE::Variables, mut variableIndexList: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exactEquationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unMeasuredVariablesOfInterest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    for mut index in &*variableIndexList.clone() {
        let mut index = index.clone();
        var = BackendVariable::getVarAt(allVariables.clone(), index.clone())?;
        if BackendVariable::varHasUncertainValueRefine(BackendVariable::getVarAt(allVariables.clone(), index.clone())?) {
            knowns = cons(index.clone(), knowns.clone());
        } else if BackendVariable::hasOpenModelicaBoundaryConditionAnnotation(var.clone())? {
            boundaryConditionVars = cons(index.clone(), boundaryConditionVars.clone());
        } else {
            exactEquationVars = cons(index.clone(), exactEquationVars.clone());
        }
        if BackendVariable::varHasUncertainValuePropagate(BackendVariable::getVarAt(allVariables.clone(), index.clone())?) {
            unMeasuredVariablesOfInterest = cons(index.clone(), unMeasuredVariablesOfInterest.clone());
        }
    }
    Ok((knowns, boundaryConditionVars, exactEquationVars, unMeasuredVariablesOfInterest))
}

fn getUncertainRefineAndUnknownVariableIndexes(mut allVariables: BackendDAE::Variables, mut variableIndexList: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut knowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unknowns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut index in &*variableIndexList.clone() {
        let mut index = index.clone();
        if BackendVariable::varHasUncertainValueRefine(BackendVariable::getVarAt(allVariables.clone(), index.clone())?) {
            knowns = cons(index.clone(), knowns.clone());
        } else {
            unknowns = cons(index.clone(), unknowns.clone());
        }
    }
    Ok((knowns, unknowns))
}

pub fn dumpListList(mut lstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut heading: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(lstLst.clone(), (std::sync::Arc::new(fnptr!(dumplistInteger, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

pub fn getEquationsTaggedApproximatedOrBoundaryCondition(mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut index: i32) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut approximatedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boundaryConditionEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut isApproximateEquations: bool = false;
    let mut isConstantEquations: bool = false;
    let mut i: i32 = 0;
    i = index.clone();
    for mut eq in &*eqs.clone() {
        let mut eq = eq.clone();
        (isApproximateEquations, isConstantEquations) = isEquationTaggedApproximatedOrBoundaryCondition(eq.clone())?;
        if isApproximateEquations.clone() {
            approximatedEquations = cons(i.clone(), approximatedEquations.clone());
        } else if isConstantEquations.clone() {
            boundaryConditionEquations = cons(i.clone(), boundaryConditionEquations.clone());
        }
        i = i.clone() + 1;
    }
    Ok((approximatedEquations, boundaryConditionEquations))
}

fn isEquationTaggedApproximatedOrBoundaryCondition(mut eqn: Arc<BackendDAE::Equation>) -> Result<(bool, bool)> {
    let mut approximatedEquations: bool = false;
    let mut boundaryConditionEquations: bool = false;
    (approximatedEquations, boundaryConditionEquations) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { source: Deref @ DAE::ElementSource { comment, .. }, .. } => {
            let mut isApproximatedEquation: bool = false;
            let mut isboundaryConditionEquations: bool = false;
            (isApproximatedEquation, isboundaryConditionEquations) = isEquationTaggedApproximatedOrBoundaryConditionHelper(comment.clone())?;
            (isApproximatedEquation.clone(), isboundaryConditionEquations.clone())
        },
        _ => {
            (false, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((approximatedEquations, boundaryConditionEquations))
}

fn isEquationTaggedApproximatedOrBoundaryConditionHelper(mut commentIn: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<(bool, bool)> {
    let mut approximatedEquations: bool = false;
    let mut boundaryConditionEquations: bool = false;
    (approximatedEquations, boundaryConditionEquations) = 'mc: {
        let __mc_input = commentIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((false, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst, .. } }), .. }, tail: t } => {
                    let mut isApproximatedEquation: bool = false;
                    let mut isboundaryConditionEquation: bool = false;
                    isApproximatedEquation = List::any(subModLst.clone(), (std::sync::Arc::new(fnptr!(isEquationTaggedApproximated, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>)) || (isEquationTaggedApproximatedOrBoundaryConditionHelper(t.clone())?).0;
                    isboundaryConditionEquation = List::any(subModLst.clone(), (std::sync::Arc::new(fnptr!(isEquationTaggedBoundaryCondition, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>)) || (isEquationTaggedApproximatedOrBoundaryConditionHelper(t.clone())?).0;
                    Ok((isApproximatedEquation.clone(), isboundaryConditionEquation.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: t } => {
                    let mut isApproximatedEquation: bool = false;
                    let mut isboundaryConditionEquation: bool = false;
                    (isApproximatedEquation, isboundaryConditionEquation) = isEquationTaggedApproximatedOrBoundaryConditionHelper(t.clone())?;
                    Ok((isApproximatedEquation.clone(), isboundaryConditionEquation.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((approximatedEquations, boundaryConditionEquations))
}

fn isEquationTaggedApproximated(mut m: Arc<SCode::SubMod>) -> bool {
    let mut approximatedEquations: bool = false;
    approximatedEquations = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_ApproximatedEquation", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    approximatedEquations
}

fn isEquationTaggedBoundaryCondition(mut m: Arc<SCode::SubMod>) -> bool {
    let mut boundaryCondition: bool = false;
    boundaryCondition = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_BoundaryCondition", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    boundaryCondition
}

fn isEquationTaggedConstant(mut m: Arc<SCode::SubMod>) -> bool {
    let mut constantEquations: bool = false;
    constantEquations = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_ExactConstantEquation", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    constantEquations
}

pub fn getSBLTAdjacencyMatrix(mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> ExtAdjacencyMatrix {
    let mut extAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = adjacencyMatrix.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut vars in __range0 {
        extAdjacencyMatrix = cons((count.clone(), vars.clone()), extAdjacencyMatrix.clone());
        count = count.clone() + 1;
    }
    extAdjacencyMatrix = extAdjacencyMatrix.clone().reverse();
    extAdjacencyMatrix
}

/*
 Block Target Alogrithm
*/
pub fn findBlockTargets(mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix, mut blockranks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut debug: bool) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>>> {
    let mut outlist: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>> = metamodelica::nil();
    let mut targetblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut eBLTBlocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut targetvarlist: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut blockvarlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ranklist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blocks1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rank: i32 = 0;
    let mut updatedblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    if debug.clone() {
        println!("{}", (literal!("\nDetailed BlockTarget Dependency tree:\n========================================\n")).clone());
    }
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        if (i.clone()).get(1)? > 0 {
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFIND Blocks target of :")); __mm_s.push_str(&*anyString(i.clone())); __mm_s.push_str(&*literal!("\n========================")); ArcStr::from(__mm_s) }).clone());
            }
            (targetblocks, eBLTBlocks) = findBlockTargetsHelper(list![i.clone()], inlist2.clone(), solvedvariables.clone(), mxt.clone(), inlist1.clone(), debug.clone());
            targetblocks = listAppend(cons(i.clone(), targetblocks.clone()), eBLTBlocks.clone());
            (updatedblocks, ranklist) = findBlocksRanks(blockranks.clone(), targetblocks.clone())?;
            updatedblocks = sortBlocks(ranklist.clone(), updatedblocks.clone());
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFinal-Target-Blocks : ")); __mm_s.push_str(&*anyString(updatedblocks.clone())); __mm_s.push_str(&*literal!(" || rankList")); __mm_s.push_str(&*anyString(ranklist.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            targetvarlist = metamodelica::nil();
            for mut blocks in &*updatedblocks.clone() {
                let mut blocks = blocks.clone();
                (blocks1, rank) = blocks.clone();
                blockvarlst = getBlockVarList(blocks1.clone(), inlist1.clone(), inlist2.clone());
                targetvarlist = cons((blockvarlst.clone(), rank.clone()), targetvarlist.clone());
            }
            outlist = cons((i.clone(), updatedblocks.clone(), targetvarlist.clone().reverse()), outlist.clone());
        }
    }
    outlist = outlist.clone().reverse();
    Ok(outlist)
}

pub fn findBlockTargetsHelper(mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix, mut actualblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut debug: bool) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) {
    let mut outSBLT: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut outEBLT: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (outSBLT, outEBLT) = (::match_deref::match_deref! { match &((inlist1.clone(), inlist2.clone(), solvedvariables.clone(), mxt.clone(), actualblocks.clone(), debug.clone())) {
        (Deref @ metamodelica::List::Cons { head: first, tail: rest }, Deref @ metamodelica::List::Cons { head: firstitem, tail: restitem }, solvar, mxt1, originalblocks, b) => {
            let mut dependencyequation: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut targetblocks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut targetblocks1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eBLTList1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eBLTList2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            (dependencyequation, eBLTList1) = findBlockTargetsHelper1(cons(first.clone(), rest.clone()), solvar.clone(), mxt1.clone());
            if debug.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTargetBlocks :")); __mm_s.push_str(&*anyString(dependencyequation.clone())); __mm_s.push_str(&*literal!(" || EBLT_Block")); __mm_s.push_str(&*anyString(eBLTList1.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            targetblocks = getActualBlocks(dependencyequation.clone(), originalblocks.clone(), first.clone());
            (targetblocks1, eBLTList2) = findBlockTargetsHelper(targetblocks.clone(), cons(firstitem.clone(), restitem.clone()), solvar.clone(), mxt1.clone(), originalblocks.clone(), b.clone());
            (List::unique(listAppend(targetblocks.clone(), targetblocks1.clone())), List::unique(listAppend(eBLTList1.clone(), eBLTList2.clone())))
        },
        (_, _, _, _, _, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outSBLT, outEBLT)
}

pub fn findBlockTargetsHelper1(mut inlist: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut mxt: ExtAdjacencyMatrix) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) {
    let mut outSBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outEBLT: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpSBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpEBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*inlist.clone() {
        let mut i = i.clone();
        (tmpSBLT, tmpEBLT) = getDependencyequation(i.clone(), metamodelica::nil(), solvedvariables.clone(), mxt.clone());
        outSBLT = listAppend(outSBLT.clone(), tmpSBLT.clone());
        outEBLT = List::appendElt(tmpEBLT.clone(), outEBLT.clone());
    }
    (outSBLT, outEBLT)
}

pub fn getDependencyequation(mut inlist: Arc<metamodelica::List<i32>>, mut inlist1: Arc<metamodelica::List<i32>>, mut solvedvariables: Arc<metamodelica::List<(i32, i32)>>, mut m: ExtAdjacencyMatrix) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut outSBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outEBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut t: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nonsq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnumber: i32 = 0;
    let mut varnumber: i32 = 0;
    for mut eqnumber in &*inlist.clone() {
        let mut eqnumber = eqnumber.clone();
        varnumber = getSolvedVariableNumber(eqnumber.clone(), solvedvariables.clone());
        (nonsq, outEBLT) = getdirectOccurrencesinEquation(m.clone(), eqnumber.clone(), varnumber.clone());
        for mut lst in &*nonsq.clone() {
            let mut lst = lst.clone();
            if !(listMember(lst.clone(), inlist.clone())) {
                t = cons(lst.clone(), t.clone());
            }
        }
    }
    outSBLT = listAppend(t.clone(), inlist1.clone());
    (outSBLT, outEBLT)
}

pub fn getdirectOccurrencesinEquation(mut m: ExtAdjacencyMatrix, mut eqnumber: i32, mut varnumber: i32) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut outSBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outEBLT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    for mut i in &*m.clone() {
        let mut i = i.clone();
        (eq, vars) = i.clone();
        if !(intEq(eq.clone(), eqnumber.clone())) {
            if listMember(varnumber.clone(), vars.clone()) {
                if eq.clone() > 0 {
                    outSBLT = cons(eq.clone(), outSBLT.clone());
                } else {
                    outEBLT = cons(eq.clone(), outEBLT.clone());
                    break;
                }
            }
        }
    }
    outSBLT = outSBLT.clone().reverse();
    outEBLT = outEBLT.clone().reverse();
    (outSBLT, outEBLT)
}

pub fn findBlocksRanks(mut inlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut outlist: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut ranklist: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut s_BLTRanks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_BLTRanks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rank: i32 = 0;
    for mut i in &*inlist2.clone() {
        let mut i = i.clone();
        for mut j in &*inlist1.clone() {
            let mut j = j.clone();
            (blocks, rank) = j.clone();
            if i.clone() == blocks.clone() {
                outlist = cons((i.clone(), rank.clone()), outlist.clone());
                if rank.clone() > 0 {
                    s_BLTRanks = cons(rank.clone(), s_BLTRanks.clone());
                } else {
                    e_BLTRanks = cons(rank.clone(), e_BLTRanks.clone());
                }
            }
        }
    }
    outlist = outlist.clone().reverse();
    ranklist = listAppend(List::sort(s_BLTRanks.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, e_BLTRanks.clone().reverse());
    Ok((outlist, ranklist))
}

pub fn sortBlocks(mut sortedranklist: Arc<metamodelica::List<i32>>, mut inlist2: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>) -> Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> {
    let mut outlist: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut e1: i32 = 0;
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

pub fn getBlockVarList(mut blocktofind: Arc<metamodelica::List<i32>>, mut inlist1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inlist2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outstringlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut blockFound: bool = false;
    for mut i in &*inlist1.clone() {
        let mut i = i.clone();
        blockFound = List::setEqualOnTrue(i.clone(), blocktofind.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        if blockFound.clone() {
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

/* ### End of Block-target Algorithm functions ### */
/*
  finding PredecessorBlocks Algorithm
*/
pub fn findPredecessorBlocks(mut blockinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>> {
    let mut outblockinfo: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut dependencyequation: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constantEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut targetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut tmptargetblocks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut targetblocksvar: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>> = metamodelica::nil();
    let mut blockitems1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut foundblockranks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut tmpcount: i32 = 0;
    let mut exist: bool = false;
    let mut targetexist: bool = false;
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
        if !(targetexist.clone()) {
            (exist, dependencyequation, constantEquations, foundblockranks) = findSquareAndNonSquareBlocksHelper1(targetblocks.clone(), targetblocksvar.clone());
            outblockinfo = cons((blockitems1.clone(), targetblocks.clone(), targetblocksvar.clone(), dependencyequation.clone(), constantEquations.clone(), foundblockranks.clone()), outblockinfo.clone());
        }
        count = count.clone() + 1;
    }
    outblockinfo = outblockinfo.clone().reverse();
    Ok(outblockinfo)
}

pub fn findSquareAndNonSquareBlocksHelper1(mut inlist1: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>>, mut inlist2: Arc<metamodelica::List<(Arc<metamodelica::List<ArcStr>>, i32)>>) -> (bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut exists: bool = false;
    let mut foundknownblocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constantBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockranks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut blocksvarlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut rank: i32 = 0;
    let mut targetblocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*inlist2.clone() {
        let mut i = i.clone();
        (blocksvarlist, rank) = i.clone();
        if rank.clone() > 0 && count.clone() == 1 {
            (targetblocks, _) = (inlist1.clone()).get(count.clone()).unwrap();
            if listMember((literal!("knowns")).clone(), blocksvarlist.clone()) {
                exists = true;
                blockranks = cons(rank.clone(), blockranks.clone());
                foundknownblocks = getKnownOrExactEquationBlocksHelper(blocksvarlist.clone(), targetblocks.clone(), (literal!("knowns")).clone());
            } else if listMember((literal!("constants")).clone(), blocksvarlist.clone()) {
                exists = true;
                blockranks = cons(rank.clone(), blockranks.clone());
                constantBlocks = getKnownOrExactEquationBlocksHelper(blocksvarlist.clone(), targetblocks.clone(), (literal!("constants")).clone());
            }
        }
        count = count.clone() + 1;
    }
    foundknownblocks = foundknownblocks.clone().reverse();
    blockranks = blockranks.clone().reverse();
    (exists, foundknownblocks, constantBlocks, blockranks)
}

fn getKnownOrExactEquationBlocksHelper(mut blocksVarList: Arc<metamodelica::List<ArcStr>>, mut targetBlocks: Arc<metamodelica::List<i32>>, mut knownOrConstant: ArcStr) -> Arc<metamodelica::List<i32>> {
    let mut outBlocks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut count: i32 = 1;
    for mut j in &*blocksVarList.clone() {
        let mut j = j.clone();
        if (j.clone()).clone() == knownOrConstant.clone() {
            outBlocks = cons((targetBlocks.clone()).get(count.clone()).unwrap(), outBlocks.clone());
            return outBlocks.clone();
        }
        count = count.clone() + 1;
    }
    outBlocks
}

/* end of finding PredecessorBlocks Algorithm */
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

fn VerifySetSPrime(mut boundaryConditionsVars: BackendDAE::Variables, mut intermediateVars: BackendDAE::Variables, mut knownVars: BackendDAE::Variables, mut extraVarsinSetSPrime: Arc<metamodelica::List<BackendDAE::Var>>, mut boundaryConditionsEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut intermediateEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut auxillaryEquations: i32, mut numRelatedBoundaryConditions: i32, mut stateEstimation: bool) -> Result<()> {
    let mut eqSize: i32 = 0;
    let mut varSize: i32 = 0;
    let mut count: i32 = 0;
    let mut extraVarLength: i32 = 0;
    let mut condition5: ArcStr = arcstr::literal!("");
    let mut msg: ArcStr = arcstr::literal!("");
    eqSize = intAdd(BackendEquation::equationArraySize(boundaryConditionsEquations.clone())?, BackendEquation::equationArraySize(intermediateEquations.clone())?);
    varSize = intAdd((BackendVariable::varList(boundaryConditionsVars.clone())?.len() as i32), (BackendVariable::varList(intermediateVars.clone())?.len() as i32));
    if !(intEq(eqSize.clone(), varSize.clone())) {
        condition5 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Set-S' has ")); __mm_s.push_str(&*intString(eqSize.clone())); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(varSize.clone())); __mm_s.push_str(&*literal!(" variables")); ArcStr::from(__mm_s) }).clone();
        msg = (literal!("Boundary condition(s) ")).clone();
        for mut var in &*BackendVariable::varList(boundaryConditionsVars.clone())? {
            let mut var = var.clone();
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*BackendDump::varStringShort(var.clone())?); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone();
        }
        msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(" cannot be computed from the variables of interest only. They must be computed also from boundary conditions(s) ")); ArcStr::from(__mm_s) }).clone();
        extraVarLength = (extraVarsinSetSPrime.clone().len() as i32);
        count = 1;
        for mut var in &*extraVarsinSetSPrime.clone() {
            let mut var = var.clone();
            if intEq(count.clone(), extraVarLength.clone()) {
                msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*BackendDump::varStringShort(var.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
            } else {
                msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*BackendDump::varStringShort(var.clone())?); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone();
            }
            count = count.clone() + 1;
        }
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(" Therefore, the problem is ill-posed regarding the computation of boundary conditions from the variables of interest only.")); ArcStr::from(__mm_s) }).clone()])?;
        if stateEstimation.clone() {
            generateCompileTimeHtmlReport(shared.clone(), (literal!("")).clone(), (intString(BackendEquation::equationArraySize(boundaryConditionsEquations.clone())?)).clone(), (intString((BackendVariable::varList(knownVars.clone())?.len() as i32))).clone(), (literal!(""), metamodelica::nil()), (literal!(""), metamodelica::nil()), (literal!("")).clone(), (literal!(""), metamodelica::nil()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(" Therefore, the problem is ill-posed regarding the computation of unmeasured variables of interest from the variables of interest only.")); ArcStr::from(__mm_s) }).clone(), false, true, auxillaryEquations.clone(), numRelatedBoundaryConditions.clone(), 0)?;
        } else {
            generateCompileTimeHtmlReport(shared.clone(), (literal!("")).clone(), (intString(BackendEquation::equationArraySize(boundaryConditionsEquations.clone())?)).clone(), (intString((BackendVariable::varList(knownVars.clone())?.len() as i32))).clone(), (literal!(""), metamodelica::nil()), (literal!(""), metamodelica::nil()), (literal!("")).clone(), (literal!(""), metamodelica::nil()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(" Therefore, the problem is ill-posed regarding the computation of boundary conditions from the variables of interest only.")); ArcStr::from(__mm_s) }).clone(), true, false, auxillaryEquations.clone(), numRelatedBoundaryConditions.clone(), 0)?;
        }
        bail!("fail");
    }
    Ok(())
}

fn VerifyDataReconciliation(mut setc: Arc<metamodelica::List<i32>>, mut sets: Arc<metamodelica::List<i32>>, mut knowns: Arc<metamodelica::List<i32>>, mut unknowns: Arc<metamodelica::List<i32>>, mut mExt: ExtAdjacencyMatrix, mut solvedvar: Arc<metamodelica::List<(i32, i32)>>, mut constantvars: Arc<metamodelica::List<i32>>, mut approximatedEquations: Arc<metamodelica::List<i32>>, mut allVars: BackendDAE::Variables, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut outsetS_vars: BackendDAE::Variables, mut outsetS_eq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut mappedSetC: Arc<metamodelica::List<i32>>, mut mappedSetS: Arc<metamodelica::List<i32>>, mut unMeasuredVariablesOfInterest: i32) -> Result<()> {
    let mut matchedeq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedknownssetc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedunknownssetc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedknownssets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut matchedunknownssets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplist1sets: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplistvar1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplistvar2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmplistvar3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut resstr: ArcStr = arcstr::literal!("");
    let mut condition1: ArcStr = arcstr::literal!("");
    let mut condition2: ArcStr = arcstr::literal!("");
    let mut condition3: ArcStr = arcstr::literal!("");
    let mut condition4: ArcStr = arcstr::literal!("");
    let mut condition5: ArcStr = arcstr::literal!("");
    let mut auxilliaryConditions: ArcStr = arcstr::literal!("");
    let mut varsToReconcile: ArcStr = arcstr::literal!("");
    let mut var: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut rule2: bool = true;
    let mut condition1_eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nAutomatic Verification Steps of DataReconciliation Algorithm")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    var = List::map1r(knowns.clone().reverse(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone());
    BackendDump::dumpVarList(var.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("knownVariables:")); __mm_s.push_str(&*dumplistInteger(knowns.clone().reverse())); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C:")); __mm_s.push_str(&*dumplistInteger(mappedSetC.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_S:")); __mm_s.push_str(&*dumplistInteger(mappedSetS.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    auxilliaryConditions = (intString((mappedSetC.clone().len() as i32))).clone();
    varsToReconcile = (intString((knowns.clone().len() as i32))).clone();
    condition1 = (literal!("Condition-1 \"SET_C and SET_S must not have no equations in common\"")).clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*condition1.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    matchedeq = List::intersectionOnTrue(mappedSetC.clone(), mappedSetS.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    if matchedeq.clone().is_empty() {
        println!("{}", (literal!("-Passed\n\n")).clone());
    } else {
        println!("{}", (literal!("-Failed\n")).clone());
        condition1_eqs = List::map1r(matchedeq.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::get, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), allEqs.clone());
        BackendDump::dumpEquationList(condition1_eqs.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sets C and S have equations in common")); __mm_s.push_str(&*dumplistInteger(matchedeq.clone())); ArcStr::from(__mm_s) }).clone());
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 1-Failed: SET_C and SET_S must not have no equations in common: The data reconciliation problem is ill-posed")).clone()])?;
        generateCompileTimeHtmlReport(shared.clone(), (literal!("Internal Error: Condition 1-Failed: \"SET_C and SET_S must not have no equations in common\": The data reconciliation problem is ill-posed")).clone(), (auxilliaryConditions.clone()).clone(), (varsToReconcile.clone()).clone(), (literal!("Sets C and S have equations in common"), condition1_eqs.clone()), (literal!(""), metamodelica::nil()), (literal!("")).clone(), (literal!(""), metamodelica::nil()), (literal!("")).clone(), false, false, 0, 0, unMeasuredVariablesOfInterest.clone())?;
        bail!("fail");
    }
    (matchedknownssetc, matchedunknownssetc) = getVariableOccurence(setc.clone(), mExt.clone(), knowns.clone());
    (matchedknownssets, matchedunknownssets) = getVariableOccurence(sets.clone(), mExt.clone(), knowns.clone());
    condition2 = (literal!("Condition-2 \"All variables of interest must be involved in SET_C or SET_S\"")).clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*condition2.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    (tmplist1, tmplist2, tmplist3) = List::intersection1OnTrue(matchedknownssetc.clone(), knowns.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if tmplist3.clone().is_empty() {
        println!("{}", (literal!("-Passed\n")).clone());
        BackendDump::dumpVarList(List::map1r(tmplist1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C has all known variables:")); __mm_s.push_str(&*dumplistInteger(tmplist1.clone())); ArcStr::from(__mm_s) }).clone());
    } else if !(tmplist3.clone().is_empty()) {
        (tmplist1sets, tmplist2, _) = List::intersection1OnTrue(tmplist3.clone(), matchedknownssets.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        if !(tmplist2.clone().is_empty()) {
            r#str = (dumplistInteger(tmplist2.clone())).clone();
            println!("{}", (literal!("-Failed\n")).clone());
            BackendDump::dumpVarList(List::map1r(tmplist2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("knownVariables not Found:")); __mm_s.push_str(&*dumplistInteger(tmplist2.clone())); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 2-Failed: All variables of interest must be involved in Set-C or Set-S: The data reconciliation problem is ill-posed")).clone()])?;
            rule2 = false;
            r#str = (dumpToCsv((literal!("")).clone(), List::map1r(tmplist2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()))?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_NonReconcilcedVars.txt")); ArcStr::from(__mm_s) }).clone(), (r#str.clone()).clone())?;
        }
        if rule2.clone() {
            println!("{}", (literal!("-Passed\n")).clone());
        }
        BackendDump::dumpVarList(List::map1r(tmplist1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C has known variables:")); __mm_s.push_str(&*dumplistInteger(tmplist1.clone())); ArcStr::from(__mm_s) }).clone());
        BackendDump::dumpVarList(List::map1r(tmplist1sets.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_S has known variables:")); __mm_s.push_str(&*dumplistInteger(tmplist1sets.clone())); ArcStr::from(__mm_s) }).clone());
    }
    condition3 = (literal!("Condition-3 \"SET_C equations must be strictly less than Variable of Interest\"")).clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*condition3.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if (setc.clone().len() as i32) < (knowns.clone().len() as i32) && !(setc.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_C contains:")); __mm_s.push_str(&*intString((setc.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations < ")); __mm_s.push_str(&*intString((knowns.clone().len() as i32))); __mm_s.push_str(&*literal!(" known variables\n\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        condition3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Set-C has ")); __mm_s.push_str(&*intString((setc.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString((knowns.clone().len() as i32))); __mm_s.push_str(&*literal!(" variables to be reconciled")); ArcStr::from(__mm_s) }).clone();
        resstr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Failed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*condition3.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
        println!("{}", (resstr.clone()).clone());
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 3-Failed: The number of auxiliary conditions must be strictly less than the number of variables to be reconciled. The data reconciliation problem is ill-posed")).clone()])?;
        if setc.clone().is_empty() {
            condition3 = (literal!("<b>User Error:</b> Condition 7 failed: \"The set of auxiliary conditions is empty.\" The data reconciliation problem is ill-posed")).clone();
            generateCompileTimeHtmlReport(shared.clone(), (literal!("")).clone(), (auxilliaryConditions.clone()).clone(), (varsToReconcile.clone()).clone(), (literal!(""), metamodelica::nil()), (literal!(""), metamodelica::nil()), (condition3.clone()).clone(), (literal!(""), metamodelica::nil()), (literal!("")).clone(), false, false, 0, 0, unMeasuredVariablesOfInterest.clone())?;
        } else {
            generateCompileTimeHtmlReport(shared.clone(), (literal!("<b>User Error:</b> Condition 3-Failed: \"The number of auxiliary conditions must be strictly less than the number of variables to be reconciled.\": The data reconciliation problem is ill-posed")).clone(), (auxilliaryConditions.clone()).clone(), (varsToReconcile.clone()).clone(), (literal!(""), metamodelica::nil()), (literal!(""), metamodelica::nil()), (condition3.clone()).clone(), (literal!(""), metamodelica::nil()), (literal!("")).clone(), false, false, 0, 0, unMeasuredVariablesOfInterest.clone())?;
        }
        bail!("fail");
    }
    condition4 = (literal!("Condition-4 \"SET_S should contain all intermediate variables involved in SET_C\"")).clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*condition4.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    (tmplistvar1, tmplistvar2, tmplistvar3) = List::intersection1OnTrue(matchedunknownssetc.clone(), matchedunknownssets.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if matchedunknownssetc.clone().is_empty() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_C contains No Intermediate Variables\n\n")); ArcStr::from(__mm_s) }).clone());
        return Ok(());
    } else {
        BackendDump::dumpVarList(List::map1r(matchedunknownssetc.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_C has intermediate variables:")); __mm_s.push_str(&*dumplistInteger(matchedunknownssetc.clone())); ArcStr::from(__mm_s) }).clone());
        if tmplistvar2.clone().is_empty() {
            BackendDump::dumpVarList(List::map1r(tmplistvar1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_S has intermediate variables involved in SET_C:")); __mm_s.push_str(&*dumplistInteger(tmplistvar1.clone())); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("-Passed\n\n")).clone());
        } else {
            BackendDump::dumpVarList(List::map1r(tmplistvar2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-SET_S does not have intermediate variables involved in SET_C:")); __mm_s.push_str(&*dumplistInteger(tmplistvar2.clone())); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 4-Failed: SET_S should contain all intermediate variables involved in SET_C: The data reconciliation problem is ill-posed")).clone()])?;
            generateCompileTimeHtmlReport(shared.clone(), (literal!("<b>Internal Error:</b> Condition 4-Failed: \"SET_S should contain all intermediate variables involved in SET_C\": The data reconciliation problem is ill-posed")).clone(), (auxilliaryConditions.clone()).clone(), (varsToReconcile.clone()).clone(), (literal!(""), metamodelica::nil()), (literal!(""), metamodelica::nil()), (literal!("")).clone(), (literal!("Set-S does not have intermediate variables involved in Set-C"), List::map1r(tmplistvar2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), allVars.clone())), (literal!("")).clone(), false, false, 0, 0, unMeasuredVariablesOfInterest.clone())?;
            bail!("fail");
        }
    }
    condition5 = (literal!("Condition-5 \"SET_S should be square\"")).clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*condition5.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if outsetS_eq.clone().is_empty() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("-SET_S contains 0 intermediate variables and 0 equations\n\n")); ArcStr::from(__mm_s) }).clone());
        return Ok(());
    } else {
        if BackendEquation::equationArraySize(BackendEquation::listEquation(outsetS_eq.clone())?)? == (BackendVariable::varList(outsetS_vars.clone())?.len() as i32) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Passed")); __mm_s.push_str(&*literal!("\n ")); __mm_s.push_str(&*literal!("Set_S has ")); __mm_s.push_str(&*intString((sets.clone().len() as i32))); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString((BackendVariable::varList(outsetS_vars.clone())?.len() as i32))); __mm_s.push_str(&*literal!(" variables\n\n")); ArcStr::from(__mm_s) }).clone());
        } else {
            condition5 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Set-S has ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(BackendEquation::listEquation(outsetS_eq.clone())?)?)); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString((BackendVariable::varList(outsetS_vars.clone())?.len() as i32))); __mm_s.push_str(&*literal!(" variables")); ArcStr::from(__mm_s) }).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Failed")); __mm_s.push_str(&*literal!("\n ")); __mm_s.push_str(&*condition5.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!(": Condition 5-Failed: Set_S should be square: The data reconciliation problem is ill-posed")).clone()])?;
            generateCompileTimeHtmlReport(shared.clone(), (literal!("<b>Internal Error:</b> Condition 5-Failed: \"Set_S should be square\": The data reconciliation problem is ill-posed")).clone(), (auxilliaryConditions.clone()).clone(), (varsToReconcile.clone()).clone(), (literal!(""), metamodelica::nil()), (literal!(""), metamodelica::nil()), (literal!("")).clone(), (literal!(""), metamodelica::nil()), (condition5.clone()).clone(), false, false, 0, 0, unMeasuredVariablesOfInterest.clone())?;
            bail!("fail");
        }
    }
    Ok(())
}

fn generateCompileTimeHtmlReport(mut shared: Arc<BackendDAE::Shared>, mut conditions: ArcStr, mut auxilliaryConditions: ArcStr, mut varsToReconcile: ArcStr, mut condition1: (ArcStr, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>), mut condition2: (ArcStr, Arc<metamodelica::List<BackendDAE::Var>>), mut condition3: ArcStr, mut condition4: (ArcStr, Arc<metamodelica::List<BackendDAE::Var>>), mut condition5: ArcStr, mut boundaryCondition: bool, mut stateEstimation: bool, mut setC: i32, mut numRelatedBoundaryConditions: i32, mut unMeasuredVariables: i32) -> Result<()> {
    let mut data: ArcStr = arcstr::literal!("");
    let mut condition1_msg: ArcStr = arcstr::literal!("");
    let mut condition4_msg: ArcStr = arcstr::literal!("");
    let mut condition1_eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut condition4_vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    if boundaryCondition.clone() {
        data = (literal!("<html> \n <head> <h1> Boundary Condition Report</h1></head> \n <body> \n <h2> Overview: </h2> \n")).clone();
    } else {
        data = (literal!("<html> \n <head> <h1> Data Reconciliation Report</h1></head> \n <body> \n <h2> Overview: </h2> \n")).clone();
    }
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<table> \n <tr> \n <th align=right> Model file: </th> \n")); ArcStr::from(__mm_s) }).clone();
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<td>")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".mo")); __mm_s.push_str(&*literal!("</td>\n</tr>\n")); ArcStr::from(__mm_s) }).clone();
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!(" <tr> \n <th align=right> Model name: </th>\n")); ArcStr::from(__mm_s) }).clone();
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<td>")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("</td>\n</tr>\n")); ArcStr::from(__mm_s) }).clone();
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr> \n <th align=right> Generated: </th>\n")); ArcStr::from(__mm_s) }).clone();
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<td>")); __mm_s.push_str(&*System::getCurrentTimeStr()?); __mm_s.push_str(&*literal!("<b> by OpenModelica ")); __mm_s.push_str(&*Settings::getVersionNr()); __mm_s.push_str(&*literal!("</b>")); __mm_s.push_str(&*literal!("</td>\n</tr>\n <table>\n")); ArcStr::from(__mm_s) }).clone();
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h2> Analysis: </h2>\n<table>")); ArcStr::from(__mm_s) }).clone();
    if boundaryCondition.clone() {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr>\n <th align=right> Number of boundary conditions: </th> \n <td>")); __mm_s.push_str(&*auxilliaryConditions.clone()); __mm_s.push_str(&*literal!("</td>\n</tr>\n")); ArcStr::from(__mm_s) }).clone();
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr>\n <th align=right> Number of measured variables: </th> \n <td>")); __mm_s.push_str(&*varsToReconcile.clone()); __mm_s.push_str(&*literal!("</td>\n</tr>\n</table>")); ArcStr::from(__mm_s) }).clone();
    } else {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr>\n <th align=right> Number of auxiliary conditions: </th> \n <td>")); __mm_s.push_str(&*intString(setC.clone())); __mm_s.push_str(&*literal!("</td>\n</tr>\n")); ArcStr::from(__mm_s) }).clone();
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr>\n <th align=right> Number of measured variables: </th> \n <td>")); __mm_s.push_str(&*varsToReconcile.clone()); __mm_s.push_str(&*literal!("</td>\n</tr>\n")); ArcStr::from(__mm_s) }).clone();
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr>\n <th align=right> Number of unmeasured variables: </th> \n <td>")); __mm_s.push_str(&*intString(unMeasuredVariables.clone())); __mm_s.push_str(&*literal!("</td>\n</tr>\n")); ArcStr::from(__mm_s) }).clone();
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<tr>\n <th align=right> Number of related boundary conditions: </th> \n <td>")); __mm_s.push_str(&*intString(numRelatedBoundaryConditions.clone())); __mm_s.push_str(&*literal!("</td>\n</tr>\n</table>")); ArcStr::from(__mm_s) }).clone();
    }
    if boundaryCondition.clone() {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h3> <a href=")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionsEquations.html target=_blank> Boundary conditions </a> </h3>")); ArcStr::from(__mm_s) }).clone();
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h3> <a href=")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html target=_blank> Intermediate equations </a> </h3>")); ArcStr::from(__mm_s) }).clone();
    } else {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h3> <a href=")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_IntermediateEquations.html target=_blank> Intermediate equations for measured variables </a> </h3>")); ArcStr::from(__mm_s) }).clone();
        if numRelatedBoundaryConditions.clone() > 0 {
            data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h3> <a href=")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionsEquations.html target=_blank> Boundary conditions </a> </h3>")); ArcStr::from(__mm_s) }).clone();
            data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h3> <a href=")); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html target=_blank> Intermediate equations for unmeasured variables </a> </h3>")); ArcStr::from(__mm_s) }).clone();
        }
    }
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<h3> Errors: </h3> ")); __mm_s.push_str(&*literal!("\n <p>")); __mm_s.push_str(&*conditions.clone()); __mm_s.push_str(&*literal!("</p>")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    (condition1_msg, condition1_eqs) = condition1.clone();
    if !(condition1_eqs.clone().is_empty()) {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<p>")); __mm_s.push_str(&*condition1_msg.clone()); __mm_s.push_str(&*literal!("\n <ol>")); ArcStr::from(__mm_s) }).clone();
        for mut eq in &*condition1_eqs.clone() {
            let mut eq = eq.clone();
            data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("  <li>")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!(" </li>")); ArcStr::from(__mm_s) }).clone();
        }
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("\n</ol> \n</p>")); ArcStr::from(__mm_s) }).clone();
    }
    if !(stringEmpty((condition3.clone()).clone())) {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<p>")); __mm_s.push_str(&*condition3.clone()); __mm_s.push_str(&*literal!("</p>")); ArcStr::from(__mm_s) }).clone();
    }
    (condition4_msg, condition4_vars) = condition4.clone();
    if !(condition4_vars.clone().is_empty()) {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<p>")); __mm_s.push_str(&*condition4_msg.clone()); __mm_s.push_str(&*literal!("\n <ol>")); ArcStr::from(__mm_s) }).clone();
        for mut var in &*condition4_vars.clone() {
            let mut var = var.clone();
            data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("\n <li>")); __mm_s.push_str(&*BackendDump::varStringShort(var.clone())?); __mm_s.push_str(&*literal!("</li>")); ArcStr::from(__mm_s) }).clone();
        }
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("\n</ol>")); ArcStr::from(__mm_s) }).clone();
    }
    if !(stringEmpty((condition5.clone()).clone())) {
        data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("<p>")); __mm_s.push_str(&*condition5.clone()); __mm_s.push_str(&*literal!("</p>")); ArcStr::from(__mm_s) }).clone();
    }
    data = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*data.clone()); __mm_s.push_str(&*literal!("\n</html>")); ArcStr::from(__mm_s) }).clone();
    if boundaryCondition.clone() {
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_BoundaryConditions.html")); ArcStr::from(__mm_s) }).clone(), (data.clone()).clone())?;
    } else {
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*shared.info.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".html")); ArcStr::from(__mm_s) }).clone(), (data.clone()).clone())?;
    }
    Ok(())
}

pub fn getVariableOccurence(mut setCOrSetS: Arc<metamodelica::List<i32>>, mut mext: ExtAdjacencyMatrix, mut knowns: Arc<metamodelica::List<i32>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut knownvariables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unknownvariables: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: i32 = 0;
    for mut i in &*setCOrSetS.clone() {
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

/* function which dumps the variable names to csv file */
pub fn dumpCorrelationVarsToCsv(mut invar: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut r#str: ArcStr = literal!("Sxij,");
    for mut i in &*invar.clone() {
        let mut i = i.clone();
        cr = BackendVariable::varCref(i.clone())?;
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone();
    }
    outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*outstring.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outstring)
}

/* function which dumps non reconciledVars failing for condition -2 to a log file*/
pub fn dumpNonReconciledVars(mut invar: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    for mut i in &*invar.clone() {
        let mut i = i.clone();
        cr = BackendVariable::varCref(i.clone())?;
        outstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outstring.clone()); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outstring)
}

fn dumpEquationString(mut inEquation: Arc<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionDump::printExp2Str(e1.clone(), (literal!("")).clone(), None, None)?).clone();
            s2 = (ExpressionDump::printExp2Str(e2.clone(), (literal!("")).clone(), None, None)?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionDump::printExp2Str(e1.clone(), (literal!("")).clone(), None, None)?).clone();
            s2 = (ExpressionDump::printExp2Str(e2.clone(), (literal!("")).clone(), None, None)?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionDump::printExp2Str(e1.clone(), (literal!("")).clone(), None, None)?).clone();
            s2 = (ExpressionDump::printExp2Str(e2.clone(), (literal!("")).clone(), None, None)?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s1 = (System::stringReplace((s1.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            s1 = (System::stringReplace((s1.clone()).clone(), (literal!("$")).clone(), (literal!("")).clone())?).clone();
            s2 = (ExpressionDump::printExp2Str(e2.clone(), (literal!("")).clone(), None, None)?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (BackendDump::whenEquationString(weqn.clone(), true)?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionDump::printExp2Str(e.clone(), (literal!("")).clone(), None, None)?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!("= 0")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { source, alg, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (DAEDump::dumpAlgorithmsStr(list![Arc::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() })])?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse, eqnstrue: Deref @ metamodelica::List::Cons { head: eqns, tail: eqnstrue }, conditions: Deref @ metamodelica::List::Cons { head: e1, tail: expl }, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionDump::printExp2Str(e1.clone(), (literal!("")).clone(), None, None)?).clone();
            s2 = stringDelimitList(List::map(eqns.clone(), (std::sync::Arc::new(dumpEquationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)), (literal!("\n  ")).clone());
            s3 = stringAppendList(list![(literal!("if ")).clone(), (s1.clone()).clone(), (literal!(" then\n  ")).clone(), (s2.clone()).clone()]);
            res = (BackendDump::ifequationString(expl.clone(), eqnstrue.clone(), eqnsfalse.clone(), (s3.clone()).clone())?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { body: eqn, stop, start, iter, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionDump::printExp2Str(iter.clone(), (literal!("")).clone(), None, None)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionDump::printExp2Str(start.clone(), (literal!("")).clone(), None, None)?); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionDump::printExp2Str(stop.clone(), (literal!("")).clone(), None, None)?); ArcStr::from(__mm_s) }).clone();
            s2 = (dumpEquationString(eqn.clone())?).clone();
            res = stringAppendList(list![(literal!("for ")).clone(), (s1.clone()).clone(), (literal!(" loop\n    ")).clone(), (s2.clone()).clone(), (literal!("; end for; ")).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

