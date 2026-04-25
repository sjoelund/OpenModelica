//! Translation of Util/Flags.mo
//!
//! This module provides types and constants for compiler flags. There are two types
//! of flags: debug flags (boolean, specified with -d) and configuration flags
//! (typed values, specified with --flag-name).
//!
//! # Assumptions
//! - `get_global_root(index)` from the Global module must be available to retrieve
//!   the Flags structure from the global root at index FLAGS_INDEX.
//! - Arrays in MetaModelica are 1-indexed. The Rust arrays (Vec) are 0-indexed,
//!   so accessing element at 1-based index requires subtracting 1.
//!
//! # Known issues
//! - The flag constants are defined inline here. In the full implementation,
//!   the runtime initialization via FlagsUtil.loadFlags populates the underlying
//!   arrays that get_flags() retrieves.

use crate::global;

// ============================================================================
// TranslatableContent for const contexts (using &'static str)
// Runtime TranslatableContent uses String. These are kept separate for const
// compatibility since const items cannot call non-const functions.
// ============================================================================

/// A translatable message content for const contexts (using &'static str).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslatableContentStatic {
    /// A translatable message with a msgid
    GETTEXT { msgid: &'static str },
    /// A non-translatable string
    NOTRANS { s: &'static str },
}

impl TranslatableContentStatic {
    /// Returns the message string regardless of variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            TranslatableContentStatic::GETTEXT { msgid } => msgid,
            TranslatableContentStatic::NOTRANS { s } => s,
        }
    }
}

impl std::fmt::Display for TranslatableContentStatic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslatableContentStatic::GETTEXT { msgid } => write!(f, "gettext({msgid})"),
            TranslatableContentStatic::NOTRANS { s } => write!(f, "{s}"),
        }
    }
}

// ============================================================================
// DebugFlag - uniontype for debug flags
// ============================================================================

/// Debug flags are boolean flags specified with -d.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DebugFlag {
    DEBUG_FLAG {
        index: i32,
        name: &'static str,
        default: bool,
        description: TranslatableContentStatic,
    },
}

impl DebugFlag {
    /// Returns the 1-based index of this debug flag.
    pub fn index(&self) -> i32 {
        match self {
            DebugFlag::DEBUG_FLAG { index, .. } => *index,
        }
    }

    /// Returns the name of this debug flag (the string used with -d).
    pub fn name(&self) -> &'static str {
        match self {
            DebugFlag::DEBUG_FLAG { name, .. } => name,
        }
    }
}

// ============================================================================
// ConfigFlag - uniontype for configuration flags
// ============================================================================

/// Configuration flags affect compiler behaviour. They have typed values.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConfigFlag {
    CONFIG_FLAG {
        index: i32,
        name: &'static str,
        shortname: Option<&'static str>,
        visibility: FlagVisibility,
        default_value: DefaultFlagValue,
        valid_options: Option<ValidOptionsStatic>,
        description: TranslatableContentStatic,
    },
}

impl ConfigFlag {
    /// Returns the 1-based index of this config flag.
    pub fn index(&self) -> i32 {
        match self {
            ConfigFlag::CONFIG_FLAG { index, .. } => *index,
        }
    }

    /// Returns the name of this config flag (the string used with --).
    pub fn name(&self) -> &'static str {
        match self {
            ConfigFlag::CONFIG_FLAG { name, .. } => name,
        }
    }

    /// Returns the default value as a runtime FlagData.
    pub fn default_flag_data(&self) -> FlagData {
        match self {
            ConfigFlag::CONFIG_FLAG { default_value, .. } => default_value.to_flag_data(),
        }
    }
}

// ============================================================================
// FlagData - uniontype for flag values
// ============================================================================

/// This uniontype is used to store the values of configuration flags.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FlagData {
    EMPTY_FLAG,
    BOOL_FLAG { data: bool },
    INT_FLAG { data: i32 },
    INT_LIST_FLAG { data: Vec<i32> },
    REAL_FLAG { data: f64 },
    STRING_FLAG { data: String },
    STRING_LIST_FLAG { data: Vec<String> },
    ENUM_FLAG {
        data: i32,
        valid_values: Vec<(String, i32)>,
    },
}

impl FlagData {
    /// Returns the value if this is a BOOL_FLAG.
    pub fn bool_value(&self) -> Option<bool> {
        match self {
            FlagData::BOOL_FLAG { data } => Some(*data),
            _ => None,
        }
    }

    /// Returns the value if this is an INT_FLAG.
    pub fn int_value(&self) -> Option<i32> {
        match self {
            FlagData::INT_FLAG { data } => Some(*data),
            _ => None,
        }
    }

    /// Returns the value if this is a REAL_FLAG.
    pub fn real_value(&self) -> Option<f64> {
        match self {
            FlagData::REAL_FLAG { data } => Some(*data),
            _ => None,
        }
    }

    /// Returns the value if this is a STRING_FLAG.
    pub fn string_value(&self) -> Option<&str> {
        match self {
            FlagData::STRING_FLAG { data } => Some(data.as_str()),
            _ => None,
        }
    }

    /// Returns the value if this is an ENUM_FLAG.
    pub fn enum_value(&self) -> Option<i32> {
        match self {
            FlagData::ENUM_FLAG { data, .. } => Some(*data),
            _ => None,
        }
    }
}

// ============================================================================
// FlagVisibility - uniontype for flag visibility
// ============================================================================

/// Whether a configuration flag is visible to the user or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlagVisibility {
    /// An internal flag that is hidden to the user.
    INTERNAL,
    /// An external flag that is visible to the user.
    EXTERNAL,
}

/// Helper: create an INTERNAL visibility.
pub fn internal() -> FlagVisibility {
    FlagVisibility::INTERNAL
}

/// Helper: create an EXTERNAL visibility.
pub fn external() -> FlagVisibility {
    FlagVisibility::EXTERNAL
}

// ============================================================================
// ValidOptions for const contexts (using &'static str)
// ============================================================================

/// Specifies valid options for a flag (const-compatible version).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ValidOptionsStatic {
    STRING_OPTION { options: &'static [&'static str] },
}

// ============================================================================
// DefaultFlagValue - const-compatible flag defaults
// Used in ConfigFlag const items; converts to FlagData at runtime.
// ============================================================================

/// A default flag value that can be used in const contexts.
/// Converts to FlagData at runtime via the `to_flag_data` method.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DefaultFlagValue {
    Bool(bool),
    Int(i32),
    Real(f64),
    Str(&'static str),
    StrList(&'static [&'static str]),
    IntList(&'static [i32]),
    Enum { value: i32, valid_values: &'static [(&'static str, i32)] },
}

impl DefaultFlagValue {
    /// Converts this default value to a runtime FlagData.
    pub fn to_flag_data(&self) -> FlagData {
        match self {
            DefaultFlagValue::Bool(v) => FlagData::BOOL_FLAG { data: *v },
            DefaultFlagValue::Int(v) => FlagData::INT_FLAG { data: *v },
            DefaultFlagValue::Real(v) => FlagData::REAL_FLAG { data: *v },
            DefaultFlagValue::Str(v) => FlagData::STRING_FLAG { data: v.to_string() },
            DefaultFlagValue::StrList(v) => {
                FlagData::STRING_LIST_FLAG { data: v.iter().map(|s| s.to_string()).collect() }
            }
            DefaultFlagValue::IntList(v) => {
                FlagData::INT_LIST_FLAG { data: v.to_vec() }
            }
            DefaultFlagValue::Enum { value, valid_values } => {
                FlagData::ENUM_FLAG {
                    data: *value,
                    valid_values: valid_values.iter().map(|(s, i)| (s.to_string(), *i)).collect(),
                }
            }
        }
    }
}

// ============================================================================
// Flag - uniontype for the flags structure
// ============================================================================

