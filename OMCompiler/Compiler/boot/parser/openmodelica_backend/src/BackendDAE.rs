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

use crate::SimCode;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashTable3;
use openmodelica_frontend::HashTableCG;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::ExpandableArray;
use openmodelica_util::MMath;
use openmodelica_util_datatypes_basic::DoubleEnded;

/// Once we are in BackendDAE, the Type can be only basic types or enumeration.
/// We cannot do this in DAE because functions may contain many more types.
/// adrpo: yes we can, we just simplify the DAE.Type, see Types.simplifyType
pub type Type = Arc<DAE::Type>;

/// THE LOWERED DAE consist of variables and equations. The variables are split into
///  two lists, one for unknown variables states and algebraic and one for known variables
///  constants and parameters.
///  The equations are also split into two lists, one with simple equations, a=b, a-b=0, etc., that
///  are removed from  the set of equations to speed up calculations.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackendDAE {
    pub eqs: EqSystems,
    pub shared: Arc<Shared>,
}

impl Default for BackendDAE {
    fn default() -> Self {
        Self {
            eqs: Default::default(),
            shared: Default::default(),
        }
    }
}

pub type DAE = BackendDAE;


pub type EqSystems = Arc<metamodelica::List<Arc<EqSystem>>>;

/// An independent system of equations (and their corresponding variables)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EqSystem {
    /// ordered Variables, only states and alg. vars
    pub orderedVars: Variables,
    /// ordered Equations
    pub orderedEqs: EquationArray,
    pub m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>,
    pub mT: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>,
    /// current type of adjacency matrix, boolean is true if scalar
    pub mapping: Option<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, IndexType, bool, bool)>,
    pub matching: Arc<Matching>,
    /// the state sets of the system
    pub stateSets: StateSets,
    pub partitionKind: BaseClockPartitionKind,
    /// these are equations that cannot solve for a variable.
    ///                                             e.g. assertions, external function calls, algorithm sections without effect
    pub removedEqs: EquationArray,
}

impl Default for EqSystem {
    fn default() -> Self {
        Self {
            orderedVars: Default::default(),
            orderedEqs: Default::default(),
            m: Default::default(),
            mT: Default::default(),
            mapping: Default::default(),
            matching: Default::default(),
            stateSets: Default::default(),
            partitionKind: Default::default(),
            removedEqs: Default::default(),
        }
    }
}

pub type EQSYSTEM = EqSystem;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubClock {
    SUBCLOCK {
        factor: MMath::Rational,
        shift: MMath::Rational,
        solver: Option<ArcStr>,
    },
    INFERED_SUBCLOCK,
}
impl Default for SubClock {
    fn default() -> Self { Self::INFERED_SUBCLOCK }
}
pub use self::SubClock::{SUBCLOCK,INFERED_SUBCLOCK};

pub static DEFAULT_SUBCLOCK: std::sync::LazyLock<SubClock> = std::sync::LazyLock::new(|| { SubClock::SUBCLOCK { factor: MMath::RAT1.clone(), shift: MMath::RAT0.clone(), solver: None } });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseClockPartitionKind {
    UNKNOWN_PARTITION,
    CLOCKED_PARTITION {
        subPartIdx: i32,
    },
    CONTINUOUS_TIME_PARTITION,
    /// treated as CONTINUOUS_TIME_PARTITION
    UNSPECIFIED_PARTITION,
}
impl Default for BaseClockPartitionKind {
    fn default() -> Self { Self::UNKNOWN_PARTITION }
}
pub use self::BaseClockPartitionKind::{UNKNOWN_PARTITION,CLOCKED_PARTITION,CONTINUOUS_TIME_PARTITION,UNSPECIFIED_PARTITION};

/// Data shared for all equation-systems
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shared {
    /// variables only depending on parameters and constants [TODO: move stuff (like inputs) to localKnownVars]
    pub globalKnownVars: Variables,
    /// variables only depending on locally constant variables in the simulation step, i.e. states, input variables
    pub localKnownVars: Variables,
    /// External object variables
    pub externalObjects: Variables,
    /// Data originating from removed simple equations needed to build
    ///                                             variables' lookup table (in C output).
    ///                                             In that way, double buffering of variables in pre()-buffer, extrapolation
    ///                                             buffer and results caching, etc., is avoided, but in C-code output all the
    ///                                             data about variables' names, comments, units, etc. is preserved as well as
    ///                                             pointer to their values (trajectories).
    pub aliasVars: Variables,
    /// Initial equations
    pub initialEqs: EquationArray,
    /// these are equations that cannot solve for a variable. for example assertions, external function calls, algorithm sections without effect
    pub removedEqs: EquationArray,
    /// constraints (Optimica extension)
    pub constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>,
    /// class attributes (Optimica extension)
    pub classAttrs: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>,
    pub cache: FCore::Cache,
    pub graph: FCore::Graph,
    /// functions for Backend
    pub functionTree: Arc<AvlTreePathFunction::Tree>,
    /// eventInfo
    pub eventInfo: EventInfo,
    /// classes of external objects, contains constructor & destructor
    pub extObjClasses: ExternalObjectClasses,
    /// indicate for what the BackendDAE is used
    pub backendDAEType: BackendDAEType,
    /// Symbolic Jacobians
    pub symjacs: SymbolicJacobians,
    /// contains extra info that we send around like the model name
    pub info: ExtraInfo,
    pub partitionsInfo: PartitionsInfo,
    /// DAEMode Data
    pub daeModeData: BackendDAEModeData,
    pub dataReconciliationData: Option<DataReconciliationData>,
    /// from experiment annotation Interval, used for derivative nominal guesswork
    pub timeInterval: Option<Arc<DAE::Exp>>,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            globalKnownVars: Default::default(),
            localKnownVars: Default::default(),
            externalObjects: Default::default(),
            aliasVars: Default::default(),
            initialEqs: Default::default(),
            removedEqs: Default::default(),
            constraints: Default::default(),
            classAttrs: Default::default(),
            cache: Default::default(),
            graph: Default::default(),
            functionTree: Default::default(),
            eventInfo: Default::default(),
            extObjClasses: Default::default(),
            backendDAEType: Default::default(),
            symjacs: Default::default(),
            info: Default::default(),
            partitionsInfo: Default::default(),
            daeModeData: Default::default(),
            dataReconciliationData: Default::default(),
            timeInterval: Default::default(),
        }
    }
}

