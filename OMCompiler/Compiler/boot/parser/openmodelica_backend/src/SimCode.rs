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

use crate::AvlTreeCRToInt;
use crate::BackendDAE;
use crate::HashTableCrefSimVar;
use crate::HpcOmSimCode;
use crate::SimCodeFunction;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::HashTableCrILst;
use openmodelica_frontend_types::DAE;

// public imports
pub type ExtConstructor = (Arc<DAE::ComponentRef>, ArcStr, Arc<metamodelica::List<Arc<DAE::Exp>>>);

pub type ExtDestructor = (ArcStr, Arc<DAE::ComponentRef>);

pub type ExtAlias = (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>);

pub type SparsityPattern = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

pub type NonlinearPattern = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

// same structure but different name for the sake of maintenance
#[derive(Clone, Debug, PartialEq)]
pub struct JacobianColumn {
    pub columnEqns: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub columnVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub numberOfResultVars: i32,
    pub constantEqns: Arc<metamodelica::List<Arc<SimEqSystem>>>,
}

impl Default for JacobianColumn {
    fn default() -> Self {
        Self {
            columnEqns: Default::default(),
            columnVars: Default::default(),
            numberOfResultVars: Default::default(),
            constantEqns: Default::default(),
        }
    }
}

pub type JAC_COLUMN = JacobianColumn;


#[derive(Clone, Debug, PartialEq)]
pub struct JacobianMatrix {
    pub columns: Arc<metamodelica::List<Arc<JacobianColumn>>>,
    pub seedVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub matrixName: ArcStr,
    pub sparsity: SparsityPattern,
    pub sparsityT: SparsityPattern,
    pub nonlinear: NonlinearPattern,
    pub nonlinearT: NonlinearPattern,
    pub coloredCols: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub coloredRows: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub maxColorCols: i32,
    pub jacobianIndex: i32,
    pub partitionIndex: i32,
    pub generic_loop_calls: Arc<metamodelica::List<SimGenericCall>>,
    pub crefsHT: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))>,
    pub isAdjoint: bool,
}

impl Default for JacobianMatrix {
    fn default() -> Self {
        Self {
            columns: Default::default(),
            seedVars: Default::default(),
            matrixName: Default::default(),
            sparsity: Default::default(),
            sparsityT: Default::default(),
            nonlinear: Default::default(),
            nonlinearT: Default::default(),
            coloredCols: Default::default(),
            coloredRows: Default::default(),
            maxColorCols: Default::default(),
            jacobianIndex: Default::default(),
            partitionIndex: Default::default(),
            generic_loop_calls: Default::default(),
            crefsHT: Default::default(),
            isAdjoint: Default::default(),
        }
    }
}

pub type JAC_MATRIX = JacobianMatrix;


// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<JacobianMatrix>
// Expr: Constructor { name: 'SimCode.JacobianMatrix.JAC_MATRIX', args: [Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Lit(Str('')), Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Lit(Int(0)), Lit(Int(-1)), Lit(Int(0)), Array { elems: [], ty: List(Unknown) }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Lit(Bool(false))], named_args: [], ty: RustStruct('SimCode.JacobianMatrix'), field_names: ['columns', 'seedVars', 'matrixName', 'sparsity', 'sparsityT', 'nonlinear', 'nonlinearT', 'coloredCols', 'coloredRows', 'maxColorCols', 'jacobianIndex', 'partitionIndex', 'generic_loop_calls', 'crefsHT', 'isAdjoint'] }
pub fn emptyJacobian() -> Arc<JacobianMatrix> { todo!("non-Sync, non-const-emittable constant emptyJacobian — extend codegen") }

pub static emptyPartitionData: std::sync::LazyLock<PartitionData> = std::sync::LazyLock::new(|| { PartitionData { numPartitions: -1, partitions: metamodelica::nil(), activatorsForPartitions: metamodelica::nil(), stateToActivators: metamodelica::nil() } });