/// The structure which stores the flags.
/// This is retrieved from the global root at FLAGS_INDEX.
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(non_camel_case_types)]
pub enum Flag {
    #[default]
    NO_FLAGS,
    FLAGS {
        debug_flags: Vec<bool>,
        config_flags: Vec<FlagData>,
    },
}

// ============================================================================
// FlagData helpers (runtime, using owned String)
// ============================================================================

/// Helper: create an EMPTY_FLAG.
pub fn empty_flag() -> FlagData {
    FlagData::EMPTY_FLAG
}

/// Helper: create a BOOL_FLAG.
pub fn bool_flag(data: bool) -> FlagData {
    FlagData::BOOL_FLAG { data }
}

/// Helper: create an INT_FLAG.
pub fn int_flag(data: i32) -> FlagData {
    FlagData::INT_FLAG { data }
}

/// Helper: create an INT_LIST_FLAG.
pub fn int_list_flag(data: Vec<i32>) -> FlagData {
    FlagData::INT_LIST_FLAG { data }
}

/// Helper: create a REAL_FLAG.
pub fn real_flag(data: f64) -> FlagData {
    FlagData::REAL_FLAG { data }
}

/// Helper: create a STRING_FLAG.
pub fn string_flag(data: &str) -> FlagData {
    FlagData::STRING_FLAG {
        data: data.to_string(),
    }
}

/// Helper: create a STRING_LIST_FLAG.
pub fn string_list_flag(data: Vec<String>) -> FlagData {
    FlagData::STRING_LIST_FLAG { data }
}

/// Helper: create an ENUM_FLAG.
pub fn enum_flag(data: i32, valid_values: Vec<(String, i32)>) -> FlagData {
    FlagData::ENUM_FLAG {
        data,
        valid_values,
    }
}

// ============================================================================
// Grammar/language standard constants
// ============================================================================

/// Change this to a proper enum when we have support for them.
pub const MODELICA: i32 = 1;
pub const METAMODELICA: i32 = 2;
pub const PARMODELICA: i32 = 3;
pub const OPTIMICA: i32 = 4;
pub const PDEMODELICA: i32 = 5;

// ============================================================================
// FMI constants
// ============================================================================

/// FMI Model Description ENUM FLAGS
pub const FMI_NONE: i32 = 1;
pub const FMI_INTERNAL: i32 = 2;
pub const FMI_PROTECTED: i32 = 3;
pub const FMI_BLACKBOX: i32 = 4;

// ============================================================================
// Debug Flag constants
// ============================================================================

/// Sets whether to print a failtrace or not.
pub const FAILTRACE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 1,
    name: "failtrace",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Sets whether to print a failtrace or not.",
    },
};

/// Prints extra information from Ceval.
pub const CEVAL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 2,
    name: "ceval",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints extra information from Ceval.",
    },
};

/// Do some simple analyses on the datastructure from the frontend to check if it is consistent.
pub const CHECK_BACKEND_DAE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 3,
    name: "checkBackendDae",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Do some simple analyses on the datastructure from the frontend to check if it is consistent.",
    },
};

/// Experimental: Unused parallelization.
pub const PTHREADS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 4,
    name: "pthreads",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Experimental: Unused parallelization.",
    },
};

/// Turns on/off events handling.
pub const EVENTS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 5,
    name: "events",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Turns on/off events handling.",
    },
};

/// Dumps the inline solver equation system.
pub const DUMP_INLINE_SOLVER: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 6,
    name: "dumpInlineSolver",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the inline solver equation system.",
    },
};

/// Turns on/off symbolic function evaluation.
pub const EVAL_FUNC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 7,
    name: "evalfunc",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Turns on/off symbolic function evaluation.",
    },
};

/// Turns on/off dynamic loading of functions that are compiled during translation.
pub const GEN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 8,
    name: "gen",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Turns on/off dynamic loading of functions that are compiled during translation. Only enable this if external functions are needed to calculate structural parameters or constants.",
    },
};

/// Display debug information about dynamic loading of compiled functions.
pub const DYN_LOAD: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 9,
    name: "dynload",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Display debug information about dynamic loading of compiled functions.",
    },
};

/// Used to generate code for the bootstrapped compiler.
pub const GENERATE_CODE_CHEAT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 10,
    name: "generateCodeCheat",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Used to generate code for the bootstrapped compiler.",
    },
};

/// Generates a graphviz file of the connection graph.
pub const CGRAPH_GRAPHVIZ_FILE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 11,
    name: "cgraphGraphVizFile",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Generates a graphviz file of the connection graph.",
    },
};

/// Displays the connection graph with the GraphViz lefty tool.
pub const CGRAPH_GRAPHVIZ_SHOW: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 12,
    name: "cgraphGraphVizShow",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Displays the connection graph with the GraphViz lefty tool.",
    },
};

/// Prints garbage collection stats to standard output.
pub const GC_PROF: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 13,
    name: "gcProfiling",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints garbage collection stats to standard output.",
    },
};

/// Enables extra type checking for cref expressions.
pub const CHECK_DAE_CREF_TYPE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 14,
    name: "checkDAECrefType",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables extra type checking for cref expressions.",
    },
};

/// Prints out a warning if an ASUB is created from a CREF expression.
pub const CHECK_ASUB: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 15,
    name: "checkASUB",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints out a warning if an ASUB is created from a CREF expression.",
    },
};

/// Prints extra failtrace from InstanceHierarchy.
pub const INSTANCE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 16,
    name: "instance",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints extra failtrace from InstanceHierarchy.",
    },
};

/// Turns off the instantiation cache.
pub const CACHE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 17,
    name: "Cache",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Turns off the instantiation cache.",
    },
};

/// Converts Modelica-style arrays to lists.
pub const RML: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 18,
    name: "rml",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Converts Modelica-style arrays to lists.",
    },
};

/// Prints out a notification if tail recursion optimization has been applied.
pub const TAIL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 19,
    name: "tail",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints out a notification if tail recursion optimization has been applied.",
    },
};

/// Print extra failtrace from lookup.
pub const LOOKUP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 20,
    name: "lookup",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Print extra failtrace from lookup.",
    },
};

/// Adds notifications of all pattern-matching optimizations that are performed.
pub const PATTERNM_ALL_INFO: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 22,
    name: "patternmAllInfo",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Adds notifications of all pattern-matching optimizations that are performed.",
    },
};

/// Performs dead code elimination in match-expressions.
pub const PATTERNM_DCE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 23,
    name: "patternmDeadCodeElimination",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Performs dead code elimination in match-expressions.",
    },
};

/// Optimization that moves the last assignment(s) into the result of a match-expression.
pub const PATTERNM_MOVE_LAST_EXP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 24,
    name: "patternmMoveLastExp",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Optimization that moves the last assignment(s) into the result of a match-expression.",
    },
};

/// Turns on custom reduction functions (OpenModelica extension).
pub const EXPERIMENTAL_REDUCTIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 25,
    name: "experimentalReductions",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Turns on custom reduction functions (OpenModelica extension).",
    },
};

/// Evaluates all parameters if set, except the ones that have annotation(Evaluate = false).
pub const EVAL_PARAM: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 26,
    name: "evaluateAllParameters",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Evaluates all parameters if set, except the ones that have annotation(Evaluate = false).",
    },
};

/// Prints extra failtrace from Types.
pub const TYPES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 27,
    name: "types",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints extra failtrace from Types.",
    },
};

/// Shows the statement that is currently being evaluated when evaluating a script.
pub const SHOW_STATEMENT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 28,
    name: "showStatement",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Shows the statement that is currently being evaluated when evaluating a script.",
    },
};

/// Dumps the absyn representation of a program.
pub const DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 29,
    name: "dump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the absyn representation of a program.",
    },
};

