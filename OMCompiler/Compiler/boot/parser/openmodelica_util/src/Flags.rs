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

use crate::Gettext;
use crate::Global;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugFlag {
    /// Unique index.
    pub index: i32,
    /// The name of the flag used by -d
    pub name: ArcStr,
    /// Default enabled or not
    pub default: bool,
    /// A description of the flag.
    pub description: Gettext::TranslatableContent,
}

pub type DEBUG_FLAG = DebugFlag;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfigFlag {
    /// Unique index.
    pub index: i32,
    /// The whole name of the flag.
    pub name: ArcStr,
    /// A short name one-character name for the flag.
    pub shortname: Option<ArcStr>,
    /// Whether the flag is visible to the user or not.
    pub visibility: FlagVisibility,
    /// The default value of the flag.
    pub defaultValue: FlagData,
    /// The valid options for the flag.
    pub validOptions: Option<ValidOptions>,
    /// A description of the flag.
    pub description: Gettext::TranslatableContent,
}

impl Default for ConfigFlag {
    fn default() -> Self {
        Self {
            index: Default::default(),
            name: Default::default(),
            shortname: Default::default(),
            visibility: Default::default(),
            defaultValue: Default::default(),
            validOptions: Default::default(),
            description: Default::default(),
        }
    }
}

pub type CONFIG_FLAG = ConfigFlag;


/// This uniontype is used to store the values of configuration flags.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlagData {
    /// Only used to initialize the flag array.
    EMPTY_FLAG,
    BOOL_FLAG {
        /// Value of a boolean flag.
        data: bool,
    },
    INT_FLAG {
        /// Value of an integer flag.
        data: i32,
    },
    INT_LIST_FLAG {
        /// Value of an integer flag that can have multiple values.
        data: Arc<metamodelica::List<i32>>,
    },
    REAL_FLAG {
        /// Value of a real flag.
        data: metamodelica::Real,
    },
    STRING_FLAG {
        /// Value of a string flag.
        data: ArcStr,
    },
    STRING_LIST_FLAG {
        /// Values of a string flag that can have multiple values.
        data: Arc<metamodelica::List<ArcStr>>,
    },
    ENUM_FLAG {
        /// Value of an enumeration flag.
        data: i32,
        /// The valid values of the enum.
        validValues: Arc<metamodelica::List<(ArcStr, i32)>>,
    },
}
impl Default for FlagData {
    fn default() -> Self { Self::EMPTY_FLAG }
}
pub use self::FlagData::{EMPTY_FLAG,BOOL_FLAG,INT_FLAG,INT_LIST_FLAG,REAL_FLAG,STRING_FLAG,STRING_LIST_FLAG,ENUM_FLAG};

/// This uniontype is used to specify the visibility of a configuration flag.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlagVisibility {
    /// An internal flag that is hidden to the user.
    INTERNAL,
    /// An external flag that is visible to the user.
    EXTERNAL,
}
impl Default for FlagVisibility {
    fn default() -> Self { Self::INTERNAL }
}
pub use self::FlagVisibility::{INTERNAL,EXTERNAL};

/// The structure which stores the flags.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flag {
    FLAGS {
        debugFlags: metamodelica::Array<bool>,
        configFlags: metamodelica::Array<FlagData>,
    },
    NO_FLAGS,
}
pub use self::Flag::{FLAGS,NO_FLAGS};

/// Specifies valid options for a flag.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidOptions {
    STRING_OPTION {
        /// Options for a string flag.
        options: Arc<metamodelica::List<ArcStr>>,
    },
    STRING_DESC_OPTION {
        /// Options for a string flag, with a description for each option.
        options: Arc<metamodelica::List<(ArcStr, Gettext::TranslatableContent)>>,
    },
}
impl Default for ValidOptions {
    fn default() -> Self {
        Self::STRING_OPTION {
            options: Default::default(),
        }
    }
}
pub use self::ValidOptions::{STRING_OPTION,STRING_DESC_OPTION};

// Change this to a proper enum when we have support for them.
pub const MODELICA: i32 = 1;

pub const METAMODELICA: i32 = 2;

pub const PARMODELICA: i32 = 3;

pub const OPTIMICA: i32 = 4;

pub const PDEMODELICA: i32 = 5;

// FMI-ModelDescription-ENUM-FLAGS
pub const FMI_NONE: i32 = 1;

pub const FMI_INTERNAL: i32 = 2;

pub const FMI_PROTECTED: i32 = 3;

pub const FMI_BLACKBOX: i32 = 4;

pub static collapseArrayExpressionsText: Gettext::TranslatableContent = Gettext::TranslatableContent::gettext { msgid: literal!("Simplifies {x[1],x[2],x[3]} → x for arrays of whole variable references (simplifies code generation).") };