/// Root data structure containing information required for templates to
///  generate simulation code for a Modelica model.
#[derive(Clone, Debug, PartialEq)]
pub struct SimCode {
    pub modelInfo: ModelInfo,
    /// shared literals
    pub literals: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    pub recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>,
    pub externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>,
    pub generic_loop_calls: Arc<metamodelica::List<SimGenericCall>>,
    /// state and input dependent variables, that are not inserted into any partion
    pub localKnownVars: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub allEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub odeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimEqSystem>>>>>,
    pub algebraicEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimEqSystem>>>>>,
    pub clockedPartitions: Arc<metamodelica::List<ClockedPartition>>,
    pub initialEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub initialEquations_lambda0: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub removedInitialEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub startValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub nominalValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub minValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub maxValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub parameterEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub removedEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub algorithmAndEquationAsserts: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub equationsForZeroCrossings: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub jacobianEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub stateSets: Arc<metamodelica::List<StateSet>>,
    pub constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>,
    pub classAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>,
    pub zeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>,
    /// only used by c runtime
    pub relations: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>,
    /// only used by c runtime yet
    pub timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>,
    pub discreteModelVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub extObjInfo: ExtObjInfo,
    pub makefileParams: SimCodeFunction::MakefileParams,
    pub delayedExps: DelayedExpression,
    pub spatialInfo: SpatialDistributionInfo,
    pub jacobianMatrices: Arc<metamodelica::List<Arc<JacobianMatrix>>>,
    pub simulationSettingsOpt: Option<SimulationSettings>,
    /// Prefix for all enerated C files. Usually the model name with dots replaced by underscores.
    pub fileNamePrefix: ArcStr,
    /// Used in FMI where files are generated in a special directory
    pub fullPathPrefix: ArcStr,
    /// Name of FMU file <fmuTargetName>.fmu
    pub fmuTargetName: ArcStr,
    pub hpcomData: HpcOmSimCode::HpcOmData,
    /// Used in FMI
    pub valueReferences: Arc<AvlTreeCRToInt::Tree>,
    pub varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)),
    pub varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)),
    /// hidden from typeview - used by cref2simvar() for cref -> SIMVAR lookup available in templates.
    pub crefToSimVarHT: HashTableCrefToSimVar,
    /// map variables to clock indices
    pub crefToClockIndexHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)),
    pub backendMapping: Option<BackendMapping>,
    pub modelStructure: Option<FmiModelStructure>,
    pub fmiSimulationFlags: Option<FmiSimulationFlags>,
    pub partitionData: PartitionData,
    pub daeModeData: Option<DaeModeData>,
    pub inlineEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    /// used for OMSI to generate equations code
    pub omsiData: Option<OMSIData>,
    pub scalarized: bool,
}

pub type SIMCODE = SimCode;


#[derive(Clone, Debug, PartialEq)]
pub struct ClockedPartition {
    pub baseClock: Arc<DAE::ClockKind>,
    pub subPartitions: Arc<metamodelica::List<SubPartition>>,
}

impl Default for ClockedPartition {
    fn default() -> Self {
        Self {
            baseClock: Default::default(),
            subPartitions: Default::default(),
        }
    }
}

pub type CLOCKED_PARTITION = ClockedPartition;


#[derive(Clone, Debug, PartialEq)]
pub struct SubPartition {
    pub vars: Arc<metamodelica::List<(SimCodeVar::SimVar, bool)>>,
    pub equations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub removedEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub subClock: BackendDAE::SubClock,
    pub holdEvents: bool,
}

impl Default for SubPartition {
    fn default() -> Self {
        Self {
            vars: Default::default(),
            equations: Default::default(),
            removedEquations: Default::default(),
            subClock: Default::default(),
            holdEvents: Default::default(),
        }
    }
}

pub type SUBPARTITION = SubPartition;


#[derive(Clone, Debug, PartialEq)]
pub enum BackendMapping {
    BACKENDMAPPING {
        m: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        mT: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        eqMapping: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>,
        varMapping: Arc<metamodelica::List<(i32, i32)>>,
        eqMatch: metamodelica::Array<i32>,
        varMatch: metamodelica::Array<i32>,
        eqTree: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        simVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>,
    },
    NO_MAPPING,
}
impl Default for BackendMapping {
    fn default() -> Self { Self::NO_MAPPING }
}
pub use self::BackendMapping::{BACKENDMAPPING,NO_MAPPING};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionData {
    pub numPartitions: i32,
    pub partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub activatorsForPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub stateToActivators: Arc<metamodelica::List<i32>>,
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            numPartitions: Default::default(),
            partitions: Default::default(),
            activatorsForPartitions: Default::default(),
            stateToActivators: Default::default(),
        }
    }
}