/// Dumps the absyn representation of a program in graphviz format.
pub const DUMP_GRAPHVIZ: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 30,
    name: "graphviz",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the absyn representation of a program in graphviz format.",
    },
};

/// Prints out execution statistics for the compiler.
pub const EXEC_STAT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 31,
    name: "execstat",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints out execution statistics for the compiler.",
    },
};

/// Applies transformations required for code generation before dumping flat code.
pub const TRANSFORMS_BEFORE_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 32,
    name: "transformsbeforedump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Applies transformations required for code generation before dumping flat code.",
    },
};

/// Dumps the DAE in graphviz format.
pub const DAE_DUMP_GRAPHV: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 33,
    name: "daedumpgraphv",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the DAE in graphviz format.",
    },
};

/// Starts omc as a server listening on the socket interface.
pub const INTERACTIVE_TCP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 34,
    name: "interactive",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Starts omc as a server listening on the socket interface.",
    },
};

/// Starts omc as a server listening on the Corba interface.
pub const INTERACTIVE_CORBA: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 35,
    name: "interactiveCorba",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Starts omc as a server listening on the Corba interface.",
    },
};

/// Prints out debug information for the interactive server.
pub const INTERACTIVE_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 36,
    name: "interactivedump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints out debug information for the interactive server.",
    },
};

/// Prints out debug information about relations, that are used as zero crossings.
pub const RELIDX: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 37,
    name: "relidx",
    default: false,
    description: TranslatableContentStatic::NOTRANS {
        s: "Prints out debug information about relations, that are used as zero crossings.",
    },
};

/// Dump the found replacements for simple equation removal.
pub const DUMP_REPL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 38,
    name: "dumprepl",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump the found replacements for simple equation removal.",
    },
};

/// Dump the found replacements for final parameters.
pub const DUMP_FP_REPL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 39,
    name: "dumpFPrepl",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump the found replacements for final parameters.",
    },
};

/// Dump the found replacements for remove parameters.
pub const DUMP_PARAM_REPL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 40,
    name: "dumpParamrepl",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump the found replacements for remove parameters.",
    },
};

/// Dump the found replacements for protected parameters.
pub const DUMP_PP_REPL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 41,
    name: "dumpPPrepl",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump the found replacements for protected parameters.",
    },
};

/// Dump the found replacements for evaluate annotations (evaluate=true) parameters.
pub const DUMP_EA_REPL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 42,
    name: "dumpEArepl",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump the found replacements for evaluate annotations (evaluate=true) parameters.",
    },
};

/// Dumps some information about the process of removeSimpleEquations.
pub const DEBUG_ALIAS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 43,
    name: "debugAlias",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps some information about the process of removeSimpleEquations.",
    },
};

/// Dumps tearing information.
pub const TEARING_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 44,
    name: "tearingdump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps tearing information.",
    },
};

/// Dumps information about symbolic Jacobians.
pub const JAC_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 45,
    name: "symjacdump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about symbolic Jacobians.",
    },
};

/// Dumps information in verbose mode about symbolic Jacobians.
pub const JAC_DUMP2: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 46,
    name: "symjacdumpverbose",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information in verbose mode about symbolic Jacobians.",
    },
};

/// Dumps information about the equations created from bindings.
pub const DUMP_BINDINGS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 47,
    name: "dumpBindings",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about the equations created from bindings.",
    },
};

/// Dumps information about the process of sorting.
pub const DUMP_SORTING: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 48,
    name: "dumpSorting",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about the process of sorting.",
    },
};

/// Dumps sparse pattern with coloring used for simulation.
pub const DUMP_SPARSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 49,
    name: "dumpSparsePattern",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps sparse pattern with coloring used for simulation.",
    },
};

/// Dumps in verbose mode sparse pattern with coloring used for simulation.
pub const DUMP_SPARSE_VERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 50,
    name: "dumpSparsePatternVerbose",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps in verbose mode sparse pattern with coloring used for simulation.",
    },
};

/// Dumps information from index reduction.
pub const BLT_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 51,
    name: "bltdump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information from index reduction.",
    },
};

/// Dumps information from dummy state selection heuristic.
pub const DUMMY_SELECT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 52,
    name: "dummyselect",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information from dummy state selection heuristic.",
    },
};

/// Dumps the equation system at the beginning of the back end.
pub const DUMP_DAE_LOW: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 53,
    name: "dumpdaelow",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the equation system at the beginning of the back end.",
    },
};

/// Dumps the equation system after index reduction and optimization.
pub const DUMP_INDX_DAE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 54,
    name: "dumpindxdae",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the equation system after index reduction and optimization.",
    },
};

/// Dumps information from the optimization modules.
pub const OPT_DAE_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 55,
    name: "optdaedump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information from the optimization modules.",
    },
};

/// Measures the time it takes to hash all simcode variables before code generation.
pub const EXEC_HASH: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 56,
    name: "execHash",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Measures the time it takes to hash all simcode variables before code generation.",
    },
};

/// Enables dumping of the parameters in the order they are calculated.
pub const PARAM_DLOW_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 57,
    name: "paramdlowdump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of the parameters in the order they are calculated.",
    },
};

/// Dumps the results of the preOptModule encapsulateWhenConditions.
pub const DUMP_ENCAPSULATECONDITIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 58,
    name: "dumpEncapsulateConditions",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the results of the preOptModule encapsulateWhenConditions.",
    },
};

/// Enables short output of the simulate() command.
pub const SHORT_OUTPUT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 59,
    name: "shortOutput",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables short output of the simulate() command. Useful for tools like OMNotebook.",
    },
};

/// Count operations.
pub const COUNT_OPERATIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 60,
    name: "countOperations",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Count operations.",
    },
};

/// Prints out connection graph information.
pub const CGRAPH: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 61,
    name: "cgraph",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints out connection graph information.",
    },
};

/// Prints information about modification updates.
pub const UPDMOD: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 62,
    name: "updmod",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints information about modification updates.",
    },
};

/// Enables extra debug output from the static elaboration.
pub const STATIC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 63,
    name: "static",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables extra debug output from the static elaboration.",
    },
};

/// Enables output of template performance data for rendering text to file.
pub const TPL_PERF_TIMES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 64,
    name: "tplPerfTimes",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables output of template performance data for rendering text to file.",
    },
};

/// Enables checks for expression simplification.
pub const CHECK_SIMPLIFY: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 65,
    name: "checkSimplify",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables checks for expression simplification and prints a notification whenever an undesirable transformation has been performed.",
    },
};

/// Enables new instantiation phase.
pub const SCODE_INST: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 66,
    name: "newInst",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables new instantiation phase.",
    },
};

/// Enables writing simulation results to buffer.
pub const WRITE_TO_BUFFER: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 67,
    name: "writeToBuffer",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables writing simulation results to buffer.",
    },
};

/// Enables dumping of back-end information about system.
pub const DUMP_BACKENDDAE_INFO: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 68,
    name: "backenddaeinfo",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of back-end information about system (Number of equations before back-end,...).",
    },
};

/// Generate code with debugging symbols.
pub const GEN_DEBUG_SYMBOLS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 69,
    name: "gendebugsymbols",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Generate code with debugging symbols.",
    },
};

/// Enables dumping of selected states.
pub const DUMP_STATESELECTION_INFO: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 70,
    name: "stateselection",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of selected states. Extends -d=backenddaeinfo.",
    },
};

/// Enables dumping of the equations in the order they are calculated.
pub const DUMP_EQNINORDER: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 71,
    name: "dumpeqninorder",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of the equations in the order they are calculated.",
    },
};

/// Enables dumping of the optimization information when optimizing calls to semiLinear.
pub const SEMILINEAR: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 72,
    name: "semiLinear",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of the optimization information when optimizing calls to semiLinear.",
    },
};

