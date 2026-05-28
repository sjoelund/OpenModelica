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

use crate::BackendDAE::VarKind;
use crate::SimCode::ModelInfo;
use crate::SimCode::SimCode as SIMCODE;
use crate::SimCode::SimulationSettings;
use crate::SimCode::VarInfo;
use crate::SimCode;
use crate::SimCodeUtil;
use crate::SimCodeVar::AliasVariable;
use crate::SimCodeVar::Causality;
use crate::SimCodeVar::SimVar;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference as CR;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Exp;
use openmodelica_frontend_types::DAE::Type;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File::Escape::XML;
use openmodelica_util::File;
use openmodelica_util::Settings;
use openmodelica_util::Util;

pub fn simulationInitFile(mut simCode: SIMCODE, mut guid: ArcStr) -> Result<()> {
    let true = (simulationInitFileReturnBool(simCode.clone(), (guid.clone()).clone())) else { bail!("pattern mismatch") };
    Ok(())
}

pub fn simulationInitFileReturnBool(mut simCode: SIMCODE, mut guid: ArcStr) -> bool {
    let mut success: bool = false;
    let mut vi: VarInfo = <VarInfo as ::std::default::Default>::default();
    let mut s: SimulationSettings = <SimulationSettings as ::std::default::Default>::default();
    let mut file: File::File = File::File(File::noReference()).unwrap();
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut FMUType: ArcStr = arcstr::literal!("");
    if '__try0: {
        fileName = ((::match_deref::match_deref! { match &(unwrap_break_err!(Config::simCodeTarget(), '__try0)) {
        Deref @ "omsic" => { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) },
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        File::open(file.clone(), (fileName.clone()).clone(), File::Mode::Write.clone());
        vi = simCode.modelInfo.varInfo.clone();
        let Some(__pa1) = (simCode.simulationSettingsOpt.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        s = __pa1.clone();
        FMUType = ((::match_deref::match_deref! { match &(unwrap_break_err!(Config::simCodeTarget(), '__try0)) {
        Deref @ "omsic" => literal!("2.0"),
        Deref @ "omsicpp" => literal!("2.0"),
        _ => literal!("1.0"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        File::write(file.clone(), (literal!("<?xml version = \"1.0\" encoding=\"UTF-8\"?>\n\n")).clone());
        File::write(file.clone(), (literal!("<!-- description of the model interface using an extention of the FMI standard -->\n")).clone());
        File::write(file.clone(), (literal!("<fmiModelDescription\n")).clone());
        File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  fmiVersion                          = \"")); __mm_s.push_str(&*FMUType.clone()); __mm_s.push_str(&*literal!("\"\n\n")); ArcStr::from(__mm_s) }).clone());
        File::write(file.clone(), (literal!("  modelName                           = \"")).clone());
        unwrap_break_err!(Dump::writePath(file.clone(), simCode.modelInfo.name.clone(), File::Escape::None.clone(), (literal!(".")).clone(), false), '__try0);
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("  modelIdentifier                     = \"")).clone());
        unwrap_break_err!(Dump::writePath(file.clone(), simCode.modelInfo.name.clone(), File::Escape::None.clone(), (literal!("_")).clone(), false), '__try0);
        File::write(file.clone(), (literal!("\"\n\n")).clone());
        File::write(file.clone(), (literal!("  OPENMODELICAHOME                    = \"")).clone());
        File::write(file.clone(), (simCode.makefileParams.omhome.clone()).clone());
        File::write(file.clone(), (literal!("\"\n\n")).clone());
        File::write(file.clone(), (literal!("  guid                                = \"{")).clone());
        File::write(file.clone(), (guid.clone()).clone());
        File::write(file.clone(), (literal!("}\"\n\n")).clone());
        File::write(file.clone(), (literal!("  description                         = \"")).clone());
        File::writeEscape(file.clone(), (simCode.modelInfo.description.clone()).clone(), XML.clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("  generationTool                      = \"OpenModelica Compiler ")).clone());
        File::write(file.clone(), (Settings::getVersionNr()).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("  generationDateAndTime               = \"")).clone());
        xsdateTime(file.clone(), Util::getCurrentDateTime());
        File::write(file.clone(), (literal!("\"\n\n")).clone());
        File::write(file.clone(), (literal!("  variableNamingConvention            = \"structured\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfEventIndicators             = \"")).clone());
        File::writeInt(file.clone(), vi.numZeroCrossings.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfEventIndicators             = \"NG:       number of zero crossings,                           FMI\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfTimeEvents                  = \"")).clone());
        File::writeInt(file.clone(), vi.numTimeEvents.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfTimeEvents                  = \"NG_SAM:   number of zero crossings that are samples,          OMC\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfInputVariables              = \"")).clone());
        File::writeInt(file.clone(), vi.numInVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfInputVariables              = \"NI:       number of inputvar on topmodel,                     OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfOutputVariables             = \"")).clone());
        File::writeInt(file.clone(), vi.numOutVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfOutputVariables             = \"NO:       number of outputvar on topmodel,                    OMC\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfExternalObjects             = \"")).clone());
        File::writeInt(file.clone(), vi.numExternalObjects.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfExternalObjects             = \"NEXT:     number of external objects,                         OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfFunctions                   = \"")).clone());
        File::writeInt(file.clone(), (simCode.modelInfo.functions.clone().len() as i32), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfFunctions                   = \"NFUNC:    number of functions used by the simulation,         OMC\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfContinuousStates            = \"")).clone());
        File::writeInt(file.clone(), vi.numStateVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfContinuousStates            = \"NX:       number of states,                                   FMI\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfRealAlgebraicVariables      = \"")).clone());
        File::writeInt(file.clone(), vi.numAlgVars.clone() + vi.numDiscreteReal.clone() + vi.numOptimizeConstraints.clone() + vi.numOptimizeFinalConstraints.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfRealAlgebraicVariables      = \"NY:       number of real variables,                           OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfRealAlgebraicAliasVariables = \"")).clone());
        File::writeInt(file.clone(), vi.numAlgAliasVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfRealAlgebraicAliasVariables = \"NA:       number of alias variables,                          OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfRealParameters              = \"")).clone());
        File::writeInt(file.clone(), vi.numParams.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfRealParameters              = \"NP:       number of parameters,                               OMC\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfIntegerAlgebraicVariables   = \"")).clone());
        File::writeInt(file.clone(), vi.numIntAlgVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfIntegerAlgebraicVariables   = \"NYINT:    number of alg. int variables,                       OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfIntegerAliasVariables       = \"")).clone());
        File::writeInt(file.clone(), vi.numIntAliasVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfIntegerAliasVariables       = \"NAINT:    number of alias int variables,                      OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfIntegerParameters           = \"")).clone());
        File::writeInt(file.clone(), vi.numIntParams.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfIntegerParameters           = \"NPINT:    number of int parameters,                           OMC\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfStringAlgebraicVariables    = \"")).clone());
        File::writeInt(file.clone(), vi.numStringAlgVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfStringAlgebraicVariables    = \"NYSTR:    number of alg. string variables,                    OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfStringAliasVariables        = \"")).clone());
        File::writeInt(file.clone(), vi.numStringAliasVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfStringAliasVariables        = \"NASTR:    number of alias string variables,                   OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfStringParameters            = \"")).clone());
        File::writeInt(file.clone(), vi.numStringParamVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfStringParameters            = \"NPSTR:    number of string parameters,                        OMC\"\n\n")).clone());
        File::write(file.clone(), (literal!("  numberOfBooleanAlgebraicVariables   = \"")).clone());
        File::writeInt(file.clone(), vi.numBoolAlgVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfBooleanAlgebraicVariables   = \"NYBOOL:   number of alg. bool variables,                      OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfBooleanAliasVariables       = \"")).clone());
        File::writeInt(file.clone(), vi.numBoolAliasVars.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfBooleanAliasVariables       = \"NABOOL:   number of alias bool variables,                     OMC\"\n")).clone());
        File::write(file.clone(), (literal!("  numberOfBooleanParameters           = \"")).clone());
        File::writeInt(file.clone(), vi.numBoolParams.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"  cmt_numberOfBooleanParameters           = \"NPBOOL:   number of bool parameters,                          OMC\" >\n\n\n")).clone());
        File::write(file.clone(), (literal!("  <!-- startTime, stopTime, tolerance are FMI specific, all others are OMC specific -->\n")).clone());
        File::write(file.clone(), (literal!("  <DefaultExperiment\n")).clone());
        File::write(file.clone(), (literal!("    startTime      = \"")).clone());
        File::writeReal(file.clone(), s.startTime.clone(), (literal!("%.15g")).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("    stopTime       = \"")).clone());
        File::writeReal(file.clone(), s.stopTime.clone(), (literal!("%.15g")).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("    stepSize       = \"")).clone());
        File::writeReal(file.clone(), s.stepSize.clone(), (literal!("%.15g")).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("    tolerance      = \"")).clone());
        File::writeReal(file.clone(), s.tolerance.clone(), (literal!("%.15g")).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("    solver         = \"")).clone());
        File::write(file.clone(), (s.method.clone()).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("    outputFormat   = \"")).clone());
        File::write(file.clone(), (s.outputFormat.clone()).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
        File::write(file.clone(), (literal!("    variableFilter = \"")).clone());
        File::write(file.clone(), (s.variableFilter.clone()).clone());
        File::write(file.clone(), (literal!("\" />\n\n")).clone());
        File::write(file.clone(), (literal!("  <!-- variables in the model -->\n")).clone());
        File::write(file.clone(), (literal!("  <ModelVariables>\n\n")).clone());
        unwrap_break_err!(modelVariables(file.clone(), simCode.modelInfo.vars.clone()), '__try0);
        File::write(file.clone(), (literal!("\n\n\n  </ModelVariables>\n\n")).clone());
        File::write(file.clone(), (literal!("\n</fmiModelDescription>\n\n")).clone());
        success = true;
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    success
}

fn modelVariables(mut file: File::File, mut vars: SimCodeVar::SimVars) -> Result<()> {
    let mut vr: i32 = 0;
    let mut ix: i32 = 0;
    vr = (::match_deref::match_deref! { match &(Config::simCodeTarget()?) {
        Deref @ "omsic" => 0,
        Deref @ "omsicpp" => 0,
        _ => 1000,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (vr, _) = scalarVariables(file.clone(), vars.stateVars.clone(), (literal!("rSta")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.derivativeVars.clone(), (literal!("rDer")).clone(), vr.clone(), 0)?;
    (vr, ix) = scalarVariables(file.clone(), vars.algVars.clone(), (literal!("rAlg")).clone(), vr.clone(), ix.clone())?;
    (vr, ix) = scalarVariables(file.clone(), vars.discreteAlgVars.clone(), (literal!("rAlg")).clone(), vr.clone(), ix.clone())?;
    (vr, ix) = scalarVariables(file.clone(), vars.realOptimizeConstraintsVars.clone(), (literal!("rAlg")).clone(), vr.clone(), ix.clone())?;
    (vr, ix) = scalarVariables(file.clone(), vars.realOptimizeFinalConstraintsVars.clone(), (literal!("rAlg")).clone(), vr.clone(), ix.clone())?;
    (vr, _) = scalarVariables(file.clone(), vars.paramVars.clone(), (literal!("rPar")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.aliasVars.clone(), (literal!("rAli")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.intAlgVars.clone(), (literal!("iAlg")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.intParamVars.clone(), (literal!("iPar")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.intAliasVars.clone(), (literal!("iAli")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.boolAlgVars.clone(), (literal!("bAlg")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.boolParamVars.clone(), (literal!("bPar")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.boolAliasVars.clone(), (literal!("bAli")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.stringAlgVars.clone(), (literal!("sAlg")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.stringParamVars.clone(), (literal!("sPar")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.stringAliasVars.clone(), (literal!("sAli")).clone(), vr.clone(), 0)?;
    (vr, _) = scalarVariables(file.clone(), vars.sensitivityVars.clone(), (literal!("rSen")).clone(), vr.clone(), 0)?;
    Ok(())
}

fn scalarVariables(mut file: File::File, mut vars: Arc<metamodelica::List<SimVar>>, mut classType: ArcStr, mut valueReference: i32, mut index: i32) -> Result<(i32, i32)> {
    let mut valueReference: i32 = valueReference;
    let mut index: i32 = index;
    for mut var in &*vars.clone() {
        let mut var = var.clone();
        scalarVariable(file.clone(), var.clone(), (classType.clone()).clone(), valueReference.clone(), index.clone())?;
        index = index.clone() + 1;
        valueReference = valueReference.clone() + 1;
    }
    Ok((valueReference, index))
}

fn scalarVariable(mut file: File::File, mut var: SimVar, mut classType: ArcStr, mut valueReference: i32, mut classIndex: i32) -> Result<()> {
    let mut type_name: ArcStr = if (DAEUtil::expTypeArray(var.type_.clone())) {literal!("ArrayVariable")} else {literal!("ScalarVariable")};
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  <")); __mm_s.push_str(&*type_name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    scalarVariableAttribute(file.clone(), var.clone(), (classType.clone()).clone(), valueReference.clone(), classIndex.clone())?;
    File::write(file.clone(), (literal!("    ")).clone());
    scalarVariableType(file.clone(), var.clone())?;
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  </")); __mm_s.push_str(&*type_name.clone()); __mm_s.push_str(&*literal!(">\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn scalarVariableAttribute(mut file: File::File, mut simVar: SimVar, mut classType: ArcStr, mut valueReference: i32, mut classIndex: i32) -> Result<()> {
    let mut inputIndex: i32 = SimCodeUtil::getInputIndex(simVar.clone())?;
    let mut info: SourceInfo = simVar.source.info.clone();
    File::write(file.clone(), (literal!("    name = \"")).clone());
    CR::writeCref(file.clone(), simVar.name.clone(), XML.clone())?;
    File::write(file.clone(), (literal!("\"\n")).clone());
    File::write(file.clone(), (literal!("    valueReference = \"")).clone());
    File::writeInt(file.clone(), valueReference.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("\"\n")).clone());
    if simVar.comment.clone() != literal!("") {
        File::write(file.clone(), (literal!("    description = \"")).clone());
        File::writeEscape(file.clone(), (simVar.comment.clone()).clone(), XML.clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
    }
    File::write(file.clone(), (literal!("    variability = \"")).clone());
    File::write(file.clone(), (getVariablity(simVar.varKind.clone())).clone());
    File::write(file.clone(), (literal!("\" isDiscrete = \"")).clone());
    File::write(file.clone(), ArcStr::from(::std::format!("{}", simVar.isDiscrete.clone())));
    File::write(file.clone(), (literal!("\"\n")).clone());
    File::write(file.clone(), (literal!("    causality = \"")).clone());
    File::write(file.clone(), (getCausality(simVar.causality.clone())).clone());
    File::write(file.clone(), (literal!("\" isValueChangeable = \"")).clone());
    File::write(file.clone(), ArcStr::from(::std::format!("{}", simVar.isValueChangeable.clone())));
    File::write(file.clone(), (literal!("\"\n")).clone());
    if inputIndex.clone() != -1 {
        File::write(file.clone(), (literal!("    inputIndex = \"")).clone());
        File::writeInt(file.clone(), inputIndex.clone(), (literal!("%d")).clone());
        File::write(file.clone(), (literal!("\"\n")).clone());
    }
    File::write(file.clone(), (literal!("    alias = ")).clone());
    getAliasVar(file.clone(), simVar.clone())?;
    File::write(file.clone(), (literal!("\n")).clone());
    File::write(file.clone(), (literal!("    classIndex = \"")).clone());
    File::writeInt(file.clone(), classIndex.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("\" classType = \"")).clone());
    File::write(file.clone(), (classType.clone()).clone());
    File::write(file.clone(), (literal!("\"\n")).clone());
    File::write(file.clone(), (literal!("    isProtected = \"")).clone());
    File::write(file.clone(), ArcStr::from(::std::format!("{}", simVar.isProtected.clone())));
    File::write(file.clone(), (literal!("\" hideResult = \"")).clone());
    File::write(file.clone(), (Util::applyOptionOrDefault(simVar.hideResult.clone(), (std::sync::Arc::new(fnptr!(boolString, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<ArcStr> + 'static>), (literal!("")).clone())).clone());
    File::write(file.clone(), (literal!("\" isEncrypted = \"")).clone());
    File::write(file.clone(), (boolString(simVar.isEncrypted.clone())).clone());
    File::write(file.clone(), (literal!("\" initNonlinear = \"")).clone());
    File::write(file.clone(), (boolString(simVar.initNonlinear.clone())).clone());
    File::write(file.clone(), (literal!("\"\n")).clone());
    File::write(file.clone(), (literal!("    fileName = \"")).clone());
    File::writeEscape(file.clone(), info.fileName.clone(), XML.clone());
    File::write(file.clone(), (literal!("\" startLine = \"")).clone());
    File::writeInt(file.clone(), info.lineNumberStart.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("\" startColumn = \"")).clone());
    File::writeInt(file.clone(), info.columnNumberStart.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("\" endLine = \"")).clone());
    File::writeInt(file.clone(), info.lineNumberEnd.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("\" endColumn = \"")).clone());
    File::writeInt(file.clone(), info.columnNumberEnd.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("\" fileWritable = \"")).clone());
    File::write(file.clone(), ArcStr::from(::std::format!("{}", !(info.isReadOnly.clone()))));
    File::write(file.clone(), (literal!("\">\n")).clone());
    for mut dim in &*Expression::arrayDimension(simVar.type_.clone()) {
        let mut dim = dim.clone();
        File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    <Dimension start=\"")); __mm_s.push_str(&*intString(Expression::dimensionSize(dim.clone())?)); __mm_s.push_str(&*literal!("\"/>\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn scalarVariableType(mut file: File::File, mut v: SimVar) -> Result<()> {
    let mut path: Arc<Absyn::Path>;
    let () = (::match_deref::match_deref! { match &(Types::arrayElementType(v.type_.clone())) {
        Deref @ Type::T_INTEGER { .. } => {
            File::write(file.clone(), (literal!("<Integer")).clone());
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), (literal!("start")).clone());
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeAttribute(file.clone(), v.minValue.clone(), (literal!("min")).clone());
            scalarVariableTypeAttribute(file.clone(), v.maxValue.clone(), (literal!("max")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.unit.clone()).clone(), (literal!("unit")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.displayUnit.clone()).clone(), (literal!("displayUnit")).clone());
            File::write(file.clone(), (literal!(" />")).clone());
            ()
        },
        Deref @ Type::T_REAL { .. } => {
            File::write(file.clone(), (literal!("<Real")).clone());
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), (literal!("start")).clone());
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeUseAttribute(file.clone(), v.nominalValue.clone(), (literal!("useNominal")).clone(), (literal!("nominal")).clone());
            scalarVariableTypeAttribute(file.clone(), v.minValue.clone(), (literal!("min")).clone());
            scalarVariableTypeAttribute(file.clone(), v.maxValue.clone(), (literal!("max")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.unit.clone()).clone(), (literal!("unit")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.displayUnit.clone()).clone(), (literal!("displayUnit")).clone());
            File::write(file.clone(), (literal!(" />")).clone());
            ()
        },
        Deref @ Type::T_BOOL { .. } => {
            File::write(file.clone(), (literal!("<Boolean")).clone());
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), (literal!("start")).clone());
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.unit.clone()).clone(), (literal!("unit")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.displayUnit.clone()).clone(), (literal!("displayUnit")).clone());
            File::write(file.clone(), (literal!(" />")).clone());
            ()
        },
        Deref @ Type::T_STRING { .. } => {
            File::write(file.clone(), (literal!("<String")).clone());
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), (literal!("start")).clone());
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.unit.clone()).clone(), (literal!("unit")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.displayUnit.clone()).clone(), (literal!("displayUnit")).clone());
            File::write(file.clone(), (literal!(" />")).clone());
            ()
        },
        Deref @ Type::T_ENUMERATION { .. } => {
            File::write(file.clone(), (literal!("<Integer")).clone());
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), (literal!("start")).clone());
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.unit.clone()).clone(), (literal!("unit")).clone());
            scalarVariableTypeStringAttribute(file.clone(), (v.displayUnit.clone()).clone(), (literal!("displayUnit")).clone());
            File::write(file.clone(), (literal!(" />")).clone());
            ()
        },
        Deref @ Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path }, .. } => {
            File::write(file.clone(), (literal!("<ExternalObject path=\"")).clone());
            Dump::writePath(file.clone(), path.clone(), XML.clone(), (literal!(".")).clone(), true)?;
            File::write(file.clone(), (literal!("\" />")).clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SerializeInitXML.scalarVariableType")); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*TypesDump::unparseType(v.type_.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn scalarVariableTypeUseAttribute(mut file: File::File, mut attr: Option<Arc<Exp>>, mut r#use: ArcStr, mut name: ArcStr) -> () {
    let mut expStr: ArcStr = arcstr::literal!("");
    File::write(file.clone(), (literal!(" ")).clone());
    File::write(file.clone(), (r#use.clone()).clone());
    File::write(file.clone(), (literal!("=\"")).clone());
    File::write(file.clone(), ArcStr::from(::std::format!("{}", isSome(attr.clone()))));
    File::write(file.clone(), (literal!("\"")).clone());
    scalarVariableTypeAttribute(file.clone(), attr.clone(), (name.clone()).clone());
    ()
}

fn scalarVariableTypeFixedAttribute(mut file: File::File, mut isFixed: bool) -> () {
    File::write(file.clone(), (literal!(" fixed=\"")).clone());
    File::write(file.clone(), ArcStr::from(::std::format!("{}", isFixed.clone())));
    File::write(file.clone(), (literal!("\"")).clone());
    ()
}

fn scalarVariableTypeAttribute(mut file: File::File, mut attr: Option<Arc<Exp>>, mut name: ArcStr) -> () {
    let mut expStr: ArcStr = arcstr::literal!("");
    if '__try0: {
        expStr = (unwrap_break_err!(expString(Util::getOption(attr.clone()).unwrap()), '__try0)).clone();
        File::write(file.clone(), (literal!(" ")).clone());
        File::write(file.clone(), (name.clone()).clone());
        File::write(file.clone(), (literal!("=\"")).clone());
        File::write(file.clone(), (expStr.clone()).clone());
        File::write(file.clone(), (literal!("\"")).clone());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    ()
}

fn scalarVariableTypeStringAttribute(mut file: File::File, mut attr: ArcStr, mut name: ArcStr) -> () {
    if attr.clone() == literal!("") {
        return ();
    }
    File::write(file.clone(), (literal!(" ")).clone());
    File::write(file.clone(), (name.clone()).clone());
    File::write(file.clone(), (literal!("=\"")).clone());
    File::writeEscape(file.clone(), (attr.clone()).clone(), XML.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    ()
}

fn getCausality(mut c: Option<Causality>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match c.clone() {
        Some(SimCodeVar::Causality::NONECAUS) => literal!("none"),
        Some(SimCodeVar::Causality::OUTPUT) => literal!("output"),
        Some(SimCodeVar::Causality::INPUT) => literal!("input"),
        Some(SimCodeVar::Causality::LOCAL) => literal!("local"),
        Some(SimCodeVar::Causality::PARAMETER) => literal!("parameter"),
        Some(SimCodeVar::Causality::CALCULATED_PARAMETER) => literal!("calculatedParameter"),
        _ => literal!("local"),
    })).clone();
    r#str
}

fn getVariablity(mut varKind: VarKind) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match varKind.clone() {
        VarKind::DISCRETE => literal!("discrete"),
        VarKind::PARAM => literal!("parameter"),
        VarKind::CONST => literal!("constant"),
        _ => literal!("continuous"),
    })).clone();
    r#str
}

fn getAliasVar(mut file: File::File, mut simVar: SimVar) -> Result<()> {
    let () = (match simVar.clone() {
        SimVar { aliasvar: mut aliasvar @ SimCodeVar::AliasVariable::ALIAS { .. }, .. } => {
            File::write(file.clone(), (literal!("\"alias\" aliasVariable=\"")).clone());
            CR::writeCref(file.clone(), var_field!(aliasvar.varName, AliasVariable::ALIAS).clone(), XML.clone())?;
            File::write(file.clone(), (literal!("\" aliasVariableId=\"")).clone());
            File::write(file.clone(), (SimCodeUtil::getValueReference(simVar.clone(), SimCodeUtil::getSimCode()?, true)?).clone());
            File::write(file.clone(), (literal!("\"")).clone());
            ()
        },
        SimVar { aliasvar: mut aliasvar @ SimCodeVar::AliasVariable::NEGATEDALIAS { .. }, .. } => {
            File::write(file.clone(), (literal!("\"negatedAlias\" aliasVariable=\"")).clone());
            CR::writeCref(file.clone(), var_field!(aliasvar.varName, AliasVariable::NEGATEDALIAS).clone(), XML.clone())?;
            File::write(file.clone(), (literal!("\" aliasVariableId=\"")).clone());
            File::write(file.clone(), (SimCodeUtil::getValueReference(simVar.clone(), SimCodeUtil::getSimCode()?, true)?).clone());
            File::write(file.clone(), (literal!("\"")).clone());
            ()
        },
        _ => {
            File::write(file.clone(), (literal!("\"noAlias\"")).clone());
            ()
        },
    });
    Ok(())
}

fn xsdateTime(mut file: File::File, mut dt: Util::DateTime) -> () {
    File::writeInt(file.clone(), dt.year.clone(), (literal!("%d")).clone());
    File::writeInt(file.clone(), dt.mon.clone(), (literal!("-%02d")).clone());
    File::writeInt(file.clone(), dt.mday.clone(), (literal!("-%02d")).clone());
    File::writeInt(file.clone(), dt.hour.clone(), (literal!("T%02d")).clone());
    File::writeInt(file.clone(), dt.min.clone(), (literal!(":%02d")).clone());
    File::writeInt(file.clone(), dt.sec.clone(), (literal!(":%02dZ")).clone());
    ()
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn expString(mut exp: Arc<Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Exp::ICONST { .. } => intString(var_field!((*exp).integer, Exp::ICONST).clone()),
        Deref @ Exp::RCONST { .. } => realString(var_field!((*exp).real, Exp::RCONST).clone()),
        Deref @ Exp::SCONST { .. } => Util::escapeModelicaStringToXmlString((var_field!((*exp).string, Exp::SCONST).clone()).clone())?,
        Deref @ Exp::BCONST { .. } => boolString(var_field!((*exp).bool, Exp::BCONST).clone()),
        Deref @ Exp::ENUM_LITERAL { .. } => intString(var_field!((*exp).index, Exp::ENUM_LITERAL).clone()),
        Deref @ Exp::ARRAY { .. } if (Expression::isSimpleLiteralValue(exp.clone(), true)) => stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).array, Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = expString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(" ")).clone()),
        Deref @ Exp::REDUCTION { .. } => expString(var_field!((*exp).expr, Exp::REDUCTION).clone())?,
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