pub type SHARED = Shared;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InlineData {
    pub inlineSystems: EqSystems,
    pub knownVariables: Variables,
}

impl Default for InlineData {
    fn default() -> Self {
        Self {
            inlineSystems: Default::default(),
            knownVariables: Default::default(),
        }
    }
}

pub type INLINE_DATA = InlineData;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasePartition {
    pub clock: Arc<DAE::ClockKind>,
    pub nSubClocks: i32,
}

impl Default for BasePartition {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            nSubClocks: Default::default(),
        }
    }
}

pub type BASE_PARTITION = BasePartition;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubPartition {
    pub clock: SubClock,
    pub holdEvents: bool,
    pub prevVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
}

impl Default for SubPartition {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            holdEvents: Default::default(),
            prevVars: Default::default(),
        }
    }
}

pub type SUB_PARTITION = SubPartition;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartitionsInfo {
    pub basePartitions: metamodelica::Array<BasePartition>,
    pub subPartitions: metamodelica::Array<SubPartition>,
}

impl Default for PartitionsInfo {
    fn default() -> Self {
        Self {
            basePartitions: Default::default(),
            subPartitions: Default::default(),
        }
    }
}

pub type PARTITIONS_INFO = PartitionsInfo;


/// extra information that we should send around with the DAE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtraInfo {
    /// the model description string
    pub description: ArcStr,
    /// the model name to be used in the dumps
    pub fileNamePrefix: ArcStr,
    /// simulation settings options needed for data reconciliation to apply start values from csv files
    pub simSettingsOption: Option<SimCode::SimulationSettings>,
}

impl Default for ExtraInfo {
    fn default() -> Self {
        Self {
            description: Default::default(),
            fileNamePrefix: Default::default(),
            simSettingsOption: Default::default(),
        }
    }
}

pub type EXTRA_INFO = ExtraInfo;


/// BackendDAEType to indicate different types of BackendDAEs.
///  For example for simulation, initialization, Jacobian, algebraic loops etc.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackendDAEType {
    /// Type for the normal BackendDAE.DAE for simulation
    SIMULATION,
    /// Type for Jacobian BackendDAE.DAE
    JACOBIAN,
    /// Type for algebraic loop BackendDAE.DAE
    ALGEQSYSTEM,
    /// Type for multi dim equation arrays BackendDAE.DAE
    ARRAYSYSTEM,
    /// Type for parameter system BackendDAE.DAE
    PARAMETERSYSTEM,
    /// Type for initial system BackendDAE.DAE
    INITIALSYSTEM,
    /// Type for inline system BackendDAE.DAE
    INLINESYSTEM,
    /// Type for DAEmode system BackendDAE.DAE
    DAEMODESYSTEM,
}
impl Default for BackendDAEType {
    fn default() -> Self { Self::SIMULATION }
}
pub use self::BackendDAEType::{SIMULATION,JACOBIAN,ALGEQSYSTEM,ARRAYSYSTEM,PARAMETERSYSTEM,INITIALSYSTEM,INLINESYSTEM,DAEMODESYSTEM};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataReconciliationData {
    /// jacobians for set-C and set-S
    pub symbolicJacobian: Arc<Jacobian>,
    /// setc solved vars
    pub setcVars: Variables,
    pub datareconinputs: Variables,
    /// setB solved vars which computes boundary conditions
    pub setBVars: Option<Variables>,
    /// For solving state estimation we need two Jacobians F for data Reconciliation and H for boundary conditions set-B and set-Sprime
    pub symbolicJacobianH: Option<Arc<Jacobian>>,
    /// count number of boundary conditions which failed the extraction algorithm
    pub relatedBoundaryConditions: i32,
}

impl Default for DataReconciliationData {
    fn default() -> Self {
        Self {
            symbolicJacobian: Default::default(),
            setcVars: Default::default(),
            datareconinputs: Default::default(),
            setBVars: Default::default(),
            symbolicJacobianH: Default::default(),
            relatedBoundaryConditions: Default::default(),
        }
    }
}

pub type DATA_RECON = DataReconciliationData;


//
//  variables and equations definition
//
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variables {
    /// HashTB, cref->indx
    pub crefIndices: metamodelica::Array<Arc<metamodelica::List<CrefIndex>>>,
    /// Array of variables
    pub varArr: VariableArray,
    /// bucket size
    pub bucketSize: i32,
    /// no. of vars
    pub numberOfVars: i32,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            crefIndices: Default::default(),
            varArr: Default::default(),
            bucketSize: Default::default(),
            numberOfVars: Default::default(),
        }
    }
}

pub type VARIABLES = Variables;


/// Component Reference Index
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrefIndex {
    pub cref: Arc<DAE::ComponentRef>,
    pub index: i32,
}

impl Default for CrefIndex {
    fn default() -> Self {
        Self {
            cref: Default::default(),
            index: Default::default(),
        }
    }
}

pub type CREFINDEX = CrefIndex;


/// array of Equations are expandable, to amortize the cost of adding
///  equations in a more efficient manner
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableArray {
    /// no. elements
    pub numberOfElements: i32,
    pub varOptArr: metamodelica::Array<Option<Var>>,
}

impl Default for VariableArray {
    fn default() -> Self {
        Self {
            numberOfElements: Default::default(),
            varOptArr: Default::default(),
        }
    }
}

pub type VARIABLE_ARRAY = VariableArray;


pub type EquationArray = Arc<ExpandableArray::ExpandableArray<Arc<Equation>>>;