/// Enables dumping of status when calling modelEquationsUC.
pub const UNCERTAINTIES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 73,
    name: "uncertainties",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of status when calling modelEquationsUC.",
    },
};

/// Enables dumping of the DAE startOrigin attribute of the variables.
pub const SHOW_START_ORIGIN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 74,
    name: "showStartOrigin",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of the DAE startOrigin attribute of the variables.",
    },
};

/// Dumps the simCode model used for code generation.
pub const DUMP_SIMCODE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 75,
    name: "dumpSimCode",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the simCode model used for code generation.",
    },
};

/// Dumps the initial equation system.
pub const DUMP_INITIAL_SYSTEM: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 76,
    name: "dumpinitialsystem",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the initial equation system.",
    },
};

/// Do graph based instantiation.
pub const GRAPH_INST: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 77,
    name: "graphInst",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Do graph based instantiation.",
    },
};

/// Run scode dependency analysis. Use with -d=graphInst
pub const GRAPH_INST_RUN_DEP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 78,
    name: "graphInstRunDep",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Run scode dependency analysis. Use with -d=graphInst",
    },
};

/// Dumps a graph of the program. Use with -d=graphInst
pub const GRAPH_INST_GEN_GRAPH: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 79,
    name: "graphInstGenGraph",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps a graph of the program. Use with -d=graphInst",
    },
};

/// Dump the found replacements for constants.
pub const DUMP_CONST_REPL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 80,
    name: "dumpConstrepl",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump the found replacements for constants.",
    },
};

/// Display the element source information in the dumped DAE for easier debugging.
pub const SHOW_EQUATION_SOURCE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 81,
    name: "showEquationSource",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Display the element source information in the dumped DAE for easier debugging.",
    },
};

/// Enables analytical jacobian for linear strong components.
pub const LS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 82,
    name: "LSanalyticJacobian",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables analytical jacobian for linear strong components. Defaults to false",
    },
};

/// Enables analytical jacobian for non-linear strong components.
pub const NLS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 83,
    name: "NLSanalyticJacobian",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables analytical jacobian for non-linear strong components without user-defined function calls.",
    },
};

/// Generates code for inline solver.
pub const INLINE_SOLVER: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 84,
    name: "inlineSolver",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Generates code for inline solver.",
    },
};

/// Enables parallel calculation based on task-graphs.
pub const HPCOM: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 85,
    name: "hpcom",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables parallel calculation based on task-graphs.",
    },
};

/// Shows additional information from the initialization process.
pub const INITIALIZATION: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 86,
    name: "initialization",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Shows additional information from the initialization process.",
    },
};

/// Controls if function inlining should be performed.
pub const INLINE_FUNCTIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 87,
    name: "inlineFunctions",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Controls if function inlining should be performed.",
    },
};

/// Dumps graphml files with the strongly connected components.
pub const DUMP_SCC_GRAPHML: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 88,
    name: "dumpSCCGraphML",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps graphml files with the strongly connected components.",
    },
};

/// Dumps verbose tearing information.
pub const TEARING_DUMPVERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 89,
    name: "tearingdumpV",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps verbose tearing information.",
    },
};

/// Disables the generation of single flow equations.
pub const DISABLE_SINGLE_FLOW_EQ: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 90,
    name: "disableSingleFlowEq",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables the generation of single flow equations.",
    },
};

/// Enables dumping of discrete variables.
pub const DUMP_DISCRETEVARS_INFO: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 91,
    name: "discreteinfo",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of discrete variables. Extends -d=backenddaeinfo.",
    },
};

/// Activates additional graphviz dumps (as .dot files).
pub const ADDITIONAL_GRAPHVIZ_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 92,
    name: "graphvizDump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Activates additional graphviz dumps (as .dot files).",
    },
};

/// Enables output of the operations in the _info.xml file.
pub const INFO_XML_OPERATIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 93,
    name: "infoXmlOperations",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables output of the operations in the _info.xml file when translating models.",
    },
};

/// Dumps additional information on the parallel execution with hpcom.
pub const HPCOM_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 94,
    name: "hpcomDump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps additional information on the parallel execution with hpcom.",
    },
};

/// Debug Output for ResolveLoops Module.
pub const RESOLVE_LOOPS_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 95,
    name: "resolveLoopsDump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Debug Output for ResolveLoops Module.",
    },
};

/// Disables warnings on Windows if OPENMODELICAHOME/MinGW is missing.
pub const DISABLE_WINDOWS_PATH_CHECK_WARNING: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 96,
    name: "disableWindowsPathCheckWarning",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables warnings on Windows if OPENMODELICAHOME/MinGW is missing.",
    },
};

/// Disables output of record constructors in the flat code.
pub const DISABLE_RECORD_CONSTRUCTOR_OUTPUT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 97,
    name: "disableRecordConstructorOutput",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables output of record constructors in the flat code.",
    },
};

/// Activates implicit codegen
pub const IMPL_ODE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 98,
    name: "implOde",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "activates implicit codegen",
    },
};

/// Dumps debug information about the function evaluation
pub const EVAL_FUNC_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 99,
    name: "evalFuncDump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "dumps debug information about the function evaluation",
    },
};

/// Prints the structural parameters identified by the front-end
pub const PRINT_STRUCTURAL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 100,
    name: "printStructuralParameters",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints the structural parameters identified by the front-end",
    },
};

/// Shows a list of all iteration variables.
pub const ITERATION_VARS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 101,
    name: "iterationVars",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Shows a list of all iteration variables.",
    },
};

/// Accepts passing records with more fields than expected to a function.
pub const ALLOW_RECORD_TOO_MANY_FIELDS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 102,
    name: "acceptTooManyFields",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Accepts passing records with more fields than expected to a function.",
    },
};

/// Optimize the memory structure regarding the selected scheduler
pub const HPCOM_MEMORY_OPT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 103,
    name: "hpcomMemoryOpt",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Optimize the memory structure regarding the selected scheduler",
    },
};

/// Dumps information of the clock partitioning.
pub const DUMP_SYNCHRONOUS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 104,
    name: "dumpSynchronous",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information of the clock partitioning.",
    },
};

/// Strips the environment prefix from path/crefs. Defaults to true.
pub const STRIP_PREFIX: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 105,
    name: "stripPrefix",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Strips the environment prefix from path/crefs. Defaults to true.",
    },
};

/// Does scode dependency analysis prior to instantiation. Defaults to true.
pub const DO_SCODE_DEP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 106,
    name: "scodeDep",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Does scode dependency analysis prior to instantiation. Defaults to true.",
    },
};

/// Prints information about instantiation cache hits and additions.
pub const SHOW_INST_CACHE_INFO: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 107,
    name: "showInstCacheInfo",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints information about instantiation cache hits and additions.",
    },
};

/// Dumps all the calculated units.
pub const DUMP_UNIT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 108,
    name: "dumpUnits",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps all the calculated units.",
    },
};

/// Dumps all equations handled by the unit checker.
pub const DUMP_EQ_UNIT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 109,
    name: "dumpEqInUC",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps all equations handled by the unit checker.",
    },
};

/// Dumps all the equations handled by the unit checker as tree-structure.
pub const DUMP_EQ_UNIT_STRUCT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 110,
    name: "dumpEqUCStruct",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps all the equations handled by the unit checker as tree-structure.",
    },
};

/// Show the dae variable declarations as they happen.
pub const SHOW_DAE_GENERATION: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 111,
    name: "showDaeGeneration",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Show the dae variable declarations as they happen.",
    },
};

/// Reshuffles the systems of equations.
pub const RESHUFFLE_POST: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 112,
    name: "reshufflePost",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Reshuffles the systems of equations.",
    },
};

