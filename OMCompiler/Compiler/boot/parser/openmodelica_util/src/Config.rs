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

use crate::Error;
use crate::Flags;
use crate::FlagsUtil;
use crate::System;

/// Defines the various modelica language versions that OMC can use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum LanguageStandard {
    _1_x = 1,
    _2_x = 2,
    _3_0 = 3,
    _3_1 = 4,
    _3_2 = 5,
    _3_3 = 6,
    _3_4 = 7,
    _3_5 = 8,
    _3_6 = 9,
    latest = 10,
    experimental = 11,
}
impl PartialOrd for LanguageStandard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for LanguageStandard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn typeinfo() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::TYPE_INFO.clone())?;
    Ok(outBoolean)
}

pub fn splitArrays() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = !(Flags::getConfigBool(Flags::KEEP_ARRAYS.clone())?);
    Ok(outBoolean)
}

pub fn modelicaOutput() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    Ok(outBoolean)
}

pub fn noProc() -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = noProcWork(Flags::getConfigInt(Flags::NUM_PROC.clone())?);
    Ok(outInteger)
}

fn noProcWork(mut inProc: i32) -> i32 {
    let mut outInteger: i32 = 0;
    outInteger = (match inProc.clone() {
        0 => System::numProcessors(),
        _ => inProc.clone(),
    });
    outInteger
}

pub fn simulationCg() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::SIMULATION_CG.clone())?;
    Ok(outBoolean)
}

pub fn simulation() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::SIMULATION.clone())?;
    Ok(outBoolean)
}

pub fn simulationCodeTarget() -> Result<ArcStr> {
    let mut outCodeTarget: ArcStr = arcstr::literal!("");
    outCodeTarget = (Flags::getConfigString(Flags::TARGET.clone())?).clone();
    Ok(outCodeTarget)
}

pub fn classToInstantiate() -> Result<ArcStr> {
    let mut modelName: ArcStr = arcstr::literal!("");
    modelName = (Flags::getConfigString(Flags::INST_CLASS.clone())?).clone();
    Ok(modelName)
}

pub fn silent() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::SILENT.clone())?;
    Ok(outBoolean)
}

pub fn versionRequest() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::SHOW_VERSION.clone())?;
    Ok(outBoolean)
}

pub fn helpRequest() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = !(stringEq((Flags::getConfigString(Flags::HELP.clone())?).clone(), (literal!("")).clone()));
    Ok(outBoolean)
}

pub fn acceptedGrammar() -> Result<i32> {
    let mut outGrammer: i32 = 0;
    outGrammer = Flags::getConfigEnum(Flags::GRAMMAR.clone())?;
    Ok(outGrammer)
}

pub fn acceptMetaModelicaGrammar() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::METAMODELICA.clone());
    Ok(outBoolean)
}

pub fn acceptParModelicaGrammar() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PARMODELICA.clone());
    Ok(outBoolean)
}

pub fn acceptOptimicaGrammar() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::OPTIMICA.clone());
    Ok(outBoolean)
}

pub fn acceptPDEModelicaGrammar() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone());
    Ok(outBoolean)
}

pub fn getAnnotationVersion() -> Result<ArcStr> {
    let mut annotationVersion: ArcStr = arcstr::literal!("");
    annotationVersion = (Flags::getConfigString(Flags::ANNOTATION_VERSION.clone())?).clone();
    Ok(annotationVersion)
}

pub fn setAnnotationVersion(mut annotationVersion: ArcStr) -> Result<()> {
    FlagsUtil::setConfigString(Flags::ANNOTATION_VERSION.clone(), (annotationVersion.clone()).clone())?;
    Ok(())
}

pub fn getNoSimplify() -> Result<bool> {
    let mut noSimplify: bool = false;
    noSimplify = Flags::getConfigBool(Flags::NO_SIMPLIFY.clone())?;
    Ok(noSimplify)
}

pub fn setNoSimplify(mut noSimplify: bool) -> Result<()> {
    FlagsUtil::setConfigBool(Flags::NO_SIMPLIFY.clone(), noSimplify.clone())?;
    Ok(())
}