pub type PARTITIONDATA = PartitionData;


/// Delayed expressions type
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DelayedExpression {
    pub delayedExps: Arc<metamodelica::List<(i32, (Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Exp>))>>,
    pub maxDelayedIndex: i32,
}

impl Default for DelayedExpression {
    fn default() -> Self {
        Self {
            delayedExps: Default::default(),
            maxDelayedIndex: Default::default(),
        }
    }
}

pub type DELAYED_EXPRESSIONS = DelayedExpression;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpatialDistributionInfo {
    pub spatialDistributions: Arc<metamodelica::List<SpatialDistribution>>,
    pub maxIndex: i32,
}

impl Default for SpatialDistributionInfo {
    fn default() -> Self {
        Self {
            spatialDistributions: Default::default(),
            maxIndex: Default::default(),
        }
    }
}

pub type SPATIAL_DISTRIBUTION_INFO = SpatialDistributionInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpatialDistribution {
    /// uniqueIndex
    pub index: i32,
    /// input 0
    pub in0: Arc<DAE::Exp>,
    /// input 1
    pub in1: Arc<DAE::Exp>,
    /// current pos
    pub pos: Arc<DAE::Exp>,
    /// flow direction
    pub dir: Arc<DAE::Exp>,
    /// initial grid points
    pub initPnts: Arc<DAE::Exp>,
    /// initial grid values
    pub initVals: Arc<DAE::Exp>,
    /// number of initial points
    pub initSize: i32,
}

pub type SPATIAL_DISTRIBUTION = SpatialDistribution;


/// unitDefinitions for fmi modelDescription.xml
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitDefinition {
    pub name: ArcStr,
    pub baseUnit: BaseUnit,
}

impl Default for UnitDefinition {
    fn default() -> Self {
        Self {
            name: Default::default(),
            baseUnit: Default::default(),
        }
    }
}

pub type UNITDEFINITION = UnitDefinition;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseUnit {
    BASEUNIT {
        /// exponent
        s: i32,
        /// exponent
        m: i32,
        /// exponent
        kg: i32,
        /// exponent
        A: i32,
        /// exponent
        K: i32,
        /// exponent
        mol: i32,
        /// exponent
        cd: i32,
        /// prefix
        factor: metamodelica::Real,
        /// offset
        offset: metamodelica::Real,
    },
    /// no baseunit definition available
    NOBASEUNIT,
}
impl Default for BaseUnit {
    fn default() -> Self { Self::NOBASEUNIT }
}
pub use self::BaseUnit::{BASEUNIT,NOBASEUNIT};

/// Container for metadata about a Modelica model.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelInfo {
    pub name: Arc<Absyn::Path>,
    pub description: ArcStr,
    pub version: ArcStr,
    pub author: ArcStr,
    pub license: ArcStr,
    pub copyright: ArcStr,
    pub directory: ArcStr,
    pub fileName: ArcStr,
    pub varInfo: VarInfo,
    pub vars: SimCodeVar::SimVars,
    pub functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>,
    pub labels: Arc<metamodelica::List<ArcStr>>,
    /// Paths of all resources used by the model. Used in FMI2 to package resources in the FMU.
    pub resourcePaths: Arc<metamodelica::List<ArcStr>>,
    pub sortedClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>,
    pub nClocks: i32,
    pub nSubClocks: i32,
    pub nSpatialDistributions: i32,
    pub hasLargeLinearEquationSystems: bool,
    pub linearSystems: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub nonLinearSystems: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    /// export unitDefintion in modelDescription.xml
    pub unitDefinitions: Arc<metamodelica::List<UnitDefinition>>,
}

pub type MODELINFO = ModelInfo;


pub type Files = Arc<metamodelica::List<FileInfo>>;

/// contains all the .mo files present in all SourceInfo and DAE.ElementSource.info
///   of all the variables, functions, etc from SimCode that have origin info.
///   it is used to generate the file information in one place and use an index
///   whenever we need to refer to one file from a var or function.
///   this is done so that we don't repeat long filenames everywhere.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileInfo {
    /// fileName where the class/component is defined in
    pub fileName: ArcStr,
    /// isReadOnly : (true|false). Should be true for libraries
    pub isReadOnly: bool,
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            isReadOnly: Default::default(),
        }
    }
}