/// Show information about expandable connector handling.
pub const SHOW_EXPANDABLE_INFO: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 113,
    name: "showExpandableInfo",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Show information about expandable connector handling.",
    },
};

/// Dumps the results of the postOptModule optimizeHomotopyCalls.
pub const DUMP_HOMOTOPY: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 114,
    name: "dumpHomotopy",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the results of the postOptModule optimizeHomotopyCalls.",
    },
};

/// Generates relocatable code.
pub const OMC_RELOCATABLE_FUNCTIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 115,
    name: "relocatableFunctions",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Generates relocatable code: all functions become function pointers and can be replaced at run-time.",
    },
};

/// Dumps .graphml files for the bipartite graph after Index Reduction.
pub const GRAPHML: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 116,
    name: "graphml",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps .graphml files for the bipartite graph after Index Reduction and a task graph for the SCCs.",
    },
};

/// Add MPI init and finalize to main method (CPPruntime).
pub const USEMPI: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 117,
    name: "useMPI",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Add MPI init and finalize to main method (CPPruntime).",
    },
};

/// Additional output for CSE module.
pub const DUMP_CSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 118,
    name: "dumpCSE",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Additional output for CSE module.",
    },
};

/// Additional output for CSE module (verbose).
pub const DUMP_CSE_VERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 119,
    name: "dumpCSE_verbose",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Additional output for CSE module.",
    },
};

/// Deactivates the pre-calculation of start values during compile-time.
pub const NO_START_CALC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 120,
    name: "disableStartCalc",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Deactivates the pre-calculation of start values during compile-time.",
    },
};

/// Solves linear systems with constant Jacobian and variable b-Vector symbolically
pub const CONSTJAC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 121,
    name: "constjac",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "solves linear systems with constant Jacobian and variable b-Vector symbolically",
    },
};

/// Outputs a xml-file that contains information for visualization.
pub const VISUAL_XML: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 122,
    name: "visxml",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Outputs a xml-file that contains information for visualization.",
    },
};

/// Activates vectorization in the backend.
pub const VECTORIZE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 123,
    name: "vectorize",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Activates vectorization in the backend.",
    },
};

/// Use the autotools project in the Resources folder to build missing external libraries.
pub const CHECK_EXT_LIBS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 124,
    name: "buildExternalLibs",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Use the autotools project in the Resources folder of the library to build missing external libraries.",
    },
};

/// Use the static simulation runtime libraries (C++ simulation runtime).
pub const RUNTIME_STATIC_LINKING: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 125,
    name: "runtimeStaticLinking",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Use the static simulation runtime libraries (C++ simulation runtime).",
    },
};

/// Dumps debug output for the modules sortEqnsVars.
pub const SORT_EQNS_AND_VARS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 126,
    name: "dumpSortEqnsAndVars",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps debug output for the modules sortEqnsVars.",
    },
};

/// Dump between steps of simplifyLoops
pub const DUMP_SIMPLIFY_LOOPS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 127,
    name: "dumpSimplifyLoops",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump between steps of simplifyLoops",
    },
};

/// Dump between steps of recursiveTearing
pub const DUMP_RTEARING: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 128,
    name: "dumpRecursiveTearing",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump between steps of recursiveTearing",
    },
};

/// For FMI 2.0 only dependency analysis will be performed.
pub const DIS_SYMJAC_FMI20: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 129,
    name: "disableDirectionalDerivatives",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "For FMI 2.0 only dependecy analysis will be perform.",
    },
};

/// Generates equations to calculate top level outputs only.
pub const EVAL_OUTPUT_ONLY: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 130,
    name: "evalOutputOnly",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Generates equations to calculate top level outputs only.",
    },
};

/// Embed the start values of variables and parameters into the c++ code.
pub const HARDCODED_START_VALUES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 131,
    name: "hardcodedStartValues",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Embed the start values of variables and parameters into the c++ code and do not read it from xml file.",
    },
};

/// Add functions to backend dumps.
pub const DUMP_FUNCTIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 132,
    name: "dumpFunctions",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Add functions to backend dumps.",
    },
};

/// Dumps debug output for the differentiation process.
pub const DEBUG_DIFFERENTIATION: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 133,
    name: "debugDifferentiation",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps debug output for the differentiation process.",
    },
};

/// Dumps verbose debug output for the differentiation process.
pub const DEBUG_DIFFERENTIATION_VERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 134,
    name: "debugDifferentiationVerbose",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps verbose debug output for the differentiation process.",
    },
};

/// Adds features to the FMI export that are considered experimental.
pub const FMU_EXPERIMENTAL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 135,
    name: "fmuExperimental",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Adds features to the FMI export that are considered experimental as of now.",
    },
};

/// Enables dumping of the information whether DGESV is used.
pub const DUMP_DGESV: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 136,
    name: "dumpdgesv",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables dumping of the information whether DGESV is used to solve linear systems.",
    },
};

/// The solver can switch partitions in the system.
pub const MULTIRATE_PARTITION: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 137,
    name: "multirate",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "The solver can switch partitions in the system.",
    },
};

/// This flag dumps all expression that are excluded from differentiation of a symbolic Jacobian.
pub const DUMP_EXCLUDED_EXP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 138,
    name: "dumpExcludedSymJacExps",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "This flags dumps all expression that are excluded from differentiation of a symbolic Jacobian.",
    },
};

/// Dumps debug output while creating symbolic jacobians for non-linear systems.
pub const DEBUG_ALGLOOP_JACOBIAN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 139,
    name: "debugAlgebraicLoopsJacobian",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps debug output while creating symbolic jacobians for non-linear systems.",
    },
};

/// Disables calculation of jacobians to detect if a SCC is linear or non-linear.
pub const DISABLE_JACSCC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 140,
    name: "disableJacsforSCC",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables calculation of jacobians to detect if a SCC is linear or non-linear.",
    },
};

/// Forces calculation analytical jacobian also for non-linear strong components with user-defined functions.
pub const FORCE_NLS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 141,
    name: "forceNLSanalyticJacobian",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Forces calculation analytical jacobian also for non-linear strong components with user-defined functions.",
    },
};

/// Dumps loop equation.
pub const DUMP_LOOPS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 142,
    name: "dumpLoops",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps loop equation.",
    },
};

/// Dumps loop equation and enhanced adjacency matrix.
pub const DUMP_LOOPS_VERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 143,
    name: "dumpLoopsVerbose",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps loop equation and enhanced adjacency matrix.",
    },
};

/// Used when bootstrapping to preserve the input output parsing of the code output by the list command.
pub const SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 144,
    name: "skipInputOutputSyntacticSugar",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Used when bootstrapping to preserve the input output parsing of the code output by the list command.",
    },
};

/// Instrument the source code to record memory allocations.
pub const OMC_RECORD_ALLOC_WORDS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 145,
    name: "metaModelicaRecordAllocWords",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Instrument the source code to record memory allocations.",
    },
};

/// Dumps total tearing information.
pub const TOTAL_TEARING_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 146,
    name: "totaltearingdump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps total tearing information.",
    },
};

/// Dumps verbose total tearing information.
pub const TOTAL_TEARING_DUMPVERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 147,
    name: "totaltearingdumpV",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps verbose total tearing information.",
    },
};

/// Enables code generation in parallel.
pub const PARALLEL_CODEGEN: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 148,
    name: "parallelCodegen",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables code generation in parallel (disable this if compiling a model causes you to run out of RAM).",
    },
};

/// Reports serialized sizes of various data structures used in the compiler.
pub const SERIALIZED_SIZE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 149,
    name: "reportSerializedSize",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Reports serialized sizes of various data structures used in the compiler.",
    },
};

/// When enabled, the environment is kept when entering the backend.
pub const BACKEND_KEEP_ENV_GRAPH: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 150,
    name: "backendKeepEnv",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "When enabled, the environment is kept when entering the backend.",
    },
};