pub fn vectorizationLimit() -> Result<i32> {
    let mut limit: i32 = 0;
    limit = Flags::getConfigInt(Flags::VECTORIZATION_LIMIT.clone())?;
    Ok(limit)
}

pub fn setVectorizationLimit(mut limit: i32) -> Result<()> {
    FlagsUtil::setConfigInt(Flags::VECTORIZATION_LIMIT.clone(), limit.clone())?;
    Ok(())
}

pub fn getDefaultOpenCLDevice() -> Result<i32> {
    let mut defdevid: i32 = 0;
    defdevid = Flags::getConfigInt(Flags::DEFAULT_OPENCL_DEVICE.clone())?;
    Ok(defdevid)
}

pub fn setDefaultOpenCLDevice(mut defdevid: i32) -> Result<()> {
    FlagsUtil::setConfigInt(Flags::DEFAULT_OPENCL_DEVICE.clone(), defdevid.clone())?;
    Ok(())
}

pub fn showAnnotations() -> Result<bool> {
    let mut show: bool = false;
    show = Flags::getConfigBool(Flags::SHOW_ANNOTATIONS.clone())?;
    Ok(show)
}

pub fn setShowAnnotations(mut show: bool) -> Result<()> {
    FlagsUtil::setConfigBool(Flags::SHOW_ANNOTATIONS.clone(), show.clone())?;
    Ok(())
}

pub fn showStructuralAnnotations() -> Result<bool> {
    let mut show: bool = false;
    show = Flags::getConfigBool(Flags::SHOW_STRUCTURAL_ANNOTATIONS.clone())?;
    Ok(show)
}

pub fn showStartOrigin() -> Result<bool> {
    let mut show: bool = false;
    show = Flags::isSet(Flags::SHOW_START_ORIGIN.clone())?;
    Ok(show)
}

pub fn getEvaluateParametersInAnnotations() -> Result<bool> {
    let mut shouldEvaluate: bool = false;
    shouldEvaluate = Flags::getConfigBool(Flags::EVAL_PARAMS_IN_ANNOTATIONS.clone())?;
    Ok(shouldEvaluate)
}

pub fn setEvaluateParametersInAnnotations(mut shouldEvaluate: bool) -> Result<()> {
    FlagsUtil::setConfigBool(Flags::EVAL_PARAMS_IN_ANNOTATIONS.clone(), shouldEvaluate.clone())?;
    Ok(())
}

pub fn getGraphicsExpMode() -> Result<bool> {
    let mut graphicsExpMode: bool = false;
    graphicsExpMode = Flags::getConfigBool(Flags::GRAPHICS_EXP_MODE.clone())?;
    Ok(graphicsExpMode)
}

pub fn setGraphicsExpMode(mut graphicsExpMode: bool) -> Result<()> {
    FlagsUtil::setConfigBool(Flags::GRAPHICS_EXP_MODE.clone(), graphicsExpMode.clone())?;
    Ok(())
}

pub fn orderConnections() -> Result<bool> {
    let mut show: bool = false;
    show = Flags::getConfigBool(Flags::ORDER_CONNECTIONS.clone())?;
    Ok(show)
}

pub fn setOrderConnections(mut show: bool) -> Result<()> {
    FlagsUtil::setConfigBool(Flags::ORDER_CONNECTIONS.clone(), show.clone())?;
    Ok(())
}

pub fn getPreOptModules() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = Flags::getConfigStringList(Flags::PRE_OPT_MODULES.clone())?;
    Ok(outStringLst)
}

pub fn getPostOptModules() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = Flags::getConfigStringList(Flags::POST_OPT_MODULES.clone())?;
    Ok(outStringLst)
}

pub fn getPostOptModulesDAE() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = Flags::getConfigStringList(Flags::POST_OPT_MODULES_DAE.clone())?;
    Ok(outStringLst)
}

pub fn getInitOptModules() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = Flags::getConfigStringList(Flags::INIT_OPT_MODULES.clone())?;
    Ok(outStringLst)
}

pub fn setPreOptModules(mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    FlagsUtil::setConfigStringList(Flags::PRE_OPT_MODULES.clone(), inStringLst.clone())?;
    Ok(())
}