/// variables
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    /// variable name
    pub varName: Arc<DAE::ComponentRef>,
    /// kind of variable
    pub varKind: VarKind,
    /// input, output or bidirectional
    pub varDirection: DAE::VarDirection,
    /// parallelism of the variable. parglobal, parlocal or non-parallel
    pub varParallelism: DAE::VarParallelism,
    /// built-in type or enumeration
    pub varType: Type,
    /// Binding expression e.g. for parameters
    pub bindExp: Option<Arc<DAE::Exp>>,
    /// Variable is part of a tuple. Needed for the globalKnownVars and localKnownVars
    pub tplExp: Option<Arc<DAE::Exp>>,
    /// array dimensions of non-expanded var
    pub arryDim: Arc<metamodelica::List<Arc<DAE::Dimension>>>,
    /// origin of variable
    pub source: Arc<DAE::ElementSource>,
    /// values on built-in attributes
    pub values: Option<Arc<DAE::VariableAttributes>>,
    /// value for TearingSelect
    pub tearingSelectOption: Option<TearingSelect>,
    /// expression from the hideResult annotation
    pub hideResult: Option<Arc<DAE::Exp>>,
    /// this contains the comment and annotation from Absyn
    pub comment: Option<Arc<SCode::Comment>>,
    /// flow, stream, unspecified or not connector.
    pub connectorType: Arc<DAE::ConnectorType>,
    /// inner, outer, inner outer or unspecified
    pub innerOuter: DAE::VarInnerOuter,
    /// indicates if it is allowed to replace this variable
    pub unreplaceable: bool,
    /// indicates if the variable is a nonlinear iteration variable during initialization
    pub initNonlinear: bool,
    /// true if the variable belongs to an encrypted class
    pub encrypted: bool,
}

impl Default for Var {
    fn default() -> Self {
        Self {
            varName: Default::default(),
            varKind: Default::default(),
            varDirection: Default::default(),
            varParallelism: Default::default(),
            varType: Default::default(),
            bindExp: Default::default(),
            tplExp: Default::default(),
            arryDim: Default::default(),
            source: Default::default(),
            values: Default::default(),
            tearingSelectOption: Default::default(),
            hideResult: Default::default(),
            comment: Default::default(),
            connectorType: Default::default(),
            innerOuter: Default::default(),
            unreplaceable: Default::default(),
            initNonlinear: Default::default(),
            encrypted: Default::default(),
        }
    }
}

pub type VAR = Var;


/// variable kind
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarKind {
    VARIABLE,
    STATE {
        /// how often this states was differentiated
        index: i32,
        /// the name of the derivative
        derName: Option<Arc<DAE::ComponentRef>>,
        /// false if it was forced by StateSelect.always or StateSelect.prefer or generated by index reduction
        natural: bool,
    },
    STATE_DER,
    DUMMY_DER,
    DUMMY_STATE,
    CLOCKED_STATE {
        /// the name of the previous variable
        previousName: Arc<DAE::ComponentRef>,
        /// is fixed at first clock tick
        isStartFixed: bool,
    },
    DISCRETE,
    PARAM,
    CONST,
    EXTOBJ {
        fullClassName: Arc<Absyn::Path>,
    },
    JAC_VAR,
    JAC_TMP_VAR,
    SEED_VAR,
    OPT_CONSTR,
    OPT_FCONSTR,
    OPT_INPUT_WITH_DER,
    OPT_INPUT_DER,
    OPT_TGRID,
    OPT_LOOP_INPUT {
        replaceExp: Arc<DAE::ComponentRef>,
    },
    /// algebraic state used by inline solver
    ALG_STATE,
    /// algebraic state old value used by inline solver
    ALG_STATE_OLD,
    /// variable kind used for DAEmode
    DAE_RESIDUAL_VAR,
    /// auxiliary variable used for DAEmode
    DAE_AUX_VAR,
    /// used in SIMCODE, iteration variables in algebraic loops
    LOOP_ITERATION,
    /// used in SIMCODE, inner variables of a torn algebraic loop
    LOOP_SOLVED,
}
impl Default for VarKind {
    fn default() -> Self { Self::VARIABLE }
}
pub use self::VarKind::{VARIABLE,STATE,STATE_DER,DUMMY_DER,DUMMY_STATE,CLOCKED_STATE,DISCRETE,PARAM,CONST,EXTOBJ,JAC_VAR,JAC_TMP_VAR,SEED_VAR,OPT_CONSTR,OPT_FCONSTR,OPT_INPUT_WITH_DER,OPT_INPUT_DER,OPT_TGRID,OPT_LOOP_INPUT,ALG_STATE,ALG_STATE_OLD,DAE_RESIDUAL_VAR,DAE_AUX_VAR,LOOP_ITERATION,LOOP_SOLVED};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TearingSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}
impl Default for TearingSelect {
    fn default() -> Self { Self::NEVER }
}
pub use self::TearingSelect::{NEVER,AVOID,DEFAULT,PREFER,ALWAYS};

pub const WHENCLK_PRREFIX: &'static str = "$whenclk";

/// equation kind
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquationKind {
    BINDING_EQUATION,
    DYNAMIC_EQUATION,
    INITIAL_EQUATION,
    CLOCKED_EQUATION {
        clk: i32,
    },
    DISCRETE_EQUATION,
    AUX_EQUATION,
    UNKNOWN_EQUATION_KIND,
}
impl Default for EquationKind {
    fn default() -> Self { Self::BINDING_EQUATION }
}
pub use self::EquationKind::{BINDING_EQUATION,DYNAMIC_EQUATION,INITIAL_EQUATION,CLOCKED_EQUATION,DISCRETE_EQUATION,AUX_EQUATION,UNKNOWN_EQUATION_KIND};

/// evaluation stages
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluationStages {
    pub dynamicEval: bool,
    pub algebraicEval: bool,
    pub zerocrossEval: bool,
    pub discreteEval: bool,
}

impl Default for EvaluationStages {
    fn default() -> Self {
        Self {
            dynamicEval: Default::default(),
            algebraicEval: Default::default(),
            zerocrossEval: Default::default(),
            discreteEval: Default::default(),
        }
    }
}

pub type EVALUATION_STAGES = EvaluationStages;


pub static defaultEvalStages: EvaluationStages = EvaluationStages { dynamicEval: false, algebraicEval: false, zerocrossEval: false, discreteEval: false };

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquationAttributes {
    /// true if the equation was differentiated, and should not be differentiated again to avoid equal equations
    pub differentiated: bool,
    pub kind: EquationKind,
    pub evalStages: EvaluationStages,
}

impl Default for EquationAttributes {
    fn default() -> Self {
        Self {
            differentiated: Default::default(),
            kind: Default::default(),
            evalStages: Default::default(),
        }
    }
}

pub type EQUATION_ATTRIBUTES = EquationAttributes;