/// Dumps debug output while inline function.
pub const DUMPBACKENDINLINE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 151,
    name: "dumpBackendInline",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps debug output while inline function.",
    },
};

/// Dumps debug output while inline function (verbose).
pub const DUMPBACKENDINLINE_VERBOSE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 152,
    name: "dumpBackendInlineVerbose",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps debug output while inline function.",
    },
};

/// Dumps the blt matrix in html file.
pub const BLT_MATRIX_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 153,
    name: "bltmatrixdump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the blt matrix in html file. IE seems to be very good in displaying large matrices.",
    },
};

/// Print notifications about bad usage of listAppend.
pub const LIST_REVERSE_WRONG_ORDER: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 154,
    name: "listAppendWrongOrder",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Print notifications about bad usage of listAppend.",
    },
};

/// This flag controls if partitioning is applied to the initialization system.
pub const PARTITION_INITIALIZATION: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 155,
    name: "partitionInitialization",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "This flag controls if partitioning is applied to the initialization system.",
    },
};

/// Dumps information for evaluating parameters.
pub const EVAL_PARAM_DUMP: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 156,
    name: "evalParameterDump",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information for evaluating parameters.",
    },
};

/// Checks the consistency of units in equation.
pub const NF_UNITCHECK: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 157,
    name: "frontEndUnitCheck",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Checks the consistency of units in equation.",
    },
};

/// Disables coloring algorithm while sparsity detection.
pub const DISABLE_COLORING: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 158,
    name: "disableColoring",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables coloring algorithm while sparsity detection.",
    },
};

/// Disables coloring algorithm while sparsity detection (merge).
pub const MERGE_ALGORITHM_SECTIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 159,
    name: "mergeAlgSections",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables coloring algorithm while sparsity detection.",
    },
};

/// Prints the iteration variables in the initialization and simulation DAE without nominal value.
pub const WARN_NO_NOMINAL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 160,
    name: "warnNoNominal",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints the iteration variables in the initialization and simulation DAE, which do not have a nominal value.",
    },
};

/// Prints all Reduce DAE debug information.
pub const REDUCE_DAE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 161,
    name: "backendReduceDAE",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints all Reduce DAE debug information.",
    },
};

/// Ignores cycles between constant/parameter components.
pub const IGNORE_CYCLES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 162,
    name: "ignoreCycles",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Ignores cycles between constant/parameter components.",
    },
};

/// Dumps alias sets with different start or nominal values.
pub const ALIAS_CONFLICTS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 163,
    name: "aliasConflicts",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps alias sets with different start or nominal values.",
    },
};

/// Makes Susan generate code using try/else to better debug match semantics.
pub const SUSAN_MATCHCONTINUE_DEBUG: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 164,
    name: "susanDebug",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Makes Susan generate code using try/else to better debug which function broke the expected match semantics.",
    },
};

/// Checks the consistency of units in equation (for the old front-end).
pub const OLD_FE_UNITCHECK: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 165,
    name: "oldFrontEndUnitCheck",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Checks the consistency of units in equation (for the old front-end).",
    },
};

/// When running execstat, also perform an extra full garbage collection.
pub const EXEC_STAT_EXTRA_GC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 166,
    name: "execstatGCcollect",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "When running execstat, also perform an extra full garbage collection.",
    },
};

/// Dump debug output for the DAEmode.
pub const DEBUG_DAEMODE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 167,
    name: "debugDAEmode",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dump debug output for the DAEmode.",
    },
};

/// Run scalarization in NF, default true.
pub const NF_SCALARIZE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 168,
    name: "nfScalarize",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Run scalarization in NF, default true.",
    },
};

/// Evaluate all functions with constant arguments in the new frontend.
pub const NF_EVAL_CONST_ARG_FUNCS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 169,
    name: "nfEvalConstArgFuncs",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Evaluate all functions with constant arguments in the new frontend.",
    },
};

/// Expand all unary/binary operations to scalar expressions in the new frontend.
pub const NF_EXPAND_OPERATIONS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 170,
    name: "nfExpandOperations",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Expand all unary/binary operations to scalar expressions in the new frontend.",
    },
};

/// Enables experimental new instantiation use in the OMC API.
pub const NF_API: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 171,
    name: "nfAPI",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables experimental new instantiation use in the OMC API.",
    },
};

/// Show DynamicSelect(static, dynamic) in annotations.
pub const NF_API_DYNAMIC_SELECT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 172,
    name: "nfAPIDynamicSelect",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Show DynamicSelect(static, dynamic) in annotations.",
    },
};

/// Enables error display for the experimental new instantiation use in the OMC API.
pub const NF_API_NOISE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 173,
    name: "nfAPINoise",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables error display for the experimental new instantiation use in the OMC API.",
    },
};

/// Disables the dependency analysis and generation for FMI 2.0.
pub const FMI20_DEPENDENCIES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 174,
    name: "disableFMIDependency",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Disables the dependency analysis and generation for FMI 2.0.",
    },
};

/// Makes a warning assert from min/max variable attributes instead of error.
pub const WARNING_MINMAX_ATTRIBUTES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 175,
    name: "warnMinMax",
    default: true,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Makes a warning assert from min/max variable attributes instead of error.",
    },
};

/// Expand all function arguments in the new frontend.
pub const NF_EXPAND_FUNC_ARGS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 176,
    name: "nfExpandFuncArgs",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Expand all function arguments in the new frontend.",
    },
};

/// Dumps the absyn representation of a program as a Julia representation
pub const DUMP_JL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 177,
    name: "dumpJL",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the absyn representation of a program as a Julia representation",
    },
};

/// Dumps the conversion process of analytical to structural singularities.
pub const DUMP_ASSC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 178,
    name: "dumpASSC",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the conversion process of analytical to structural singularities.",
    },
};

/// Generates all symbolic Jacobians with splitted constant parts.
pub const SPLIT_CONSTANT_PARTS_SYMJAC: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 179,
    name: "symJacConstantSplit",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Generates all symbolic Jacobians with splitted constant parts.",
    },
};

/// Force to export all fmi attributes to the modelDescription.xml.
pub const DUMP_FORCE_FMI_ATTRIBUTES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 180,
    name: "force-fmi-attributes",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Force to export all fmi attributes to the modelDescription.xml, including those which have default values",
    },
};

/// Dumps all the dataReconciliation extraction algorithm procedure
pub const DUMP_DATARECONCILIATION: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 181,
    name: "dataReconciliation",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps all the dataReconciliation extraction algorithm procedure",
    },
};

/// Use experimental array connection handler.
pub const ARRAY_CONNECT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 182,
    name: "arrayConnect",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Use experimental array connection handler.",
    },
};

/// Move all subscripts to the end of component references.
pub const COMBINE_SUBSCRIPTS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 183,
    name: "combineSubscripts",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Move all subscripts to the end of component references.",
    },
};

/// When opening a zmq connection, listen on all interfaces.
pub const ZMQ_LISTEN_TO_ALL: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 184,
    name: "zmqDangerousAcceptConnectionsFromAnywhere",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "When opening a zmq connection, listen on all interfaces instead of only connections from 127.0.0.1.",
    },
};

/// Dumps the rules when converting a package using a conversion script.
pub const DUMP_CONVERSION_RULES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 185,
    name: "dumpConversionRules",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps the rules when converting a package using a conversion script.",
    },
};

/// Prints out record types as part of the flat code.
pub const PRINT_RECORD_TYPES: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 186,
    name: "printRecordTypes",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Prints out record types as part of the flat code.",
    },
};

/// Dumps expressions before and after simplification.
pub const DUMP_SIMPLIFY: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 187,
    name: "dumpSimplify",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps expressions before and after simplification.",
    },
};

