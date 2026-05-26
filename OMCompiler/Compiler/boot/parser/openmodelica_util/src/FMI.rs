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

use crate::Flags;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Info {
    pub fmiVersion: ArcStr,
    pub fmiType: i32,
    pub fmiModelName: ArcStr,
    pub fmiModelIdentifier: ArcStr,
    pub fmiGuid: ArcStr,
    pub fmiDescription: ArcStr,
    pub fmiGenerationTool: ArcStr,
    pub fmiGenerationDateAndTime: ArcStr,
    pub fmiVariableNamingConvention: ArcStr,
    pub fmiNumberOfContinuousStates: Arc<metamodelica::List<i32>>,
    pub fmiNumberOfEventIndicators: Arc<metamodelica::List<i32>>,
}

pub type INFO = Info;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeDefinitions {
    pub name: ArcStr,
    pub description: ArcStr,
    pub quantity: ArcStr,
    pub min: i32,
    pub max: i32,
    pub items: Arc<metamodelica::List<EnumerationItem>>,
}

pub type ENUMERATIONTYPE = TypeDefinitions;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumerationItem {
    pub name: ArcStr,
    pub description: ArcStr,
}

pub type ENUMERATIONITEM = EnumerationItem;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExperimentAnnotation {
    pub fmiExperimentStartTime: metamodelica::Real,
    pub fmiExperimentStopTime: metamodelica::Real,
    pub fmiExperimentTolerance: metamodelica::Real,
}