pub type FILEINFO = FileInfo;


/// Number of variables of various types in a Modelica model.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInfo {
    pub numZeroCrossings: i32,
    pub numTimeEvents: i32,
    pub numRelations: i32,
    pub numMathEventFunctions: i32,
    pub numStateVars: i32,
    pub numAlgVars: i32,
    pub numDiscreteReal: i32,
    pub numIntAlgVars: i32,
    pub numBoolAlgVars: i32,
    pub numAlgAliasVars: i32,
    pub numIntAliasVars: i32,
    pub numBoolAliasVars: i32,
    pub numParams: i32,
    pub numIntParams: i32,
    pub numBoolParams: i32,
    pub numOutVars: i32,
    pub numInVars: i32,
    pub numExternalObjects: i32,
    pub numStringAlgVars: i32,
    pub numStringParamVars: i32,
    pub numStringAliasVars: i32,
    pub numEquations: i32,
    pub numLinearSystems: i32,
    pub numNonLinearSystems: i32,
    pub numMixedSystems: i32,
    pub numStateSets: i32,
    pub numJacobians: i32,
    pub numOptimizeConstraints: i32,
    pub numOptimizeFinalConstraints: i32,
    pub numSensitivityParameters: i32,
    pub numSetcVars: i32,
    pub numDataReconVars: i32,
    /// for fmi cs to interpolate inputs
    pub numRealInputVars: i32,
    /// for data reconciliation setB vars
    pub numSetbVars: i32,
    /// for data reconciliation count number of boundary conditions which failed the extraction algorithm
    pub numRelatedBoundaryConditions: i32,
}

impl Default for VarInfo {
    fn default() -> Self {
        Self {
            numZeroCrossings: Default::default(),
            numTimeEvents: Default::default(),
            numRelations: Default::default(),
            numMathEventFunctions: Default::default(),
            numStateVars: Default::default(),
            numAlgVars: Default::default(),
            numDiscreteReal: Default::default(),
            numIntAlgVars: Default::default(),
            numBoolAlgVars: Default::default(),
            numAlgAliasVars: Default::default(),
            numIntAliasVars: Default::default(),
            numBoolAliasVars: Default::default(),
            numParams: Default::default(),
            numIntParams: Default::default(),
            numBoolParams: Default::default(),
            numOutVars: Default::default(),
            numInVars: Default::default(),
            numExternalObjects: Default::default(),
            numStringAlgVars: Default::default(),
            numStringParamVars: Default::default(),
            numStringAliasVars: Default::default(),
            numEquations: Default::default(),
            numLinearSystems: Default::default(),
            numNonLinearSystems: Default::default(),
            numMixedSystems: Default::default(),
            numStateSets: Default::default(),
            numJacobians: Default::default(),
            numOptimizeConstraints: Default::default(),
            numOptimizeFinalConstraints: Default::default(),
            numSensitivityParameters: Default::default(),
            numSetcVars: Default::default(),
            numDataReconVars: Default::default(),
            numRealInputVars: Default::default(),
            numSetbVars: Default::default(),
            numRelatedBoundaryConditions: Default::default(),
        }
    }
}

pub type VARINFO = VarInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DaeModeConfig {
    ALL_EQUATIONS,
    DYNAMIC_EQUATIONS,
}
impl Default for DaeModeConfig {
    fn default() -> Self { Self::ALL_EQUATIONS }
}
pub use self::DaeModeConfig::{ALL_EQUATIONS,DYNAMIC_EQUATIONS};

/// contains data that belongs to the dae mode
#[derive(Clone, Debug, PartialEq)]
pub struct DaeModeData {
    /// daeModel residuals equations
    pub daeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimEqSystem>>>>>,
    /// contains the sparsity pattern for the daeMode
    pub sparsityPattern: Option<Arc<JacobianMatrix>>,
    /// variable used to calculate residuals of a DAE form, they are real
    pub residualVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub algebraicVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub auxiliaryVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub modeCreated: DaeModeConfig,
}

impl Default for DaeModeData {
    fn default() -> Self {
        Self {
            daeEquations: Default::default(),
            sparsityPattern: Default::default(),
            residualVars: Default::default(),
            algebraicVars: Default::default(),
            auxiliaryVars: Default::default(),
            modeCreated: Default::default(),
        }
    }
}

