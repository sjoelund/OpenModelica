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
use openmodelica_frontend_types::DAE;

// public imports
/// Container for metadata about variables in a Modelica model.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimVars {
    pub stateVars: Arc<metamodelica::List<SimVar>>,
    pub derivativeVars: Arc<metamodelica::List<SimVar>>,
    pub algVars: Arc<metamodelica::List<SimVar>>,
    pub discreteAlgVars: Arc<metamodelica::List<SimVar>>,
    pub intAlgVars: Arc<metamodelica::List<SimVar>>,
    pub boolAlgVars: Arc<metamodelica::List<SimVar>>,
    pub inputVars: Arc<metamodelica::List<SimVar>>,
    pub outputVars: Arc<metamodelica::List<SimVar>>,
    pub aliasVars: Arc<metamodelica::List<SimVar>>,
    pub intAliasVars: Arc<metamodelica::List<SimVar>>,
    pub boolAliasVars: Arc<metamodelica::List<SimVar>>,
    pub paramVars: Arc<metamodelica::List<SimVar>>,
    pub intParamVars: Arc<metamodelica::List<SimVar>>,
    pub boolParamVars: Arc<metamodelica::List<SimVar>>,
    pub stringAlgVars: Arc<metamodelica::List<SimVar>>,
    pub stringParamVars: Arc<metamodelica::List<SimVar>>,
    pub stringAliasVars: Arc<metamodelica::List<SimVar>>,
    pub extObjVars: Arc<metamodelica::List<SimVar>>,
    pub constVars: Arc<metamodelica::List<SimVar>>,
    pub intConstVars: Arc<metamodelica::List<SimVar>>,
    pub boolConstVars: Arc<metamodelica::List<SimVar>>,
    pub stringConstVars: Arc<metamodelica::List<SimVar>>,
    pub jacobianVars: Arc<metamodelica::List<SimVar>>,
    pub seedVars: Arc<metamodelica::List<SimVar>>,
    pub realOptimizeConstraintsVars: Arc<metamodelica::List<SimVar>>,
    pub realOptimizeFinalConstraintsVars: Arc<metamodelica::List<SimVar>>,
    /// variable used to calculate sensitivities for parameters nSensitivitityParameters + nRealParam*nStates
    pub sensitivityVars: Arc<metamodelica::List<SimVar>>,
    pub dataReconSetcVars: Arc<metamodelica::List<SimVar>>,
    pub dataReconinputVars: Arc<metamodelica::List<SimVar>>,
    pub dataReconSetBVars: Arc<metamodelica::List<SimVar>>,
}

impl Default for SimVars {
    fn default() -> Self {
        Self {
            stateVars: Default::default(),
            derivativeVars: Default::default(),
            algVars: Default::default(),
            discreteAlgVars: Default::default(),
            intAlgVars: Default::default(),
            boolAlgVars: Default::default(),
            inputVars: Default::default(),
            outputVars: Default::default(),
            aliasVars: Default::default(),
            intAliasVars: Default::default(),
            boolAliasVars: Default::default(),
            paramVars: Default::default(),
            intParamVars: Default::default(),
            boolParamVars: Default::default(),
            stringAlgVars: Default::default(),
            stringParamVars: Default::default(),
            stringAliasVars: Default::default(),
            extObjVars: Default::default(),
            constVars: Default::default(),
            intConstVars: Default::default(),
            boolConstVars: Default::default(),
            stringConstVars: Default::default(),
            jacobianVars: Default::default(),
            seedVars: Default::default(),
            realOptimizeConstraintsVars: Default::default(),
            realOptimizeFinalConstraintsVars: Default::default(),
            sensitivityVars: Default::default(),
            dataReconSetcVars: Default::default(),
            dataReconinputVars: Default::default(),
            dataReconSetBVars: Default::default(),
        }
    }
}

pub type SIMVARS = SimVars;


thread_local! { static __emptySimVars_TLS: SimVars = SimVars { stateVars: metamodelica::nil(), derivativeVars: metamodelica::nil(), algVars: metamodelica::nil(), discreteAlgVars: metamodelica::nil(), intAlgVars: metamodelica::nil(), boolAlgVars: metamodelica::nil(), inputVars: metamodelica::nil(), outputVars: metamodelica::nil(), aliasVars: metamodelica::nil(), intAliasVars: metamodelica::nil(), boolAliasVars: metamodelica::nil(), paramVars: metamodelica::nil(), intParamVars: metamodelica::nil(), boolParamVars: metamodelica::nil(), stringAlgVars: metamodelica::nil(), stringParamVars: metamodelica::nil(), stringAliasVars: metamodelica::nil(), extObjVars: metamodelica::nil(), constVars: metamodelica::nil(), intConstVars: metamodelica::nil(), boolConstVars: metamodelica::nil(), stringConstVars: metamodelica::nil(), jacobianVars: metamodelica::nil(), seedVars: metamodelica::nil(), realOptimizeConstraintsVars: metamodelica::nil(), realOptimizeFinalConstraintsVars: metamodelica::nil(), sensitivityVars: metamodelica::nil(), dataReconSetcVars: metamodelica::nil() }; }
pub fn emptySimVars() -> SimVars { __emptySimVars_TLS.with(|__t| __t.clone()) }