pub type EXPERIMENTANNOTATION = ExperimentAnnotation;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModelVariables {
    REALVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: metamodelica::Real,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    INTEGERVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: i32,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    BOOLEANVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: bool,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    STRINGVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: ArcStr,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    ENUMERATIONVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: i32,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
}
pub use self::ModelVariables::{REALVARIABLE,INTEGERVARIABLE,BOOLEANVARIABLE,STRINGVARIABLE,ENUMERATIONVARIABLE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiImport {
    pub platform: ArcStr,
    pub fmuFileName: ArcStr,
    pub fmuWorkingDirectory: ArcStr,
    pub fmiLogLevel: i32,
    pub fmiDebugOutput: bool,
    pub fmiContext: Option<i32>,
    pub fmiInstance: Option<i32>,
    pub fmiInfo: Info,
    pub fmiTypeDefinitionsList: Arc<metamodelica::List<TypeDefinitions>>,
    pub fmiExperimentAnnotation: ExperimentAnnotation,
    pub fmiModelVariablesInstance: Option<i32>,
    pub fmiModelVariablesList: Arc<metamodelica::List<ModelVariables>>,
    pub generateInputConnectors: bool,
    pub generateOutputConnectors: bool,
}

pub type FMIIMPORT = FmiImport;


pub fn getFMIModelIdentifier(mut inFMIInfo: Info) -> Result<ArcStr> {
    let mut fmiModelIdentifier: ArcStr = arcstr::literal!("");
    fmiModelIdentifier = ((match inFMIInfo.clone() {
        Info { fmiModelIdentifier: mut modelIdentifier, .. } => {
            modelIdentifier.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(fmiModelIdentifier)
}

pub fn getFMIType(mut inFMIInfo: Info) -> Result<ArcStr> {
    let mut fmiType: ArcStr = arcstr::literal!("");
    fmiType = ((::match_deref::match_deref! { match &(inFMIInfo.clone()) {
        Info { fmiType: 0, fmiVersion: Deref @ "1.0", .. } => literal!("me"),
        Info { fmiType: 1, fmiVersion: Deref @ "1.0", .. } => literal!("cs_st"),
        Info { fmiType: 2, fmiVersion: Deref @ "1.0", .. } => literal!("cs_tool"),
        Info { fmiType: 1, fmiVersion: Deref @ "2.0", .. } => literal!("me"),
        Info { fmiType: 2, fmiVersion: Deref @ "2.0", .. } => literal!("cs"),
        Info { fmiType: 3, fmiVersion: Deref @ "2.0", .. } => literal!("me_cs"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(fmiType)
}

pub fn getFMIVersion(mut inFMIInfo: Info) -> Result<ArcStr> {
    let mut fmiVersion: ArcStr = arcstr::literal!("");
    fmiVersion = ((match inFMIInfo.clone() {
        Info { fmiVersion: mut version, .. } => {
            version.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(fmiVersion)
}

pub fn checkFMIVersion(mut inFMIVersion: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &(inFMIVersion.clone()) {
        Deref @ "1.0" => true,
        Deref @ "2.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMIVersion10(mut inFMUVersion: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &(inFMUVersion.clone()) {
        Deref @ "1.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMIVersion20(mut inFMUVersion: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &(inFMUVersion.clone()) {
        Deref @ "2.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn getFMIVersionString() -> ArcStr {
    let mut version: ArcStr = Flags::getConfigString(Flags::FMI_VERSION.clone()).unwrap();
    version
}

pub fn checkFMIType(mut inFMIType: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "me" => true,
        Deref @ "cs" => true,
        Deref @ "me_cs" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn canExportFMU(mut inFMUVersion: ArcStr, mut inFMIType: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &((inFMUVersion.clone(), inFMIType.clone())) {
        (Deref @ "1.0", Deref @ "me") => true,
        (Deref @ "2.0", Deref @ "me") => true,
        (Deref @ "2.0", Deref @ "cs") => true,
        (Deref @ "2.0", Deref @ "me_cs") => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMIMEType(mut inFMIType: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "me" => true,
        Deref @ "me_cs" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMICSType(mut inFMIType: ArcStr) -> bool {
    let mut success: bool = false;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "cs" => true,
        Deref @ "me_cs" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getEnumerationTypeFromTypes(mut inTypeDefinitionsList: Arc<metamodelica::List<TypeDefinitions>>, mut inBaseType: ArcStr) -> Result<ArcStr> {
    let mut outEnumerationType: ArcStr = arcstr::literal!("");
    outEnumerationType = ((::match_deref::match_deref! { match &((inTypeDefinitionsList.clone(), inBaseType.clone())) {
        (Deref @ metamodelica::List::Cons { head: TypeDefinitions { name: name_, .. }, tail: _ }, baseType) if (stringEqual((name_.clone()).clone(), (baseType.clone()).clone())) => {
            name_.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: xs }, baseType) => {
            let mut name_: ArcStr = arcstr::literal!("");
            name_ = (getEnumerationTypeFromTypes(xs.clone(), (baseType.clone()).clone())?).clone();
            name_.clone()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outEnumerationType)
}

pub fn filterModelVariables(mut inModelVariables: Arc<metamodelica::List<ModelVariables>>, mut tipe: ArcStr, mut variableCausality: ArcStr) -> Arc<metamodelica::List<ModelVariables>> {
    let mut outModelVariables: Arc<metamodelica::List<ModelVariables>> = metamodelica::nil();
    outModelVariables = List::filter2OnTrue(inModelVariables.clone(), Arc::new(fnptr!(filterModelVariable, ModelVariables, ArcStr, ArcStr)), (tipe.clone()).clone(), (variableCausality.clone()).clone());
    outModelVariables
}

fn filterModelVariable(mut modelVar: ModelVariables, mut tipe: ArcStr, mut variableCausality: ArcStr) -> bool {
    let mut result: bool = false;
    result = (match modelVar.clone() {
        ModelVariables::REALVARIABLE { causality: mut causality, .. } if (tipe.clone() == literal!("real") && causality.clone() == variableCausality.clone()) => {
            true
        },
        ModelVariables::INTEGERVARIABLE { causality: mut causality, .. } if (tipe.clone() == literal!("integer") && causality.clone() == variableCausality.clone()) => {
            true
        },
        ModelVariables::BOOLEANVARIABLE { causality: mut causality, .. } if (tipe.clone() == literal!("boolean") && causality.clone() == variableCausality.clone()) => {
            true
        },
        ModelVariables::STRINGVARIABLE { causality: mut causality, .. } if (tipe.clone() == literal!("string") && causality.clone() == variableCausality.clone()) => {
            true
        },
        _ => {
            false
        },
    });
    result
}