pub static EQ_ATTR_DEFAULT_DYNAMIC: std::sync::LazyLock<EquationAttributes> = std::sync::LazyLock::new(|| { EquationAttributes { differentiated: false, kind: crate::BackendDAE::EquationKind::DYNAMIC_EQUATION, evalStages: defaultEvalStages.clone() } });

pub static EQ_ATTR_DEFAULT_BINDING: std::sync::LazyLock<EquationAttributes> = std::sync::LazyLock::new(|| { EquationAttributes { differentiated: false, kind: crate::BackendDAE::EquationKind::BINDING_EQUATION, evalStages: defaultEvalStages.clone() } });

pub static EQ_ATTR_DEFAULT_INITIAL: std::sync::LazyLock<EquationAttributes> = std::sync::LazyLock::new(|| { EquationAttributes { differentiated: false, kind: crate::BackendDAE::EquationKind::INITIAL_EQUATION, evalStages: defaultEvalStages.clone() } });

pub static EQ_ATTR_DEFAULT_DISCRETE: std::sync::LazyLock<EquationAttributes> = std::sync::LazyLock::new(|| { EquationAttributes { differentiated: false, kind: crate::BackendDAE::EquationKind::DISCRETE_EQUATION, evalStages: defaultEvalStages.clone() } });

pub static EQ_ATTR_DEFAULT_AUX: std::sync::LazyLock<EquationAttributes> = std::sync::LazyLock::new(|| { EquationAttributes { differentiated: false, kind: crate::BackendDAE::EquationKind::AUX_EQUATION, evalStages: defaultEvalStages.clone() } });