/// Information about a variable in a Modelica model.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimVar {
    pub name: Arc<DAE::ComponentRef>,
    pub varKind: BackendDAE::VarKind,
    pub comment: ArcStr,
    pub unit: ArcStr,
    pub displayUnit: ArcStr,
    pub index: i32,
    pub minValue: Option<Arc<DAE::Exp>>,
    pub maxValue: Option<Arc<DAE::Exp>>,
    pub initialValue: Option<Arc<DAE::Exp>>,
    pub nominalValue: Option<Arc<DAE::Exp>>,
    pub isFixed: bool,
    pub type_: Arc<DAE::Type>,
    pub isDiscrete: bool,
    /// the name of the array if this variable is the first in that array
    pub arrayCref: Option<Arc<DAE::ComponentRef>>,
    pub aliasvar: AliasVariable,
    pub source: Arc<DAE::ElementSource>,
    pub causality: Option<Causality>,
    /// valueReference
    pub variable_index: Option<i32>,
    /// index of variable in modelDescription.xml
    pub fmi_index: Option<i32>,
    pub numArrayElement: Arc<metamodelica::List<ArcStr>>,
    pub isValueChangeable: bool,
    pub isProtected: bool,
    pub hideResult: Option<bool>,
    pub isEncrypted: bool,
    pub inputIndex: Option<metamodelica::Array<i32>>,
    /// true if the variable is a nonlinear jacobian var
    pub initNonlinear: bool,
    /// if the varibale is a jacobian var, this is the corresponding matrix
    pub matrixName: Option<ArcStr>,
    /// FMI-2.0 variabilty attribute
    pub variability: Option<Variability>,
    /// FMI-2.0 initial attribute
    pub initial_: Option<Initial>,
    /// variables will only be exported to the modelDescription.xml if this attribute is SOME(cref) and this cref is only used in ModelDescription.xml for FMI-2.0 export
    pub exportVar: Option<Arc<DAE::ComponentRef>>,
    /// annotation(absoluteValue=false) If false, then the variable defines a relativeQuantity=true else relativeQuantity=false
    pub relativeQuantity: bool,
}

impl Default for SimVar {
    fn default() -> Self {
        Self {
            name: Default::default(),
            varKind: Default::default(),
            comment: Default::default(),
            unit: Default::default(),
            displayUnit: Default::default(),
            index: Default::default(),
            minValue: Default::default(),
            maxValue: Default::default(),
            initialValue: Default::default(),
            nominalValue: Default::default(),
            isFixed: Default::default(),
            type_: Default::default(),
            isDiscrete: Default::default(),
            arrayCref: Default::default(),
            aliasvar: Default::default(),
            source: Default::default(),
            causality: Default::default(),
            variable_index: Default::default(),
            fmi_index: Default::default(),
            numArrayElement: Default::default(),
            isValueChangeable: Default::default(),
            isProtected: Default::default(),
            hideResult: Default::default(),
            isEncrypted: Default::default(),
            inputIndex: Default::default(),
            initNonlinear: Default::default(),
            matrixName: Default::default(),
            variability: Default::default(),
            initial_: Default::default(),
            exportVar: Default::default(),
            relativeQuantity: Default::default(),
        }
    }
}

pub type SIMVAR = SimVar;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AliasVariable {
    NOALIAS,
    ALIAS {
        varName: Arc<DAE::ComponentRef>,
    },
    NEGATEDALIAS {
        varName: Arc<DAE::ComponentRef>,
    },
}
impl Default for AliasVariable {
    fn default() -> Self { Self::NOALIAS }
}
pub use self::AliasVariable::{NOALIAS,ALIAS,NEGATEDALIAS};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Causality {
    /// needed for FMI-1.0
    NONECAUS,
    OUTPUT,
    INPUT,
    LOCAL,
    PARAMETER,
    CALCULATED_PARAMETER,
}
impl Default for Causality {
    fn default() -> Self { Self::NONECAUS }
}
pub use self::Causality::{NONECAUS,OUTPUT,INPUT,LOCAL,PARAMETER,CALCULATED_PARAMETER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Initial {
    NONE_INITIAL,
    EXACT,
    APPROX,
    CALCULATED,
}
impl Default for Initial {
    fn default() -> Self { Self::NONE_INITIAL }
}
pub use self::Initial::{NONE_INITIAL,EXACT,APPROX,CALCULATED};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variability {
    CONSTANT,
    FIXED,
    TUNABLE,
    DISCRETE,
    CONTINUOUS,
}
impl Default for Variability {
    fn default() -> Self { Self::CONSTANT }
}
pub use self::Variability::{CONSTANT,FIXED,TUNABLE,DISCRETE,CONTINUOUS};