/// Dumps times for each backend module (only new backend).
pub const DUMP_BACKEND_CLOCKS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 188,
    name: "dumpBackendClocks",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps times for each backend module (only new backend).",
    },
};

/// Dumps information about set based graphs for efficient array handling.
pub const DUMP_SET_BASED_GRAPHS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 189,
    name: "dumpSetBasedGraphs",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about set based graphs for efficient array handling (only new frontend and new backend).",
    },
};

/// Enables automatic merging of components into arrays.
pub const MERGE_COMPONENTS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 190,
    name: "mergeComponents",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Enables automatic merging of components into arrays.",
    },
};

/// Dumps information about the slicing process (pseudo-array causalization).
pub const DUMP_SLICE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 191,
    name: "dumpSlice",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about the slicing process (pseudo-array causalization).",
    },
};

/// Turns on vectorization of bindings when scalarization is turned off.
pub const VECTORIZE_BINDINGS: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 192,
    name: "vectorizeBindings",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Turns on vectorization of bindings when scalarization is turned off.",
    },
};

/// Dumps information about the detected event functions.
pub const DUMP_EVENTS_FLAG: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 193,
    name: "dumpEvents",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about the detected event functions.",
    },
};

/// Dumps information about resizable parameter handling.
pub const DUMP_RESIZABLE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 194,
    name: "dumpResizable",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about resizable parameter handling.",
    },
};

/// Dumps information about equation solving.
pub const DUMP_SOLVE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 195,
    name: "dumpSolve",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps information about equation solving.",
    },
};

/// Forces scalarization to be done when it would normally be automatically disabled.
pub const FORCE_SCALARIZE: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 196,
    name: "forceScalarize",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Forces scalarization to be done when it would normally be automatically disabled.",
    },
};

/// Dumps debug output for the adjoint differentiation process in the new backend.
pub const DEBUG_ADJOINT: DebugFlag = DebugFlag::DEBUG_FLAG {
    index: 197,
    name: "debugAdjoint",
    default: false,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Dumps debug output for the adjoint differentiation process in the new backend.",
    },
};

// ============================================================================
// Configuration Flag constants (the most commonly used)
// ============================================================================

/// Sets debug flags. Use --help=debug to see available flags.
pub const DEBUG: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 1,
    name: "debug",
    shortname: Some("d"),
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::StrList(&[]),
    valid_options: None,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Sets debug flags. Use --help=debug to see available flags.",
    },
};

/// Displays the help text.
pub const HELP: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 2,
    name: "help",
    shortname: Some("h"),
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Str(""),
    valid_options: None,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Displays the help text. Use --help=topics for more information.",
    },
};

/// Used when running the testsuite.
pub const RUNNING_TESTSUITE: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 3,
    name: "running-testsuite",
    shortname: None,
    visibility: FlagVisibility::INTERNAL,
    default_value: DefaultFlagValue::Str(""),
    valid_options: None,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Used when running the testsuite.",
    },
};

/// Print the version and exit.
pub const SHOW_VERSION: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 4,
    name: "version",
    shortname: None,
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Bool(false),
    valid_options: None,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Print the version and exit.",
    },
};

/// Sets the target compiler to use.
pub const TARGET: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 5,
    name: "target",
    shortname: None,
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Str("gcc"),
    valid_options: Some(ValidOptionsStatic::STRING_OPTION {
        options: &[
            "gcc", "msvc", "msvc10", "msvc12", "msvc13", "msvc15",
            "msvc19", "vxworks69", "debugrt",
        ],
    }),
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Sets the target compiler to use.",
    },
};

/// Sets the grammar and semantics to accept.
pub const GRAMMAR: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 6,
    name: "grammar",
    shortname: Some("g"),
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Enum {
        value: MODELICA,
        valid_values: &[
            ("Modelica", MODELICA),
            ("MetaModelica", METAMODELICA),
            ("ParModelica", PARMODELICA),
            ("Optimica", OPTIMICA),
            ("PDEModelica", PDEMODELICA),
        ],
    },
    valid_options: Some(ValidOptionsStatic::STRING_OPTION {
        options: &[
            "Modelica", "MetaModelica", "ParModelica", "Optimica", "PDEModelica",
        ],
    }),
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Sets the grammar and semantics to accept.",
    },
};

/// Sets the annotation version that should be used.
pub const ANNOTATION_VERSION: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 7,
    name: "annotationVersion",
    shortname: None,
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Str("3.x"),
    valid_options: Some(ValidOptionsStatic::STRING_OPTION {
        options: &["1.x", "2.x", "3.x"],
    }),
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Sets the annotation version that should be used.",
    },
};

/// Sets the language standard that should be used.
pub const LANGUAGE_STANDARD: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 8,
    name: "std",
    shortname: None,
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Enum {
        value: 1000,
        valid_values: &[
            ("1.x", 10), ("2.x", 20),
            ("3.0", 30), ("3.1", 31),
            ("3.2", 32), ("3.3", 33),
            ("3.4", 34), ("3.5", 35),
            ("3.6", 36), ("latest", 1000),
            ("experimental", 9999),
        ],
    },
    valid_options: Some(ValidOptionsStatic::STRING_OPTION {
        options: &[
            "1.x", "2.x", "3.1", "3.2", "3.3", "3.4", "3.5", "3.6",
            "latest", "experimental",
        ],
    }),
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Sets the language standard that should be used.",
    },
};

/// Show error messages immediately when they happen.
pub const SHOW_ERROR_MESSAGES: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 9,
    name: "showErrorMessages",
    shortname: None,
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Bool(false),
    valid_options: None,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Show error messages immediately when they happen.",
    },
};

/// Show annotations.
pub const SHOW_ANNOTATIONS: ConfigFlag = ConfigFlag::CONFIG_FLAG {
    index: 10,
    name: "showAnnotations",
    shortname: None,
    visibility: FlagVisibility::EXTERNAL,
    default_value: DefaultFlagValue::Bool(false),
    valid_options: None,
    description: TranslatableContentStatic::GETTEXT {
        msgid: "Show annotations",
    },
};

// ============================================================================
// Helper: get element from a 1-indexed array (MetaModelica style)
// ============================================================================

/// Gets an element from a 0-indexed slice treating it as 1-indexed (MetaModelica style).
/// Index 1 returns the first element (index 0 in Vec).
pub fn array_get<T: Clone>(slice: &[T], index: i32) -> T {
    let idx = (index - 1) as usize;
    slice[idx].clone()
}

// ============================================================================
// Public functions
// ============================================================================

/// Loads the flags with get_global_root. Assumes flags have been loaded.
/// Corresponds to: flags := getGlobalRoot(Global.flagsIndex);
pub fn get_flags() -> Flag {
    let flags = global::get_global_root::<Flag>(global::FLAGS_INDEX);
    flags
}

/// Checks if a debug flag is set.
/// Corresponds to the `isSet` function in MetaModelica.
///
/// # Parameters
/// * `in_flag` - The debug flag to check
///
/// # Returns
/// `true` if the flag is enabled, `false` otherwise.
pub fn is_set(in_flag: &DebugFlag) -> bool {
    let index = in_flag.index();
    let flags = get_flags();
    match flags {
        Flag::FLAGS { debug_flags, .. } => {
            // MetaModelica arrays are 1-indexed
            let idx = (index - 1) as usize;
            if idx < debug_flags.len() {
                debug_flags[idx]
            } else {
                // Out of bounds - return the default
                match in_flag {
                    DebugFlag::DEBUG_FLAG { default, .. } => *default,
                }
            }
        }
        Flag::NO_FLAGS => {
            // No flags loaded - return default
            match in_flag {
                DebugFlag::DEBUG_FLAG { default, .. } => *default,
            }
        }
    }
}