pub static EQ_ATTR_DEFAULT_UNKNOWN: std::sync::LazyLock<EquationAttributes> = std::sync::LazyLock::new(|| { EquationAttributes { differentiated: false, kind: crate::BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND, evalStages: defaultEvalStages.clone() } });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Equation {
    EQUATION {
        exp: Arc<DAE::Exp>,
        scalar: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    ARRAY_EQUATION {
        /// dimension sizes
        dimSize: Arc<metamodelica::List<i32>>,
        /// lhs
        left: Arc<DAE::Exp>,
        /// rhs
        right: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
        /// NONE() if not a record
        recordSize: Option<i32>,
    },
    SOLVED_EQUATION {
        componentRef: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    RESIDUAL_EQUATION {
        /// not present from FrontEnd
        exp: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    ALGORITHM {
        /// size of equation
        size: i32,
        alg: Arc<DAE::Algorithm>,
        /// origin of algorithm
        source: Arc<DAE::ElementSource>,
        /// this algorithm was translated from an equation. we should not expand array crefs!
        expand: DAE::Expand,
        attr: EquationAttributes,
    },
    WHEN_EQUATION {
        /// size of equation
        size: i32,
        whenEquation: Arc<WhenEquation>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    /// complex equations: recordX = function call(x, y, ..);
    COMPLEX_EQUATION {
        /// size of equation
        size: i32,
        /// lhs
        left: Arc<DAE::Exp>,
        /// rhs
        right: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    /// an if-equation
    IF_EQUATION {
        /// Condition
        conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>,
        /// Equations of true branch
        eqnstrue: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Equation>>>>>,
        /// Equations of false branch
        eqnsfalse: Arc<metamodelica::List<Arc<Equation>>>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    /// a for-equation
    FOR_EQUATION {
        /// the iterator variable
        iter: Arc<DAE::Exp>,
        /// start of iteration
        start: Arc<DAE::Exp>,
        /// end of iteration
        stop: Arc<DAE::Exp>,
        /// iterated equation
        body: Arc<Equation>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
        attr: EquationAttributes,
    },
    DUMMY_EQUATION,
}
impl Default for Equation {
    fn default() -> Self { Self::DUMMY_EQUATION }
}
pub use self::Equation::{EQUATION,ARRAY_EQUATION,SOLVED_EQUATION,RESIDUAL_EQUATION,ALGORITHM,WHEN_EQUATION,COMPLEX_EQUATION,IF_EQUATION,FOR_EQUATION,DUMMY_EQUATION};

/// equation when condition then cr = exp, reinit(...), terminate(...) or assert(...)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WhenEquation {
    /// the when-condition
    pub condition: Arc<DAE::Exp>,
    pub whenStmtLst: Arc<metamodelica::List<WhenOperator>>,
    /// elsewhen equation with the same cref on the left hand side.
    pub elsewhenPart: Option<Arc<WhenEquation>>,
}

impl Default for WhenEquation {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            whenStmtLst: Default::default(),
            elsewhenPart: Default::default(),
        }
    }
}

pub type WHEN_STMTS = WhenEquation;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WhenOperator {
    /// left_cr = right_exp
    ASSIGN {
        /// left hand side of equation
        left: Arc<DAE::Exp>,
        /// right hand side of equation
        right: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
    },
    /// Reinit Statement
    REINIT {
        /// State variable to reinit
        stateVar: Arc<DAE::ComponentRef>,
        /// Value after reinit
        value: Arc<DAE::Exp>,
        /// origin of equation
        source: Arc<DAE::ElementSource>,
    },
    ASSERT {
        condition: Arc<DAE::Exp>,
        message: Arc<DAE::Exp>,
        level: Arc<DAE::Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<DAE::ElementSource>,
    },
    /// The Modelica built-in terminate(msg)
    TERMINATE {
        message: Arc<DAE::Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<DAE::ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///    Typically side effect call of external function but also
    ///    Connections.* i.e. Connections.root(...) functions.
    NORETCALL {
        exp: Arc<DAE::Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<DAE::ElementSource>,
    },
}
impl Default for WhenOperator {
    fn default() -> Self {
        Self::TERMINATE {
            message: Default::default(),
            source: Default::default(),
        }
    }
}
pub use self::WhenOperator::{ASSIGN,REINIT,ASSERT,TERMINATE,NORETCALL};

/// classes of external objects stored in list
pub type ExternalObjectClasses = Arc<metamodelica::List<ExternalObjectClass>>;

/// class of external objects
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalObjectClass {
    /// className of external object
    pub path: Arc<Absyn::Path>,
    /// origin of equation
    pub source: Arc<DAE::ElementSource>,
}

impl Default for ExternalObjectClass {
    fn default() -> Self {
        Self {
            path: Default::default(),
            source: Default::default(),
        }
    }
}

pub type EXTOBJCLASS = ExternalObjectClass;


//
//  Matching, strong components and StateSets
//
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Matching {
    /// matching has not yet been performed
    NO_MATCHING,
    /// not yet used
    MATCHING {
        /// ass[varindx]=eqnindx
        ass1: metamodelica::Array<i32>,
        /// ass[eqnindx]=varindx
        ass2: metamodelica::Array<i32>,
        comps: StrongComponents,
    },
}
impl Default for Matching {
    fn default() -> Self { Self::NO_MATCHING }
}
pub use self::Matching::{NO_MATCHING,MATCHING};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndexReduction {
    /// Use index reduction during matching
    INDEX_REDUCTION,
    /// do not use index reduction during matching
    NO_INDEX_REDUCTION,
}
pub use self::IndexReduction::{INDEX_REDUCTION,NO_INDEX_REDUCTION};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquationConstraints {
    /// for e.g. initial eqns.
    ///                  where not all variables
    ///                  have a solution
    ALLOW_UNDERCONSTRAINED,
    /// exact as many equations
    ///                   as variables
    EXACT,
}
pub use self::EquationConstraints::{ALLOW_UNDERCONSTRAINED,EXACT};

pub type MatchingOptions = (IndexReduction, EquationConstraints);

/// StateOrder,ConstraintEqns,Eqn->EqnsIndxes,EqnIndex->Eqns,NrOfEqnsbeforeIndexReduction
pub type StructurallySingularSystemHandlerArg = (StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);

pub type ConstraintEquations = metamodelica::Array<Arc<metamodelica::List<Arc<Equation>>>>;

#[derive(Clone)]
pub enum StateOrder {
    STATEORDER {
        /// x -> dx
        hashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)),
        /// dx -> {x,y,z}
        invHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr)),
    },
    /// Index reduction disabled; don't need big hashtables
    NOSTATEORDER,
}
impl PartialEq for StateOrder {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::STATEORDER { hashTable: __l_hashTable, invHashTable: __l_invHashTable }, Self::STATEORDER { hashTable: __r_hashTable, invHashTable: __r_invHashTable }) => (match (__l_hashTable, __r_hashTable) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (__lt0 == __rt0) && (__lt1 == __rt1) && (__lt2 == __rt2) && (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => std::sync::Arc::ptr_eq(__lt0, __rt0) && std::sync::Arc::ptr_eq(__lt1, __rt1) && std::sync::Arc::ptr_eq(__lt2, __rt2) && std::sync::Arc::ptr_eq(__lt3, __rt3) }) }) && (match (__l_invHashTable, __r_invHashTable) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (__lt0 == __rt0) && (__lt1 == __rt1) && (__lt2 == __rt2) && (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => std::sync::Arc::ptr_eq(__lt0, __rt0) && std::sync::Arc::ptr_eq(__lt1, __rt1) && std::sync::Arc::ptr_eq(__lt2, __rt2) && std::sync::Arc::ptr_eq(__lt3, __rt3) }) }),
            (Self::NOSTATEORDER, Self::NOSTATEORDER) => true,
            _ => false,
        }
    }
}
impl Eq for StateOrder {}
impl PartialOrd for StateOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for StateOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn __variant_idx(__v: &StateOrder) -> u32 {
            match __v {
                StateOrder::STATEORDER { .. } => 0,
                StateOrder::NOSTATEORDER => 1,
            }
        }
        match __variant_idx(self).cmp(&__variant_idx(other)) {
            std::cmp::Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match (self, other) {
            (Self::STATEORDER { hashTable: __l_hashTable, invHashTable: __l_invHashTable }, Self::STATEORDER { hashTable: __r_hashTable, invHashTable: __r_invHashTable }) => (match (__l_hashTable, __r_hashTable) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0.cmp(__rt0).then_with(|| __lt1.cmp(__rt1).then_with(|| __lt2.cmp(__rt2).then_with(|| (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (std::sync::Arc::as_ptr(__lt0) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt1) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt2) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt3) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))))) })))) }).then_with(|| (match (__l_invHashTable, __r_invHashTable) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0.cmp(__rt0).then_with(|| __lt1.cmp(__rt1).then_with(|| __lt2.cmp(__rt2).then_with(|| (match (__lt3, __rt3) { ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (std::sync::Arc::as_ptr(__lt0) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt1) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt2) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ())).then_with(|| (std::sync::Arc::as_ptr(__lt3) as *const ()).cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))))) })))) })),
            (Self::NOSTATEORDER, Self::NOSTATEORDER) => std::cmp::Ordering::Equal,
            _ => unreachable!("variant-index equality already implies same variant"),
        }
    }
}
impl std::fmt::Debug for StateOrder {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STATEORDER { hashTable: __d_hashTable, invHashTable: __d_invHashTable } => {
                let mut __ds = __f.debug_struct("STATEORDER");
                __ds.field("hashTable", &format_args!("<dyn-fn-container@{:p}>", __d_hashTable as *const _));
                __ds.field("invHashTable", &format_args!("<dyn-fn-container@{:p}>", __d_invHashTable as *const _));
                __ds.finish()
            }
            Self::NOSTATEORDER => __f.debug_struct("NOSTATEORDER").finish(),
        }
    }
}

pub use self::StateOrder::{STATEORDER,NOSTATEORDER};