pub fn setPostOptModules(mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    FlagsUtil::setConfigStringList(Flags::POST_OPT_MODULES.clone(), inStringLst.clone())?;
    Ok(())
}

pub fn getIndexReductionMethod() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Flags::getConfigString(Flags::INDEX_REDUCTION_METHOD.clone())?).clone();
    Ok(outString)
}

pub fn setIndexReductionMethod(mut inString: ArcStr) -> Result<()> {
    FlagsUtil::setConfigString(Flags::INDEX_REDUCTION_METHOD.clone(), (inString.clone()).clone())?;
    Ok(())
}

pub fn getCheapMatchingAlgorithm() -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = Flags::getConfigInt(Flags::CHEAPMATCHING_ALGORITHM.clone())?;
    Ok(outInteger)
}

pub fn setCheapMatchingAlgorithm(mut inInteger: i32) -> Result<()> {
    FlagsUtil::setConfigInt(Flags::CHEAPMATCHING_ALGORITHM.clone(), inInteger.clone())?;
    Ok(())
}

pub fn getMatchingAlgorithm() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Flags::getConfigString(Flags::MATCHING_ALGORITHM.clone())?).clone();
    Ok(outString)
}

pub fn setMatchingAlgorithm(mut inString: ArcStr) -> Result<()> {
    FlagsUtil::setConfigString(Flags::MATCHING_ALGORITHM.clone(), (inString.clone()).clone())?;
    Ok(())
}

pub fn getTearingMethod() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Flags::getConfigString(Flags::TEARING_METHOD.clone())?).clone();
    Ok(outString)
}

pub fn setTearingMethod(mut inString: ArcStr) -> Result<()> {
    FlagsUtil::setConfigString(Flags::TEARING_METHOD.clone(), (inString.clone()).clone())?;
    Ok(())
}

pub fn getTearingHeuristic() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Flags::getConfigString(Flags::TEARING_HEURISTIC.clone())?).clone();
    Ok(outString)
}

pub fn setTearingHeuristic(mut inString: ArcStr) -> Result<()> {
    FlagsUtil::setConfigString(Flags::TEARING_HEURISTIC.clone(), (inString.clone()).clone())?;
    Ok(())
}

pub fn simCodeTarget() -> Result<ArcStr> {
    let mut target: ArcStr = arcstr::literal!("");
    target = (Flags::getConfigString(Flags::SIMCODE_TARGET.clone())?).clone();
    Ok(target)
}

pub fn setsimCodeTarget(mut inString: ArcStr) -> Result<()> {
    FlagsUtil::setConfigString(Flags::SIMCODE_TARGET.clone(), (inString.clone()).clone())?;
    Ok(())
}

pub fn getLanguageStandard() -> Result<LanguageStandard> {
    let mut outStandard: LanguageStandard = LanguageStandard::_1_x;
    outStandard = intLanguageStandard(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?)?;
    Ok(outStandard)
}

pub fn setLanguageStandard(mut inStandard: LanguageStandard) -> Result<()> {
    FlagsUtil::setConfigEnum(Flags::LANGUAGE_STANDARD.clone(), languageStandardInt(inStandard.clone())?)?;
    Ok(())
}

pub fn languageStandardAtLeast(mut inStandard: LanguageStandard) -> Result<bool> {
    let mut outRes: bool = false;
    let mut std: LanguageStandard = LanguageStandard::_1_x;
    std = getLanguageStandard()?;
    outRes = intGe(languageStandardInt(std.clone())?, languageStandardInt(inStandard.clone())?);
    Ok(outRes)
}

pub fn languageStandardAtMost(mut inStandard: LanguageStandard) -> Result<bool> {
    let mut outRes: bool = false;
    let mut std: LanguageStandard = LanguageStandard::_1_x;
    std = getLanguageStandard()?;
    outRes = intLe(languageStandardInt(std.clone())?, languageStandardInt(inStandard.clone())?);
    Ok(outRes)
}

fn languageStandardInt(mut inStandard: LanguageStandard) -> Result<i32> {
    let mut outValue: i32 = 0;
    let lookup: metamodelica::Array<i32> = metamodelica::Dangerous::listArray(list![10, 20, 30, 31, 32, 33, 34, 35, 36, 1000, 9999]);
    outValue = lookup.clone().borrow()[(((inStandard.clone()) as i32)-1) as usize].clone();
    Ok(outValue)
}