// DEBUG FLAGS
pub static FAILTRACE: DebugFlag = DebugFlag { index: 1, name: literal!("failtrace"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets whether to print a failtrace or not.") } };

pub static CEVAL: DebugFlag = DebugFlag { index: 2, name: literal!("ceval"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints extra information from Ceval.") } };

pub static CHECK_BACKEND_DAE: DebugFlag = DebugFlag { index: 3, name: literal!("checkBackendDae"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Do some simple analyses on the datastructure from the frontend to check if it is consistent.") } };

pub static PTHREADS: DebugFlag = DebugFlag { index: 4, name: literal!("pthreads"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Experimental: Unused parallelization.") } };

pub static EVENTS: DebugFlag = DebugFlag { index: 5, name: literal!("events"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on/off events handling.") } };

pub static DUMP_INLINE_SOLVER: DebugFlag = DebugFlag { index: 6, name: literal!("dumpInlineSolver"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the inline solver equation system.") } };

pub static EVAL_FUNC: DebugFlag = DebugFlag { index: 7, name: literal!("evalfunc"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on/off symbolic function evaluation.") } };

pub static GEN: DebugFlag = DebugFlag { index: 8, name: literal!("gen"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on/off dynamic loading of functions that are compiled during translation. Only enable this if external functions are needed to calculate structural parameters or constants.") } };

pub static DYN_LOAD: DebugFlag = DebugFlag { index: 9, name: literal!("dynload"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Display debug information about dynamic loading of compiled functions.") } };

pub static GENERATE_CODE_CHEAT: DebugFlag = DebugFlag { index: 10, name: literal!("generateCodeCheat"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Used to generate code for the bootstrapped compiler.") } };

pub static CGRAPH_GRAPHVIZ_FILE: DebugFlag = DebugFlag { index: 11, name: literal!("cgraphGraphVizFile"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates a graphviz file of the connection graph.") } };

pub static CGRAPH_GRAPHVIZ_SHOW: DebugFlag = DebugFlag { index: 12, name: literal!("cgraphGraphVizShow"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Displays the connection graph with the GraphViz lefty tool.") } };

pub static GC_PROF: DebugFlag = DebugFlag { index: 13, name: literal!("gcProfiling"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints garbage collection stats to standard output.") } };

pub static CHECK_DAE_CREF_TYPE: DebugFlag = DebugFlag { index: 14, name: literal!("checkDAECrefType"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables extra type checking for cref expressions.") } };

pub static CHECK_ASUB: DebugFlag = DebugFlag { index: 15, name: literal!("checkASUB"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out a warning if an ASUB is created from a CREF expression.") } };

pub static INSTANCE: DebugFlag = DebugFlag { index: 16, name: literal!("instance"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints extra failtrace from InstanceHierarchy.") } };

pub static CACHE: DebugFlag = DebugFlag { index: 17, name: literal!("Cache"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns off the instantiation cache.") } };

pub static RML: DebugFlag = DebugFlag { index: 18, name: literal!("rml"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Converts Modelica-style arrays to lists.") } };

pub static TAIL: DebugFlag = DebugFlag { index: 19, name: literal!("tail"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out a notification if tail recursion optimization has been applied.") } };

pub static LOOKUP: DebugFlag = DebugFlag { index: 20, name: literal!("lookup"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Print extra failtrace from lookup.") } };

pub static PATTERNM_SKIP_FILTER_UNUSED_AS_BINDINGS: DebugFlag = DebugFlag { index: 21, name: literal!("patternmSkipFilterUnusedBindings"), default: false, description: Gettext::TranslatableContent::notrans { r#str: literal!("") } };

pub static PATTERNM_ALL_INFO: DebugFlag = DebugFlag { index: 22, name: literal!("patternmAllInfo"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Adds notifications of all pattern-matching optimizations that are performed.") } };

pub static PATTERNM_DCE: DebugFlag = DebugFlag { index: 23, name: literal!("patternmDeadCodeElimination"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Performs dead code elimination in match-expressions.") } };

pub static PATTERNM_MOVE_LAST_EXP: DebugFlag = DebugFlag { index: 24, name: literal!("patternmMoveLastExp"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Optimization that moves the last assignment(s) into the result of a match-expression. For example: equation c = fn(b); then c; => then fn(b);") } };

pub static EXPERIMENTAL_REDUCTIONS: DebugFlag = DebugFlag { index: 25, name: literal!("experimentalReductions"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on custom reduction functions (OpenModelica extension).") } };

pub static EVAL_PARAM: DebugFlag = DebugFlag { index: 26, name: literal!("evaluateAllParameters"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Evaluates all parameters if set, except the ones that have annotation(Evaluate = false).") } };

pub static TYPES: DebugFlag = DebugFlag { index: 27, name: literal!("types"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints extra failtrace from Types.") } };

pub static SHOW_STATEMENT: DebugFlag = DebugFlag { index: 28, name: literal!("showStatement"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Shows the statement that is currently being evaluated when evaluating a script.") } };

pub static DUMP: DebugFlag = DebugFlag { index: 29, name: literal!("dump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the absyn representation of a program.") } };

pub static DUMP_GRAPHVIZ: DebugFlag = DebugFlag { index: 30, name: literal!("graphviz"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the absyn representation of a program in graphviz format.") } };

pub static EXEC_STAT: DebugFlag = DebugFlag { index: 31, name: literal!("execstat"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out execution statistics for the compiler.") } };

pub static TRANSFORMS_BEFORE_DUMP: DebugFlag = DebugFlag { index: 32, name: literal!("transformsbeforedump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Applies transformations required for code generation before dumping flat code.") } };

pub static DAE_DUMP_GRAPHV: DebugFlag = DebugFlag { index: 33, name: literal!("daedumpgraphv"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the DAE in graphviz format.") } };

pub static INTERACTIVE_TCP: DebugFlag = DebugFlag { index: 34, name: literal!("interactive"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Starts omc as a server listening on the socket interface.") } };

pub static INTERACTIVE_CORBA: DebugFlag = DebugFlag { index: 35, name: literal!("interactiveCorba"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Starts omc as a server listening on the Corba interface.") } };

pub static INTERACTIVE_DUMP: DebugFlag = DebugFlag { index: 36, name: literal!("interactivedump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out debug information for the interactive server.") } };

pub static RELIDX: DebugFlag = DebugFlag { index: 37, name: literal!("relidx"), default: false, description: Gettext::TranslatableContent::notrans { r#str: literal!("Prints out debug information about relations, that are used as zero crossings.") } };

pub static DUMP_REPL: DebugFlag = DebugFlag { index: 38, name: literal!("dumprepl"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump the found replacements for simple equation removal.") } };

pub static DUMP_FP_REPL: DebugFlag = DebugFlag { index: 39, name: literal!("dumpFPrepl"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump the found replacements for final parameters.") } };

pub static DUMP_PARAM_REPL: DebugFlag = DebugFlag { index: 40, name: literal!("dumpParamrepl"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump the found replacements for remove parameters.") } };

pub static DUMP_PP_REPL: DebugFlag = DebugFlag { index: 41, name: literal!("dumpPPrepl"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump the found replacements for protected parameters.") } };

pub static DUMP_EA_REPL: DebugFlag = DebugFlag { index: 42, name: literal!("dumpEArepl"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump the found replacements for evaluate annotations (evaluate=true) parameters.") } };

pub static DEBUG_ALIAS: DebugFlag = DebugFlag { index: 43, name: literal!("debugAlias"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps some information about the process of removeSimpleEquations.") } };

pub static TEARING_DUMP: DebugFlag = DebugFlag { index: 44, name: literal!("tearingdump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps tearing information.") } };

pub static JAC_DUMP: DebugFlag = DebugFlag { index: 45, name: literal!("symjacdump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about symbolic Jacobians.") } };

pub static JAC_DUMP2: DebugFlag = DebugFlag { index: 46, name: literal!("symjacdumpverbose"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information in verbose mode about symbolic Jacobians.") } };

pub static DUMP_BINDINGS: DebugFlag = DebugFlag { index: 47, name: literal!("dumpBindings"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about the equations created from bindings.") } };

pub static DUMP_SORTING: DebugFlag = DebugFlag { index: 48, name: literal!("dumpSorting"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about the process of sorting.") } };

pub static DUMP_SPARSE: DebugFlag = DebugFlag { index: 49, name: literal!("dumpSparsePattern"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps sparse pattern with coloring used for simulation.") } };

pub static DUMP_SPARSE_VERBOSE: DebugFlag = DebugFlag { index: 50, name: literal!("dumpSparsePatternVerbose"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps in verbose mode sparse pattern with coloring used for simulation.") } };

pub static BLT_DUMP: DebugFlag = DebugFlag { index: 51, name: literal!("bltdump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information from index reduction.") } };

pub static DUMMY_SELECT: DebugFlag = DebugFlag { index: 52, name: literal!("dummyselect"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information from dummy state selection heuristic.") } };

pub static DUMP_DAE_LOW: DebugFlag = DebugFlag { index: 53, name: literal!("dumpdaelow"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the equation system at the beginning of the back end.") } };

pub static DUMP_INDX_DAE: DebugFlag = DebugFlag { index: 54, name: literal!("dumpindxdae"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the equation system after index reduction and optimization.") } };

pub static OPT_DAE_DUMP: DebugFlag = DebugFlag { index: 55, name: literal!("optdaedump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information from the optimization modules.") } };

pub static EXEC_HASH: DebugFlag = DebugFlag { index: 56, name: literal!("execHash"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Measures the time it takes to hash all simcode variables before code generation.") } };

pub static PARAM_DLOW_DUMP: DebugFlag = DebugFlag { index: 57, name: literal!("paramdlowdump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of the parameters in the order they are calculated.") } };

pub static DUMP_ENCAPSULATECONDITIONS: DebugFlag = DebugFlag { index: 58, name: literal!("dumpEncapsulateConditions"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the results of the preOptModule encapsulateWhenConditions.") } };

pub static SHORT_OUTPUT: DebugFlag = DebugFlag { index: 59, name: literal!("shortOutput"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables short output of the simulate() command. Useful for tools like OMNotebook.") } };

pub static COUNT_OPERATIONS: DebugFlag = DebugFlag { index: 60, name: literal!("countOperations"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Count operations.") } };

pub static CGRAPH: DebugFlag = DebugFlag { index: 61, name: literal!("cgraph"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out connection graph information.") } };

pub static UPDMOD: DebugFlag = DebugFlag { index: 62, name: literal!("updmod"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints information about modification updates.") } };

pub static STATIC: DebugFlag = DebugFlag { index: 63, name: literal!("static"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables extra debug output from the static elaboration.") } };

pub static TPL_PERF_TIMES: DebugFlag = DebugFlag { index: 64, name: literal!("tplPerfTimes"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables output of template performance data for rendering text to file.") } };

pub static CHECK_SIMPLIFY: DebugFlag = DebugFlag { index: 65, name: literal!("checkSimplify"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables checks for expression simplification and prints a notification whenever an undesirable transformation has been performed.") } };

pub static SCODE_INST: DebugFlag = DebugFlag { index: 66, name: literal!("newInst"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables new instantiation phase.") } };

pub static WRITE_TO_BUFFER: DebugFlag = DebugFlag { index: 67, name: literal!("writeToBuffer"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables writing simulation results to buffer.") } };

pub static DUMP_BACKENDDAE_INFO: DebugFlag = DebugFlag { index: 68, name: literal!("backenddaeinfo"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of back-end information about system (Number of equations before back-end,...).") } };

pub static GEN_DEBUG_SYMBOLS: DebugFlag = DebugFlag { index: 69, name: literal!("gendebugsymbols"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generate code with debugging symbols.") } };

pub static DUMP_STATESELECTION_INFO: DebugFlag = DebugFlag { index: 70, name: literal!("stateselection"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of selected states. Extends -d=backenddaeinfo.") } };

pub static DUMP_EQNINORDER: DebugFlag = DebugFlag { index: 71, name: literal!("dumpeqninorder"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of the equations in the order they are calculated.") } };

pub static SEMILINEAR: DebugFlag = DebugFlag { index: 72, name: literal!("semiLinear"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of the optimization information when optimizing calls to semiLinear.") } };

pub static UNCERTAINTIES: DebugFlag = DebugFlag { index: 73, name: literal!("uncertainties"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of status when calling modelEquationsUC.") } };

pub static SHOW_START_ORIGIN: DebugFlag = DebugFlag { index: 74, name: literal!("showStartOrigin"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of the DAE startOrigin attribute of the variables.") } };

pub static DUMP_SIMCODE: DebugFlag = DebugFlag { index: 75, name: literal!("dumpSimCode"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the simCode model used for code generation.") } };

pub static DUMP_INITIAL_SYSTEM: DebugFlag = DebugFlag { index: 76, name: literal!("dumpinitialsystem"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the initial equation system.") } };

pub static GRAPH_INST: DebugFlag = DebugFlag { index: 77, name: literal!("graphInst"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Do graph based instantiation.") } };

pub static GRAPH_INST_RUN_DEP: DebugFlag = DebugFlag { index: 78, name: literal!("graphInstRunDep"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Run scode dependency analysis. Use with -d=graphInst") } };

pub static GRAPH_INST_GEN_GRAPH: DebugFlag = DebugFlag { index: 79, name: literal!("graphInstGenGraph"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps a graph of the program. Use with -d=graphInst") } };

pub static DUMP_CONST_REPL: DebugFlag = DebugFlag { index: 80, name: literal!("dumpConstrepl"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump the found replacements for constants.") } };

pub static SHOW_EQUATION_SOURCE: DebugFlag = DebugFlag { index: 81, name: literal!("showEquationSource"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Display the element source information in the dumped DAE for easier debugging.") } };

pub static LS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag { index: 82, name: literal!("LSanalyticJacobian"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables analytical jacobian for linear strong components. Defaults to false") } };

pub static NLS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag { index: 83, name: literal!("NLSanalyticJacobian"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables analytical jacobian for non-linear strong components without user-defined function calls, for that see forceNLSanalyticJacobian") } };

pub static INLINE_SOLVER: DebugFlag = DebugFlag { index: 84, name: literal!("inlineSolver"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates code for inline solver.") } };

pub static HPCOM: DebugFlag = DebugFlag { index: 85, name: literal!("hpcom"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables parallel calculation based on task-graphs.") } };

pub static INITIALIZATION: DebugFlag = DebugFlag { index: 86, name: literal!("initialization"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Shows additional information from the initialization process.") } };

pub static INLINE_FUNCTIONS: DebugFlag = DebugFlag { index: 87, name: literal!("inlineFunctions"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Controls if function inlining should be performed.") } };

pub static DUMP_SCC_GRAPHML: DebugFlag = DebugFlag { index: 88, name: literal!("dumpSCCGraphML"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps graphml files with the strongly connected components.") } };

pub static TEARING_DUMPVERBOSE: DebugFlag = DebugFlag { index: 89, name: literal!("tearingdumpV"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps verbose tearing information.") } };

pub static DISABLE_SINGLE_FLOW_EQ: DebugFlag = DebugFlag { index: 90, name: literal!("disableSingleFlowEq"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables the generation of single flow equations.") } };

pub static DUMP_DISCRETEVARS_INFO: DebugFlag = DebugFlag { index: 91, name: literal!("discreteinfo"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of discrete variables. Extends -d=backenddaeinfo.") } };

pub static ADDITIONAL_GRAPHVIZ_DUMP: DebugFlag = DebugFlag { index: 92, name: literal!("graphvizDump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Activates additional graphviz dumps (as .dot files). It can be used in addition to one of the following flags: {dumpdaelow|dumpinitialsystems|dumpindxdae}.") } };

pub static INFO_XML_OPERATIONS: DebugFlag = DebugFlag { index: 93, name: literal!("infoXmlOperations"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables output of the operations in the _info.xml file when translating models.") } };

pub static HPCOM_DUMP: DebugFlag = DebugFlag { index: 94, name: literal!("hpcomDump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps additional information on the parallel execution with hpcom.") } };

pub static RESOLVE_LOOPS_DUMP: DebugFlag = DebugFlag { index: 95, name: literal!("resolveLoopsDump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Debug Output for ResolveLoops Module.") } };

pub static DISABLE_WINDOWS_PATH_CHECK_WARNING: DebugFlag = DebugFlag { index: 96, name: literal!("disableWindowsPathCheckWarning"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables warnings on Windows if OPENMODELICAHOME/MinGW is missing.") } };

pub static DISABLE_RECORD_CONSTRUCTOR_OUTPUT: DebugFlag = DebugFlag { index: 97, name: literal!("disableRecordConstructorOutput"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables output of record constructors in the flat code.") } };

pub static IMPL_ODE: DebugFlag = DebugFlag { index: 98, name: literal!("implOde"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("activates implicit codegen") } };

pub static EVAL_FUNC_DUMP: DebugFlag = DebugFlag { index: 99, name: literal!("evalFuncDump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("dumps debug information about the function evaluation") } };

pub static PRINT_STRUCTURAL: DebugFlag = DebugFlag { index: 100, name: literal!("printStructuralParameters"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints the structural parameters identified by the front-end") } };

pub static ITERATION_VARS: DebugFlag = DebugFlag { index: 101, name: literal!("iterationVars"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Shows a list of all iteration variables.") } };

pub static ALLOW_RECORD_TOO_MANY_FIELDS: DebugFlag = DebugFlag { index: 102, name: literal!("acceptTooManyFields"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Accepts passing records with more fields than expected to a function. This is not allowed, but is used in Fluid.Dissipation. See https://trac.modelica.org/Modelica/ticket/1245 for details.") } };

pub static HPCOM_MEMORY_OPT: DebugFlag = DebugFlag { index: 103, name: literal!("hpcomMemoryOpt"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Optimize the memory structure regarding the selected scheduler") } };

pub static DUMP_SYNCHRONOUS: DebugFlag = DebugFlag { index: 104, name: literal!("dumpSynchronous"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information of the clock partitioning.") } };

pub static STRIP_PREFIX: DebugFlag = DebugFlag { index: 105, name: literal!("stripPrefix"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Strips the environment prefix from path/crefs. Defaults to true.") } };

pub static DO_SCODE_DEP: DebugFlag = DebugFlag { index: 106, name: literal!("scodeDep"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Does scode dependency analysis prior to instantiation. Defaults to true.") } };

pub static SHOW_INST_CACHE_INFO: DebugFlag = DebugFlag { index: 107, name: literal!("showInstCacheInfo"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints information about instantiation cache hits and additions. Defaults to false.") } };

pub static DUMP_UNIT: DebugFlag = DebugFlag { index: 108, name: literal!("dumpUnits"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps all the calculated units.") } };

pub static DUMP_EQ_UNIT: DebugFlag = DebugFlag { index: 109, name: literal!("dumpEqInUC"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps all equations handled by the unit checker.") } };

pub static DUMP_EQ_UNIT_STRUCT: DebugFlag = DebugFlag { index: 110, name: literal!("dumpEqUCStruct"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps all the equations handled by the unit checker as tree-structure.") } };

pub static SHOW_DAE_GENERATION: DebugFlag = DebugFlag { index: 111, name: literal!("showDaeGeneration"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Show the dae variable declarations as they happen.") } };

pub static RESHUFFLE_POST: DebugFlag = DebugFlag { index: 112, name: literal!("reshufflePost"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Reshuffles the systems of equations.") } };

pub static SHOW_EXPANDABLE_INFO: DebugFlag = DebugFlag { index: 113, name: literal!("showExpandableInfo"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Show information about expandable connector handling.") } };

pub static DUMP_HOMOTOPY: DebugFlag = DebugFlag { index: 114, name: literal!("dumpHomotopy"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the results of the postOptModule optimizeHomotopyCalls.") } };

pub static OMC_RELOCATABLE_FUNCTIONS: DebugFlag = DebugFlag { index: 115, name: literal!("relocatableFunctions"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates relocatable code: all functions become function pointers and can be replaced at run-time.") } };

pub static GRAPHML: DebugFlag = DebugFlag { index: 116, name: literal!("graphml"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps .graphml files for the bipartite graph after Index Reduction and a task graph for the SCCs. Can be displayed with yEd. ") } };

pub static USEMPI: DebugFlag = DebugFlag { index: 117, name: literal!("useMPI"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Add MPI init and finalize to main method (CPPruntime). ") } };

pub static DUMP_CSE: DebugFlag = DebugFlag { index: 118, name: literal!("dumpCSE"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Additional output for CSE module.") } };

pub static DUMP_CSE_VERBOSE: DebugFlag = DebugFlag { index: 119, name: literal!("dumpCSE_verbose"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Additional output for CSE module.") } };

pub static NO_START_CALC: DebugFlag = DebugFlag { index: 120, name: literal!("disableStartCalc"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Deactivates the pre-calculation of start values during compile-time.") } };

pub static CONSTJAC: DebugFlag = DebugFlag { index: 121, name: literal!("constjac"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("solves linear systems with constant Jacobian and variable b-Vector symbolically") } };

pub static VISUAL_XML: DebugFlag = DebugFlag { index: 122, name: literal!("visxml"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Outputs a xml-file that contains information for visualization.") } };

pub static VECTORIZE: DebugFlag = DebugFlag { index: 123, name: literal!("vectorize"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Activates vectorization in the backend.") } };

pub static CHECK_EXT_LIBS: DebugFlag = DebugFlag { index: 124, name: literal!("buildExternalLibs"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Use the autotools project in the Resources folder of the library to build missing external libraries.") } };

pub static RUNTIME_STATIC_LINKING: DebugFlag = DebugFlag { index: 125, name: literal!("runtimeStaticLinking"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Use the static simulation runtime libraries (C++ simulation runtime).") } };

pub static SORT_EQNS_AND_VARS: DebugFlag = DebugFlag { index: 126, name: literal!("dumpSortEqnsAndVars"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps debug output for the modules sortEqnsVars.") } };

pub static DUMP_SIMPLIFY_LOOPS: DebugFlag = DebugFlag { index: 127, name: literal!("dumpSimplifyLoops"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump between steps of simplifyLoops") } };

pub static DUMP_RTEARING: DebugFlag = DebugFlag { index: 128, name: literal!("dumpRecursiveTearing"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump between steps of recursiveTearing") } };

pub static DIS_SYMJAC_FMI20: DebugFlag = DebugFlag { index: 129, name: literal!("disableDirectionalDerivatives"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("For FMI 2.0 only dependecy analysis will be perform.") } };

pub static EVAL_OUTPUT_ONLY: DebugFlag = DebugFlag { index: 130, name: literal!("evalOutputOnly"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates equations to calculate top level outputs only.") } };

pub static HARDCODED_START_VALUES: DebugFlag = DebugFlag { index: 131, name: literal!("hardcodedStartValues"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Embed the start values of variables and parameters into the c++ code and do not read it from xml file.") } };

pub static DUMP_FUNCTIONS: DebugFlag = DebugFlag { index: 132, name: literal!("dumpFunctions"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Add functions to backend dumps.") } };

pub static DEBUG_DIFFERENTIATION: DebugFlag = DebugFlag { index: 133, name: literal!("debugDifferentiation"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps debug output for the differentiation process.") } };

pub static DEBUG_DIFFERENTIATION_VERBOSE: DebugFlag = DebugFlag { index: 134, name: literal!("debugDifferentiationVerbose"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps verbose debug output for the differentiation process.") } };

pub static FMU_EXPERIMENTAL: DebugFlag = DebugFlag { index: 135, name: literal!("fmuExperimental"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Adds features to the FMI export that are considered experimental as of now: fmi2GetSpecificDerivatives, canGetSetFMUState, canSerializeFMUstate") } };

pub static DUMP_DGESV: DebugFlag = DebugFlag { index: 136, name: literal!("dumpdgesv"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables dumping of the information whether DGESV is used to solve linear systems.") } };

pub static MULTIRATE_PARTITION: DebugFlag = DebugFlag { index: 137, name: literal!("multirate"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("The solver can switch partitions in the system.") } };

pub static DUMP_EXCLUDED_EXP: DebugFlag = DebugFlag { index: 138, name: literal!("dumpExcludedSymJacExps"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("This flags dumps all expression that are excluded from differentiation of a symbolic Jacobian.") } };

pub static DEBUG_ALGLOOP_JACOBIAN: DebugFlag = DebugFlag { index: 139, name: literal!("debugAlgebraicLoopsJacobian"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps debug output while creating symbolic jacobians for non-linear systems.") } };

pub static DISABLE_JACSCC: DebugFlag = DebugFlag { index: 140, name: literal!("disableJacsforSCC"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables calculation of jacobians to detect if a SCC is linear or non-linear. By disabling all SCC will handled like non-linear.") } };

pub static FORCE_NLS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag { index: 141, name: literal!("forceNLSanalyticJacobian"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Forces calculation analytical jacobian also for non-linear strong components with user-defined functions.") } };

pub static DUMP_LOOPS: DebugFlag = DebugFlag { index: 142, name: literal!("dumpLoops"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps loop equation.") } };

pub static DUMP_LOOPS_VERBOSE: DebugFlag = DebugFlag { index: 143, name: literal!("dumpLoopsVerbose"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps loop equation and enhanced adjacency matrix.") } };

pub static SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR: DebugFlag = DebugFlag { index: 144, name: literal!("skipInputOutputSyntacticSugar"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Used when bootstrapping to preserve the input output parsing of the code output by the list command.") } };

pub static OMC_RECORD_ALLOC_WORDS: DebugFlag = DebugFlag { index: 145, name: literal!("metaModelicaRecordAllocWords"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Instrument the source code to record memory allocations (requires run-time and generated files compiled with -DOMC_RECORD_ALLOC_WORDS).") } };

pub static TOTAL_TEARING_DUMP: DebugFlag = DebugFlag { index: 146, name: literal!("totaltearingdump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps total tearing information.") } };

pub static TOTAL_TEARING_DUMPVERBOSE: DebugFlag = DebugFlag { index: 147, name: literal!("totaltearingdumpV"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps verbose total tearing information.") } };

pub static PARALLEL_CODEGEN: DebugFlag = DebugFlag { index: 148, name: literal!("parallelCodegen"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables code generation in parallel (disable this if compiling a model causes you to run out of RAM).") } };

pub static SERIALIZED_SIZE: DebugFlag = DebugFlag { index: 149, name: literal!("reportSerializedSize"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Reports serialized sizes of various data structures used in the compiler.") } };

pub static BACKEND_KEEP_ENV_GRAPH: DebugFlag = DebugFlag { index: 150, name: literal!("backendKeepEnv"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("When enabled, the environment is kept when entering the backend, which enables CevalFunction (function interpretation) to work. This module not essential for the backend to function in most cases, but can improve simulation performance by evaluating functions. The drawback to keeping the environment graph in memory is that it is huge (~80% of the total memory in use when returning the frontend DAE).") } };

pub static DUMPBACKENDINLINE: DebugFlag = DebugFlag { index: 151, name: literal!("dumpBackendInline"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps debug output while inline function.") } };

pub static DUMPBACKENDINLINE_VERBOSE: DebugFlag = DebugFlag { index: 152, name: literal!("dumpBackendInlineVerbose"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps debug output while inline function.") } };

pub static BLT_MATRIX_DUMP: DebugFlag = DebugFlag { index: 153, name: literal!("bltmatrixdump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the blt matrix in html file. IE seems to be very good in displaying large matrices.") } };

pub static LIST_REVERSE_WRONG_ORDER: DebugFlag = DebugFlag { index: 154, name: literal!("listAppendWrongOrder"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Print notifications about bad usage of listAppend.") } };

pub static PARTITION_INITIALIZATION: DebugFlag = DebugFlag { index: 155, name: literal!("partitionInitialization"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("This flag controls if partitioning is applied to the initialization system.") } };

pub static EVAL_PARAM_DUMP: DebugFlag = DebugFlag { index: 156, name: literal!("evalParameterDump"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information for evaluating parameters.") } };

pub static NF_UNITCHECK: DebugFlag = DebugFlag { index: 157, name: literal!("frontEndUnitCheck"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Checks the consistency of units in equation.") } };

pub static DISABLE_COLORING: DebugFlag = DebugFlag { index: 158, name: literal!("disableColoring"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables coloring algorithm while spasity detection.") } };

pub static MERGE_ALGORITHM_SECTIONS: DebugFlag = DebugFlag { index: 159, name: literal!("mergeAlgSections"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables coloring algorithm while sparsity detection.") } };

pub static WARN_NO_NOMINAL: DebugFlag = DebugFlag { index: 160, name: literal!("warnNoNominal"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints the iteration variables in the initialization and simulation DAE, which do not have a nominal value.") } };

pub static REDUCE_DAE: DebugFlag = DebugFlag { index: 161, name: literal!("backendReduceDAE"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints all Reduce DAE debug information.") } };

pub static IGNORE_CYCLES: DebugFlag = DebugFlag { index: 162, name: literal!("ignoreCycles"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Ignores cycles between constant/parameter components.") } };

pub static ALIAS_CONFLICTS: DebugFlag = DebugFlag { index: 163, name: literal!("aliasConflicts"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps alias sets with different start or nominal values.") } };

pub static SUSAN_MATCHCONTINUE_DEBUG: DebugFlag = DebugFlag { index: 164, name: literal!("susanDebug"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Makes Susan generate code using try/else to better debug which function broke the expected match semantics.") } };

pub static OLD_FE_UNITCHECK: DebugFlag = DebugFlag { index: 165, name: literal!("oldFrontEndUnitCheck"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Checks the consistency of units in equation (for the old front-end).") } };

pub static EXEC_STAT_EXTRA_GC: DebugFlag = DebugFlag { index: 166, name: literal!("execstatGCcollect"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("When running execstat, also perform an extra full garbage collection.") } };

pub static DEBUG_DAEMODE: DebugFlag = DebugFlag { index: 167, name: literal!("debugDAEmode"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dump debug output for the DAEmode.") } };

pub static NF_SCALARIZE: DebugFlag = DebugFlag { index: 168, name: literal!("nfScalarize"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Run scalarization in NF, default true.") } };

pub static NF_EVAL_CONST_ARG_FUNCS: DebugFlag = DebugFlag { index: 169, name: literal!("nfEvalConstArgFuncs"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Evaluate all functions with constant arguments in the new frontend.") } };

pub static NF_EXPAND_OPERATIONS: DebugFlag = DebugFlag { index: 170, name: literal!("nfExpandOperations"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Expand all unary/binary operations to scalar expressions in the new frontend.") } };

pub static NF_API: DebugFlag = DebugFlag { index: 171, name: literal!("nfAPI"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables experimental new instantiation use in the OMC API.") } };

pub static NF_API_DYNAMIC_SELECT: DebugFlag = DebugFlag { index: 172, name: literal!("nfAPIDynamicSelect"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Show DynamicSelect(static, dynamic) in annotations. Default to false and will select the first (static) expression") } };

pub static NF_API_NOISE: DebugFlag = DebugFlag { index: 173, name: literal!("nfAPINoise"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables error display for the experimental new instantiation use in the OMC API.") } };

pub static FMI20_DEPENDENCIES: DebugFlag = DebugFlag { index: 174, name: literal!("disableFMIDependency"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables the dependency analysis and generation for FMI 2.0.") } };

pub static WARNING_MINMAX_ATTRIBUTES: DebugFlag = DebugFlag { index: 175, name: literal!("warnMinMax"), default: true, description: Gettext::TranslatableContent::gettext { msgid: literal!("Makes a warning assert from min/max variable attributes instead of error.") } };

pub static NF_EXPAND_FUNC_ARGS: DebugFlag = DebugFlag { index: 176, name: literal!("nfExpandFuncArgs"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Expand all function arguments in the new frontend.") } };

pub static DUMP_JL: DebugFlag = DebugFlag { index: 177, name: literal!("dumpJL"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the absyn representation of a program as a Julia representation") } };

pub static DUMP_ASSC: DebugFlag = DebugFlag { index: 178, name: literal!("dumpASSC"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the conversion process of analytical to structural singularities.") } };

pub static SPLIT_CONSTANT_PARTS_SYMJAC: DebugFlag = DebugFlag { index: 179, name: literal!("symJacConstantSplit"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates all symbolic Jacobians with splitted constant parts.") } };

pub static DUMP_FORCE_FMI_ATTRIBUTES: DebugFlag = DebugFlag { index: 180, name: literal!("force-fmi-attributes"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Force to export all fmi attributes to the modelDescription.xml, including those which have default values") } };

pub static DUMP_DATARECONCILIATION: DebugFlag = DebugFlag { index: 181, name: literal!("dataReconciliation"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps all the dataReconciliation extraction algorithm procedure") } };

pub static ARRAY_CONNECT: DebugFlag = DebugFlag { index: 182, name: literal!("arrayConnect"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Use experimental array connection handler.") } };

pub static COMBINE_SUBSCRIPTS: DebugFlag = DebugFlag { index: 183, name: literal!("combineSubscripts"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Move all subscripts to the end of component references.") } };

pub static ZMQ_LISTEN_TO_ALL: DebugFlag = DebugFlag { index: 184, name: literal!("zmqDangerousAcceptConnectionsFromAnywhere"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("When opening a zmq connection, listen on all interfaces instead of only connections from 127.0.0.1.") } };

pub static DUMP_CONVERSION_RULES: DebugFlag = DebugFlag { index: 185, name: literal!("dumpConversionRules"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps the rules when converting a package using a conversion script.") } };

pub static PRINT_RECORD_TYPES: DebugFlag = DebugFlag { index: 186, name: literal!("printRecordTypes"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out record types as part of the flat code.") } };

pub static DUMP_SIMPLIFY: DebugFlag = DebugFlag { index: 187, name: literal!("dumpSimplify"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps expressions before and after simplification.") } };

pub static DUMP_BACKEND_CLOCKS: DebugFlag = DebugFlag { index: 188, name: literal!("dumpBackendClocks"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps times for each backend module (only new backend).") } };

pub static DUMP_SET_BASED_GRAPHS: DebugFlag = DebugFlag { index: 189, name: literal!("dumpSetBasedGraphs"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about set based graphs for efficient array handling (only new frontend and new backend).") } };

pub static MERGE_COMPONENTS: DebugFlag = DebugFlag { index: 190, name: literal!("mergeComponents"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables automatic merging of components into arrays.") } };

pub static DUMP_SLICE: DebugFlag = DebugFlag { index: 191, name: literal!("dumpSlice"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about the slicing process (pseudo-array causalization).") } };

pub static VECTORIZE_BINDINGS: DebugFlag = DebugFlag { index: 192, name: literal!("vectorizeBindings"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on vectorization of bindings when scalarization is turned off.") } };

pub static DUMP_EVENTS: DebugFlag = DebugFlag { index: 193, name: literal!("dumpEvents"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about the detected event functions.") } };

pub static DUMP_RESIZABLE: DebugFlag = DebugFlag { index: 194, name: literal!("dumpResizable"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about resizable paremeter handling.") } };

pub static DUMP_SOLVE: DebugFlag = DebugFlag { index: 195, name: literal!("dumpSolve"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps information about equation solving.") } };

pub static FORCE_SCALARIZE: DebugFlag = DebugFlag { index: 196, name: literal!("forceScalarize"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Forces scalarization to be done when it would normally be automatically disabled.") } };

pub static DEBUG_ADJOINT: DebugFlag = DebugFlag { index: 197, name: literal!("debugAdjoint"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Dumps debug output for the adjoint differentiation process in the new backend.") } };

pub static FLOW_ALIAS_ELIMINATION: DebugFlag = DebugFlag { index: 198, name: literal!("flowAliasElimination"), default: false, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables simple alias elimination of flow variables in stream connectors.") } };

// CONFIGURATION FLAGS
pub static DEBUG: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 1, name: (literal!("debug")).clone(), shortname: Some((literal!("d")).clone()), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets debug flags. Use --help=debug to see available flags.")).clone() } } });

pub static HELP: ConfigFlag = ConfigFlag { index: 2, name: literal!("help"), shortname: Some(literal!("h")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Displays the help text. Use --help=topics for more information.") } };

pub static RUNNING_TESTSUITE: ConfigFlag = ConfigFlag { index: 3, name: literal!("running-testsuite"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Used when running the testsuite.") } };

pub static SHOW_VERSION: ConfigFlag = ConfigFlag { index: 4, name: literal!("version"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Print the version and exit.") } };

pub static TARGET: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 5, name: (literal!("target")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("gcc")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("gcc")).clone(), (literal!("msvc")).clone(), (literal!("msvc10")).clone(), (literal!("msvc12")).clone(), (literal!("msvc13")).clone(), (literal!("msvc15")).clone(), (literal!("msvc19")).clone(), (literal!("vxworks69")).clone(), (literal!("debugrt")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the target compiler to use.")).clone() } } });

pub static GRAMMAR: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 6, name: (literal!("grammar")).clone(), shortname: Some((literal!("g")).clone()), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: MODELICA.clone(), validValues: list![(literal!("Modelica"), MODELICA.clone()), (literal!("MetaModelica"), METAMODELICA.clone()), (literal!("ParModelica"), PARMODELICA.clone()), (literal!("Optimica"), OPTIMICA.clone()), (literal!("PDEModelica"), PDEMODELICA.clone())] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("Modelica")).clone(), (literal!("MetaModelica")).clone(), (literal!("ParModelica")).clone(), (literal!("Optimica")).clone(), (literal!("PDEModelica")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the grammar and semantics to accept.")).clone() } } });

pub static ANNOTATION_VERSION: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 7, name: (literal!("annotationVersion")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("3.x")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("1.x")).clone(), (literal!("2.x")).clone(), (literal!("3.x")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the annotation version that should be used.")).clone() } } });

pub static LANGUAGE_STANDARD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 8, name: (literal!("std")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 1000, validValues: list![(literal!("1.x"), 10), (literal!("2.x"), 20), (literal!("3.0"), 30), (literal!("3.1"), 31), (literal!("3.2"), 32), (literal!("3.3"), 33), (literal!("3.4"), 34), (literal!("3.5"), 35), (literal!("3.6"), 36), (literal!("latest"), 1000), (literal!("experimental"), 9999)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("1.x")).clone(), (literal!("2.x")).clone(), (literal!("3.1")).clone(), (literal!("3.2")).clone(), (literal!("3.3")).clone(), (literal!("3.4")).clone(), (literal!("3.5")).clone(), (literal!("3.6")).clone(), (literal!("latest")).clone(), (literal!("experimental")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the language standard that should be used.")).clone() } } });

pub static SHOW_ERROR_MESSAGES: ConfigFlag = ConfigFlag { index: 9, name: literal!("showErrorMessages"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Show error messages immediately when they happen.") } };

pub static SHOW_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 10, name: literal!("showAnnotations"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Show annotations in the flattened code.") } };

pub static NO_SIMPLIFY: ConfigFlag = ConfigFlag { index: 11, name: literal!("noSimplify"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Do not simplify expressions if set.") } };

pub static removeSimpleEquationDesc: Gettext::TranslatableContent = Gettext::TranslatableContent::gettext { msgid: literal!("Performs alias elimination and removes constant variables from the DAE, replacing all occurrences of the old variable reference with the new value (constants) or variable reference (alias elimination).") };

pub static PRE_OPT_MODULES: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 12, name: (literal!("preOptModules")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("normalInlineFunction")).clone(), (literal!("evaluateParameters")).clone(), (literal!("simplifyIfEquations")).clone(), (literal!("expandDerOperator")).clone(), (literal!("clockPartitioning")).clone(), (literal!("findStateOrder")).clone(), (literal!("replaceEdgeChange")).clone(), (literal!("inlineArrayEqn")).clone(), (literal!("removeEqualRHS")).clone(), (literal!("removeSimpleEquations")).clone(), (literal!("comSubExp")).clone(), (literal!("resolveLoops")).clone(), (literal!("evalFunc")).clone(), (literal!("encapsulateWhenConditions")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("introduceOutputAliases"), Gettext::TranslatableContent::gettext { msgid: (literal!("Introduces aliases for top-level outputs.")).clone() }), (literal!("clockPartitioning"), Gettext::TranslatableContent::gettext { msgid: (literal!("Does the clock partitioning.")).clone() }), (literal!("collapseArrayExpressions"), collapseArrayExpressionsText.clone()), (literal!("comSubExp"), Gettext::TranslatableContent::gettext { msgid: (literal!("Introduces alias assignments for variables which are assigned to simple terms i.e. a = b/c; d = b/c; --> a=d")).clone() }), (literal!("dumpDAE"), Gettext::TranslatableContent::gettext { msgid: (literal!("dumps the DAE representation of the current transformation state")).clone() }), (literal!("dumpDAEXML"), Gettext::TranslatableContent::gettext { msgid: (literal!("dumps the DAE as xml representation of the current transformation state")).clone() }), (literal!("encapsulateWhenConditions"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module replaces each when condition with a boolean variable.")).clone() }), (literal!("evalFunc"), Gettext::TranslatableContent::gettext { msgid: (literal!("evaluates functions partially")).clone() }), (literal!("evaluateParameters"), Gettext::TranslatableContent::gettext { msgid: (literal!("Evaluates parameters with annotation(Evaluate=true). Use '--evaluateFinalParameters=true' or '--evaluateProtectedParameters=true' to specify additional parameters to be evaluated. Use '--replaceEvaluatedParameters=true' if the evaluated parameters should be replaced in the DAE. To evaluate all parameters in the Frontend use -d=evaluateAllParameters.")).clone() }), (literal!("expandDerOperator"), Gettext::TranslatableContent::notrans { r#str: (literal!("Expands der(expr) using Derive.differentiteExpTime.")).clone() }), (literal!("findStateOrder"), Gettext::TranslatableContent::notrans { r#str: (literal!("Sets derivative information to states.")).clone() }), (literal!("inlineArrayEqn"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module expands all array equations to scalar equations.")).clone() }), (literal!("normalInlineFunction"), Gettext::TranslatableContent::gettext { msgid: (literal!("Perform function inlining for function with annotation Inline=true.")).clone() }), (literal!("inputDerivativesForDynOpt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allowed derivatives of inputs in dyn. optimization.")).clone() }), (literal!("introduceDerAlias"), Gettext::TranslatableContent::notrans { r#str: (literal!("Adds for every der-call an alias equation e.g. dx = der(x).")).clone() }), (literal!("removeEqualRHS"), Gettext::TranslatableContent::notrans { r#str: (literal!("Detects equal expressions of the form a=<exp> and b=<exp> and substitutes them to get speed up.")).clone() }), (literal!("removeProtectedParameters"), Gettext::TranslatableContent::gettext { msgid: (literal!("Replace all parameters with protected=true in the system.")).clone() }), (literal!("removeSimpleEquations"), removeSimpleEquationDesc.clone()), (literal!("removeUnusedParameter"), Gettext::TranslatableContent::gettext { msgid: (literal!("Strips all parameter not present in the equations from the system.")).clone() }), (literal!("removeUnusedVariables"), Gettext::TranslatableContent::gettext { msgid: (literal!("Strips all variables not present in the equations from the system.")).clone() }), (literal!("removeVerySimpleEquations"), Gettext::TranslatableContent::gettext { msgid: (literal!("[Experimental] Like removeSimpleEquations, but less thorough. Note that this always uses the experimental new alias elimination, --removeSimpleEquations=new, which makes it unstable. In particular, MultiBody systems fail to translate correctly. It can be used for simple (but large) systems of equations.")).clone() }), (literal!("replaceEdgeChange"), Gettext::TranslatableContent::gettext { msgid: (literal!("Replace edge(b) = b and not pre(b) and change(b) = v <> pre(v).")).clone() }), (literal!("residualForm"), Gettext::TranslatableContent::gettext { msgid: (literal!("Transforms simple equations x=y to zero-sum equations 0=y-x.")).clone() }), (literal!("resolveLoops"), Gettext::TranslatableContent::gettext { msgid: (literal!("resolves linear equations in loops")).clone() }), (literal!("simplifyAllExpressions"), Gettext::TranslatableContent::notrans { r#str: (literal!("Does simplifications on all expressions.")).clone() }), (literal!("simplifyIfEquations"), Gettext::TranslatableContent::gettext { msgid: (literal!("Tries to simplify if equations by use of information from evaluated parameters.")).clone() }), (literal!("sortEqnsVars"), Gettext::TranslatableContent::notrans { r#str: (literal!("Heuristic sorting for equations and variables.")).clone() }), (literal!("unitChecking"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module is no longer available and its use is deprecated. Use --unitChecking instead.")).clone() }), (literal!("wrapFunctionCalls"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module introduces variables for each function call and substitutes all these calls with the newly introduced variables.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the pre optimization modules to use in the back end. See --help=optmodules for more info.")).clone() } } });

pub static CHEAPMATCHING_ALGORITHM: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 13, name: (literal!("cheapmatchingAlgorithm")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 3 }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("0"), Gettext::TranslatableContent::gettext { msgid: (literal!("No cheap matching.")).clone() }), (literal!("1"), Gettext::TranslatableContent::gettext { msgid: (literal!("Cheap matching, traverses all equations and match the first free variable.")).clone() }), (literal!("3"), Gettext::TranslatableContent::gettext { msgid: (literal!("Random Karp-Sipser: R. M. Karp and M. Sipser. Maximum matching in sparse random graphs.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the cheap matching algorithm to use. A cheap matching algorithm gives a jump start matching by heuristics.")).clone() } } });

pub static MATCHING_ALGORITHM: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 14, name: (literal!("matchingAlgorithm")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("PFPlusExt")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("BFSB"), Gettext::TranslatableContent::gettext { msgid: (literal!("Breadth First Search based algorithm.")).clone() }), (literal!("DFSB"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based algorithm.")).clone() }), (literal!("MC21A"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based algorithm with look ahead feature.")).clone() }), (literal!("PF"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based algorithm with look ahead feature.")).clone() }), (literal!("PFPlus"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based algorithm with look ahead feature and fair row traversal.")).clone() }), (literal!("HK"), Gettext::TranslatableContent::gettext { msgid: (literal!("Combined BFS and DFS algorithm.")).clone() }), (literal!("HKDW"), Gettext::TranslatableContent::gettext { msgid: (literal!("Combined BFS and DFS algorithm.")).clone() }), (literal!("ABMP"), Gettext::TranslatableContent::gettext { msgid: (literal!("Combined BFS and DFS algorithm.")).clone() }), (literal!("PR"), Gettext::TranslatableContent::gettext { msgid: (literal!("Matching algorithm using push relabel mechanism.")).clone() }), (literal!("DFSBExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based Algorithm external c implementation.")).clone() }), (literal!("BFSBExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Breadth First Search based Algorithm external c implementation.")).clone() }), (literal!("MC21AExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based Algorithm with look ahead feature external c implementation.")).clone() }), (literal!("PFExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based Algorithm with look ahead feature external c implementation.")).clone() }), (literal!("PFPlusExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Depth First Search based Algorithm with look ahead feature and fair row traversal external c implementation.")).clone() }), (literal!("HKExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Combined BFS and DFS algorithm external c implementation.")).clone() }), (literal!("HKDWExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Combined BFS and DFS algorithm external c implementation.")).clone() }), (literal!("ABMPExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Combined BFS and DFS algorithm external c implementation.")).clone() }), (literal!("PRExt"), Gettext::TranslatableContent::gettext { msgid: (literal!("Matching algorithm using push relabel mechanism external c implementation.")).clone() }), (literal!("BB"), Gettext::TranslatableContent::gettext { msgid: (literal!("BBs try.")).clone() }), (literal!("SBGraph"), Gettext::TranslatableContent::gettext { msgid: (literal!("Set-Based Graph matching algorithm for efficient array handling.")).clone() }), (literal!("pseudo"), Gettext::TranslatableContent::gettext { msgid: (literal!("Pseudo array matching that uses scalar matching and reconstructs arrays afterwards as much as possible.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the matching algorithm to use. See --help=optmodules for more info.")).clone() } } });

pub static INDEX_REDUCTION_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 15, name: (literal!("indexReductionMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("dynamicStateSelection")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Skip index reduction")).clone() }), (literal!("uode"), Gettext::TranslatableContent::gettext { msgid: (literal!("Use the underlying ODE without the constraints.")).clone() }), (literal!("dynamicStateSelection"), Gettext::TranslatableContent::gettext { msgid: (literal!("Simple index reduction method, select (dynamic) dummy states based on analysis of the system.")).clone() }), (literal!("dummyDerivatives"), Gettext::TranslatableContent::gettext { msgid: (literal!("Simple index reduction method, select (static) dummy states based on heuristic.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the index reduction method to use. See --help=optmodules for more info.")).clone() } } });

pub static POST_OPT_MODULES: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 16, name: (literal!("postOptModules")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("lateInlineFunction")).clone(), (literal!("wrapFunctionCalls")).clone(), (literal!("inlineArrayEqn")).clone(), (literal!("constantLinearSystem")).clone(), (literal!("simplifysemiLinear")).clone(), (literal!("removeSimpleEquations")).clone(), (literal!("simplifyComplexFunction")).clone(), (literal!("solveSimpleEquations")).clone(), (literal!("tearingSystem")).clone(), (literal!("inputDerivativesUsed")).clone(), (literal!("calculateStrongComponentJacobians")).clone(), (literal!("calculateStateSetsJacobians")).clone(), (literal!("symbolicJacobian")).clone(), (literal!("removeConstants")).clone(), (literal!("simplifyTimeIndepFuncCalls")).clone(), (literal!("simplifyAllExpressions")).clone(), (literal!("findZeroCrossings")).clone(), (literal!("collapseArrayExpressions")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("addScaledVars_states"), Gettext::TranslatableContent::notrans { r#str: (literal!("added var_norm = var/nominal, where var is state")).clone() }), (literal!("addScaledVars_inputs"), Gettext::TranslatableContent::notrans { r#str: (literal!("added var_norm = var/nominal, where var is input")).clone() }), (literal!("addTimeAsState"), Gettext::TranslatableContent::gettext { msgid: (literal!("Experimental feature: this replaces each occurrence of variable time with a new introduced state $time with equation der($time) = 1.0")).clone() }), (literal!("calculateStateSetsJacobians"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates analytical jacobian for dynamic state selection sets.")).clone() }), (literal!("calculateStrongComponentJacobians"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates analytical jacobian for torn linear and non-linear strong components. By default linear components and non-linear components with user-defined function calls are skipped. See also debug flags: LSanalyticJacobian, NLSanalyticJacobian and forceNLSanalyticJacobian")).clone() }), (literal!("collapseArrayExpressions"), collapseArrayExpressionsText.clone()), (literal!("constantLinearSystem"), Gettext::TranslatableContent::gettext { msgid: (literal!("Evaluates constant linear systems (a*x+b*y=c; d*x+e*y=f; a,b,c,d,e,f are constants) at compile-time.")).clone() }), (literal!("countOperations"), Gettext::TranslatableContent::gettext { msgid: (literal!("Count the mathematical operations of the system.")).clone() }), (literal!("cseBinary"), Gettext::TranslatableContent::gettext { msgid: (literal!("Common Sub-expression Elimination")).clone() }), (literal!("dumpComponentsGraphStr"), Gettext::TranslatableContent::notrans { r#str: (literal!("Dumps the assignment graph used to determine strong components to format suitable for Mathematica")).clone() }), (literal!("dumpDAE"), Gettext::TranslatableContent::gettext { msgid: (literal!("dumps the DAE representation of the current transformation state")).clone() }), (literal!("dumpDAEXML"), Gettext::TranslatableContent::gettext { msgid: (literal!("dumps the DAE as xml representation of the current transformation state")).clone() }), (literal!("evaluateParameters"), Gettext::TranslatableContent::gettext { msgid: (literal!("Evaluates parameters with annotation(Evaluate=true). Use '--evaluateFinalParameters=true' or '--evaluateProtectedParameters=true' to specify additional parameters to be evaluated. Use '--replaceEvaluatedParameters=true' if the evaluated parameters should be replaced in the DAE. To evaluate all parameters in the Frontend use -d=evaluateAllParameters.")).clone() }), (literal!("extendDynamicOptimization"), Gettext::TranslatableContent::gettext { msgid: (literal!("Move loops to constraints.")).clone() }), (literal!("generateSymbolicLinearization"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates symbolic linearization matrices A,B,C,D for linear model:\n\t:math:`\\dot{x} = Ax + Bu`\n\t:math:`y = Cx + Du`")).clone() }), (literal!("generateSymbolicSensitivities"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates symbolic Sensivities matrix, where der(x) is differentiated w.r.t. param.")).clone() }), (literal!("inlineArrayEqn"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module expands all array equations to scalar equations.")).clone() }), (literal!("inputDerivativesUsed"), Gettext::TranslatableContent::gettext { msgid: (literal!("Checks if derivatives of inputs are need to calculate the model.")).clone() }), (literal!("lateInlineFunction"), Gettext::TranslatableContent::gettext { msgid: (literal!("Perform function inlining for function with annotation LateInline=true.")).clone() }), (literal!("partlintornsystem"), Gettext::TranslatableContent::notrans { r#str: (literal!("partitions linear torn systems.")).clone() }), (literal!("recursiveTearing"), Gettext::TranslatableContent::notrans { r#str: (literal!("inline and repeat tearing")).clone() }), (literal!("reduceDynamicOptimization"), Gettext::TranslatableContent::notrans { r#str: (literal!("Removes equations which are not needed for the calculations of cost and constraints. This module requires --postOptModules+=reduceDynamicOptimization.")).clone() }), (literal!("relaxSystem"), Gettext::TranslatableContent::notrans { r#str: (literal!("relaxation from gausian elemination")).clone() }), (literal!("removeConstants"), Gettext::TranslatableContent::gettext { msgid: (literal!("Remove all constants in the system.")).clone() }), (literal!("removeEqualRHS"), Gettext::TranslatableContent::notrans { r#str: (literal!("Detects equal function calls of the form a=f(b) and c=f(b) and substitutes them to get speed up.")).clone() }), (literal!("removeSimpleEquations"), removeSimpleEquationDesc.clone()), (literal!("removeUnusedParameter"), Gettext::TranslatableContent::gettext { msgid: (literal!("Strips all parameter not present in the equations from the system to get speed up for compilation of target code.")).clone() }), (literal!("removeUnusedVariables"), Gettext::TranslatableContent::notrans { r#str: (literal!("Strips all variables not present in the equations from the system to get speed up for compilation of target code.")).clone() }), (literal!("reshufflePost"), Gettext::TranslatableContent::gettext { msgid: (literal!("Reshuffles algebraic loops.")).clone() }), (literal!("simplifyAllExpressions"), Gettext::TranslatableContent::notrans { r#str: (literal!("Does simplifications on all expressions.")).clone() }), (literal!("simplifyComplexFunction"), Gettext::TranslatableContent::notrans { r#str: (literal!("Some simplifications on complex functions (complex refers to the internal data structure)")).clone() }), (literal!("simplifyConstraints"), Gettext::TranslatableContent::notrans { r#str: (literal!("Rewrites nonlinear constraints into box constraints if possible. This module requires +gDynOpt.")).clone() }), (literal!("simplifyLoops"), Gettext::TranslatableContent::notrans { r#str: (literal!("Simplifies algebraic loops. This modules requires +simplifyLoops.")).clone() }), (literal!("simplifyTimeIndepFuncCalls"), Gettext::TranslatableContent::gettext { msgid: (literal!("Simplifies time independent built in function calls like pre(param) -> param, der(param) -> 0.0, change(param) -> false, edge(param) -> false.")).clone() }), (literal!("simplifysemiLinear"), Gettext::TranslatableContent::gettext { msgid: (literal!("Simplifies calls to semiLinear.")).clone() }), (literal!("solveLinearSystem"), Gettext::TranslatableContent::notrans { r#str: (literal!("solve linear system with newton step")).clone() }), (literal!("solveSimpleEquations"), Gettext::TranslatableContent::notrans { r#str: (literal!("Solves simple equations")).clone() }), (literal!("symSolver"), Gettext::TranslatableContent::notrans { r#str: (literal!("Rewrites the ode system for implicit Euler method. This module requires +symSolver.")).clone() }), (literal!("symbolicJacobian"), Gettext::TranslatableContent::notrans { r#str: (literal!("Detects the sparse pattern of the ODE system and calculates also the symbolic Jacobian if flag '--generateDynamicJacobian=symbolic'.")).clone() }), (literal!("tearingSystem"), Gettext::TranslatableContent::notrans { r#str: (literal!("For method selection use flag tearingMethod.")).clone() }), (literal!("wrapFunctionCalls"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module introduces variables for each function call and substitutes all these calls with the newly introduced variables.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the post optimization modules to use in the back end. See --help=optmodules for more info.")).clone() } } });

pub static SIMCODE_TARGET: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 17, name: (literal!("simCodeTarget")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("C")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("None")).clone(), (literal!("C")).clone(), (literal!("Cpp")).clone(), (literal!("omsicpp")).clone(), (literal!("ExperimentalEmbeddedC")).clone(), (literal!("JavaScript")).clone(), (literal!("omsic")).clone(), (literal!("XML")).clone(), (literal!("MidC")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the target language for the code generation.")).clone() } } });

pub static ORDER_CONNECTIONS: ConfigFlag = ConfigFlag { index: 18, name: literal!("orderConnections"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Orders connect equations alphabetically if set.") } };

pub static TYPE_INFO: ConfigFlag = ConfigFlag { index: 19, name: literal!("typeinfo"), shortname: Some(literal!("t")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prints out extra type information if set.") } };

pub static KEEP_ARRAYS: ConfigFlag = ConfigFlag { index: 20, name: literal!("keepArrays"), shortname: Some(literal!("a")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets whether to split arrays or not.") } };

pub static MODELICA_OUTPUT: ConfigFlag = ConfigFlag { index: 21, name: literal!("modelicaOutput"), shortname: Some(literal!("m")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables valid modelica output for flat modelica.") } };

pub static SILENT: ConfigFlag = ConfigFlag { index: 22, name: literal!("silent"), shortname: Some(literal!("q")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on silent mode.") } };

pub static CORBA_SESSION: ConfigFlag = ConfigFlag { index: 23, name: literal!("corbaSessionName"), shortname: Some(literal!("c")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the name of the corba session if -d=interactiveCorba or --interactive=corba is used.") } };

pub static NUM_PROC: ConfigFlag = ConfigFlag { index: 24, name: literal!("numProcs"), shortname: Some(literal!("n")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the number of processors to use (0=default=auto).") } };

pub static INST_CLASS: ConfigFlag = ConfigFlag { index: 25, name: literal!("instClass"), shortname: Some(literal!("i")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Instantiate the class given by the fully qualified path.") } };

pub static VECTORIZATION_LIMIT: ConfigFlag = ConfigFlag { index: 26, name: literal!("vectorizationLimit"), shortname: Some(literal!("v")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the vectorization limit, arrays and matrices larger than this will not be vectorized.") } };

pub static SIMULATION_CG: ConfigFlag = ConfigFlag { index: 27, name: literal!("simulationCg"), shortname: Some(literal!("s")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on simulation code generation.") } };

pub static EVAL_PARAMS_IN_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 28, name: literal!("evalAnnotationParams"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets whether to evaluate parameters in annotations or not.") } };

pub static CHECK_MODEL: ConfigFlag = ConfigFlag { index: 29, name: literal!("checkModel"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Set when checkModel is used to turn on specific features for checking.") } };

pub static CEVAL_EQUATION: ConfigFlag = ConfigFlag { index: 30, name: literal!("cevalEquation"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::notrans { r#str: literal!("") } };

pub static UNIT_CHECKING: ConfigFlag = ConfigFlag { index: 31, name: literal!("unitChecking"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::notrans { r#str: literal!("Enable unit checking.") } };

pub static GENERATE_LABELED_SIMCODE: ConfigFlag = ConfigFlag { index: 32, name: literal!("generateLabeledSimCode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on labeled SimCode generation for reduction algorithms.") } };

pub static REDUCE_TERMS: ConfigFlag = ConfigFlag { index: 33, name: literal!("reduceTerms"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on reducing terms for reduction algorithms.") } };

pub static REDUCTION_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 34, name: (literal!("reductionMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("deletion")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("deletion")).clone(), (literal!("substitution")).clone(), (literal!("linearization")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the reduction method to be used.")).clone() } } });

pub static DEMO_MODE: ConfigFlag = ConfigFlag { index: 35, name: literal!("demoMode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disable Warning/Error Massages.") } };

pub static LOCALE_FLAG: ConfigFlag = ConfigFlag { index: 36, name: literal!("locale"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Override the locale from the environment.") } };

pub static DEFAULT_OPENCL_DEVICE: ConfigFlag = ConfigFlag { index: 37, name: literal!("defaultOCLDevice"), shortname: Some(literal!("o")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the default OpenCL device to be used for parallel execution.") } };

pub static MAXTRAVERSALS: ConfigFlag = ConfigFlag { index: 38, name: literal!("maxTraversals"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 2 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Maximal traversals to find simple equations in the acausal system.") } };

pub static DUMP_TARGET: ConfigFlag = ConfigFlag { index: 39, name: literal!("dumpTarget"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Redirect the dump to file. If the file ends with .html HTML code is generated.") } };

pub static DELAY_BREAK_LOOP: ConfigFlag = ConfigFlag { index: 40, name: literal!("delayBreakLoop"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables (very) experimental code to break algebraic loops using the delay() operator. Probably messes with initialization.") } };

pub static TEARING_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 41, name: (literal!("tearingMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("cellier")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("noTearing"), Gettext::TranslatableContent::gettext { msgid: (literal!("Deprecated, use minimalTearing.")).clone() }), (literal!("minimalTearing"), Gettext::TranslatableContent::gettext { msgid: (literal!("Minimal tearing method to only tear discrete variables.")).clone() }), (literal!("omcTearing"), Gettext::TranslatableContent::gettext { msgid: (literal!("Tearing method developed by TU Dresden: Frenkel, Schubert.")).clone() }), (literal!("cellier"), Gettext::TranslatableContent::gettext { msgid: (literal!("Tearing based on Celliers method, revised by FH Bielefeld: Täuber, Patrick")).clone() }), (literal!("guruTearing"), Gettext::TranslatableContent::gettext { msgid: (literal!("Tearing based solely on TearingSelect annotation. Forces prefer/always variables to be iteration variables.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the tearing method to use. Select no tearing or choose tearing method.")).clone() } } });

pub static TEARING_HEURISTIC: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 42, name: (literal!("tearingHeuristic")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("MC3")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("MC1"), Gettext::TranslatableContent::gettext { msgid: (literal!("Original cellier with consideration of impossible assignments and discrete Vars.")).clone() }), (literal!("MC2"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified cellier, drop first step.")).clone() }), (literal!("MC11"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC1, new last step 'count impossible assignments'.")).clone() }), (literal!("MC21"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC2, new last step 'count impossible assignments'.")).clone() }), (literal!("MC12"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC1, step 'count impossible assignments' before last step.")).clone() }), (literal!("MC22"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC2, step 'count impossible assignments' before last step.")).clone() }), (literal!("MC13"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC1, build sum of impossible assignment and causalizable equations, choose var with biggest sum.")).clone() }), (literal!("MC23"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC2, build sum of impossible assignment and causalizable equations, choose var with biggest sum.")).clone() }), (literal!("MC231"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified MC23, Two rounds, choose better potentials-set.")).clone() }), (literal!("MC3"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified cellier, build sum of impossible assignment and causalizable equations for all vars, choose var with biggest sum.")).clone() }), (literal!("MC4"), Gettext::TranslatableContent::gettext { msgid: (literal!("Modified cellier, use all heuristics, choose var that occurs most in potential sets")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the tearing heuristic to use for Cellier-tearing.")).clone() } } });

pub static SCALARIZE_MINMAX: ConfigFlag = ConfigFlag { index: 43, name: literal!("scalarizeMinMax"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Scalarizes the builtin min/max reduction operators if true.") } };

pub static STRICT: ConfigFlag = ConfigFlag { index: 44, name: literal!("strict"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables stricter enforcement of Modelica language rules.") } };

pub static SCALARIZE_BINDINGS: ConfigFlag = ConfigFlag { index: 45, name: literal!("scalarizeBindings"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Always scalarizes bindings if set.") } };

pub static CORBA_OBJECT_REFERENCE_FILE_PATH: ConfigFlag = ConfigFlag { index: 46, name: literal!("corbaObjectReferenceFilePath"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the path for corba object reference file if -d=interactiveCorba is used.") } };

pub static HPCOM_SCHEDULER: ConfigFlag = ConfigFlag { index: 47, name: literal!("hpcomScheduler"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("level") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the scheduler for task graph scheduling (list | listr | level | levelfix | ext | metis | mcp | taskdep | tds | bls | rand | none). Default: level.") } };

pub static HPCOM_CODE: ConfigFlag = ConfigFlag { index: 48, name: literal!("hpcomCode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("openmp") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the code-type produced by hpcom (openmp | pthreads | pthreads_spin | tbb | mpi). Default: openmp.") } };

pub static REWRITE_RULES_FILE: ConfigFlag = ConfigFlag { index: 49, name: literal!("rewriteRulesFile"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Activates user given rewrite rules for Absyn expressions. The rules are read from the given file and are of the form rewrite(fromExp, toExp);") } };

pub static REPLACE_HOMOTOPY: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 50, name: (literal!("replaceHomotopy")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Default, do not replace homotopy.")).clone() }), (literal!("actual"), Gettext::TranslatableContent::gettext { msgid: (literal!("Replace homotopy(actual, simplified) with actual.")).clone() }), (literal!("simplified"), Gettext::TranslatableContent::gettext { msgid: (literal!("Replace homotopy(actual, simplified) with simplified.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Replaces homotopy(actual, simplified) with the actual expression or the simplified expression. Good for debugging models which use homotopy. The default is to not replace homotopy.")).clone() } } });

pub static GENERATE_DYNAMIC_JACOBIAN: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 51, name: (literal!("generateDynamicJacobian")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("numeric")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Does not generate Jacobian. For use with explicit solvers.")).clone() }), (literal!("numeric"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates sparsity pattern for numeric Jacobian.")).clone() }), (literal!("symbolic"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates symbolic Jacobian. Used by dassl or ida solver with simulation flag '-jacobian'.")).clone() }), (literal!("symbolicadjoint"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates adjoint Jacobian symbolically.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Select how Jacobian matrix is generated, where der(x) is differentiated w.r.t. x.")).clone() } } });

pub static GENERATE_SYMBOLIC_LINEARIZATION: ConfigFlag = ConfigFlag { index: 52, name: literal!("generateSymbolicLinearization"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates symbolic linearization matrices A,B,C,D for linear model:\n\t\t:math:`\\dot{x} = Ax + Bu`\n\t\t:math:`y = Cx + Du`") } };

pub static INT_ENUM_CONVERSION: ConfigFlag = ConfigFlag { index: 53, name: literal!("intEnumConversion"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Allow Integer to enumeration conversion.") } };

pub static PROFILING_LEVEL: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 54, name: (literal!("profiling")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate code without profiling")).clone() }), (literal!("blocks"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate code for profiling function calls as well as linear and non-linear systems of equations")).clone() }), (literal!("blocks+html"), Gettext::TranslatableContent::gettext { msgid: (literal!("Like blocks, but also run xsltproc and gnuplot to generate an html report")).clone() }), (literal!("all"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate code for profiling of all functions and equations")).clone() }), (literal!("all_perf"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate code for profiling of all functions and equations with additional performance data using the papi-interface (cpp-runtime)")).clone() }), (literal!("all_stat"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate code for profiling of all functions and equations with additional statistics (cpp-runtime)")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the profiling level to use. Profiled equations and functions record execution time and count for each time step taken by the integrator.")).clone() } } });

pub static RESHUFFLE: ConfigFlag = ConfigFlag { index: 55, name: literal!("reshuffle"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 1 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("sets tolerance of reshuffling algorithm: 1: conservative, 2: more tolerant, 3 resolve all") } };

pub static GENERATE_DYN_OPTIMIZATION_PROBLEM: ConfigFlag = ConfigFlag { index: 56, name: literal!("gDynOpt"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generate dynamic optimization problem based on annotation approach.") } };

pub static MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM: ConfigFlag = ConfigFlag { index: 57, name: literal!("maxSizeSolveLinearSystem"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Max size for solveLinearSystem.") } };

pub static CPP_FLAGS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 58, name: (literal!("cppFlags")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("")).clone()] }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets extra flags for compilation with the C++ compiler (e.g. +cppFlags=-O3,-Wall)")).clone() } } });

pub static REMOVE_SIMPLE_EQUATIONS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 59, name: (literal!("removeSimpleEquations")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("default")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Disables module")).clone() }), (literal!("default"), Gettext::TranslatableContent::gettext { msgid: (literal!("Performs alias elimination and removes constant variables. Default case uses in preOpt phase the fastAcausal and in postOpt phase the causal implementation.")).clone() }), (literal!("causal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Performs alias elimination and removes constant variables. Causal implementation.")).clone() }), (literal!("fastAcausal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Performs alias elimination and removes constant variables. fastImplementation fastAcausal.")).clone() }), (literal!("allAcausal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Performs alias elimination and removes constant variables. Implementation allAcausal.")).clone() }), (literal!("new"), Gettext::TranslatableContent::gettext { msgid: (literal!("New implementation (experimental)")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Specifies method that removes simple equations.")).clone() } } });

pub static DYNAMIC_TEARING: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 60, name: (literal!("dynamicTearing")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("false")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("false"), Gettext::TranslatableContent::gettext { msgid: (literal!("No dynamic tearing.")).clone() }), (literal!("true"), Gettext::TranslatableContent::gettext { msgid: (literal!("Dynamic tearing for linear and nonlinear systems.")).clone() }), (literal!("linear"), Gettext::TranslatableContent::gettext { msgid: (literal!("Dynamic tearing only for linear systems.")).clone() }), (literal!("nonlinear"), Gettext::TranslatableContent::gettext { msgid: (literal!("Dynamic tearing only for nonlinear systems.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Activates dynamic tearing (TearingSet can be changed automatically during runtime, strict set vs. casual set.)")).clone() } } });

pub static SYM_SOLVER: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 61, name: (literal!("symSolver")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 0, validValues: list![(literal!("none"), 0), (literal!("impEuler"), 1), (literal!("expEuler"), 2)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("none")).clone(), (literal!("impEuler")).clone(), (literal!("expEuler")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Activates symbolic implicit solver (original system is not changed).")).clone() } } });

pub static LOOP2CON: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 62, name: (literal!("loop2con")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Disables module")).clone() }), (literal!("lin"), Gettext::TranslatableContent::gettext { msgid: (literal!("linear loops --> constraints")).clone() }), (literal!("noLin"), Gettext::TranslatableContent::gettext { msgid: (literal!("no linear loops --> constraints")).clone() }), (literal!("all"), Gettext::TranslatableContent::gettext { msgid: (literal!("loops --> constraints")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Specifies method that transform loops in constraints. hint: using initial guess from file!")).clone() } } });

pub static FORCE_TEARING: ConfigFlag = ConfigFlag { index: 63, name: literal!("forceTearing"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Use tearing set even if it is not smaller than the original component.") } };

pub static SIMPLIFY_LOOPS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 64, name: (literal!("simplifyLoops")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("0"), Gettext::TranslatableContent::gettext { msgid: (literal!("do nothing")).clone() }), (literal!("1"), Gettext::TranslatableContent::gettext { msgid: (literal!("special modification of residual expressions")).clone() }), (literal!("2"), Gettext::TranslatableContent::gettext { msgid: (literal!("special modification of residual expressions with helper variables")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Simplify algebraic loops.")).clone() } } });

pub static RTEARING: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 65, name: (literal!("recursiveTearing")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("0"), Gettext::TranslatableContent::gettext { msgid: (literal!("do nothing")).clone() }), (literal!("1"), Gettext::TranslatableContent::gettext { msgid: (literal!("linear tearing set of size 1")).clone() }), (literal!("2"), Gettext::TranslatableContent::gettext { msgid: (literal!("linear tearing")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Inline and repeat tearing.")).clone() } } });

pub static FLOW_THRESHOLD: ConfigFlag = ConfigFlag { index: 66, name: literal!("flowThreshold"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::REAL_FLAG { data: metamodelica::OrderedFloat(1e-7_f64) }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the minium threshold for stream flow rates") } };

pub static MATRIX_FORMAT: ConfigFlag = ConfigFlag { index: 67, name: literal!("matrixFormat"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("dense") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the matrix format type in cpp runtime which should be used (dense | sparse ). Default: dense.") } };

pub static PARTLINTORN: ConfigFlag = ConfigFlag { index: 68, name: literal!("partlintorn"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the limit for partitionin of linear torn systems.") } };

pub static INIT_OPT_MODULES: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 69, name: (literal!("initOptModules")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("simplifyComplexFunction")).clone(), (literal!("tearingSystem")).clone(), (literal!("solveSimpleEquations")).clone(), (literal!("calculateStrongComponentJacobians")).clone(), (literal!("simplifyAllExpressions")).clone(), (literal!("collapseArrayExpressions")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("calculateStrongComponentJacobians"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generates analytical jacobian for torn linear and non-linear strong components. By default linear components and non-linear components with user-defined function calls are skipped. See also debug flags: LSanalyticJacobian, NLSanalyticJacobian and forceNLSanalyticJacobian")).clone() }), (literal!("collapseArrayExpressions"), collapseArrayExpressionsText.clone()), (literal!("inlineArrayEqn"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module expands all array equations to scalar equations.")).clone() }), (literal!("constantLinearSystem"), Gettext::TranslatableContent::gettext { msgid: (literal!("Evaluates constant linear systems (a*x+b*y=c; d*x+e*y=f; a,b,c,d,e,f are constants) at compile-time.")).clone() }), (literal!("extendDynamicOptimization"), Gettext::TranslatableContent::gettext { msgid: (literal!("Move loops to constraints.")).clone() }), (literal!("generateHomotopyComponents"), Gettext::TranslatableContent::gettext { msgid: (literal!("Finds the parts of the DAE that have to be handled by the homotopy solver and creates a strong component out of it.")).clone() }), (literal!("inlineHomotopy"), Gettext::TranslatableContent::gettext { msgid: (literal!("Experimental: Inlines the homotopy expression to allow symbolic simplifications.")).clone() }), (literal!("inputDerivativesUsed"), Gettext::TranslatableContent::gettext { msgid: (literal!("Checks if derivatives of inputs are need to calculate the model.")).clone() }), (literal!("recursiveTearing"), Gettext::TranslatableContent::notrans { r#str: (literal!("inline and repeat tearing")).clone() }), (literal!("reduceDynamicOptimization"), Gettext::TranslatableContent::notrans { r#str: (literal!("Removes equations which are not needed for the calculations of cost and constraints. This module requires --postOptModules+=reduceDynamicOptimization.")).clone() }), (literal!("replaceHomotopyWithSimplified"), Gettext::TranslatableContent::notrans { r#str: (literal!("Replaces the homotopy expression homotopy(actual, simplified) with the simplified part.")).clone() }), (literal!("simplifyAllExpressions"), Gettext::TranslatableContent::notrans { r#str: (literal!("Does simplifications on all expressions.")).clone() }), (literal!("simplifyComplexFunction"), Gettext::TranslatableContent::notrans { r#str: (literal!("Some simplifications on complex functions (complex refers to the internal data structure)")).clone() }), (literal!("simplifyConstraints"), Gettext::TranslatableContent::notrans { r#str: (literal!("Rewrites nonlinear constraints into box constraints if possible. This module requires +gDynOpt.")).clone() }), (literal!("simplifyLoops"), Gettext::TranslatableContent::notrans { r#str: (literal!("Simplifies algebraic loops. This modules requires +simplifyLoops.")).clone() }), (literal!("solveSimpleEquations"), Gettext::TranslatableContent::notrans { r#str: (literal!("Solves simple equations")).clone() }), (literal!("tearingSystem"), Gettext::TranslatableContent::notrans { r#str: (literal!("For method selection use flag tearingMethod.")).clone() }), (literal!("wrapFunctionCalls"), Gettext::TranslatableContent::gettext { msgid: (literal!("This module introduces variables for each function call and substitutes all these calls with the newly introduced variables.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the initialization optimization modules to use in the back end. See --help=optmodules for more info.")).clone() } } });

pub static MAX_MIXED_DETERMINED_INDEX: ConfigFlag = ConfigFlag { index: 70, name: literal!("maxMixedDeterminedIndex"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 10 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the maximum mixed-determined index that is handled by the initialization.") } };

pub static USE_LOCAL_DIRECTION: ConfigFlag = ConfigFlag { index: 71, name: literal!("useLocalDirection"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Keeps the input/output prefix for all variables in the flat model, not only top-level ones.") } };

pub static DEFAULT_OPT_MODULES_ORDERING: ConfigFlag = ConfigFlag { index: 72, name: literal!("defaultOptModulesOrdering"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("If this is activated, then the specified pre-/post-/init-optimization modules will be rearranged to the recommended ordering.") } };

pub static PRE_OPT_MODULES_ADD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 73, name: (literal!("preOptModules+")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Enables additional pre-optimization modules, e.g. --preOptModules+=module1,module2 would additionally enable module1 and module2. See --help=optmodules for more info.")).clone() } } });

pub static PRE_OPT_MODULES_SUB: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 74, name: (literal!("preOptModules-")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Disables a list of pre-optimization modules, e.g. --preOptModules-=module1,module2 would disable module1 and module2. See --help=optmodules for more info.")).clone() } } });

pub static POST_OPT_MODULES_ADD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 75, name: (literal!("postOptModules+")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Enables additional post-optimization modules, e.g. --postOptModules+=module1,module2 would additionally enable module1 and module2. See --help=optmodules for more info.")).clone() } } });

pub static POST_OPT_MODULES_SUB: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 76, name: (literal!("postOptModules-")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Disables a list of post-optimization modules, e.g. --postOptModules-=module1,module2 would disable module1 and module2. See --help=optmodules for more info.")).clone() } } });

pub static INIT_OPT_MODULES_ADD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 77, name: (literal!("initOptModules+")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Enables additional init-optimization modules, e.g. --initOptModules+=module1,module2 would additionally enable module1 and module2. See --help=optmodules for more info.")).clone() } } });

pub static INIT_OPT_MODULES_SUB: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 78, name: (literal!("initOptModules-")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Disables a list of init-optimization modules, e.g. --initOptModules-=module1,module2 would disable module1 and module2. See --help=optmodules for more info.")).clone() } } });

pub static PERMISSIVE: ConfigFlag = ConfigFlag { index: 79, name: literal!("permissive"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables some error checks to allow erroneous models to compile.") } };

pub static HETS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 80, name: (literal!("hets")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("do nothing")).clone() }), (literal!("derCalls"), Gettext::TranslatableContent::gettext { msgid: (literal!("sort terms based on der-calls")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Heuristic equation terms sort")).clone() } } });

pub static DEFAULT_CLOCK_PERIOD: ConfigFlag = ConfigFlag { index: 81, name: literal!("defaultClockPeriod"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::REAL_FLAG { data: metamodelica::OrderedFloat(1.0_f64) }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the default clock period (in seconds) for state machines (default: 1.0).") } };

pub static INST_CACHE_SIZE: ConfigFlag = ConfigFlag { index: 82, name: literal!("instCacheSize"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 25343 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the size of the internal hash table used for instantiation caching.") } };

pub static MAX_SIZE_LINEAR_TEARING: ConfigFlag = ConfigFlag { index: 83, name: literal!("maxSizeLinearTearing"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 200 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the maximum system size for tearing of linear systems (default 200).") } };

pub static MAX_SIZE_NONLINEAR_TEARING: ConfigFlag = ConfigFlag { index: 84, name: literal!("maxSizeNonlinearTearing"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 10000 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the maximum system size for tearing of nonlinear systems (default 10000).") } };

pub static NO_TEARING_FOR_COMPONENT: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 85, name: (literal!("noTearingForComponent")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Deactivates tearing for the specified components.\nUse '-d=tearingdump' to find out the relevant indexes.")).clone() } } });

pub static CT_STATE_MACHINES: ConfigFlag = ConfigFlag { index: 86, name: literal!("ctStateMachines"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Experimental: Enable continuous-time state machine prototype") } };

pub static DAE_MODE: ConfigFlag = ConfigFlag { index: 87, name: literal!("daeMode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates code to simulate models in DAE mode. The whole system is passed directly to the DAE solver SUNDIALS/IDA and no algebraic solver is involved in the simulation process.") } };

pub static INLINE_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 88, name: (literal!("inlineMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 1, validValues: list![(literal!("replace"), 1), (literal!("append"), 2)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("replace")).clone(), (literal!("append")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sets the inline method to use.\n")); __mm_s.push_str(&*literal!("replace : This method inlines by replacing in place all expressions. Might lead to very long expression.\n")); __mm_s.push_str(&*literal!("append  : This method inlines by adding additional variables to the whole system. Might lead to much bigger system.")); ArcStr::from(__mm_s) }).clone() } } });

pub static SET_TEARING_VARS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 89, name: (literal!("setTearingVars")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the tearing variables by its strong component indexes. Use '-d=tearingdump' to find out the relevant indexes.\nUse following format: '--setTearingVars=(sci,n,t1,...,tn)*', with sci = strong component index, n = number of tearing variables, t1,...tn = tearing variables.\nE.g.: '--setTearingVars=4,2,3,5' would select variables 3 and 5 in strong component 4.")).clone() } } });

pub static SET_RESIDUAL_EQNS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 90, name: (literal!("setResidualEqns")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the residual equations by its strong component indexes. Use '-d=tearingdump' to find out the relevant indexes for the collective equations.\nUse following format: '--setResidualEqns=(sci,n,r1,...,rn)*', with sci = strong component index, n = number of residual equations, r1,...rn = residual equations.\nE.g.: '--setResidualEqns=4,2,3,5' would select equations 3 and 5 in strong component 4.\nOnly works in combination with 'setTearingVars'.")).clone() } } });

pub static IGNORE_COMMAND_LINE_OPTIONS_ANNOTATION: ConfigFlag = ConfigFlag { index: 91, name: literal!("ignoreCommandLineOptionsAnnotation"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Ignores the command line options specified as annotation in the class.") } };

pub static CALCULATE_SENSITIVITIES: ConfigFlag = ConfigFlag { index: 92, name: literal!("calculateSensitivities"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generates sensitivities variables and matrices.") } };

pub static ALARM: ConfigFlag = ConfigFlag { index: 93, name: literal!("alarm"), shortname: Some(literal!("r")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the number seconds until omc timeouts and exits. Used by the testing framework to terminate infinite running processes.") } };

pub static TOTAL_TEARING: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 94, name: (literal!("totalTearing")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Activates total tearing (determination of all possible tearing sets) for the specified components.\nUse '-d=tearingdump' to find out the relevant indexes.")).clone() } } });

pub static IGNORE_SIMULATION_FLAGS_ANNOTATION: ConfigFlag = ConfigFlag { index: 95, name: literal!("ignoreSimulationFlagsAnnotation"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Ignores the simulation flags specified as annotation in the class.") } };

pub static DYNAMIC_TEARING_FOR_INITIALIZATION: ConfigFlag = ConfigFlag { index: 96, name: literal!("dynamicTearingForInitialization"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enable Dynamic Tearing also for the initialization system.") } };

pub static PREFER_TVARS_WITH_START_VALUE: ConfigFlag = ConfigFlag { index: 97, name: literal!("preferTVarsWithStartValue"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Prefer tearing variables with start value for initialization.") } };

pub static EQUATIONS_PER_FILE: ConfigFlag = ConfigFlag { index: 98, name: literal!("equationsPerFile"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 500 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generate code for at most this many equations per C-file (partially implemented in the compiler).") } };

pub static EVALUATE_FINAL_PARAMS: ConfigFlag = ConfigFlag { index: 99, name: literal!("evaluateFinalParameters"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Evaluates all the final parameters in addition to parameters with annotation(Evaluate=true).") } };

pub static EVALUATE_PROTECTED_PARAMS: ConfigFlag = ConfigFlag { index: 100, name: literal!("evaluateProtectedParameters"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Evaluates all the protected parameters in addition to parameters with annotation(Evaluate=true).") } };

pub static REPLACE_EVALUATED_PARAMS: ConfigFlag = ConfigFlag { index: 101, name: literal!("replaceEvaluatedParameters"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Replaces all the evaluated parameters in the DAE.") } };

pub static CONDENSE_ARRAYS: ConfigFlag = ConfigFlag { index: 102, name: literal!("condenseArrays"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets whether array expressions containing function calls are condensed or not.") } };

pub static WFC_ADVANCED: ConfigFlag = ConfigFlag { index: 103, name: literal!("wfcAdvanced"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("wrapFunctionCalls ignores more then default cases, e.g. exp, sin, cos, log, (experimental flag)") } };

pub static GRAPHICS_EXP_MODE: ConfigFlag = ConfigFlag { index: 104, name: literal!("graphicsExpMode"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets whether we are in graphics exp mode (evaluating icons).") } };

pub static TEARING_STRICTNESS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 105, name: (literal!("tearingStrictness")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("strict")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("casual"), Gettext::TranslatableContent::gettext { msgid: (literal!("Loose tearing rules using ExpressionSolve to determine the solvability instead of considering the partial derivative. Allows to solve for everything that is analytically possible. This could lead to singularities during simulation.")).clone() }), (literal!("strict"), Gettext::TranslatableContent::gettext { msgid: (literal!("Robust tearing rules by consideration of the partial derivative. Allows to divide by parameters that are not equal to or close to zero.")).clone() }), (literal!("veryStrict"), Gettext::TranslatableContent::gettext { msgid: (literal!("Very strict tearing rules that do not allow to divide by any parameter. Use this if you aim at overriding parameters after compilation with values equal to or close to zero.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the strictness of the tearing method regarding the solvability restrictions.")).clone() } } });

pub static INTERACTIVE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 106, name: (literal!("interactive")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("do nothing")).clone() }), (literal!("corba"), Gettext::TranslatableContent::gettext { msgid: (literal!("Starts omc as a server listening on the Corba interface.")).clone() }), (literal!("tcp"), Gettext::TranslatableContent::gettext { msgid: (literal!("Starts omc as a server listening on the socket interface.")).clone() }), (literal!("zmq"), Gettext::TranslatableContent::gettext { msgid: (literal!("Starts omc as a ZeroMQ server listening on the socket interface.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the interactive mode for omc.")).clone() } } });

pub static ZEROMQ_FILE_SUFFIX: ConfigFlag = ConfigFlag { index: 107, name: literal!("zeroMQFileSuffix"), shortname: Some(literal!("z")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the file suffix for zeroMQ port file if --interactive=zmq is used.") } };

pub static HOMOTOPY_APPROACH: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 108, name: (literal!("homotopyApproach")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("equidistantGlobal")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("equidistantLocal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Local homotopy approach with equidistant lambda steps. The homotopy parameter only effects the local strongly connected component.")).clone() }), (literal!("adaptiveLocal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Local homotopy approach with adaptive lambda steps. The homotopy parameter only effects the local strongly connected component.")).clone() }), (literal!("equidistantGlobal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Default, global homotopy approach with equidistant lambda steps. The homotopy parameter effects the entire initialization system.")).clone() }), (literal!("adaptiveGlobal"), Gettext::TranslatableContent::gettext { msgid: (literal!("Global homotopy approach with adaptive lambda steps. The homotopy parameter effects the entire initialization system.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the homotopy approach.")).clone() } } });

pub static IGNORE_REPLACEABLE: ConfigFlag = ConfigFlag { index: 109, name: literal!("ignoreReplaceable"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets whether to ignore replaceability or not when redeclaring.") } };

pub static LABELED_REDUCTION: ConfigFlag = ConfigFlag { index: 110, name: literal!("labeledReduction"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Turns on labeling and reduce terms to do whole process of reduction.") } };

pub static DISABLE_EXTRA_LABELING: ConfigFlag = ConfigFlag { index: 111, name: literal!("disableExtraLabeling"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disable adding extra label into the whole expression with more than one term and +,- operations.") } };

pub static LOAD_MSL_MODEL: ConfigFlag = ConfigFlag { index: 112, name: literal!("loadMSLModel"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Used to know loadFile doesn't need to be called in cpp-runtime (for labeled model reduction).") } };

pub static LOAD_PACKAGE_FILE: ConfigFlag = ConfigFlag { index: 113, name: literal!("loadPackageFile"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Used when the outside name is different with the inside name of the packge, in cpp-runtime (for labeled model reduction).") } };

pub static BUILDING_FMU: ConfigFlag = ConfigFlag { index: 114, name: literal!(""), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Is true when building an FMU (so the compiler can look for URIs to package as FMI resources).") } };

pub static BUILDING_MODEL: ConfigFlag = ConfigFlag { index: 115, name: literal!(""), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Is true when building a model (as opposed to running a Modelica script).") } };

pub static POST_OPT_MODULES_DAE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 116, name: (literal!("postOptModulesDAE")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("lateInlineFunction")).clone(), (literal!("wrapFunctionCalls")).clone(), (literal!("simplifysemiLinear")).clone(), (literal!("simplifyComplexFunction")).clone(), (literal!("removeConstants")).clone(), (literal!("simplifyTimeIndepFuncCalls")).clone(), (literal!("simplifyAllExpressions")).clone(), (literal!("findZeroCrossings")).clone(), (literal!("createDAEmodeBDAE")).clone(), (literal!("symbolicJacobianDAE")).clone(), (literal!("setEvaluationStage")).clone()] }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the optimization modules for the DAEmode in the back end. See --help=optmodules for more info.")).clone() } } });

pub static EVAL_LOOP_LIMIT: ConfigFlag = ConfigFlag { index: 117, name: literal!("evalLoopLimit"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 100000 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("The loop iteration limit used when evaluating constant function calls.") } };

pub static EVAL_RECURSION_LIMIT: ConfigFlag = ConfigFlag { index: 118, name: literal!("evalRecursionLimit"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 256 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("The recursion limit used when evaluating constant function calls.") } };

pub static SINGLE_INSTANCE_AGLSOLVER: ConfigFlag = ConfigFlag { index: 119, name: literal!("singleInstanceAglSolver"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets to instantiate only  one algebraic loop solver all algebraic loops") } };

pub static SHOW_STRUCTURAL_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 120, name: literal!("showStructuralAnnotations"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Show annotations affecting the solution process in the flattened code.") } };

pub static INITIAL_STATE_SELECTION: ConfigFlag = ConfigFlag { index: 121, name: literal!("initialStateSelection"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Activates the state selection inside initialization to avoid singularities.") } };

pub static LINEARIZATION_DUMP_LANGUAGE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 122, name: (literal!("linearizationDumpLanguage")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("Don't generate code for linearization.")).clone() }), (literal!("modelica"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate linearized Modelica model.")).clone() }), (literal!("matlab"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate matlab function that returns linearization matrices A,B,C,D.")).clone() }), (literal!("julia"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate julia function that returns linearization matrices A,B,C,D.")).clone() }), (literal!("python"), Gettext::TranslatableContent::gettext { msgid: (literal!("Generate python function that returns linearization matrices A,B,C,D.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets the target language for the produced code of linearization.")).clone() } } });

pub static NO_ASSC: ConfigFlag = ConfigFlag { index: 123, name: literal!("noASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Disables analytical to structural singularity conversion.") } };

pub static FULL_ASSC: ConfigFlag = ConfigFlag { index: 124, name: literal!("fullASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables full equation replacement for BLT transformation from the ASSC algorithm.") } };

pub static REAL_ASSC: ConfigFlag = ConfigFlag { index: 125, name: literal!("realASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables the ASSC algorithm to evaluate real valued coefficients (usually only integers).") } };

pub static INIT_ASSC: ConfigFlag = ConfigFlag { index: 126, name: literal!("initASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables the ASSC algorithm for initialization.") } };

pub static MAX_SIZE_ASSC: ConfigFlag = ConfigFlag { index: 127, name: literal!("maxSizeASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 200 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the maximum system size for the analytical to structural conversion algorithm (default 200).") } };

pub static USE_ZEROMQ_IN_SIM: ConfigFlag = ConfigFlag { index: 128, name: literal!("useZeroMQInSim"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configures to use zeroMQ in simulation runtime to exchange information via ZeroMQ with other applications") } };

pub static ZEROMQ_PUB_PORT: ConfigFlag = ConfigFlag { index: 129, name: literal!("zeroMQPubPort"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::INT_FLAG { data: 3203 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configures port number for simulation runtime to send information via ZeroMQ") } };

pub static ZEROMQ_SUB_PORT: ConfigFlag = ConfigFlag { index: 130, name: literal!("zeroMQSubPort"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::INT_FLAG { data: 3204 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configures port number for simulation runtime to receive information via ZeroMQ") } };

pub static ZEROMQ_JOB_ID: ConfigFlag = ConfigFlag { index: 131, name: literal!("zeroMQJOBID"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("empty") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configures the ID with which the omc api call is labelled for zeroMQ communication.") } };

pub static ZEROMQ_SERVER_ID: ConfigFlag = ConfigFlag { index: 132, name: literal!("zeroMQServerID"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("empty") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configures the ID with which server application is labelled for zeroMQ communication.") } };

pub static ZEROMQ_CLIENT_ID: ConfigFlag = ConfigFlag { index: 133, name: literal!("zeroMQClientID"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("empty") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configures the ID with which the client application is labelled for zeroMQ communication.") } };

pub static FMI_VERSION: ConfigFlag = ConfigFlag { index: 134, name: literal!(""), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("returns the FMI Version either 1.0 or 2.0.") } };

pub static BASE_MODELICA: ConfigFlag = ConfigFlag { index: 135, name: literal!("baseModelica"), shortname: Some(literal!("f")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Outputs experimental Base Modelica.") } };

pub static FMI_FILTER: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 136, name: (literal!("fmiFilter")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: FMI_PROTECTED.clone(), validValues: list![(literal!("none"), FMI_NONE.clone()), (literal!("internal"), FMI_INTERNAL.clone()), (literal!("protected"), FMI_PROTECTED.clone()), (literal!("blackBox"), FMI_BLACKBOX.clone())] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("All variables are exposed, even variables introduced by the symbolic transformations. This is mainly for debugging purposes.")).clone() }), (literal!("internal"), Gettext::TranslatableContent::gettext { msgid: (literal!("All model variables are exposed, including protected ones. Variables introduced by the symbolic transformations are filtered out, with minor exceptions, e.g. for state sets.")).clone() }), (literal!("protected"), Gettext::TranslatableContent::gettext { msgid: (literal!("All public model variables are exposed. Internal and protected variables are filtered out, with small exceptions, e.g. for state sets.")).clone() }), (literal!("blackBox"), Gettext::TranslatableContent::gettext { msgid: (literal!("Only the interface is exposed. All other variables are hidden or exposed with concealed names.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Specifies which model variables are exposed by the modelDescription.xml")).clone() } } });

pub static FMI_SOURCES: ConfigFlag = ConfigFlag { index: 137, name: literal!("fmiSources"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Defines if FMUs will be exported with sources or not. --fmiFilter=blackBox might override this, because black box FMUs do never contain their source code.") } };

pub static FMI_FLAGS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 138, name: (literal!("fmiFlags")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Add simulation flags to FMU. Will create <fmiPrefix>_flags.json in resources folder with given flags. Use --fmiFlags or --fmiFlags=none to disable [default]. Use --fmiFlags=default for the default simulation flags. To pass flags use e.g. --fmiFlags=s:cvode,nls:homotopy or --fmiFlags=path/to/yourFlags.json.")).clone() } } });

pub static FMU_CMAKE_BUILD: ConfigFlag = ConfigFlag { index: 139, name: literal!("fmuCMakeBuild"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Configured and build FMU with CMake if true.") } };

pub static NEW_BACKEND: ConfigFlag = ConfigFlag { index: 140, name: literal!("newBackend"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Activates experimental new backend for better array handling. This also activates the new frontend. [WIP]") } };

pub static PARMODAUTO: ConfigFlag = ConfigFlag { index: 141, name: literal!("parmodauto"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Experimental: Enable parallelization of independent systems of equations in the translated model. Only works on Linux systems.") } };

pub static INTERACTIVE_PORT: ConfigFlag = ConfigFlag { index: 142, name: literal!("interactivePort"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the port used by the interactive server.") } };

pub static ALLOW_NON_STANDARD_MODELICA: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 143, name: (literal!("allowNonStandardModelica")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("nonStdMultipleExternalDeclarations"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow several external declarations in functions.\nSee: https://specification.modelica.org/maint/3.5/functions.html#function-as-a-specialized-class")).clone() }), (literal!("nonStdEnumerationAsIntegers"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow enumeration as integer without casting via Integer(Enum).\nSee: https://specification.modelica.org/maint/3.5/class-predefined-types-and-declarations.html#type-conversion-of-enumeration-values-to-string-or-integer")).clone() }), (literal!("nonStdIntegersAsEnumeration"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow integer as enumeration without casting via Enum(Integer).\nSee: https://specification.modelica.org/maint/3.5/class-predefined-types-and-declarations.html#type-conversion-of-integer-to-enumeration-values")).clone() }), (literal!("nonStdDifferentCaseFileVsClassName"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow directory or file with different case in the name than the contained class name.\nSee: https://specification.modelica.org/maint/3.5/packages.html#mapping-package-class-structures-to-a-hierarchical-file-system")).clone() }), (literal!("nonStdTopLevelOuter"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow top level outer.\nSee: https://specification.modelica.org/maint/3.6/scoping-name-lookup-and-flattening.html#S4.p1")).clone() }), (literal!("protectedAccess"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow access of protected elements")).clone() }), (literal!("reinitInAlgorithms"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow reinit in algorithm sections")).clone() }), (literal!("unbalancedModel"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow models to be locally unbalanced and to have unbalanced connectors")).clone() }), (literal!("implicitParameterStartAttribute"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow fixed parameters with no binding or start attribute")).clone() }), (literal!("initialSimplified"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow use of experimental operator `initialSimplified()`")).clone() }), (literal!("illegalConditionalContext"), Gettext::TranslatableContent::gettext { msgid: (literal!("Allow use of components with false conditions in illegal contexts")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Flags to allow non-standard Modelica.")).clone() } } });

pub static EXPORT_CLOCKS_IN_MODELDESCRIPTION: ConfigFlag = ConfigFlag { index: 144, name: literal!("exportClocksInModelDescription"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("exports clocks in modeldescription.xml for fmus, The default is false.") } };

pub static LINK_TYPE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 145, name: (literal!("linkType")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 1, validValues: list![(literal!("dynamic"), 1), (literal!("static"), 2)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("dynamic")).clone(), (literal!("static")).clone()] }), description: Gettext::TranslatableContent::gettext { msgid: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sets the link type for the simulation executable.\n")); __mm_s.push_str(&*literal!("dynamic: libraries are dynamically linked; the executable is built very fast but is not portable because of DLL dependencies.\n")); __mm_s.push_str(&*literal!("static: libraries are statically linked; the executable is built more slowly but it is portable and dependency-free.\n")); ArcStr::from(__mm_s) }).clone() } } });

pub static TEARING_ALWAYS_DERIVATIVES: ConfigFlag = ConfigFlag { index: 146, name: literal!("tearingAlwaysDer"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Always choose state derivatives as iteration variables in strong components.") } };

pub static DUMP_FLAT_MODEL: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 147, name: (literal!("dumpFlatModel")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("all")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("flatten"), Gettext::TranslatableContent::gettext { msgid: (literal!("After flattening but before connection handling.")).clone() }), (literal!("connections"), Gettext::TranslatableContent::gettext { msgid: (literal!("After connection handling.")).clone() }), (literal!("eval"), Gettext::TranslatableContent::gettext { msgid: (literal!("After evaluating constants.")).clone() }), (literal!("simplify"), Gettext::TranslatableContent::gettext { msgid: (literal!("After model simplification.")).clone() }), (literal!("scalarize"), Gettext::TranslatableContent::gettext { msgid: (literal!("After scalarizing arrays.")).clone() }), (literal!("translateResidualsDAE"), Gettext::TranslatableContent::gettext { msgid: (literal!("Show the result of the translateResidualsDAE API.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Dumps the flat model at the given stages of the frontend.")).clone() } } });

pub static SIMULATION: ConfigFlag = ConfigFlag { index: 148, name: literal!("simulation"), shortname: Some(literal!("u")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Simulates the last model in the given Modelica file.") } };

pub static OBFUSCATE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 149, name: (literal!("obfuscate")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), Gettext::TranslatableContent::gettext { msgid: (literal!("No obfuscation.")).clone() }), (literal!("encrypted"), Gettext::TranslatableContent::gettext { msgid: (literal!("Obfuscates protected variables in encrypted models")).clone() }), (literal!("protected"), Gettext::TranslatableContent::gettext { msgid: (literal!("Obfuscates protected variables in all models.")).clone() }), (literal!("full"), Gettext::TranslatableContent::gettext { msgid: (literal!("Obfuscates everything.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Obfuscates identifiers in the simulation model")).clone() } } });

pub static FMU_RUNTIME_DEPENDS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 150, name: (literal!("fmuRuntimeDepends")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("default")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("default"), Gettext::TranslatableContent::notrans { r#str: (literal!("Depending on CMake version. If CMake version >= 3.21 use  \"modelica\", otherwise use \"none\"")).clone() }), (literal!("none"), Gettext::TranslatableContent::notrans { r#str: (literal!("No runtime library dependencies are copied into the FMU.")).clone() }), (literal!("modelica"), Gettext::TranslatableContent::notrans { r#str: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All modelica runtime library dependencies are copied into the FMU.")); __mm_s.push_str(&*literal!("System librarys located in '/lib*', '/usr/lib*' and '/usr/local/lib*' are excluded.")); __mm_s.push_str(&*literal!("Needs --fmuCMakeBuild=true and CMake version >= 3.21.")); ArcStr::from(__mm_s) }).clone() }), (literal!("all"), Gettext::TranslatableContent::notrans { r#str: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All runtime library dependencies are copied into the FMU.")); __mm_s.push_str(&*literal!("System librarys are copied as well.")); __mm_s.push_str(&*literal!("Needs --fmuCMakeBuild=true and CMake version >= 3.21.")); ArcStr::from(__mm_s) }).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Defines if runtime library dependencies are included in the FMU. Only used when compiler flag fmuCMakeBuild=true.")).clone() } } });

pub static FRONTEND_INLINE: ConfigFlag = ConfigFlag { index: 151, name: literal!("frontendInline"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Enables inlining of functions in the frontend.") } };

pub static EXPOSE_LOCAL_IOS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 152, name: (literal!("exposeLocalIOs")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Introduces top-level inputs/outputs for unconnected input/output connectors at requested levels, provided they are public, ")); __mm_s.push_str(&*literal!("0 meaning top-level (standard Modelica), 1 inputs/outputs of top-level components, >1 going deeper. ")); __mm_s.push_str(&*literal!("This flag is particularly useful for FMI export.")); ArcStr::from(__mm_s) }).clone() } } });

pub static BASE_MODELICA_FORMAT: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 153, name: (literal!("baseModelicaFormat")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("scalarized"), Gettext::TranslatableContent::notrans { r#str: (literal!("Include subscripts in the quoted identifiers ('a[1].x[3]').")).clone() }), (literal!("partiallyScalarized"), Gettext::TranslatableContent::notrans { r#str: (literal!("Include subscripts in the quoted identifiers, except for the last name ('a[1].x'[3]).")).clone() }), (literal!("nonScalarized"), Gettext::TranslatableContent::notrans { r#str: (literal!("Don't include subscripts in the quoted identifiers ('a'[1].'x'[3]).")).clone() }), (literal!("withRecords"), Gettext::TranslatableContent::notrans { r#str: (literal!("Keep records and don't expand them.")).clone() }), (literal!("withoutRecords"), Gettext::TranslatableContent::notrans { r#str: (literal!("Expand records into separate components.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Formatting options for Base Modelica")).clone() } } });

pub static BASE_MODELICA_OPTIONS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 154, name: (literal!("baseModelicaOptions")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("moveBindings"), Gettext::TranslatableContent::notrans { r#str: (literal!("Moves movable binding equations to normal equations.")).clone() }), (literal!("scalarize"), Gettext::TranslatableContent::notrans { r#str: (literal!("Fully scalarize the Base Modelica model.")).clone() }), (literal!("inlineFunctions"), Gettext::TranslatableContent::notrans { r#str: (literal!("Inline all functions.")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Enables optional Base Modelica options.")).clone() } } });

pub static DEBUG_FOLLOW_EQUATIONS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 155, name: (literal!("debugFollowEquations")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: (literal!("Takes a list of equation names and prints the corresponding equations after each stage of the backend process.")).clone() } } });

pub static MAX_SIZE_LINEARIZATION: ConfigFlag = ConfigFlag { index: 156, name: literal!("maxSizeLinearization"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 1000 }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Sets the maximum system size for which linearization code is generated.") } };

pub static RESIZABLE_ARRAYS: ConfigFlag = ConfigFlag { index: 157, name: literal!("resizableArrays"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Assumes all arrays are resizable. Only works with the new backend --newBackend.") } };

pub static EVALUATE_STRUCTURAL_PARAMETERS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 158, name: (literal!("evaluateStructuralParameters")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("all")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("all"), Gettext::TranslatableContent::gettext { msgid: (literal!("Evaluates all structural parameters")).clone() }), (literal!("strictlyNecessary"), Gettext::TranslatableContent::gettext { msgid: (literal!("Evaluates only structural parameters strictly required by the frontend")).clone() })] }), description: Gettext::TranslatableContent::gettext { msgid: (literal!("Sets which structural parameters are evaluated by the frontend.")).clone() } } });

pub static LOAD_MISSING_LIBRARIES: ConfigFlag = ConfigFlag { index: 159, name: literal!("loadMissingLibraries"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Automatically try to load a matching library if a name can't be found during name lookup.") } };

pub static CAUSALIZE_DAE_MODE: ConfigFlag = ConfigFlag { index: 160, name: literal!("causalizeDaeMode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("The system is partially causalized and simple assignments are generated for equations that can be solved explicitly. Only works with --daeMode.") } };

/* please remove me once this is supported */
pub static SIM_CODE_SCALARIZE: ConfigFlag = ConfigFlag { index: 161, name: literal!("simCodeScalarize"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Scalarizes variables during simcode phase.") } };

pub static EXECUTE_COMMAND: ConfigFlag = ConfigFlag { index: 162, name: literal!("cmd"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Executes the string argument as a script before any other operation.") } };

pub static MOO_DYNAMIC_OPTIMIZATION: ConfigFlag = ConfigFlag { index: 163, name: literal!("moo"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: Gettext::TranslatableContent::gettext { msgid: literal!("Generate code for dynamic optimization library MOO.") } };

pub fn getFlags(mut initialize: bool) -> Flag {
    let mut flags: Flag = Flag::NO_FLAGS;
    flags = crate::Globals::flagsIndex.with(|__root| __root.borrow().clone());
    flags
}

pub fn isSet(mut inFlag: DebugFlag) -> Result<bool> {
    let mut outValue: bool = false;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut flags: Flag = Flag::NO_FLAGS;
    let mut index: i32 = 0;
    let DebugFlag { index: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    index = __pa0.clone();
    flags = getFlags(true);
    let Flag::FLAGS { debugFlags: __pa1, .. } = (flags.clone()) else { bail!("pattern mismatch") };
    debug_flags = __pa1.clone();
    outValue = debug_flags.clone().borrow()[(index.clone()-1) as usize].clone();
    Ok(outValue)
}

pub fn isConfigFlagSet(mut inFlag: ConfigFlag, mut hasMember: ArcStr) -> Result<bool> {
    let mut isMember: bool = false;
    isMember = listMember((hasMember.clone()).clone(), getConfigStringList(inFlag.clone())?);
    Ok(isMember)
}

pub fn getConfigName(mut inFlag: ConfigFlag) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let ConfigFlag { name: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    Ok(name)
}

pub fn getConfigValue(mut inFlag: ConfigFlag) -> Result<FlagData> {
    let mut outValue: FlagData = FlagData::EMPTY_FLAG;
    let mut config_flags: metamodelica::Array<FlagData>;
    let mut index: i32 = 0;
    let mut flags: Flag = Flag::NO_FLAGS;
    let mut name: ArcStr = arcstr::literal!("");
    let ConfigFlag { index: __pa0, name: __pa1, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    index = __pa0.clone();
    name = __pa1.clone();
    flags = getFlags(true);
    let Flag::FLAGS { configFlags: __pa2, .. } = (flags.clone()) else { bail!("pattern mismatch") };
    config_flags = __pa2.clone();
    outValue = config_flags.clone().borrow()[(index.clone()-1) as usize].clone();
    Ok(outValue)
}

pub fn getConfigBool(mut inFlag: ConfigFlag) -> Result<bool> {
    let mut outValue: bool = false;
    let FlagData::BOOL_FLAG { data: __pa0 } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigInt(mut inFlag: ConfigFlag) -> Result<i32> {
    let mut outValue: i32 = 0;
    let FlagData::INT_FLAG { data: __pa0 } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigIntList(mut inFlag: ConfigFlag) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outValue: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let FlagData::INT_LIST_FLAG { data: __pa0 } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigReal(mut inFlag: ConfigFlag) -> Result<metamodelica::Real> {
    let mut outValue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let FlagData::REAL_FLAG { data: __pa0 } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigString(mut inFlag: ConfigFlag) -> Result<ArcStr> {
    let mut outValue: ArcStr = arcstr::literal!("");
    let FlagData::STRING_FLAG { data: __pa0 } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigStringList(mut inFlag: ConfigFlag) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outValue: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let FlagData::STRING_LIST_FLAG { data: __pa0 } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigEnum(mut inFlag: ConfigFlag) -> Result<i32> {
    let mut outValue: i32 = 0;
    let FlagData::ENUM_FLAG { data: __pa0, .. } = (getConfigValue(inFlag.clone())?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