/// Order of the equations the have to be solved
pub type StrongComponents = Arc<metamodelica::List<Arc<StrongComponent>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrongComponent {
    SINGLEEQUATION {
        eqn: i32,
        var: i32,
    },
    EQUATIONSYSTEM {
        eqns: Arc<metamodelica::List<i32>>,
        /// be careful with states, this are solved for der(x)
        vars: Arc<metamodelica::List<i32>>,
        jac: Arc<Jacobian>,
        jacType: JacobianType,
        /// true for system that discrete dependencies to the iteration variables
        mixedSystem: bool,
    },
    SINGLEARRAY {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: Arc<metamodelica::List<i32>>,
    },
    SINGLEALGORITHM {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: Arc<metamodelica::List<i32>>,
    },
    SINGLECOMPLEXEQUATION {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: Arc<metamodelica::List<i32>>,
    },
    SINGLEWHENEQUATION {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: Arc<metamodelica::List<i32>>,
    },
    SINGLEIFEQUATION {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: Arc<metamodelica::List<i32>>,
    },
    TORNSYSTEM {
        strictTearingSet: TearingSet,
        casualTearingSet: Option<TearingSet>,
        linear: bool,
        /// true for system that discrete dependencies to the iteration variables
        mixedSystem: bool,
    },
}
pub use self::StrongComponent::{SINGLEEQUATION,EQUATIONSYSTEM,SINGLEARRAY,SINGLEALGORITHM,SINGLECOMPLEXEQUATION,SINGLEWHENEQUATION,SINGLEIFEQUATION,TORNSYSTEM};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TearingSet {
    pub tearingvars: Arc<metamodelica::List<i32>>,
    pub residualequations: Arc<metamodelica::List<i32>>,
    /// list of matched equations and variables; these will be solved explicitly in the given order
    pub innerEquations: InnerEquations,
    pub jac: Arc<Jacobian>,
}

impl Default for TearingSet {
    fn default() -> Self {
        Self {
            tearingvars: Default::default(),
            residualequations: Default::default(),
            innerEquations: Default::default(),
            jac: Default::default(),
        }
    }
}

pub type TEARINGSET = TearingSet;


pub type InnerEquations = Arc<metamodelica::List<InnerEquation>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InnerEquation {
    INNEREQUATION {
        eqn: i32,
        vars: Arc<metamodelica::List<i32>>,
    },
    INNEREQUATIONCONSTRAINTS {
        eqn: i32,
        vars: Arc<metamodelica::List<i32>>,
        cons: Constraints,
    },
}
impl Default for InnerEquation {
    fn default() -> Self {
        Self::INNEREQUATION {
            eqn: Default::default(),
            vars: Default::default(),
        }
    }
}
pub use self::InnerEquation::{INNEREQUATION,INNEREQUATIONCONSTRAINTS};

/// List of StateSets
pub type StateSets = Arc<metamodelica::List<StateSet>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateSet {
    pub index: i32,
    pub rang: i32,
    pub state: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    /// set.x=A*states
    pub crA: Arc<DAE::ComponentRef>,
    pub varA: Arc<metamodelica::List<Var>>,
    pub statescandidates: Arc<metamodelica::List<Var>>,
    pub ovars: Arc<metamodelica::List<Var>>,
    pub eqns: Arc<metamodelica::List<Arc<Equation>>>,
    pub oeqns: Arc<metamodelica::List<Arc<Equation>>>,
    pub crJ: Arc<DAE::ComponentRef>,
    pub varJ: Arc<metamodelica::List<Var>>,
    pub jacobian: Arc<Jacobian>,
}

impl Default for StateSet {
    fn default() -> Self {
        Self {
            index: Default::default(),
            rang: Default::default(),
            state: Default::default(),
            crA: Default::default(),
            varA: Default::default(),
            statescandidates: Default::default(),
            ovars: Default::default(),
            eqns: Default::default(),
            oeqns: Default::default(),
            crJ: Default::default(),
            varJ: Default::default(),
            jacobian: Default::default(),
        }
    }
}

pub type STATESET = StateSet;


//
// event info and stuff
//
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventInfo {
    /// stores all information related to time events
    pub timeEvents: Arc<metamodelica::List<TimeEvent>>,
    /// list of zero crossing conditions
    pub zeroCrossings: ZeroCrossingSet,
    /// list of zero crossing function as before
    pub relations: DoubleEnded::MutableList<ZeroCrossing>,
    /// [deprecated] list of sample as before, only used by cpp runtime (TODO: REMOVE ME)
    pub samples: ZeroCrossingSet,
    /// stores the number of math function that trigger events e.g. floor, ceil, integer, ...
    pub numberMathEvents: i32,
}

impl Default for EventInfo {
    fn default() -> Self {
        Self {
            timeEvents: Default::default(),
            zeroCrossings: Default::default(),
            relations: Default::default(),
            samples: Default::default(),
            numberMathEvents: Default::default(),
        }
    }
}

pub type EVENT_INFO = EventInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroCrossingSet {
    pub zc: DoubleEnded::MutableList<ZeroCrossing>,
    pub tree: metamodelica::Array<Arc<ZeroCrossings::ZeroCrossingTree::Tree>>,
}

impl Default for ZeroCrossingSet {
    fn default() -> Self {
        Self {
            zc: Default::default(),
            tree: Default::default(),
        }
    }
}

pub type ZERO_CROSSING_SET = ZeroCrossingSet;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroCrossing {
    /// zero crossing index
    pub index: i32,
    /// function
    pub relation_: Arc<DAE::Exp>,
    /// list of equations where the function occurs
    pub occurEquLst: Arc<metamodelica::List<i32>>,
    /// optional iterator for for-loops
    pub iter: Option<Arc<metamodelica::List<SimIterator>>>,
}

impl Default for ZeroCrossing {
    fn default() -> Self {
        Self {
            index: Default::default(),
            relation_: Default::default(),
            occurEquLst: Default::default(),
            iter: Default::default(),
        }
    }
}

pub type ZERO_CROSSING = ZeroCrossing;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimIterator {
    SIM_ITERATOR_RANGE {
        name: Arc<DAE::ComponentRef>,
        start: Arc<DAE::Exp>,
        step: Arc<DAE::Exp>,
        stop: Arc<DAE::Exp>,
        size: Arc<DAE::Exp>,
        non_resizable_size: i32,
        sub_iter: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, metamodelica::Array<Arc<DAE::Exp>>)>>,
    },
    SIM_ITERATOR_LIST {
        name: Arc<DAE::ComponentRef>,
        lst: Arc<metamodelica::List<i32>>,
        size: i32,
        sub_iter: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, metamodelica::Array<Arc<DAE::Exp>>)>>,
    },
}
impl Default for SimIterator {
    fn default() -> Self {
        Self::SIM_ITERATOR_LIST {
            name: Default::default(),
            lst: Default::default(),
            size: Default::default(),
            sub_iter: Default::default(),
        }
    }
}
pub use self::SimIterator::{SIM_ITERATOR_RANGE,SIM_ITERATOR_LIST};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeEvent {
    /// e.g. time > 0.5
    SIMPLE_TIME_EVENT,
    /// e.g. sample(1, 1)
    SAMPLE_TIME_EVENT {
        /// unique sample index
        index: i32,
        startExp: Arc<DAE::Exp>,
        intervalExp: Arc<DAE::Exp>,
    },
}
impl Default for TimeEvent {
    fn default() -> Self { Self::SIMPLE_TIME_EVENT }
}
pub use self::TimeEvent::{SIMPLE_TIME_EVENT,SAMPLE_TIME_EVENT};