pub type DAEMODEDATA = DaeModeData;


/// contains data for code generation for OMSI
#[derive(Clone, Debug, PartialEq)]
pub struct OMSIData {
    /// contains equations and variables for initialization problem
    pub initialization: Arc<OMSIFunction>,
    /// contains equations and variables for simulation problem
    pub simulation: Arc<OMSIFunction>,
}

impl Default for OMSIData {
    fn default() -> Self {
        Self {
            initialization: Default::default(),
            simulation: Default::default(),
        }
    }
}

pub type OMSI_DATA = OMSIData;


/// contains equations and variables for initialization or simulation problem
#[derive(Clone, Debug, PartialEq)]
pub struct OMSIFunction {
    /// causalized list of single equations and systems of equations
    pub equations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    /// list of simcode variables determining input variables for equation(s)
    pub inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    /// list of simcode variables determining output variables for equation(s)
    pub outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    /// list of simcode variables determining inner variables for equation(s), e.g $DER(x)
    pub innerVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    /// number of input, inner and output vars
    pub nAllVars: i32,
    /// contains crefToSimVar hash table for lookup function in templates
    pub context: SimCodeFunction::Context,
    /// number of linear and non-linear algebraic systems in OMSI_FUNCTION.equations
    pub nAlgebraicSystems: i32,
}

impl Default for OMSIFunction {
    fn default() -> Self {
        Self {
            equations: Default::default(),
            inputVars: Default::default(),
            outputVars: Default::default(),
            innerVars: Default::default(),
            nAllVars: Default::default(),
            context: Default::default(),
            nAlgebraicSystems: Default::default(),
        }
    }
}

pub type OMSI_FUNCTION = OMSIFunction;


// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<OMSIFunction>
// Expr: Constructor { name: 'SimCode.OMSIFunction.OMSI_FUNCTION', args: [], named_args: [('nAlgebraicSystems', Lit(Int(0))), ('context', Var { name: 'SimCodeFunction.contextOMSI', segments: [CrefSegment { name: 'SimCodeFunction', subscripts: [] }, CrefSegment { name: 'contextOMSI', subscripts: [] }], ty: RustEnum('SimCodeFunction.Context') }), ('nAllVars', Lit(Int(0))), ('innerVars', Array { elems: [], ty: List(Unknown) }), ('outputVars', Array { elems: [], ty: List(Unknown) }), ('inputVars', Array { elems: [], ty: List(Unknown) }), ('equations', Array { elems: [], ty: List(Unknown) })], ty: RustStruct('SimCode.OMSIFunction'), field_names: ['equations', 'inputVars', 'outputVars', 'innerVars', 'nAllVars', 'context', 'nAlgebraicSystems'] }
pub fn emptyOMSIFunction() -> Arc<OMSIFunction> { todo!("non-Sync, non-const-emittable constant emptyOMSIFunction — extend codegen") }