fn intLanguageStandard(mut inValue: i32) -> Result<LanguageStandard> {
    let mut outStandard: LanguageStandard = LanguageStandard::_1_x;
    outStandard = (match inValue.clone() {
        10 => LanguageStandard::_1_x.clone(),
        20 => LanguageStandard::_2_x.clone(),
        30 => LanguageStandard::_3_0.clone(),
        31 => LanguageStandard::_3_1.clone(),
        32 => LanguageStandard::_3_2.clone(),
        33 => LanguageStandard::_3_3.clone(),
        34 => LanguageStandard::_3_4.clone(),
        35 => LanguageStandard::_3_5.clone(),
        36 => LanguageStandard::_3_6.clone(),
        1000 => LanguageStandard::latest.clone(),
        9999 => LanguageStandard::experimental.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outStandard)
}

pub fn languageStandardString(mut inStandard: LanguageStandard) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let lookup: metamodelica::Array<ArcStr> = metamodelica::Dangerous::listArray(list![(literal!("1.x")).clone(), (literal!("2.x")).clone(), (literal!("3.0")).clone(), (literal!("3.1")).clone(), (literal!("3.2")).clone(), (literal!("3.3")).clone(), (literal!("3.4")).clone(), (literal!("3.5")).clone(), (literal!("3.6")).clone(), (literal!("3.6")).clone(), (literal!("experimental")).clone()]);
    outString = (lookup.clone().borrow()[(((inStandard.clone()) as i32)-1) as usize].clone()).clone();
    Ok(outString)
}

pub fn setLanguageStandardFromMSL(mut inLibraryName: ArcStr, mut force: bool) -> Result<()> {
    let mut current_std: LanguageStandard = LanguageStandard::_1_x;
    current_std = getLanguageStandard()?;
    if !(force.clone()) && current_std.clone() != LanguageStandard::latest.clone() {
        return Ok(());
    }
    let () = 'mc: {
        let __mc_input = inLibraryName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut version: ArcStr = arcstr::literal!("");
            let mut new_std: LanguageStandard = LanguageStandard::_1_x;
            let __pa0 = ::match_deref::match_deref! { match &(System::strtok((inLibraryName.clone()).clone(), (literal!(" ")).clone())) {
                Deref @ metamodelica::List::Cons { head: Deref @ "Modelica", tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            version = __pa0.clone();
            new_std = versionStringToStd((version.clone()).clone());
            if new_std.clone() != current_std.clone() {
                setLanguageStandard(new_std.clone())?;
                if hasLanguageStandardChanged(current_std.clone())? {
                    Error::addMessage(Error::CHANGED_STD_VERSION.clone(), list![(languageStandardString(new_std.clone())?).clone(), (version.clone()).clone()])?;
                }
            }
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

fn hasLanguageStandardChanged(mut inOldStandard: LanguageStandard) -> Result<bool> {
    let mut outHasChanged: bool = false;
    outHasChanged = languageStandardAtMost(LanguageStandard::_3_0.clone())?;
    Ok(outHasChanged)
}

pub fn versionStringToStd(mut inVersion: ArcStr) -> LanguageStandard {
    let mut outStandard: LanguageStandard = LanguageStandard::_1_x;
    let mut version: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    version = System::strtok((inVersion.clone()).clone(), (literal!(".")).clone());
    outStandard = versionStringToStd2(version.clone());
    outStandard
}

fn versionStringToStd2(mut inVersion: Arc<metamodelica::List<ArcStr>>) -> LanguageStandard {
    let mut outStandard: LanguageStandard = LanguageStandard::_1_x;
    outStandard = (::match_deref::match_deref! { match &(inVersion.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ "1", tail: _ } => LanguageStandard::_1_x.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ "2", tail: _ } => LanguageStandard::_2_x.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ "3", tail: Deref @ metamodelica::List::Cons { head: Deref @ "0", tail: _ } } => LanguageStandard::_3_0.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ "3", tail: Deref @ metamodelica::List::Cons { head: Deref @ "1", tail: _ } } => LanguageStandard::_3_1.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ "3", tail: _ } => LanguageStandard::_3_2.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ "4", tail: Deref @ metamodelica::List::Cons { head: Deref @ "0", tail: _ } } => LanguageStandard::_3_4.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ "4", tail: Deref @ metamodelica::List::Cons { head: Deref @ "1", tail: _ } } => LanguageStandard::_3_6.clone(),
        _ => LanguageStandard::latest.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStandard
}