//
// AdjacencyMatrices
//
pub type AdjacencyMatrixElementEntry = i32;

pub type AdjacencyMatrixElement = Arc<metamodelica::List<i32>>;

/// array<list<Integer>>
pub type AdjacencyMatrix = metamodelica::Array<Arc<metamodelica::List<i32>>>;

/// a list of equation indices (1..n), one for each variable. Equations that -only-
/// contain the state variable and not the derivative have a negative index.
pub type AdjacencyMatrixT = metamodelica::Array<Arc<metamodelica::List<i32>>>;

/// a mapping for adjacency matrices that contains:
/// array<list<Integer>>: array index -> scalar index list
/// array<Integer>      : scalar index -> array index (not unique)
/// IndexType           : the occurence condition type for the current adjacency matrix
/// Boolean             : true if scalar
/// Boolean             : true if analytical to structural singularity processing has already been done
pub type AdjacencyMatrixMapping = (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, IndexType, bool, bool);

pub type AdjacencyMatrixElementEnhancedEntry = (i32, Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>);

pub type AdjacencyMatrixElementEnhanced = Arc<metamodelica::List<(i32, Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>;

pub type AdjacencyMatrixEnhanced = metamodelica::Array<Arc<metamodelica::List<(i32, Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;

pub type AdjacencyMatrixTEnhanced = metamodelica::Array<Arc<metamodelica::List<(i32, Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Solvability {
    /// Equation is already solved for the variable
    SOLVABILITY_SOLVED,
    /// Coefficient is equal 1 or -1
    SOLVABILITY_CONSTONE,
    /// Coefficient is constant
    SOLVABILITY_CONST {
        /// false if the constant is almost zero (<1e-6)
        b: bool,
    },
    /// Coefficient contains parameters
    SOLVABILITY_PARAMETER {
        /// false if the partial derivative is zero
        b: bool,
    },
    /// Coefficient contains variables, is time varying
    SOLVABILITY_LINEAR {
        /// false if the partial derivative is zero
        b: bool,
    },
    /// The variable occurs non-linear in the equation.
    SOLVABILITY_NONLINEAR,
    /// The variable occurs in the equation, but it is not possible to solve
    ///                     the equation for it.
    SOLVABILITY_UNSOLVABLE,
    /// It is possible to solve the equation for the variable, it is not considered
    ///                     how the variable occurs in the equation.
    SOLVABILITY_SOLVABLE,
}
pub use self::Solvability::{SOLVABILITY_SOLVED,SOLVABILITY_CONSTONE,SOLVABILITY_CONST,SOLVABILITY_PARAMETER,SOLVABILITY_LINEAR,SOLVABILITY_NONLINEAR,SOLVABILITY_UNSOLVABLE,SOLVABILITY_SOLVABLE};

/// Constraints on the solvability of the (casual) tearing set; needed for proper Dynamic Tearing
pub type Constraints = Arc<metamodelica::List<Arc<DAE::Constraint>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndexType {
    /// adjacency matrix with absolute indexes
    ABSOLUTE,
    /// adjacency matrix with positive/negative indexes
    NORMAL,
    /// adjacency matrix with only solvable entries, for example {a,b,c}[d] then d is skipped
    SOLVABLE,
    /// adjacency matrix for base-clock partitioning
    BASECLOCK_IDX,
    /// adjacency matrix for sub-clock partitioning
    SUBCLOCK_IDX,
    /// adjacency matrix as normal, but add for inputs also a value
    SPARSE,
}
impl Default for IndexType {
    fn default() -> Self { Self::ABSOLUTE }
}
pub use self::IndexType::{ABSOLUTE,NORMAL,SOLVABLE,BASECLOCK_IDX,SUBCLOCK_IDX,SPARSE};

//
// Jacobian stuff
//
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JacobianType {
    /// If Jacobian has only constant values, for system
    ///               of equations this means that it can be solved statically.
    JAC_CONSTANT,
    /// If Jacobian has time varying parts, like parameters or
    ///                  algebraic variables
    JAC_LINEAR,
    /// If Jacobian contains variables that are solved for,
    ///              means that a non-linear system of equations needs to be
    ///              solved
    JAC_NONLINEAR,
    /// GENERIC_JACOBIAN Jacobian available
    JAC_GENERIC,
    /// No analytic Jacobian available
    JAC_NO_ANALYTIC,
}
pub use self::JacobianType::{JAC_CONSTANT,JAC_LINEAR,JAC_NONLINEAR,JAC_GENERIC,JAC_NO_ANALYTIC};

pub const SymbolicJacobianAIndex: i32 = 1;

pub const SymbolicJacobianBIndex: i32 = 2;

pub const SymbolicJacobianCIndex: i32 = 3;

pub const SymbolicJacobianDIndex: i32 = 4;

pub const derivativeNamePrefix: &'static str = "$DERAlias";

pub const partialDerivativeNamePrefix: &'static str = "$pDER";

pub const functionDerivativeNamePrefix: &'static str = "$funDER";

pub const outputAliasPrefix: &'static str = "$outputAlias_";

pub const optimizationMayerTermName: &'static str = "$OMC$objectMayerTerm";

pub const optimizationLagrangeTermName: &'static str = "$OMC$objectLagrangeTerm";

pub const symSolverDT: &'static str = "__OMC_DT";

pub const homotopyLambda: &'static str = "__HOM_LAMBDA";

pub type FullJacobian = Option<Arc<metamodelica::List<(i32, i32, Arc<Equation>)>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Jacobian {
    FULL_JACOBIAN {
        jacobian: FullJacobian,
    },
    GENERIC_JACOBIAN {
        jacobian: Option<(Arc<BackendDAE>, ArcStr, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>,
        sparsePattern: SparsePattern,
        coloring: SparseColoring,
        nonlinearPattern: NonlinearPattern,
    },
    EMPTY_JACOBIAN,
}
impl Default for Jacobian {
    fn default() -> Self { Self::EMPTY_JACOBIAN }
}
pub use self::Jacobian::{FULL_JACOBIAN,GENERIC_JACOBIAN,EMPTY_JACOBIAN};

pub type SymbolicJacobians = Arc<metamodelica::List<(Option<(Arc<BackendDAE>, ArcStr, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>;

pub type SymbolicJacobian = (Arc<BackendDAE>, ArcStr, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);

// symbolic equation system
// Matrix name
// diff vars (independent vars)
// diffed vars (residual vars)
// all diffed vars (residual vars + dependent vars)
// original dependent variables
pub type SparsePatternCref = (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);

pub type SparsePatternCrefs = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>;

pub type NonlinearPatternCref = (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);

pub type NonlinearPatternCrefs = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>;

pub type SparsePattern = (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);

// column-wise sparse pattern
// row-wise sparse pattern
// diff vars (independent vars) of associated jacobian
// diffed vars (residual vars) of associated jacobian
// nonZeroElements
pub type NonlinearPattern = (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);

thread_local! { static __emptySparsePattern_TLS: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) = (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0); }
pub fn emptySparsePattern() -> (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) { __emptySparsePattern_TLS.with(|__t| __t.clone()) }

thread_local! { static __emptyNonlinearPattern_TLS: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) = (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0); }
pub fn emptyNonlinearPattern() -> (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) { __emptyNonlinearPattern_TLS.with(|__t| __t.clone()) }

pub type SparseColoring = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;

// colouring
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DifferentiateInputData {
    pub independenentVars: Option<Variables>,
    pub dependenentVars: Option<Variables>,
    pub knownVars: Option<Variables>,
    pub allVars: Option<Variables>,
    pub controlVars: Arc<metamodelica::List<Var>>,
    pub diffCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub matrixName: Option<ArcStr>,
    pub diffedFunctions: Arc<AvlSetPath::Tree>,
}

impl Default for DifferentiateInputData {
    fn default() -> Self {
        Self {
            independenentVars: Default::default(),
            dependenentVars: Default::default(),
            knownVars: Default::default(),
            allVars: Default::default(),
            controlVars: Default::default(),
            diffCrefs: Default::default(),
            matrixName: Default::default(),
            diffedFunctions: Default::default(),
        }
    }
}

pub type DIFFINPUTDATA = DifferentiateInputData;


thread_local! { static __emptyInputData_TLS: DifferentiateInputData = DifferentiateInputData { independenentVars: None, dependenentVars: None, knownVars: None, allVars: None, controlVars: metamodelica::nil(), diffCrefs: metamodelica::nil(), matrixName: None, diffedFunctions: Arc::new(openmodelica_ast_collections::AvlSetPath::Tree::EMPTY) }; }
pub fn emptyInputData() -> DifferentiateInputData { __emptyInputData_TLS.with(|__t| __t.clone()) }

pub type DifferentiateInputArguments = (Arc<DAE::ComponentRef>, DifferentiateInputData, DifferentiationType, Arc<AvlTreePathFunction::Tree>);

/// Define the behaviour of differentiation method for (e.g. index reduction, ...)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DifferentiationType {
    /// Used for index reduction differentiation w.r.t. time (e.g. create dummy derivative variables)
    DIFFERENTIATION_TIME,
    /// Used to solve expression for a cref or by the older Jacobian generation, differentiation w.r.t. a given cref
    SIMPLE_DIFFERENTIATION,
    /// Used to differentiate a function call w.r.t. a given cref, which need to expand the input arguments
    ///                                  by differentiate arguments.
    DIFFERENTIATION_FUNCTION,
    /// Used to generate a full Jacobian matrix
    DIFF_FULL_JACOBIAN,
    /// Used to generate a generic gradient for generation the Jacobian matrix while the runtime.
    GENERIC_GRADIENT {
        /// true if computing for dae mode
        daeMode: bool,
    },
}
pub use self::DifferentiationType::{DIFFERENTIATION_TIME,SIMPLE_DIFFERENTIATION,DIFFERENTIATION_FUNCTION,DIFF_FULL_JACOBIAN,GENERIC_GRADIENT};

/// types to count operations for the components
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompInfo {
    COUNTER {
        comp: Arc<StrongComponent>,
        numAdds: i32,
        numMul: i32,
        numDiv: i32,
        numTrig: i32,
        numRelations: i32,
        numLog: i32,
        numOth: i32,
        funcCalls: i32,
    },
    SYSTEM {
        comp: Arc<StrongComponent>,
        allOperations: Arc<CompInfo>,
        size: i32,
        density: metamodelica::Real,
    },
    TORN_ANALYSE {
        comp: Arc<StrongComponent>,
        tornEqs: Arc<CompInfo>,
        otherEqs: Arc<CompInfo>,
        tornSize: i32,
    },
    NO_COMP {
        numAdds: i32,
        numMul: i32,
        numDiv: i32,
        numTrig: i32,
        numRelations: i32,
        numLog: i32,
        numOth: i32,
        funcCalls: i32,
    },
}
pub use self::CompInfo::{COUNTER,SYSTEM,TORN_ANALYSE,NO_COMP};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackendDAEModeData {
    pub stateVars: Arc<metamodelica::List<Var>>,
    pub algStateVars: Arc<metamodelica::List<Var>>,
    pub numResVars: i32,
    pub modelVars: Option<Variables>,
}

impl Default for BackendDAEModeData {
    fn default() -> Self {
        Self {
            stateVars: Default::default(),
            algStateVars: Default::default(),
            numResVars: Default::default(),
            modelVars: Default::default(),
        }
    }
}

pub type BDAE_MODE_DATA = BackendDAEModeData;


thread_local! { static __emptyDAEModeData_TLS: BackendDAEModeData = BackendDAEModeData { stateVars: metamodelica::nil(), algStateVars: metamodelica::nil(), numResVars: 0, modelVars: None }; }
pub fn emptyDAEModeData() -> BackendDAEModeData { __emptyDAEModeData_TLS.with(|__t| __t.clone()) }