/// Represents a single equation or a system of equations that must be solved together.
#[derive(Clone, Debug, PartialEq)]
pub enum SimEqSystem {
    SES_RESIDUAL {
        index: i32,
        res_index: i32,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_RESIDUAL {
        index: i32,
        res_index: i32,
        iterators: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a generic residual calling a for loop body function with an index list.
    SES_GENERIC_RESIDUAL {
        index: i32,
        res_index: i32,
        scal_indices: Arc<metamodelica::List<i32>>,
        iterators: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_SIMPLE_ASSIGN {
        index: i32,
        /// left hand side of equation
        cref: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// Solved inner equation of (casual) tearing set (Dynamic Tearing) with constraints on the solvability
    SES_SIMPLE_ASSIGN_CONSTRAINTS {
        index: i32,
        cref: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        cons: Arc<metamodelica::List<Arc<DAE::Constraint>>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ARRAY_CALL_ASSIGN {
        index: i32,
        lhs: Arc<DAE::Exp>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a resizable assignment calling a for loop body function.
    SES_RESIZABLE_ASSIGN {
        index: i32,
        call_index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a generic assignment calling a for loop body function with an index list.
    SES_GENERIC_ASSIGN {
        index: i32,
        call_index: i32,
        scal_indices: Arc<metamodelica::List<i32>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// entwined generic assignments calling for loop body functions with an index list and a call order.
    SES_ENTWINED_ASSIGN {
        index: i32,
        call_order: Arc<metamodelica::List<i32>>,
        single_calls: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_IFEQUATION {
        index: i32,
        ifbranches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<SimEqSystem>>>)>>,
        elsebranch: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ALGORITHM {
        index: i32,
        statements: Arc<metamodelica::List<Arc<DAE::Statement>>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_INVERSE_ALGORITHM {
        index: i32,
        statements: Arc<metamodelica::List<Arc<DAE::Statement>>>,
        /// this is a subset of output crefs of the original algorithm, which are already known
        knownOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
        insideNonLinearSystem: bool,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_LINEAR {
        lSystem: Arc<LinearSystem>,
        alternativeTearing: Option<Arc<LinearSystem>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_NONLINEAR {
        nlSystem: Arc<NonlinearSystem>,
        alternativeTearing: Option<Arc<NonlinearSystem>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_MIXED {
        index: i32,
        cont: Arc<SimEqSystem>,
        discVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
        discEqs: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        indexMixedSystem: i32,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_WHEN {
        index: i32,
        /// list of boolean variables as conditions
        conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
        /// true, if top-level branch with initial()
        initialCall: bool,
        whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>,
        elseWhen: Option<Arc<SimEqSystem>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_LOOP {
        index: i32,
        iter: Arc<DAE::Exp>,
        startIt: Arc<DAE::Exp>,
        endIt: Arc<DAE::Exp>,
        cref: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_EQUATION {
        index: i32,
        iter: Arc<DAE::Exp>,
        startIt: Arc<DAE::Exp>,
        endIt: Arc<DAE::Exp>,
        body: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ALIAS {
        index: i32,
        aliasOf: i32,
    },
    SES_ALGEBRAIC_SYSTEM {
        /// equation index
        index: i32,
        /// index of algebraic system
        algSysIndex: i32,
        /// dimension of algebraic loop (after tearing)
        dim_n: i32,
        partOfMixed: bool,
        tornSystem: bool,
        linearSystem: bool,
        residual: Arc<OMSIFunction>,
        matrix: Option<Arc<DerivativeMatrix>>,
        zeroCrossingConditions: Arc<metamodelica::List<i32>>,
        sources: Arc<metamodelica::List<Arc<DAE::ElementSource>>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
}
impl Default for SimEqSystem {
    fn default() -> Self {
        Self::SES_ALIAS {
            index: Default::default(),
            aliasOf: Default::default(),
        }
    }
}
pub use self::SimEqSystem::{SES_RESIDUAL,SES_FOR_RESIDUAL,SES_GENERIC_RESIDUAL,SES_SIMPLE_ASSIGN,SES_SIMPLE_ASSIGN_CONSTRAINTS,SES_ARRAY_CALL_ASSIGN,SES_RESIZABLE_ASSIGN,SES_GENERIC_ASSIGN,SES_ENTWINED_ASSIGN,SES_IFEQUATION,SES_ALGORITHM,SES_INVERSE_ALGORITHM,SES_LINEAR,SES_NONLINEAR,SES_MIXED,SES_WHEN,SES_FOR_LOOP,SES_FOR_EQUATION,SES_ALIAS,SES_ALGEBRAIC_SYSTEM};

#[derive(Clone, Debug, PartialEq)]
pub enum SimGenericCall {
    SINGLE_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        lhs: Arc<DAE::Exp>,
        rhs: Arc<DAE::Exp>,
        resizable: bool,
    },
    IF_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        branches: Arc<metamodelica::List<SimBranch>>,
        resizable: bool,
    },
    WHEN_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        branches: Arc<metamodelica::List<SimBranch>>,
        resizable: bool,
    },
}
impl Default for SimGenericCall {
    fn default() -> Self {
        Self::IF_GENERIC_CALL {
            index: Default::default(),
            iters: Default::default(),
            branches: Default::default(),
            resizable: Default::default(),
        }
    }
}
pub use self::SimGenericCall::{SINGLE_GENERIC_CALL,IF_GENERIC_CALL,WHEN_GENERIC_CALL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimBranch {
    SIM_BRANCH {
        condition: Option<Arc<DAE::Exp>>,
        body: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>,
    },
    SIM_BRANCH_STMT {
        condition: Option<Arc<DAE::Exp>>,
        body: Arc<metamodelica::List<Arc<DAE::Statement>>>,
    },
}
impl Default for SimBranch {
    fn default() -> Self {
        Self::SIM_BRANCH {
            condition: Default::default(),
            body: Default::default(),
        }
    }
}
pub use self::SimBranch::{SIM_BRANCH,SIM_BRANCH_STMT};

/// represents directional derivatives with sparsity and coloring
#[derive(Clone, Debug, PartialEq)]
pub struct DerivativeMatrix {
    pub columns: Arc<metamodelica::List<Arc<OMSIFunction>>>,
    /// unique matrix name
    pub matrixName: ArcStr,
    pub sparsity: SparsityPattern,
    pub sparsityT: SparsityPattern,
    pub coloredCols: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub maxColorCols: i32,
}

impl Default for DerivativeMatrix {
    fn default() -> Self {
        Self {
            columns: Default::default(),
            matrixName: Default::default(),
            sparsity: Default::default(),
            sparsityT: Default::default(),
            coloredCols: Default::default(),
            maxColorCols: Default::default(),
        }
    }
}

pub type DERIVATIVE_MATRIX = DerivativeMatrix;


#[derive(Clone, Debug, PartialEq)]
pub struct LinearSystem {
    pub index: i32,
    pub partOfMixed: bool,
    pub tornSystem: bool,
    pub vars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub beqs: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    pub simJac: Arc<metamodelica::List<(i32, i32, Arc<SimEqSystem>)>>,
    pub residual: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub jacobianMatrix: Option<Arc<JacobianMatrix>>,
    pub sources: Arc<metamodelica::List<Arc<DAE::ElementSource>>>,
    pub indexLinearSystem: i32,
    /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
    pub nUnknowns: i32,
    /// if TRUE then this system is part of a jacobian matrix
    pub partOfJac: bool,
}

impl Default for LinearSystem {
    fn default() -> Self {
        Self {
            index: Default::default(),
            partOfMixed: Default::default(),
            tornSystem: Default::default(),
            vars: Default::default(),
            beqs: Default::default(),
            simJac: Default::default(),
            residual: Default::default(),
            jacobianMatrix: Default::default(),
            sources: Default::default(),
            indexLinearSystem: Default::default(),
            nUnknowns: Default::default(),
            partOfJac: Default::default(),
        }
    }
}

pub type LINEARSYSTEM = LinearSystem;


#[derive(Clone, Debug, PartialEq)]
pub struct NonlinearSystem {
    pub index: i32,
    pub eqs: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub indexNonLinearSystem: i32,
    /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
    pub nUnknowns: i32,
    pub jacobianMatrix: Option<Arc<JacobianMatrix>>,
    pub homotopySupport: bool,
    pub mixedSystem: bool,
    pub tornSystem: bool,
    pub clockIndex: Option<i32>,
}

impl Default for NonlinearSystem {
    fn default() -> Self {
        Self {
            index: Default::default(),
            eqs: Default::default(),
            crefs: Default::default(),
            indexNonLinearSystem: Default::default(),
            nUnknowns: Default::default(),
            jacobianMatrix: Default::default(),
            homotopySupport: Default::default(),
            mixedSystem: Default::default(),
            tornSystem: Default::default(),
            clockIndex: Default::default(),
        }
    }
}

pub type NONLINEARSYSTEM = NonlinearSystem;


#[derive(Clone, Debug, PartialEq)]
pub struct StateSet {
    pub index: i32,
    pub nCandidates: i32,
    pub nStates: i32,
    pub states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub statescandidates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub crA: Arc<DAE::ComponentRef>,
    pub jacobianMatrix: Arc<JacobianMatrix>,
}

pub type SES_STATESET = StateSet;


#[derive(Clone, Debug, PartialEq)]
pub struct ExtObjInfo {
    pub vars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub aliases: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>,
}

impl Default for ExtObjInfo {
    fn default() -> Self {
        Self {
            vars: Default::default(),
            aliases: Default::default(),
        }
    }
}

pub type EXTOBJINFO = ExtObjInfo;


/// Settings for simulation init file header.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimulationSettings {
    pub startTime: metamodelica::Real,
    pub stopTime: metamodelica::Real,
    pub numberOfIntervals: i32,
    pub stepSize: metamodelica::Real,
    pub tolerance: metamodelica::Real,
    pub method: ArcStr,
    pub options: ArcStr,
    pub outputFormat: ArcStr,
    pub variableFilter: ArcStr,
    pub cflags: ArcStr,
    pub simflags: ArcStr,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            startTime: Default::default(),
            stopTime: Default::default(),
            numberOfIntervals: Default::default(),
            stepSize: Default::default(),
            tolerance: Default::default(),
            method: Default::default(),
            options: Default::default(),
            outputFormat: Default::default(),
            variableFilter: Default::default(),
            cflags: Default::default(),
            simflags: Default::default(),
        }
    }
}

pub type SIMULATION_SETTINGS = SimulationSettings;


/* ***** HashTable ComponentRef -> SimCodeVar.SimVar ******/
pub type Key = Arc<DAE::ComponentRef>;

pub type Value = SimCodeVar::SimVar;

pub type HashTableCrefToSimVar = (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));

/* FMI 2.0 Export */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiUnknown {
    pub index: i32,
    pub dependencies: Arc<metamodelica::List<i32>>,
    pub dependenciesKind: Arc<metamodelica::List<ArcStr>>,
}

impl Default for FmiUnknown {
    fn default() -> Self {
        Self {
            index: Default::default(),
            dependencies: Default::default(),
            dependenciesKind: Default::default(),
        }
    }
}

pub type FMIUNKNOWN = FmiUnknown;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiOutputs {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
}

impl Default for FmiOutputs {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
        }
    }
}

pub type FMIOUTPUTS = FmiOutputs;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiDerivatives {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
}

impl Default for FmiDerivatives {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
        }
    }
}

pub type FMIDERIVATIVES = FmiDerivatives;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiDiscreteStates {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
}

impl Default for FmiDiscreteStates {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
        }
    }
}

pub type FMIDISCRETESTATES = FmiDiscreteStates;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiInitialUnknowns {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
    /// use the sorted crefs to get the ValueReference of unknowns
    pub sortedUnknownCrefs: Arc<metamodelica::List<(i32, Arc<DAE::ComponentRef>)>>,
    /// use the sorted crefs to get the ValueReference of knowns
    pub sortedknownCrefs: Arc<metamodelica::List<(i32, Arc<DAE::ComponentRef>)>>,
}

impl Default for FmiInitialUnknowns {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
            sortedUnknownCrefs: Default::default(),
            sortedknownCrefs: Default::default(),
        }
    }
}

pub type FMIINITIALUNKNOWNS = FmiInitialUnknowns;


#[derive(Clone, Debug, PartialEq)]
pub struct FmiModelStructure {
    pub fmiOutputs: FmiOutputs,
    pub fmiDerivatives: FmiDerivatives,
    pub continuousPartialDerivatives: Option<Arc<JacobianMatrix>>,
    pub initialPartialDerivatives: Option<Arc<JacobianMatrix>>,
    pub fmiDiscreteStates: FmiDiscreteStates,
    pub fmiInitialUnknowns: FmiInitialUnknowns,
}

impl Default for FmiModelStructure {
    fn default() -> Self {
        Self {
            fmiOutputs: Default::default(),
            fmiDerivatives: Default::default(),
            continuousPartialDerivatives: Default::default(),
            initialPartialDerivatives: Default::default(),
            fmiDiscreteStates: Default::default(),
            fmiInitialUnknowns: Default::default(),
        }
    }
}

pub type FMIMODELSTRUCTURE = FmiModelStructure;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FmiSimulationFlags {
    FMI_SIMULATION_FLAGS {
        nameValueTuples: Arc<metamodelica::List<(ArcStr, ArcStr)>>,
    },
    FMI_SIMULATION_FLAGS_FILE {
        path: ArcStr,
    },
}
impl Default for FmiSimulationFlags {
    fn default() -> Self {
        Self::FMI_SIMULATION_FLAGS {
            nameValueTuples: Default::default(),
        }
    }
}
pub use self::FmiSimulationFlags::{FMI_SIMULATION_FLAGS,FMI_SIMULATION_FLAGS_FILE};

pub static defaultFmiSimulationFlags: std::sync::LazyLock<FmiSimulationFlags> = std::sync::LazyLock::new(|| { FmiSimulationFlags::FMI_SIMULATION_FLAGS { nameValueTuples: list![(literal!("s"), literal!("euler"))] } });