/// Checks if a string list config flag contains a certain string.
/// Corresponds to the `isConfigFlagSet` function in MetaModelica.
///
/// # Parameters
/// * `in_flag` - The config flag (must be a STRING_LIST_FLAG type)
/// * `has_member` - The string to check for membership
///
/// # Returns
/// `true` if the string is in the flag's list, `false` otherwise.
pub fn is_config_flag_set(in_flag: &ConfigFlag, has_member: &str) -> bool {
    let values = get_config_string_list(in_flag);
    values.contains(&has_member.to_string())
}

/// Returns name of configuration flag.
/// Corresponds to the `getConfigName` function in MetaModelica.
pub fn get_config_name(in_flag: &ConfigFlag) -> &'static str {
    in_flag.name()
}

/// Returns the value of a configuration flag.
/// Corresponds to the `getConfigValue` function in MetaModelica.
pub fn get_config_value(in_flag: &ConfigFlag) -> FlagData {
    let index = in_flag.index();
    let flags = get_flags();
    match flags {
        Flag::FLAGS { ref config_flags, .. } => {
            // MetaModelica arrays are 1-indexed
            let idx = (index - 1) as usize;
            if idx < config_flags.len() {
                config_flags[idx].clone()
            } else {
                // Out of bounds - return the default value
                match in_flag {
                    ConfigFlag::CONFIG_FLAG { default_value, .. } => default_value.to_flag_data(),
                }
            }
        }
        Flag::NO_FLAGS => {
            // No flags loaded - return default
            match in_flag {
                ConfigFlag::CONFIG_FLAG { default_value, .. } => default_value.to_flag_data(),
            }
        }
    }
}

/// Returns the value of a boolean configuration flag.
/// Corresponds to the `getConfigBool` function in MetaModelica.
pub fn get_config_bool(in_flag: &ConfigFlag) -> bool {
    let value = get_config_value(in_flag);
    match value {
        FlagData::BOOL_FLAG { data } => data,
        _ => false,
    }
}

/// Returns the value of an integer configuration flag.
/// Corresponds to the `getConfigInt` function in MetaModelica.
pub fn get_config_int(in_flag: &ConfigFlag) -> i32 {
    let value = get_config_value(in_flag);
    match value {
        FlagData::INT_FLAG { data } => data,
        _ => 0,
    }
}

/// Returns the value of an integer list configuration flag.
/// Corresponds to the `getConfigIntList` function in MetaModelica.
pub fn get_config_int_list(in_flag: &ConfigFlag) -> Vec<i32> {
    let value = get_config_value(in_flag);
    match value {
        FlagData::INT_LIST_FLAG { data } => data,
        _ => vec![],
    }
}

/// Returns the value of a real configuration flag.
/// Corresponds to the `getConfigReal` function in MetaModelica.
pub fn get_config_real(in_flag: &ConfigFlag) -> f64 {
    let value = get_config_value(in_flag);
    match value {
        FlagData::REAL_FLAG { data } => data,
        _ => 0.0,
    }
}

/// Returns the value of a string configuration flag.
/// Corresponds to the `getConfigString` function in MetaModelica.
pub fn get_config_string(in_flag: &ConfigFlag) -> String {
    let value = get_config_value(in_flag);
    match value {
        FlagData::STRING_FLAG { data } => data,
        _ => String::new(),
    }
}

/// Returns the value of a multiple-string configuration flag.
/// Corresponds to the `getConfigStringList` function in MetaModelica.
pub fn get_config_string_list(in_flag: &ConfigFlag) -> Vec<String> {
    let value = get_config_value(in_flag);
    match value {
        FlagData::STRING_LIST_FLAG { data } => data,
        _ => vec![],
    }
}

/// Returns the value of an enumeration configuration flag.
/// Corresponds to the `getConfigEnum` function in MetaModelica.
pub fn get_config_enum(in_flag: &ConfigFlag) -> i32 {
    let value = get_config_value(in_flag);
    match value {
        FlagData::ENUM_FLAG { data, .. } => data,
        _ => 0,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_data_helpers() {
        assert!(bool_flag(true).bool_value() == Some(true));
        assert!(int_flag(42).int_value() == Some(42));
        assert!(real_flag(3.14).real_value() == Some(3.14));
        assert!(string_flag("hello").string_value() == Some("hello"));
        assert!(enum_flag(1, vec![("a".to_string(), 1)]).enum_value() == Some(1));
    }

    #[test]
    fn test_flag_visibility() {
        assert_eq!(internal(), FlagVisibility::INTERNAL);
        assert_eq!(external(), FlagVisibility::EXTERNAL);
    }

    #[test]
    fn test_debug_flag_constants() {
        assert_eq!(FAILTRACE.index(), 1);
        assert_eq!(FAILTRACE.name(), "failtrace");
        assert!(!FAILTRACE.name().is_empty());

        assert_eq!(EVENTS.index(), 5);
        assert_eq!(EVENTS.name(), "events");
        // Flags not loaded yet - is_set returns the default value
        // EVENTS default is true
        assert!(is_set(&EVENTS));
        // FAILTRACE default is false
        assert!(!is_set(&FAILTRACE));
    }

    #[test]
    fn test_config_flag_constants() {
        assert_eq!(DEBUG.index(), 1);
        assert_eq!(HELP.index(), 2);
        assert_eq!(SHOW_VERSION.index(), 4);
        assert_eq!(TARGET.index(), 5);
        assert_eq!(GRAMMAR.index(), 6);
        assert_eq!(LANGUAGE_STANDARD.index(), 8);
    }

    #[test]
    fn test_grammar_constants() {
        assert_eq!(MODELICA, 1);
        assert_eq!(METAMODELICA, 2);
        assert_eq!(PARMODELICA, 3);
        assert_eq!(OPTIMICA, 4);
        assert_eq!(PDEMODELICA, 5);
    }

    #[test]
    fn test_fmi_constants() {
        assert_eq!(FMI_NONE, 1);
        assert_eq!(FMI_INTERNAL, 2);
        assert_eq!(FMI_PROTECTED, 3);
        assert_eq!(FMI_BLACKBOX, 4);
    }

    #[test]
    fn test_array_get() {
        let v = vec![10, 20, 30, 40];
        assert_eq!(array_get(&v, 1), 10); // 1-indexed: first element
        assert_eq!(array_get(&v, 2), 20);
        assert_eq!(array_get(&v, 4), 40);
    }

    #[test]
    fn test_get_flags_no_flags() {
        // Without runtime initialization, get_flags returns NO_FLAGS
        let flags = get_flags();
        assert!(matches!(flags, Flag::NO_FLAGS));
    }

    #[test]
    fn test_is_set_no_flags_returns_default() {
        // Without flags loaded, is_set should return the default value
        assert!(!is_set(&FAILTRACE)); // default: false
        // Note: EVENTS default is true, but is_set returns false because
        // get_flags() returns NO_FLAGS which matches the else branch returning false
    }

    #[test]
    fn test_get_config_value_no_flags_returns_default() {
        let val = get_config_value(&SHOW_VERSION);
        assert!(matches!(val, FlagData::BOOL_FLAG { data } if data == false));

        let val = get_config_value(&TARGET);
        assert!(matches!(val, FlagData::STRING_FLAG { ref data } if data == "gcc"));
    }

    #[test]
    fn test_get_config_bool_no_flags() {
        assert!(!get_config_bool(&SHOW_VERSION));
    }

    #[test]
    fn test_get_config_string_no_flags() {
        assert_eq!(get_config_string(&TARGET), "gcc");
    }

    #[test]
    fn test_get_config_string_list_empty() {
        let result = get_config_string_list(&DEBUG);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_config_name() {
        assert_eq!(get_config_name(&DEBUG), "debug");
        assert_eq!(get_config_name(&HELP), "help");
        assert_eq!(get_config_name(&TARGET), "target");
    }
}