pub fn showErrorMessages() -> Result<bool> {
    let mut outShowErrorMessages: bool = false;
    outShowErrorMessages = Flags::getConfigBool(Flags::SHOW_ERROR_MESSAGES.clone())?;
    Ok(outShowErrorMessages)
}

pub fn scalarizeMinMax() -> Result<bool> {
    let mut outScalarizeMinMax: bool = false;
    outScalarizeMinMax = Flags::getConfigBool(Flags::SCALARIZE_MINMAX.clone())?;
    Ok(outScalarizeMinMax)
}

pub fn scalarizeBindings() -> Result<bool> {
    let mut outScalarizeBindings: bool = false;
    outScalarizeBindings = Flags::getConfigBool(Flags::SCALARIZE_BINDINGS.clone())?;
    Ok(outScalarizeBindings)
}

pub fn intEnumConversion() -> Result<bool> {
    let mut outIntEnumConversion: bool = false;
    outIntEnumConversion = Flags::getConfigBool(Flags::INT_ENUM_CONVERSION.clone())?;
    Ok(outIntEnumConversion)
}

pub fn profileSome() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 0 == System::strncmp((Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone(), (literal!("blocks")).clone(), 6);
    Ok(outBoolean)
}

pub fn profileAll() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = stringEq((Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone(), (literal!("all")).clone());
    Ok(outBoolean)
}

pub fn profileHtml() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = stringEq((Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone(), (literal!("blocks+html")).clone());
    Ok(outBoolean)
}

pub fn profileFunctions() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = !(stringEq((Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone(), (literal!("none")).clone()));
    Ok(outBoolean)
}

pub fn dynamicTearing() -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Flags::getConfigString(Flags::DYNAMIC_TEARING.clone())?).clone();
    Ok(outString)
}

pub fn ignoreCommandLineOptionsAnnotation() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = Flags::getConfigBool(Flags::IGNORE_COMMAND_LINE_OPTIONS_ANNOTATION.clone())?;
    Ok(outBoolean)
}

pub fn globalHomotopy() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::HOMOTOPY_APPROACH.clone())?) {
        Deref @ "equidistantLocal" => false,
        Deref @ "adaptiveLocal" => false,
        Deref @ "equidistantGlobal" => true,
        Deref @ "adaptiveGlobal" => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

pub fn adaptiveHomotopy() -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::HOMOTOPY_APPROACH.clone())?) {
        Deref @ "equidistantLocal" => false,
        Deref @ "adaptiveLocal" => true,
        Deref @ "equidistantGlobal" => false,
        Deref @ "adaptiveGlobal" => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

pub fn replacedHomotopy() -> Result<bool> {
    let mut outBoolean: bool = false;
    let mut replaceHomotopy: ArcStr = arcstr::literal!("");
    replaceHomotopy = (Flags::getConfigString(Flags::REPLACE_HOMOTOPY.clone())?).clone();
    outBoolean = replaceHomotopy.clone() == literal!("actual") || replaceHomotopy.clone() == literal!("simplified");
    Ok(outBoolean)
}

pub fn synchronousFeaturesAllowed() -> bool {
    let mut outRes: bool = getLanguageStandard().unwrap() >= LanguageStandard::_3_3.clone();
    outRes
}

pub fn flatModelica() -> Result<bool> {
    let mut value: bool = false;
    value = Flags::getConfigBool(Flags::BASE_MODELICA.clone())?;
    if value.clone() && !(Flags::isSet(Flags::SCODE_INST.clone())?) {
        Error::addMessage(Error::INVALID_FLAG_CONDITION.clone(), list![(literal!("-f")).clone(), (literal!("flat modelica requires flag -d=newInst to be set")).clone()])?;
        value = false;
    }
    Ok(value)
}

