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

use crate::Corba;
use crate::Error;
use crate::ErrorExt;
use crate::Flags;
use crate::Gettext;
use crate::Global;
use crate::Print;
use crate::Settings;
use crate::StringUtil;
use crate::System;
use crate::Util;
use openmodelica_util_datatypes_basic::List;

// This is a list of all debug flags, to keep track of which flags are used. A
// flag can not be used unless it's in this list, and the list is checked at
// initialization so that all flags are sorted by index (and thus have unique
// indices).
pub static allDebugFlags: std::sync::LazyLock<Arc<metamodelica::List<Flags::DebugFlag>>> = std::sync::LazyLock::new(|| { list![Flags::FAILTRACE.clone(), Flags::CEVAL.clone(), Flags::CHECK_BACKEND_DAE.clone(), Flags::PTHREADS.clone(), Flags::EVENTS.clone(), Flags::DUMP_INLINE_SOLVER.clone(), Flags::EVAL_FUNC.clone(), Flags::GEN.clone(), Flags::DYN_LOAD.clone(), Flags::GENERATE_CODE_CHEAT.clone(), Flags::CGRAPH_GRAPHVIZ_FILE.clone(), Flags::CGRAPH_GRAPHVIZ_SHOW.clone(), Flags::GC_PROF.clone(), Flags::CHECK_DAE_CREF_TYPE.clone(), Flags::CHECK_ASUB.clone(), Flags::INSTANCE.clone(), Flags::CACHE.clone(), Flags::RML.clone(), Flags::TAIL.clone(), Flags::LOOKUP.clone(), Flags::PATTERNM_SKIP_FILTER_UNUSED_AS_BINDINGS.clone(), Flags::PATTERNM_ALL_INFO.clone(), Flags::PATTERNM_DCE.clone(), Flags::PATTERNM_MOVE_LAST_EXP.clone(), Flags::EXPERIMENTAL_REDUCTIONS.clone(), Flags::EVAL_PARAM.clone(), Flags::TYPES.clone(), Flags::SHOW_STATEMENT.clone(), Flags::DUMP.clone(), Flags::DUMP_GRAPHVIZ.clone(), Flags::EXEC_STAT.clone(), Flags::TRANSFORMS_BEFORE_DUMP.clone(), Flags::DAE_DUMP_GRAPHV.clone(), Flags::INTERACTIVE_TCP.clone(), Flags::INTERACTIVE_CORBA.clone(), Flags::INTERACTIVE_DUMP.clone(), Flags::RELIDX.clone(), Flags::DUMP_REPL.clone(), Flags::DUMP_FP_REPL.clone(), Flags::DUMP_PARAM_REPL.clone(), Flags::DUMP_PP_REPL.clone(), Flags::DUMP_EA_REPL.clone(), Flags::DEBUG_ALIAS.clone(), Flags::TEARING_DUMP.clone(), Flags::JAC_DUMP.clone(), Flags::JAC_DUMP2.clone(), Flags::DUMP_BINDINGS.clone(), Flags::DUMP_SORTING.clone(), Flags::DUMP_SPARSE.clone(), Flags::DUMP_SPARSE_VERBOSE.clone(), Flags::BLT_DUMP.clone(), Flags::DUMMY_SELECT.clone(), Flags::DUMP_DAE_LOW.clone(), Flags::DUMP_INDX_DAE.clone(), Flags::OPT_DAE_DUMP.clone(), Flags::EXEC_HASH.clone(), Flags::PARAM_DLOW_DUMP.clone(), Flags::DUMP_ENCAPSULATECONDITIONS.clone(), Flags::SHORT_OUTPUT.clone(), Flags::COUNT_OPERATIONS.clone(), Flags::CGRAPH.clone(), Flags::UPDMOD.clone(), Flags::STATIC.clone(), Flags::TPL_PERF_TIMES.clone(), Flags::CHECK_SIMPLIFY.clone(), Flags::SCODE_INST.clone(), Flags::WRITE_TO_BUFFER.clone(), Flags::DUMP_BACKENDDAE_INFO.clone(), Flags::GEN_DEBUG_SYMBOLS.clone(), Flags::DUMP_STATESELECTION_INFO.clone(), Flags::DUMP_EQNINORDER.clone(), Flags::SEMILINEAR.clone(), Flags::UNCERTAINTIES.clone(), Flags::SHOW_START_ORIGIN.clone(), Flags::DUMP_SIMCODE.clone(), Flags::DUMP_INITIAL_SYSTEM.clone(), Flags::GRAPH_INST.clone(), Flags::GRAPH_INST_RUN_DEP.clone(), Flags::GRAPH_INST_GEN_GRAPH.clone(), Flags::DUMP_CONST_REPL.clone(), Flags::SHOW_EQUATION_SOURCE.clone(), Flags::LS_ANALYTIC_JACOBIAN.clone(), Flags::NLS_ANALYTIC_JACOBIAN.clone(), Flags::INLINE_SOLVER.clone(), Flags::HPCOM.clone(), Flags::INITIALIZATION.clone(), Flags::INLINE_FUNCTIONS.clone(), Flags::DUMP_SCC_GRAPHML.clone(), Flags::TEARING_DUMPVERBOSE.clone(), Flags::DISABLE_SINGLE_FLOW_EQ.clone(), Flags::DUMP_DISCRETEVARS_INFO.clone(), Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone(), Flags::INFO_XML_OPERATIONS.clone(), Flags::HPCOM_DUMP.clone(), Flags::RESOLVE_LOOPS_DUMP.clone(), Flags::DISABLE_WINDOWS_PATH_CHECK_WARNING.clone(), Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone(), Flags::IMPL_ODE.clone(), Flags::EVAL_FUNC_DUMP.clone(), Flags::PRINT_STRUCTURAL.clone(), Flags::ITERATION_VARS.clone(), Flags::ALLOW_RECORD_TOO_MANY_FIELDS.clone(), Flags::HPCOM_MEMORY_OPT.clone(), Flags::DUMP_SYNCHRONOUS.clone(), Flags::STRIP_PREFIX.clone(), Flags::DO_SCODE_DEP.clone(), Flags::SHOW_INST_CACHE_INFO.clone(), Flags::DUMP_UNIT.clone(), Flags::DUMP_EQ_UNIT.clone(), Flags::DUMP_EQ_UNIT_STRUCT.clone(), Flags::SHOW_DAE_GENERATION.clone(), Flags::RESHUFFLE_POST.clone(), Flags::SHOW_EXPANDABLE_INFO.clone(), Flags::DUMP_HOMOTOPY.clone(), Flags::OMC_RELOCATABLE_FUNCTIONS.clone(), Flags::GRAPHML.clone(), Flags::USEMPI.clone(), Flags::DUMP_CSE.clone(), Flags::DUMP_CSE_VERBOSE.clone(), Flags::NO_START_CALC.clone(), Flags::CONSTJAC.clone(), Flags::VISUAL_XML.clone(), Flags::VECTORIZE.clone(), Flags::CHECK_EXT_LIBS.clone(), Flags::RUNTIME_STATIC_LINKING.clone(), Flags::SORT_EQNS_AND_VARS.clone(), Flags::DUMP_SIMPLIFY_LOOPS.clone(), Flags::DUMP_RTEARING.clone(), Flags::DIS_SYMJAC_FMI20.clone(), Flags::EVAL_OUTPUT_ONLY.clone(), Flags::HARDCODED_START_VALUES.clone(), Flags::DUMP_FUNCTIONS.clone(), Flags::DEBUG_DIFFERENTIATION.clone(), Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone(), Flags::FMU_EXPERIMENTAL.clone(), Flags::DUMP_DGESV.clone(), Flags::MULTIRATE_PARTITION.clone(), Flags::DUMP_EXCLUDED_EXP.clone(), Flags::DEBUG_ALGLOOP_JACOBIAN.clone(), Flags::DISABLE_JACSCC.clone(), Flags::FORCE_NLS_ANALYTIC_JACOBIAN.clone(), Flags::DUMP_LOOPS.clone(), Flags::DUMP_LOOPS_VERBOSE.clone(), Flags::SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR.clone(), Flags::OMC_RECORD_ALLOC_WORDS.clone(), Flags::TOTAL_TEARING_DUMP.clone(), Flags::TOTAL_TEARING_DUMPVERBOSE.clone(), Flags::PARALLEL_CODEGEN.clone(), Flags::SERIALIZED_SIZE.clone(), Flags::BACKEND_KEEP_ENV_GRAPH.clone(), Flags::DUMPBACKENDINLINE.clone(), Flags::DUMPBACKENDINLINE_VERBOSE.clone(), Flags::BLT_MATRIX_DUMP.clone(), Flags::LIST_REVERSE_WRONG_ORDER.clone(), Flags::PARTITION_INITIALIZATION.clone(), Flags::EVAL_PARAM_DUMP.clone(), Flags::NF_UNITCHECK.clone(), Flags::DISABLE_COLORING.clone(), Flags::MERGE_ALGORITHM_SECTIONS.clone(), Flags::WARN_NO_NOMINAL.clone(), Flags::REDUCE_DAE.clone(), Flags::IGNORE_CYCLES.clone(), Flags::ALIAS_CONFLICTS.clone(), Flags::SUSAN_MATCHCONTINUE_DEBUG.clone(), Flags::OLD_FE_UNITCHECK.clone(), Flags::EXEC_STAT_EXTRA_GC.clone(), Flags::DEBUG_DAEMODE.clone(), Flags::NF_SCALARIZE.clone(), Flags::NF_EVAL_CONST_ARG_FUNCS.clone(), Flags::NF_EXPAND_OPERATIONS.clone(), Flags::NF_API.clone(), Flags::NF_API_DYNAMIC_SELECT.clone(), Flags::NF_API_NOISE.clone(), Flags::FMI20_DEPENDENCIES.clone(), Flags::WARNING_MINMAX_ATTRIBUTES.clone(), Flags::NF_EXPAND_FUNC_ARGS.clone(), Flags::DUMP_JL.clone(), Flags::DUMP_ASSC.clone(), Flags::SPLIT_CONSTANT_PARTS_SYMJAC.clone(), Flags::DUMP_FORCE_FMI_ATTRIBUTES.clone(), Flags::DUMP_DATARECONCILIATION.clone(), Flags::ARRAY_CONNECT.clone(), Flags::COMBINE_SUBSCRIPTS.clone(), Flags::ZMQ_LISTEN_TO_ALL.clone(), Flags::DUMP_CONVERSION_RULES.clone(), Flags::PRINT_RECORD_TYPES.clone(), Flags::DUMP_SIMPLIFY.clone(), Flags::DUMP_BACKEND_CLOCKS.clone(), Flags::DUMP_SET_BASED_GRAPHS.clone(), Flags::MERGE_COMPONENTS.clone(), Flags::DUMP_SLICE.clone(), Flags::VECTORIZE_BINDINGS.clone(), Flags::DUMP_EVENTS.clone(), Flags::DUMP_RESIZABLE.clone(), Flags::DUMP_SOLVE.clone(), Flags::FORCE_SCALARIZE.clone(), Flags::DEBUG_ADJOINT.clone(), Flags::FLOW_ALIAS_ELIMINATION.clone()] });

// This is a list of all configuration flags. A flag can not be used unless it's
// in this list, and the list is checked at initialization so that all flags are
// sorted by index (and thus have unique indices).
pub static allConfigFlags: std::sync::LazyLock<Arc<metamodelica::List<Flags::ConfigFlag>>> = std::sync::LazyLock::new(|| { list![Flags::DEBUG.clone(), Flags::HELP.clone(), Flags::RUNNING_TESTSUITE.clone(), Flags::SHOW_VERSION.clone(), Flags::TARGET.clone(), Flags::GRAMMAR.clone(), Flags::ANNOTATION_VERSION.clone(), Flags::LANGUAGE_STANDARD.clone(), Flags::SHOW_ERROR_MESSAGES.clone(), Flags::SHOW_ANNOTATIONS.clone(), Flags::NO_SIMPLIFY.clone(), Flags::PRE_OPT_MODULES.clone(), Flags::CHEAPMATCHING_ALGORITHM.clone(), Flags::MATCHING_ALGORITHM.clone(), Flags::INDEX_REDUCTION_METHOD.clone(), Flags::POST_OPT_MODULES.clone(), Flags::SIMCODE_TARGET.clone(), Flags::ORDER_CONNECTIONS.clone(), Flags::TYPE_INFO.clone(), Flags::KEEP_ARRAYS.clone(), Flags::MODELICA_OUTPUT.clone(), Flags::SILENT.clone(), Flags::CORBA_SESSION.clone(), Flags::NUM_PROC.clone(), Flags::INST_CLASS.clone(), Flags::VECTORIZATION_LIMIT.clone(), Flags::SIMULATION_CG.clone(), Flags::EVAL_PARAMS_IN_ANNOTATIONS.clone(), Flags::CHECK_MODEL.clone(), Flags::CEVAL_EQUATION.clone(), Flags::UNIT_CHECKING.clone(), Flags::GENERATE_LABELED_SIMCODE.clone(), Flags::REDUCE_TERMS.clone(), Flags::REDUCTION_METHOD.clone(), Flags::DEMO_MODE.clone(), Flags::LOCALE_FLAG.clone(), Flags::DEFAULT_OPENCL_DEVICE.clone(), Flags::MAXTRAVERSALS.clone(), Flags::DUMP_TARGET.clone(), Flags::DELAY_BREAK_LOOP.clone(), Flags::TEARING_METHOD.clone(), Flags::TEARING_HEURISTIC.clone(), Flags::SCALARIZE_MINMAX.clone(), Flags::STRICT.clone(), Flags::SCALARIZE_BINDINGS.clone(), Flags::CORBA_OBJECT_REFERENCE_FILE_PATH.clone(), Flags::HPCOM_SCHEDULER.clone(), Flags::HPCOM_CODE.clone(), Flags::REWRITE_RULES_FILE.clone(), Flags::REPLACE_HOMOTOPY.clone(), Flags::GENERATE_DYNAMIC_JACOBIAN.clone(), Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), Flags::INT_ENUM_CONVERSION.clone(), Flags::PROFILING_LEVEL.clone(), Flags::RESHUFFLE.clone(), Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone(), Flags::MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM.clone(), Flags::CPP_FLAGS.clone(), Flags::REMOVE_SIMPLE_EQUATIONS.clone(), Flags::DYNAMIC_TEARING.clone(), Flags::SYM_SOLVER.clone(), Flags::LOOP2CON.clone(), Flags::FORCE_TEARING.clone(), Flags::SIMPLIFY_LOOPS.clone(), Flags::RTEARING.clone(), Flags::FLOW_THRESHOLD.clone(), Flags::MATRIX_FORMAT.clone(), Flags::PARTLINTORN.clone(), Flags::INIT_OPT_MODULES.clone(), Flags::MAX_MIXED_DETERMINED_INDEX.clone(), Flags::USE_LOCAL_DIRECTION.clone(), Flags::DEFAULT_OPT_MODULES_ORDERING.clone(), Flags::PRE_OPT_MODULES_ADD.clone(), Flags::PRE_OPT_MODULES_SUB.clone(), Flags::POST_OPT_MODULES_ADD.clone(), Flags::POST_OPT_MODULES_SUB.clone(), Flags::INIT_OPT_MODULES_ADD.clone(), Flags::INIT_OPT_MODULES_SUB.clone(), Flags::PERMISSIVE.clone(), Flags::HETS.clone(), Flags::DEFAULT_CLOCK_PERIOD.clone(), Flags::INST_CACHE_SIZE.clone(), Flags::MAX_SIZE_LINEAR_TEARING.clone(), Flags::MAX_SIZE_NONLINEAR_TEARING.clone(), Flags::NO_TEARING_FOR_COMPONENT.clone(), Flags::CT_STATE_MACHINES.clone(), Flags::DAE_MODE.clone(), Flags::INLINE_METHOD.clone(), Flags::SET_TEARING_VARS.clone(), Flags::SET_RESIDUAL_EQNS.clone(), Flags::IGNORE_COMMAND_LINE_OPTIONS_ANNOTATION.clone(), Flags::CALCULATE_SENSITIVITIES.clone(), Flags::ALARM.clone(), Flags::TOTAL_TEARING.clone(), Flags::IGNORE_SIMULATION_FLAGS_ANNOTATION.clone(), Flags::DYNAMIC_TEARING_FOR_INITIALIZATION.clone(), Flags::PREFER_TVARS_WITH_START_VALUE.clone(), Flags::EQUATIONS_PER_FILE.clone(), Flags::EVALUATE_FINAL_PARAMS.clone(), Flags::EVALUATE_PROTECTED_PARAMS.clone(), Flags::REPLACE_EVALUATED_PARAMS.clone(), Flags::CONDENSE_ARRAYS.clone(), Flags::WFC_ADVANCED.clone(), Flags::GRAPHICS_EXP_MODE.clone(), Flags::TEARING_STRICTNESS.clone(), Flags::INTERACTIVE.clone(), Flags::ZEROMQ_FILE_SUFFIX.clone(), Flags::HOMOTOPY_APPROACH.clone(), Flags::IGNORE_REPLACEABLE.clone(), Flags::LABELED_REDUCTION.clone(), Flags::DISABLE_EXTRA_LABELING.clone(), Flags::LOAD_MSL_MODEL.clone(), Flags::LOAD_PACKAGE_FILE.clone(), Flags::BUILDING_FMU.clone(), Flags::BUILDING_MODEL.clone(), Flags::POST_OPT_MODULES_DAE.clone(), Flags::EVAL_LOOP_LIMIT.clone(), Flags::EVAL_RECURSION_LIMIT.clone(), Flags::SINGLE_INSTANCE_AGLSOLVER.clone(), Flags::SHOW_STRUCTURAL_ANNOTATIONS.clone(), Flags::INITIAL_STATE_SELECTION.clone(), Flags::LINEARIZATION_DUMP_LANGUAGE.clone(), Flags::NO_ASSC.clone(), Flags::FULL_ASSC.clone(), Flags::REAL_ASSC.clone(), Flags::INIT_ASSC.clone(), Flags::MAX_SIZE_ASSC.clone(), Flags::USE_ZEROMQ_IN_SIM.clone(), Flags::ZEROMQ_PUB_PORT.clone(), Flags::ZEROMQ_SUB_PORT.clone(), Flags::ZEROMQ_JOB_ID.clone(), Flags::ZEROMQ_SERVER_ID.clone(), Flags::ZEROMQ_CLIENT_ID.clone(), Flags::FMI_VERSION.clone(), Flags::BASE_MODELICA.clone(), Flags::FMI_FILTER.clone(), Flags::FMI_SOURCES.clone(), Flags::FMI_FLAGS.clone(), Flags::FMU_CMAKE_BUILD.clone(), Flags::NEW_BACKEND.clone(), Flags::PARMODAUTO.clone(), Flags::INTERACTIVE_PORT.clone(), Flags::ALLOW_NON_STANDARD_MODELICA.clone(), Flags::EXPORT_CLOCKS_IN_MODELDESCRIPTION.clone(), Flags::LINK_TYPE.clone(), Flags::TEARING_ALWAYS_DERIVATIVES.clone(), Flags::DUMP_FLAT_MODEL.clone(), Flags::SIMULATION.clone(), Flags::OBFUSCATE.clone(), Flags::FMU_RUNTIME_DEPENDS.clone(), Flags::FRONTEND_INLINE.clone(), Flags::EXPOSE_LOCAL_IOS.clone(), Flags::BASE_MODELICA_FORMAT.clone(), Flags::BASE_MODELICA_OPTIONS.clone(), Flags::DEBUG_FOLLOW_EQUATIONS.clone(), Flags::MAX_SIZE_LINEARIZATION.clone(), Flags::RESIZABLE_ARRAYS.clone(), Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone(), Flags::LOAD_MISSING_LIBRARIES.clone(), Flags::CAUSALIZE_DAE_MODE.clone(), Flags::SIM_CODE_SCALARIZE.clone(), Flags::EXECUTE_COMMAND.clone(), Flags::MOO_DYNAMIC_OPTIMIZATION.clone()] });

pub fn new(mut inArgs: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let _ = loadFlags(true)?;
    outArgs = readArgs(inArgs.clone())?;
    Ok(outArgs)
}

pub fn saveFlags(mut inFlags: Flags::Flag) -> () {
    crate::Globals::flagsIndex.with(|__root| *__root.borrow_mut() = inFlags.clone());
    ()
}

fn createConfigFlags() -> metamodelica::Array<Flags::FlagData> {
    let mut configFlags: metamodelica::Array<Flags::FlagData>;
    configFlags = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Flags::FlagData>> = metamodelica::nil();
        for mut flag in (allConfigFlags.clone()).into_iter().cloned() {
            let __x = flag.defaultValue.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    configFlags
}

fn createDebugFlags() -> metamodelica::Array<bool> {
    let mut debugFlags: metamodelica::Array<bool>;
    debugFlags = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut flag in (allDebugFlags.clone()).into_iter().cloned() {
            let __x = flag.default.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    debugFlags
}

pub fn loadFlags(mut initialize: bool) -> Result<Flags::Flag> {
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    match '__try0: {
        flags = Flags::getFlags(true);
        Ok::<_, anyhow::Error>((flags.clone(),))
    } {
        Ok((__try0_o0,)) => {
            flags = __try0_o0;
        }
        Err(_) => {
            if initialize.clone() {
                checkDebugFlags()?;
                checkConfigFlags()?;
                flags = Flags::Flag::FLAGS { debugFlags: createDebugFlags(), configFlags: createConfigFlags() };
                saveFlags(flags.clone());
            } else {
                println!("{}", (literal!("Flag loading failed!\n")).clone());
                flags = crate::Flags::Flag::NO_FLAGS;
            }
        }
    }
    Ok(flags)
}

pub fn backupFlags() -> Result<Flags::Flag> {
    let mut outFlags: Flags::Flag = Flags::Flag::NO_FLAGS;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS { debugFlags: __pa0, configFlags: __pa1 } = (loadFlags(true)?) else { bail!("pattern mismatch") };
    debug_flags = __pa0.clone();
    config_flags = __pa1.clone();
    outFlags = Flags::Flag::FLAGS { debugFlags: metamodelica::arrayFromVec(debug_flags.clone().borrow().clone()), configFlags: metamodelica::arrayFromVec(config_flags.clone().borrow().clone()) };
    Ok(outFlags)
}

pub fn resetDebugFlags() -> Result<()> {
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS { debugFlags: _, configFlags: __pa0 } = (loadFlags(true)?) else { bail!("pattern mismatch") };
    config_flags = __pa0.clone();
    debug_flags = createDebugFlags();
    saveFlags(Flags::Flag::FLAGS { debugFlags: debug_flags.clone(), configFlags: config_flags.clone() });
    Ok(())
}

pub fn resetConfigFlags() -> Result<()> {
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS { debugFlags: __pa0, configFlags: _ } = (loadFlags(true)?) else { bail!("pattern mismatch") };
    debug_flags = __pa0.clone();
    config_flags = createConfigFlags();
    saveFlags(Flags::Flag::FLAGS { debugFlags: debug_flags.clone(), configFlags: config_flags.clone() });
    Ok(())
}

fn checkDebugFlags() -> Result<()> {
    let mut index: i32 = 0;
    let mut err_str: ArcStr = arcstr::literal!("");
    for mut flag in &*allDebugFlags.clone() {
        let mut flag = flag.clone();
        index = index.clone() + 1;
        if flag.index.clone() != index.clone() {
            err_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Invalid flag '")); __mm_s.push_str(&*flag.name.clone()); __mm_s.push_str(&*literal!("' with index ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", flag.index.clone()))); __mm_s.push_str(&*literal!(" (expected ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); __mm_s.push_str(&*literal!(") in Flags.allDebugFlags. Make sure that all flags are present and ordered correctly!")); ArcStr::from(__mm_s) }).clone();
            Error::terminateError((err_str.clone()).clone(), metamodelica::sourceInfo!())?;
        }
    }
    Ok(())
}

fn checkConfigFlags() -> Result<()> {
    let mut index: i32 = 0;
    let mut err_str: ArcStr = arcstr::literal!("");
    for mut flag in &*allConfigFlags.clone() {
        let mut flag = flag.clone();
        index = index.clone() + 1;
        if flag.index.clone() != index.clone() {
            err_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Invalid flag '")); __mm_s.push_str(&*flag.name.clone()); __mm_s.push_str(&*literal!("' with index ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", flag.index.clone()))); __mm_s.push_str(&*literal!(" (expected ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); __mm_s.push_str(&*literal!(") in Flags.allConfigFlags. Make sure that all flags are present and ordered correctly!")); ArcStr::from(__mm_s) }).clone();
            Error::terminateError((err_str.clone()).clone(), metamodelica::sourceInfo!())?;
        }
    }
    Ok(())
}

pub fn set(mut inFlag: Flags::DebugFlag, mut inValue: bool) -> Result<bool> {
    let mut outOldValue: bool = false;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS { debugFlags: __pa0, configFlags: __pa1 } = (loadFlags(true)?) else { bail!("pattern mismatch") };
    debug_flags = __pa0.clone();
    config_flags = __pa1.clone();
    (debug_flags, outOldValue) = updateDebugFlagArray(debug_flags.clone(), inValue.clone(), inFlag.clone())?;
    saveFlags(Flags::Flag::FLAGS { debugFlags: debug_flags.clone(), configFlags: config_flags.clone() });
    Ok(outOldValue)
}

pub fn enableDebug(mut inFlag: Flags::DebugFlag) -> Result<bool> {
    let mut outOldValue: bool = false;
    outOldValue = set(inFlag.clone(), true)?;
    Ok(outOldValue)
}

pub fn disableDebug(mut inFlag: Flags::DebugFlag) -> Result<bool> {
    let mut outOldValue: bool = false;
    outOldValue = set(inFlag.clone(), false)?;
    Ok(outOldValue)
}

pub fn getConfigOptionsStringList(mut inFlag: Flags::ConfigFlag) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outOptions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outComments: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outOptions, outComments) = (match inFlag.clone() {
        Flags::ConfigFlag { validOptions: Some(Flags::ValidOptions::STRING_DESC_OPTION { options: mut options }), .. } => {
            (List::map(options.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _))), List::mapMap(options.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)), (std::sync::Arc::new(Gettext::translateContent) as std::sync::Arc<dyn ::std::ops::Fn(Gettext::TranslatableContent) -> Result<ArcStr> + 'static>)))
        },
        Flags::ConfigFlag { validOptions: Some(Flags::ValidOptions::STRING_OPTION { options: ref flags }), .. } => {
            (flags.clone(), List::fill((literal!("")).clone(), (flags.clone().len() as i32)))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outOptions, outComments))
}

fn updateDebugFlagArray(mut inFlags: metamodelica::Array<bool>, mut inValue: bool, mut inFlag: Flags::DebugFlag) -> Result<(metamodelica::Array<bool>, bool)> {
    let mut outFlags: metamodelica::Array<bool>;
    let mut outOldValue: bool = false;
    let mut index: i32 = 0;
    let Flags::DEBUG_FLAG { index: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    index = __pa0.clone();
    outOldValue = inFlags.clone().borrow()[(index.clone()-1) as usize].clone();
    outFlags = {let _arr = inFlags.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = inValue.clone(); _arr};
    Ok((outFlags, outOldValue))
}

fn updateConfigFlagArray(mut inFlags: metamodelica::Array<Flags::FlagData>, mut inValue: Flags::FlagData, mut inFlag: Flags::ConfigFlag) -> Result<metamodelica::Array<Flags::FlagData>> {
    let mut outFlags: metamodelica::Array<Flags::FlagData>;
    let mut index: i32 = 0;
    let Flags::CONFIG_FLAG { index: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    index = __pa0.clone();
    outFlags = {let _arr = inFlags.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = inValue.clone(); _arr};
    applySideEffects(inFlag.clone(), inValue.clone())?;
    Ok(outFlags)
}

pub fn readArgs(mut inArgs: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    let mut numError: i32 = 0;
    let mut arg: ArcStr = arcstr::literal!("");
    let mut rest_args: Arc<metamodelica::List<ArcStr>> = inArgs.clone();
    numError = Error::getNumErrorMessages();
    flags = loadFlags(true)?;
    while !(rest_args.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        if arg.clone() == literal!("--") {
            break;
        } else {
            (rest_args, outArgs) = readArg((arg.clone()).clone(), flags.clone(), rest_args.clone(), outArgs.clone())?;
        }
    }
    outArgs = List::append_reverse(outArgs.clone(), rest_args.clone());
    let _ = List::map2(outArgs.clone(), (std::sync::Arc::new(fnptr!(System::iconv, ArcStr, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("UTF-8")).clone(), (literal!("UTF-8")).clone());
    Error::assertionOrAddSourceMessage(numError.clone() == Error::getNumErrorMessages(), Error::UTF8_COMMAND_LINE_ARGS.clone(), metamodelica::nil(), Util::dummyInfo.clone())?;
    saveFlags(flags.clone());
    handleDeprecatedFlags()?;
    Ok(outArgs)
}

fn readArg(mut inArg: ArcStr, mut inFlags: Flags::Flag, mut restArgs: Arc<metamodelica::List<ArcStr>>, mut nonFlags: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut restArgs: Arc<metamodelica::List<ArcStr>> = restArgs;
    let mut nonFlags: Arc<metamodelica::List<ArcStr>> = nonFlags;
    let mut flagtype: ArcStr = arcstr::literal!("");
    let mut len: i32 = 0;
    flagtype = (stringGetStringChar((inArg.clone()).clone(), 1)?).clone();
    len = ((inArg.clone()).clone().len() as i32);
    if flagtype.clone() == literal!("+") {
        if len.clone() == 1 {
            parseFlag((inArg.clone()).clone(), crate::Flags::Flag::NO_FLAGS, restArgs.clone(), (literal!("")).clone())?;
        } else {
            restArgs = parseFlag(substring((inArg.clone()).clone(), 2, len.clone())?, inFlags.clone(), restArgs.clone(), (flagtype.clone()).clone())?;
        }
    } else if flagtype.clone() == literal!("-") {
        if len.clone() == 1 {
            parseFlag((inArg.clone()).clone(), crate::Flags::Flag::NO_FLAGS, restArgs.clone(), (literal!("")).clone())?;
        } else if len.clone() == 2 {
            restArgs = parseFlag(substring((inArg.clone()).clone(), 2, 2)?, inFlags.clone(), restArgs.clone(), (flagtype.clone()).clone())?;
        } else if stringGetStringChar((inArg.clone()).clone(), 2)? == literal!("-") {
            if len.clone() < 4 || stringGetStringChar((inArg.clone()).clone(), 4)? == literal!("=") {
                parseFlag((inArg.clone()).clone(), crate::Flags::Flag::NO_FLAGS, restArgs.clone(), (literal!("")).clone())?;
            } else {
                restArgs = parseFlag(substring((inArg.clone()).clone(), 3, len.clone())?, inFlags.clone(), restArgs.clone(), (literal!("--")).clone())?;
            }
        } else {
            if stringGetStringChar((inArg.clone()).clone(), 3)? == literal!("=") {
                restArgs = parseFlag(substring((inArg.clone()).clone(), 2, len.clone())?, inFlags.clone(), restArgs.clone(), (flagtype.clone()).clone())?;
            } else {
                parseFlag((inArg.clone()).clone(), crate::Flags::Flag::NO_FLAGS, restArgs.clone(), (literal!("")).clone())?;
            }
        }
    } else {
        nonFlags = cons((inArg.clone()).clone(), nonFlags.clone());
    }
    Ok((restArgs, nonFlags))
}

fn parseFlag(mut inFlag: ArcStr, mut inFlags: Flags::Flag, mut restArgs: Arc<metamodelica::List<ArcStr>>, mut inFlagPrefix: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut restArgs: Arc<metamodelica::List<ArcStr>> = restArgs;
    let mut flag: ArcStr = arcstr::literal!("");
    let mut values: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut value: ArcStr = arcstr::literal!("");
    let mut missing_value: bool = false;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok((inFlag.clone()).clone(), (literal!("=")).clone())) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    flag = __pa0.clone();
    values = __pa1.clone();
    value = stringAppendList(values.clone());
    missing_value = stringEmpty((value.clone()).clone()) && !(StringUtil::endsWith((inFlag.clone()).clone(), (literal!("=")).clone()));
    restArgs = parseConfigFlag((flag.clone()).clone(), (value.clone()).clone(), inFlags.clone(), restArgs.clone(), (inFlagPrefix.clone()).clone(), missing_value.clone())?;
    Ok(restArgs)
}

fn parseConfigFlag(mut inFlag: ArcStr, mut inValue: ArcStr, mut inFlags: Flags::Flag, mut restArgs: Arc<metamodelica::List<ArcStr>>, mut inFlagPrefix: ArcStr, mut missingValue: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut restArgs: Arc<metamodelica::List<ArcStr>> = restArgs;
    let mut config_flag: Flags::ConfigFlag;
    let mut value: ArcStr = arcstr::literal!("");
    config_flag = lookupConfigFlag((inFlag.clone()).clone(), (inFlagPrefix.clone()).clone())?;
    if missingValue.clone() && flagRequiresValue(config_flag.clone()) && !(restArgs.clone().is_empty()) && !(StringUtil::startsWith((listHead(restArgs.clone())?).clone(), (literal!("-")).clone())) {
        value = (listHead(restArgs.clone())?).clone();
        restArgs = listRest(restArgs.clone())?;
    } else {
        value = (inValue.clone()).clone();
    }
    evaluateConfigFlag(config_flag.clone(), (value.clone()).clone(), inFlags.clone())?;
    Ok(restArgs)
}

fn lookupConfigFlag(mut inFlag: ArcStr, mut inFlagPrefix: ArcStr) -> Result<Flags::ConfigFlag> {
    let mut outFlag: Flags::ConfigFlag;
    if let Ok(__iflet0) = List::getMemberOnTrue((inFlag.clone()).clone(), allConfigFlags.clone(), (std::sync::Arc::new(matchConfigFlag) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Flags::ConfigFlag) -> Result<bool> + 'static>)) {
        outFlag = __iflet0;
    } else {
        Error::addMessage(Error::UNKNOWN_OPTION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*inFlagPrefix.clone()); __mm_s.push_str(&*inFlag.clone()); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(outFlag)
}

fn configFlagEq(mut inFlag1: Flags::ConfigFlag, mut inFlag2: Flags::ConfigFlag) -> Result<bool> {
    let mut eq: bool = false;
    eq = (match (inFlag1.clone(), inFlag2.clone()) {
        (Flags::ConfigFlag { index: mut index1, .. }, Flags::ConfigFlag { index: mut index2, .. }) => {
            index1.clone() == index2.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(eq)
}

fn flagRequiresValue(mut flag: Flags::ConfigFlag) -> bool {
    let mut requiresValue: bool = false;
    requiresValue = (match flag.clone() {
        Flags::ConfigFlag { defaultValue: Flags::FlagData::BOOL_FLAG { .. }, .. } => false,
        _ => true,
    });
    requiresValue
}

fn setAdditionalOptModules(mut inFlag: Flags::ConfigFlag, mut inOppositeFlag: Flags::ConfigFlag, mut inValues: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut values: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut value in &*inValues.clone() {
        let mut value = value.clone();
        values = Flags::getConfigStringList(inOppositeFlag.clone())?;
        values = List::removeOnTrue((value.clone()).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), values.clone());
        setConfigStringList(inOppositeFlag.clone(), values.clone())?;
        values = Flags::getConfigStringList(inFlag.clone())?;
        values = List::removeOnTrue((value.clone()).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), values.clone());
        setConfigStringList(inFlag.clone(), cons((value.clone()).clone(), values.clone()))?;
    }
    Ok(())
}

fn evaluateConfigFlag(mut inFlag: Flags::ConfigFlag, mut inValue: ArcStr, mut inFlags: Flags::Flag) -> Result<()> {
    let _ = (match (inFlag.clone(), inFlags.clone()) {
        (Flags::ConfigFlag { index: 1, .. }, Flags::Flag::FLAGS { debugFlags: mut debug_flags, .. }) => {
            List::map1_0(splitCSV((inValue.clone()).clone()), (std::sync::Arc::new(setDebugFlag) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<bool>) -> Result<()> + 'static>), debug_flags.clone());
            ()
        },
        (Flags::ConfigFlag { index: 2, .. }, _) => {
            let mut values: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            values = splitCSV((System::tolower((inValue.clone()).clone())).clone());
            System::gettextInit((if (Flags::getConfigString(Flags::RUNNING_TESTSUITE.clone())? == literal!("")) {Flags::getConfigString(Flags::LOCALE_FLAG.clone())?} else {literal!("C")}).clone());
            println!("{}", (printHelp(values.clone())?).clone());
            setConfigString(Flags::HELP.clone(), (literal!("omc")).clone())?;
            ()
        },
        (_, _) if (configFlagEq(inFlag.clone(), Flags::PRE_OPT_MODULES_ADD.clone())?) => {
            setAdditionalOptModules(Flags::PRE_OPT_MODULES_ADD.clone(), Flags::PRE_OPT_MODULES_SUB.clone(), splitCSV((inValue.clone()).clone()))?;
            ()
        },
        (_, _) if (configFlagEq(inFlag.clone(), Flags::PRE_OPT_MODULES_SUB.clone())?) => {
            setAdditionalOptModules(Flags::PRE_OPT_MODULES_SUB.clone(), Flags::PRE_OPT_MODULES_ADD.clone(), splitCSV((inValue.clone()).clone()))?;
            ()
        },
        (_, _) if (configFlagEq(inFlag.clone(), Flags::POST_OPT_MODULES_ADD.clone())?) => {
            setAdditionalOptModules(Flags::POST_OPT_MODULES_ADD.clone(), Flags::POST_OPT_MODULES_SUB.clone(), splitCSV((inValue.clone()).clone()))?;
            ()
        },
        (_, _) if (configFlagEq(inFlag.clone(), Flags::POST_OPT_MODULES_SUB.clone())?) => {
            setAdditionalOptModules(Flags::POST_OPT_MODULES_SUB.clone(), Flags::POST_OPT_MODULES_ADD.clone(), splitCSV((inValue.clone()).clone()))?;
            ()
        },
        (_, _) if (configFlagEq(inFlag.clone(), Flags::INIT_OPT_MODULES_ADD.clone())?) => {
            setAdditionalOptModules(Flags::INIT_OPT_MODULES_ADD.clone(), Flags::INIT_OPT_MODULES_SUB.clone(), splitCSV((inValue.clone()).clone()))?;
            ()
        },
        (_, _) if (configFlagEq(inFlag.clone(), Flags::INIT_OPT_MODULES_SUB.clone())?) => {
            setAdditionalOptModules(Flags::INIT_OPT_MODULES_SUB.clone(), Flags::INIT_OPT_MODULES_ADD.clone(), splitCSV((inValue.clone()).clone()))?;
            ()
        },
        (_, Flags::Flag::FLAGS { configFlags: mut config_flags, .. }) => {
            setConfigFlag(inFlag.clone(), config_flags.clone(), (inValue.clone()).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn setDebugFlag(mut inFlag: ArcStr, mut inFlags: metamodelica::Array<bool>) -> Result<()> {
    let mut negated: bool = false;
    let mut neg1: bool = false;
    let mut neg2: bool = false;
    let mut flag_str: ArcStr = arcstr::literal!("");
    neg1 = stringEq((stringGetStringChar((inFlag.clone()).clone(), 1)?).clone(), (literal!("-")).clone());
    neg2 = System::strncmp((literal!("no")).clone(), (inFlag.clone()).clone(), 2) == 0;
    negated = neg1.clone() || neg2.clone();
    flag_str = (if (negated.clone()) {StringUtil::rest((inFlag.clone()).clone())} else {inFlag.clone()}).clone();
    flag_str = (if (neg2.clone()) {StringUtil::rest((flag_str.clone()).clone())} else {flag_str.clone()}).clone();
    setDebugFlag2((flag_str.clone()).clone(), !(negated.clone()), inFlags.clone())?;
    Ok(())
}

fn setDebugFlag2(mut inFlag: ArcStr, mut inValue: bool, mut inFlags: metamodelica::Array<bool>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inFlag.clone(), inValue.clone(), inFlags.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut flag: Flags::DebugFlag;
            flag = List::getMemberOnTrue((inFlag.clone()).clone(), allDebugFlags.clone(), (std::sync::Arc::new(matchDebugFlag) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Flags::DebugFlag) -> Result<bool> + 'static>))?;
            (_, _) = updateDebugFlagArray(inFlags.clone(), inValue.clone(), flag.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addMessage(Error::UNKNOWN_DEBUG_FLAG.clone(), list![(inFlag.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn matchDebugFlag(mut inFlagName: ArcStr, mut inFlag: Flags::DebugFlag) -> Result<bool> {
    let mut outMatches: bool = false;
    let mut name: ArcStr = arcstr::literal!("");
    let Flags::DEBUG_FLAG { name: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    outMatches = stringEq((inFlagName.clone()).clone(), (name.clone()).clone());
    Ok(outMatches)
}

fn matchConfigFlag(mut inFlagName: ArcStr, mut inFlag: Flags::ConfigFlag) -> Result<bool> {
    let mut outMatches: bool = false;
    let mut opt_shortname: Option<ArcStr> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut shortname: ArcStr = arcstr::literal!("");
    let Flags::CONFIG_FLAG { shortname: __pa0, name: __pa1, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    opt_shortname = __pa0.clone();
    name = __pa1.clone();
    shortname = (Util::getOptionOrDefault(opt_shortname.clone(), (literal!("")).clone())).clone();
    outMatches = stringEq((inFlagName.clone()).clone(), (shortname.clone()).clone()) || stringEq((System::tolower((inFlagName.clone()).clone())).clone(), (System::tolower((name.clone()).clone())).clone());
    Ok(outMatches)
}

fn setConfigFlag(mut inFlag: Flags::ConfigFlag, mut inConfigData: metamodelica::Array<Flags::FlagData>, mut inValue: ArcStr) -> Result<()> {
    let mut data: Flags::FlagData = Flags::FlagData::EMPTY_FLAG;
    let mut default_value: Flags::FlagData = Flags::FlagData::EMPTY_FLAG;
    let mut name: ArcStr = arcstr::literal!("");
    let mut validOptions: Option<Flags::ValidOptions> = None;
    let Flags::CONFIG_FLAG { validOptions: __pa0, defaultValue: __pa1, name: __pa2, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    validOptions = __pa0.clone();
    default_value = __pa1.clone();
    name = __pa2.clone();
    data = stringFlagData((inValue.clone()).clone(), default_value.clone(), validOptions.clone(), (name.clone()).clone())?;
    let _ = updateConfigFlagArray(inConfigData.clone(), data.clone(), inFlag.clone())?;
    Ok(())
}

fn stringFlagData(mut inValue: ArcStr, mut inExpectedType: Flags::FlagData, mut validOptions: Option<Flags::ValidOptions>, mut inName: ArcStr) -> Result<Flags::FlagData> {
    let mut outValue: Flags::FlagData = Flags::FlagData::EMPTY_FLAG;
    outValue = 'mc: {
        let __mc_input = (inValue.clone(), inExpectedType.clone(), validOptions.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", Flags::FlagData::BOOL_FLAG { .. }, _, _) => {
                    Ok(Flags::FlagData::BOOL_FLAG { data: true })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::BOOL_FLAG { .. }, _, _) => {
                    let mut b: bool = false;
                    b = Util::stringBool((inValue.clone()).clone())?;
                    Ok(Flags::FlagData::BOOL_FLAG { data: b.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::INT_FLAG { .. }, _, _) => {
                    let mut i: i32 = 0;
                    i = stringInt((inValue.clone()).clone())?;
                    let true = (stringEq((intString(i.clone())).clone(), (inValue.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(Flags::FlagData::INT_FLAG { data: i.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::INT_LIST_FLAG { .. }, _, _) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    ilst = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (splitCSV((inValue.clone()).clone())).into_iter().cloned() {
                    let __x = stringInt((v.clone()).clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Ok(Flags::FlagData::INT_LIST_FLAG { data: ilst.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::REAL_FLAG { .. }, _, _) => {
                    Ok(Flags::FlagData::REAL_FLAG { data: stringReal((inValue.clone()).clone())? })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::STRING_FLAG { .. }, Some(options), _) => {
                    let mut flags: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    flags = getValidStringOptions(options.clone())?;
                    let true = (listMember((inValue.clone()).clone(), flags.clone())) else { bail!("pattern mismatch") };
                    Ok(Flags::FlagData::STRING_FLAG { data: (inValue.clone()).clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::STRING_FLAG { .. }, None, _) => {
                    if !((!(stringEmpty((inValue.clone()).clone())))) { bail!("guard") }
                    Ok(Flags::FlagData::STRING_FLAG { data: (inValue.clone()).clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::STRING_LIST_FLAG { .. }, _, _) => {
                    Ok(Flags::FlagData::STRING_LIST_FLAG { data: splitCSV((inValue.clone()).clone()) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::ENUM_FLAG { validValues: enums, .. }, _, _) => {
                    let mut i: i32 = 0;
                    i = Util::assoc((inValue.clone()).clone(), enums.clone())?;
                    Ok(Flags::FlagData::ENUM_FLAG { data: i.clone(), validValues: enums.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, None, _) => {
                    let mut et: ArcStr = arcstr::literal!("");
                    let mut at: ArcStr = arcstr::literal!("");
                    et = (printExpectedTypeStr(inExpectedType.clone())?).clone();
                    at = (printActualTypeStr((inValue.clone()).clone())?).clone();
                    Error::addMessage(Error::INVALID_FLAG_TYPE.clone(), list![(inName.clone()).clone(), (et.clone()).clone(), (at.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(options), _) => {
                    let mut et: ArcStr = arcstr::literal!("");
                    let mut at: ArcStr = arcstr::literal!("");
                    let mut flags: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    flags = getValidStringOptions(options.clone())?;
                    et = stringDelimitList(flags.clone(), (literal!(", ")).clone());
                    at = (printActualTypeStr((inValue.clone()).clone())?).clone();
                    Error::addMessage(Error::INVALID_FLAG_TYPE_STRINGS.clone(), list![(inName.clone()).clone(), (et.clone()).clone(), (at.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn printExpectedTypeStr(mut inType: Flags::FlagData) -> Result<ArcStr> {
    let mut outTypeStr: ArcStr = arcstr::literal!("");
    outTypeStr = ((match inType.clone() {
        Flags::FlagData::BOOL_FLAG { .. } => {
            literal!("a boolean value")
        },
        Flags::FlagData::INT_FLAG { .. } => {
            literal!("an integer value")
        },
        Flags::FlagData::REAL_FLAG { .. } => {
            literal!("a floating-point value")
        },
        Flags::FlagData::STRING_FLAG { .. } => {
            literal!("a string")
        },
        Flags::FlagData::STRING_LIST_FLAG { .. } => {
            literal!("a comma-separated list of strings")
        },
        Flags::FlagData::ENUM_FLAG { validValues: ref enums, .. } => {
            let mut enum_strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            enum_strs = List::map(enums.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("one of the values {")); __mm_s.push_str(&*stringDelimitList(enum_strs.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outTypeStr)
}

fn printActualTypeStr(mut inType: ArcStr) -> Result<ArcStr> {
    let mut outTypeStr: ArcStr = arcstr::literal!("");
    outTypeStr = ('mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Ok(literal!("nothing"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Util::stringBool((inType.clone()).clone())?;
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the boolean value ")); __mm_s.push_str(&*inType.clone()); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut i: i32 = 0;
                    i = stringInt((inType.clone()).clone())?;
                    let true = (stringEq((intString(i.clone())).clone(), (inType.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the string \"")); __mm_s.push_str(&*inType.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outTypeStr)
}

fn configFlagsIsEqualIndex(mut inFlag1: Flags::ConfigFlag, mut inFlag2: Flags::ConfigFlag) -> Result<bool> {
    let mut outEqualIndex: bool = false;
    let mut index1: i32 = 0;
    let mut index2: i32 = 0;
    let Flags::CONFIG_FLAG { index: __pa0, .. } = (inFlag1.clone()) else { bail!("pattern mismatch") };
    index1 = __pa0.clone();
    let Flags::CONFIG_FLAG { index: __pa1, .. } = (inFlag2.clone()) else { bail!("pattern mismatch") };
    index2 = __pa1.clone();
    outEqualIndex = intEq(index1.clone(), index2.clone());
    Ok(outEqualIndex)
}

fn handleDeprecatedFlags() -> Result<()> {
    let mut remaining_flags: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if Flags::isSet(Flags::NF_UNITCHECK.clone())? {
        disableDebug(Flags::NF_UNITCHECK.clone())?;
        setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
        Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("-d=frontEndUnitCheck")).clone(), (literal!("--unitChecking")).clone()])?;
    }
    if Flags::isSet(Flags::OLD_FE_UNITCHECK.clone())? {
        disableDebug(Flags::OLD_FE_UNITCHECK.clone())?;
        setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
        Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("-d=oldFrontEndUnitCheck")).clone(), (literal!("--unitChecking")).clone()])?;
    }
    if Flags::isSet(Flags::INTERACTIVE_TCP.clone())? {
        disableDebug(Flags::INTERACTIVE_TCP.clone())?;
        setConfigString(Flags::INTERACTIVE.clone(), (literal!("tcp")).clone())?;
        Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("-d=interactive")).clone(), (literal!("--interactive=tcp")).clone()])?;
        println!("{}", (literal!("The flag -d=interactive is depreciated. Please use --interactive=tcp instead.\n")).clone());
    }
    if Flags::isSet(Flags::INTERACTIVE_CORBA.clone())? {
        disableDebug(Flags::INTERACTIVE_CORBA.clone())?;
        setConfigString(Flags::INTERACTIVE.clone(), (literal!("corba")).clone())?;
        Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("-d=interactiveCorba")).clone(), (literal!("--interactive=corba")).clone()])?;
        println!("{}", (literal!("The flag -d=interactiveCorba is depreciated. Please use --interactive=corba instead.\n")).clone());
    }
    if Flags::getConfigString(Flags::TEARING_METHOD.clone())? == literal!("noTearing") {
        setConfigString(Flags::TEARING_METHOD.clone(), (literal!("minimalTearing")).clone())?;
        Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("--tearingMethod=noTearing")).clone(), (literal!("--tearingMethod=minimalTearing")).clone()])?;
    }
    remaining_flags = metamodelica::nil();
    for mut flag in &*Flags::getConfigStringList(Flags::PRE_OPT_MODULES.clone())? {
        let mut flag = flag.clone();
        if flag.clone() == literal!("unitChecking") {
            setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
            Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("--preOptModules=unitChecking")).clone(), (literal!("--unitChecking")).clone()])?;
        } else {
            remaining_flags = cons((flag.clone()).clone(), remaining_flags.clone());
        }
    }
    setConfigStringList(Flags::PRE_OPT_MODULES.clone(), remaining_flags.clone().reverse())?;
    remaining_flags = metamodelica::nil();
    for mut flag in &*Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())? {
        let mut flag = flag.clone();
        if flag.clone() == literal!("unitChecking") {
            setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
            Error::addMessage(Error::DEPRECATED_FLAG.clone(), list![(literal!("--preOptModules+=unitChecking")).clone(), (literal!("--unitChecking")).clone()])?;
        } else {
            remaining_flags = cons((flag.clone()).clone(), remaining_flags.clone());
        }
    }
    setConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone(), remaining_flags.clone().reverse())?;
    Ok(())
}

fn applySideEffects(mut inFlag: Flags::ConfigFlag, mut inValue: Flags::FlagData) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inFlag.clone(), inValue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut value: bool = false;
            let true = (configFlagsIsEqualIndex(inFlag.clone(), Flags::SHOW_ERROR_MESSAGES.clone())?) else { bail!("pattern mismatch") };
            let Flags::BOOL_FLAG { data: __pa0 } = (inValue.clone()) else { bail!("pattern mismatch") };
            value = __pa0.clone();
            ErrorExt::setShowErrorMessages(value.clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut corba_objid_path: ArcStr = arcstr::literal!("");
            let true = (configFlagsIsEqualIndex(inFlag.clone(), Flags::CORBA_OBJECT_REFERENCE_FILE_PATH.clone())?) else { bail!("pattern mismatch") };
            let Flags::STRING_FLAG { data: __pa0 } = (inValue.clone()) else { bail!("pattern mismatch") };
            corba_objid_path = __pa0.clone();
            Corba::setObjectReferenceFilePath((corba_objid_path.clone()).clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut corba_name: ArcStr = arcstr::literal!("");
            let true = (configFlagsIsEqualIndex(inFlag.clone(), Flags::CORBA_SESSION.clone())?) else { bail!("pattern mismatch") };
            let Flags::STRING_FLAG { data: __pa0 } = (inValue.clone()) else { bail!("pattern mismatch") };
            corba_name = __pa0.clone();
            Corba::setSessionName((corba_name.clone()).clone())?;
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

pub fn setConfigValue(mut inFlag: Flags::ConfigFlag, mut inValue: Flags::FlagData) -> Result<()> {
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    flags = loadFlags(true)?;
    let Flags::FLAGS { debugFlags: __pa0, configFlags: __pa1 } = (flags.clone()) else { bail!("pattern mismatch") };
    debug_flags = __pa0.clone();
    config_flags = __pa1.clone();
    config_flags = updateConfigFlagArray(config_flags.clone(), inValue.clone(), inFlag.clone())?;
    saveFlags(Flags::Flag::FLAGS { debugFlags: debug_flags.clone(), configFlags: config_flags.clone() });
    Ok(())
}

pub fn setConfigBool(mut inFlag: Flags::ConfigFlag, mut inValue: bool) -> Result<()> {
    setConfigValue(inFlag.clone(), Flags::FlagData::BOOL_FLAG { data: inValue.clone() })?;
    Ok(())
}

pub fn setConfigInt(mut inFlag: Flags::ConfigFlag, mut inValue: i32) -> Result<()> {
    setConfigValue(inFlag.clone(), Flags::FlagData::INT_FLAG { data: inValue.clone() })?;
    Ok(())
}

pub fn setConfigReal(mut inFlag: Flags::ConfigFlag, mut inValue: metamodelica::Real) -> Result<()> {
    setConfigValue(inFlag.clone(), Flags::FlagData::REAL_FLAG { data: inValue.clone() })?;
    Ok(())
}

pub fn setConfigString(mut inFlag: Flags::ConfigFlag, mut inValue: ArcStr) -> Result<()> {
    setConfigValue(inFlag.clone(), Flags::FlagData::STRING_FLAG { data: (inValue.clone()).clone() })?;
    Ok(())
}

pub fn setConfigStringList(mut inFlag: Flags::ConfigFlag, mut inValue: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    setConfigValue(inFlag.clone(), Flags::FlagData::STRING_LIST_FLAG { data: inValue.clone() })?;
    Ok(())
}

pub fn appendConfigStringList(mut flag: Flags::ConfigFlag, mut value: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut oldValues: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    oldValues = Flags::getConfigStringList(flag.clone())?;
    if !(listMember((value.clone()).clone(), oldValues.clone())) {
        setConfigStringList(flag.clone(), cons((value.clone()).clone(), oldValues.clone()))?;
    }
    Ok(oldValues)
}

pub fn setConfigEnum(mut inFlag: Flags::ConfigFlag, mut inValue: i32) -> Result<()> {
    let mut valid_values: Arc<metamodelica::List<(ArcStr, i32)>> = metamodelica::nil();
    let Flags::CONFIG_FLAG { defaultValue: Flags::ENUM_FLAG { validValues: __pa0, .. }, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    valid_values = __pa0.clone();
    setConfigValue(inFlag.clone(), Flags::FlagData::ENUM_FLAG { data: inValue.clone(), validValues: valid_values.clone() })?;
    Ok(())
}

// Used by the print functions below to indent descriptions.
pub const descriptionIndent: &'static str = "                            ";

pub fn printHelp(mut inTopics: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut help: ArcStr = arcstr::literal!("");
    help = ('mc: {
        let __mc_input = inTopics.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(printUsage()?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "omc", tail: Deref @ metamodelica::List::Nil } => {
                    Ok(printUsage()?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "omcall-sphinxoutput", tail: Deref @ metamodelica::List::Nil } => {
                    Ok(printUsageSphinxAll()?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "topics", tail: Deref @ metamodelica::List::Nil } => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut topics: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    let mut help: ArcStr = help.clone();
                    topics = list![(literal!("omc"), System::gettext((literal!("The command-line options available for omc.")).clone())), (literal!("debug"), System::gettext((literal!("Flags that enable debugging, diagnostics, and research prototypes.")).clone())), (literal!("optmodules"), System::gettext((literal!("Flags that determine which symbolic methods are used to produce the causalized equation system.")).clone())), (literal!("simulation"), System::gettext((literal!("The command-line options available for simulation executables generated by OpenModelica.")).clone())), (literal!("<flagname>"), System::gettext((literal!("Displays option descriptions for multi-option flag <flagname>.")).clone())), (literal!("topics"), System::gettext((literal!("This help-text.")).clone()))];
                    r#str = (System::gettext((literal!("The available topics (help(\"topics\")) are as follows:\n")).clone())).clone();
                    strs = List::map(topics.clone(), (std::sync::Arc::new(makeTopicString) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, ArcStr)) -> Result<ArcStr> + 'static>));
                    help = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringDelimitList(strs.clone(), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(help.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "simulation", tail: Deref @ metamodelica::List::Nil } => {
                    let mut help: ArcStr = help.clone();
                    help = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The simulation executable takes the following flags:\n\n")).clone())); __mm_s.push_str(&*System::getSimulationHelpText(true, false)); ArcStr::from(__mm_s) }).clone();
                    Ok(help.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "simulation-sphinxoutput", tail: Deref @ metamodelica::List::Nil } => {
                    let mut help: ArcStr = help.clone();
                    help = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The simulation executable takes the following flags:\n\n")).clone())); __mm_s.push_str(&*System::getSimulationHelpText(true, true)); ArcStr::from(__mm_s) }).clone();
                    Ok(help.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "debug", tail: Deref @ metamodelica::List::Nil } => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut help: ArcStr = help.clone();
                    str1 = (System::gettext((literal!("The debug flag takes a comma-separated list of flags which are used by the\ncompiler for debugging or experimental purposes.\nFlags prefixed with \"-\" or \"no\" will be disabled.\n")).clone())).clone();
                    str2 = (System::gettext((literal!("The available flags are (+ are enabled by default, - are disabled):\n\n")).clone())).clone();
                    strs = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut flag in (List::sort(allDebugFlags.clone(), (std::sync::Arc::new(compareDebugFlags) as std::sync::Arc<dyn ::std::ops::Fn(Flags::DebugFlag, Flags::DebugFlag) -> Result<bool> + 'static>))?).into_iter().cloned() {
                    let __x = printDebugFlag(flag.clone(), false)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    help = stringAppendList(cons((str1.clone()).clone(), cons((str2.clone()).clone(), strs.clone())));
                    Ok(help.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "optmodules", tail: Deref @ metamodelica::List::Nil } => {
                    let mut data: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str1a: ArcStr = arcstr::literal!("");
                    let mut str1b: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut str3a: ArcStr = arcstr::literal!("");
                    let mut str3b: ArcStr = arcstr::literal!("");
                    let mut str4: ArcStr = arcstr::literal!("");
                    let mut str5: ArcStr = arcstr::literal!("");
                    let mut str5a: ArcStr = arcstr::literal!("");
                    let mut str5b: ArcStr = arcstr::literal!("");
                    let mut str6: ArcStr = arcstr::literal!("");
                    let mut str7: ArcStr = arcstr::literal!("");
                    let mut str7a: ArcStr = arcstr::literal!("");
                    let mut str7b: ArcStr = arcstr::literal!("");
                    let mut str8: ArcStr = arcstr::literal!("");
                    let mut str9: ArcStr = arcstr::literal!("");
                    let mut str9a: ArcStr = arcstr::literal!("");
                    let mut str9b: ArcStr = arcstr::literal!("");
                    let mut str10: ArcStr = arcstr::literal!("");
                    let mut help: ArcStr = help.clone();
                    str1 = (System::gettext((literal!("The --preOptModules flag sets the optimization modules which are used before the\nmatching and index reduction in the back end. These modules are specified as a comma-separated list.")).clone())).clone();
                    str1 = stringAppendList(StringUtil::wordWrap((str1.clone()).clone(), System::getTerminalWidth(), (literal!("\n")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_LIST_FLAG { data: __pa0 }, .. } = (Flags::PRE_OPT_MODULES.clone()) else { bail!("pattern mismatch") };
                    data = __pa0.clone();
                    str1a = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The modules used by default are:")).clone())); __mm_s.push_str(&*literal!("\n--preOptModules=")); __mm_s.push_str(&*stringDelimitList(data.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
                    str1b = (System::gettext((literal!("The valid modules are:")).clone())).clone();
                    str2 = (printFlagValidOptionsDesc(Flags::PRE_OPT_MODULES.clone())?).clone();
                    str3 = (System::gettext((literal!("The --matchingAlgorithm sets the method that is used for the matching algorithm, after the pre optimization modules.")).clone())).clone();
                    str3 = stringAppendList(StringUtil::wordWrap((str3.clone()).clone(), System::getTerminalWidth(), (literal!("\n")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_FLAG { data: __pa1 }, .. } = (Flags::MATCHING_ALGORITHM.clone()) else { bail!("pattern mismatch") };
                    str3a = __pa1.clone();
                    str3a = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The method used by default is:")).clone())); __mm_s.push_str(&*literal!("\n--matchingAlgorithm=")); __mm_s.push_str(&*str3a.clone()); ArcStr::from(__mm_s) }).clone();
                    str3b = (System::gettext((literal!("The valid methods are:")).clone())).clone();
                    str4 = (printFlagValidOptionsDesc(Flags::MATCHING_ALGORITHM.clone())?).clone();
                    str5 = (System::gettext((literal!("The --indexReductionMethod sets the method that is used for the index reduction, after the pre optimization modules.")).clone())).clone();
                    str5 = stringAppendList(StringUtil::wordWrap((str5.clone()).clone(), System::getTerminalWidth(), (literal!("\n")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_FLAG { data: __pa2 }, .. } = (Flags::INDEX_REDUCTION_METHOD.clone()) else { bail!("pattern mismatch") };
                    str5a = __pa2.clone();
                    str5a = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The method used by default is:")).clone())); __mm_s.push_str(&*literal!("\n--indexReductionMethod=")); __mm_s.push_str(&*str5a.clone()); ArcStr::from(__mm_s) }).clone();
                    str5b = (System::gettext((literal!("The valid methods are:")).clone())).clone();
                    str6 = (printFlagValidOptionsDesc(Flags::INDEX_REDUCTION_METHOD.clone())?).clone();
                    str7 = (System::gettext((literal!("The --initOptModules then sets the optimization modules which are used after the index reduction to optimize the system for initialization, specified as a comma-separated list.")).clone())).clone();
                    str7 = stringAppendList(StringUtil::wordWrap((str7.clone()).clone(), System::getTerminalWidth(), (literal!("\n")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_LIST_FLAG { data: __pa3 }, .. } = (Flags::INIT_OPT_MODULES.clone()) else { bail!("pattern mismatch") };
                    data = __pa3.clone();
                    str7a = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The modules used by default are:")).clone())); __mm_s.push_str(&*literal!("\n--initOptModules=")); __mm_s.push_str(&*stringDelimitList(data.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
                    str7b = (System::gettext((literal!("The valid modules are:")).clone())).clone();
                    str8 = (printFlagValidOptionsDesc(Flags::INIT_OPT_MODULES.clone())?).clone();
                    str9 = (System::gettext((literal!("The --postOptModules then sets the optimization modules which are used after the index reduction to optimize the system for simulation, specified as a comma-separated list.")).clone())).clone();
                    str9 = stringAppendList(StringUtil::wordWrap((str9.clone()).clone(), System::getTerminalWidth(), (literal!("\n")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_LIST_FLAG { data: __pa4 }, .. } = (Flags::POST_OPT_MODULES.clone()) else { bail!("pattern mismatch") };
                    data = __pa4.clone();
                    str9a = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("The modules used by default are:")).clone())); __mm_s.push_str(&*literal!("\n--postOptModules=")); __mm_s.push_str(&*stringDelimitList(data.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
                    str9b = (System::gettext((literal!("The valid modules are:")).clone())).clone();
                    str10 = (printFlagValidOptionsDesc(Flags::POST_OPT_MODULES.clone())?).clone();
                    help = stringAppendList(list![(str1.clone()).clone(), (literal!("\n\n")).clone(), (str1a.clone()).clone(), (literal!("\n\n")).clone(), (str1b.clone()).clone(), (literal!("\n")).clone(), (str2.clone()).clone(), (literal!("\n")).clone(), (str3.clone()).clone(), (literal!("\n\n")).clone(), (str3a.clone()).clone(), (literal!("\n\n")).clone(), (str3b.clone()).clone(), (literal!("\n")).clone(), (str4.clone()).clone(), (literal!("\n")).clone(), (str5.clone()).clone(), (literal!("\n\n")).clone(), (str5a.clone()).clone(), (literal!("\n\n")).clone(), (str5b.clone()).clone(), (literal!("\n")).clone(), (str6.clone()).clone(), (literal!("\n")).clone(), (str7.clone()).clone(), (literal!("\n\n")).clone(), (str7a.clone()).clone(), (literal!("\n\n")).clone(), (str7b.clone()).clone(), (literal!("\n")).clone(), (str8.clone()).clone(), (literal!("\n")).clone(), (str9.clone()).clone(), (literal!("\n\n")).clone(), (str9a.clone()).clone(), (literal!("\n\n")).clone(), (str9b.clone()).clone(), (literal!("\n")).clone(), (str10.clone()).clone(), (literal!("\n")).clone()]);
                    Ok(help.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } => {
                    let mut desc: Gettext::TranslatableContent;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut config_flag: Flags::ConfigFlag;
                    let mut r#str = (*r#str).clone();
                    let mut help: ArcStr = help.clone();
                    let ref __pa2 @ Flags::CONFIG_FLAG { description: ref __pa0, name: ref __pa1, .. } = (List::getMemberOnTrue((r#str.clone()).clone(), allConfigFlags.clone(), (std::sync::Arc::new(matchConfigFlag) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Flags::ConfigFlag) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    desc = __pa0.clone();
                    name = __pa1.clone();
                    config_flag = __pa2.clone();
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
                    str2 = stringAppendList(StringUtil::wordWrap((Gettext::translateContent(desc.clone())?).clone(), System::getTerminalWidth(), (literal!("\n")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
                    r#str = (printFlagValidOptionsDesc(config_flag.clone())?).clone();
                    help = stringAppendList(list![(str1.clone()).clone(), (literal!("\n")).clone(), (str2.clone()).clone(), (literal!("\n")).clone(), (r#str.clone()).clone()]);
                    Ok(help.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("I'm sorry, I don't know what ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" is.\n")); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#str, tail: rest_topics @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
                    let mut r#str = (*r#str).clone();
                    let mut help: ArcStr = help.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*printHelp(list![(r#str.clone()).clone()])?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    help = (printHelp(rest_topics.clone())?).clone();
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*help.clone()); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(help)
}

pub fn getValidOptionsAndDescription(mut flagName: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<ArcStr>>)> {
    let mut validStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut mainDescriptionStr: ArcStr = arcstr::literal!("");
    let mut descriptions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut validOptions: Flags::ValidOptions;
    let mut mainDescription: Gettext::TranslatableContent;
    let Flags::CONFIG_FLAG { validOptions: Some(__pa0), description: __pa1, .. } = (List::getMemberOnTrue((flagName.clone()).clone(), allConfigFlags.clone(), (std::sync::Arc::new(matchConfigFlag) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Flags::ConfigFlag) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
    validOptions = __pa0.clone();
    mainDescription = __pa1.clone();
    mainDescriptionStr = (Gettext::translateContent(mainDescription.clone())?).clone();
    (validStrings, descriptions) = getValidOptionsAndDescription2(validOptions.clone())?;
    Ok((validStrings, mainDescriptionStr, descriptions))
}

fn getValidOptionsAndDescription2(mut validOptions: Flags::ValidOptions) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut validStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut descriptions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (validStrings, descriptions) = (match validOptions.clone() {
        Flags::ValidOptions::STRING_OPTION { options: ref validStrings } => {
            (validStrings.clone(), metamodelica::nil())
        },
        Flags::ValidOptions::STRING_DESC_OPTION { options: mut options } => {
            validStrings = List::map(options.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
            descriptions = List::mapMap(options.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)), (std::sync::Arc::new(Gettext::translateContent) as std::sync::Arc<dyn ::std::ops::Fn(Gettext::TranslatableContent) -> Result<ArcStr> + 'static>));
            (validStrings.clone(), descriptions.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((validStrings, descriptions))
}

fn compareDebugFlags(mut flag1: Flags::DebugFlag, mut flag2: Flags::DebugFlag) -> Result<bool> {
    let mut b: bool = false;
    let mut name1: ArcStr = arcstr::literal!("");
    let mut name2: ArcStr = arcstr::literal!("");
    let Flags::DEBUG_FLAG { name: __pa0, .. } = (flag1.clone()) else { bail!("pattern mismatch") };
    name1 = __pa0.clone();
    let Flags::DEBUG_FLAG { name: __pa1, .. } = (flag2.clone()) else { bail!("pattern mismatch") };
    name2 = __pa1.clone();
    b = stringCompare((name1.clone()).clone(), (name2.clone()).clone()) > 0;
    Ok(b)
}

fn makeTopicString(mut topic: (ArcStr, ArcStr)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut str1: ArcStr = arcstr::literal!("");
    let mut str2: ArcStr = arcstr::literal!("");
    (str1, str2) = topic.clone();
    str1 = (Util::stringPadRight((str1.clone()).clone(), 13, (literal!(" ")).clone())).clone();
    r#str = stringAppendList(StringUtil::wordWrap(({ let mut __mm_s = String::new(); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }).clone(), System::getTerminalWidth(), (literal!("\n               ")).clone(), metamodelica::OrderedFloat(0.3_f64))?);
    Ok(r#str)
}

pub fn printUsage() -> Result<ArcStr> {
    let mut usage: ArcStr = arcstr::literal!("");
    Print::clearBuf();
    Print::printBuf((literal!("OpenModelica Compiler ")).clone())?;
    Print::printBuf((Settings::getVersionNr()).clone())?;
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf((System::gettext((literal!("Copyright © 2019 Open Source Modelica Consortium (OSMC)\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("Distributed under OMSC-PL and GPL, see www.openmodelica.org\n\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("Usage: omc [Options] (Model.mo | Script.mos) [Libraries | .mo-files]\n* Libraries: Fully qualified names of libraries to load before processing Model or Script.\n             The libraries should be separated by spaces: Lib1 Lib2 ... LibN.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("\n* Options:\n")).clone())).clone())?;
    Print::printBuf((printAllConfigFlags()).clone())?;
    Print::printBuf((System::gettext((literal!("\nFor more details on a specific topic, use --help=topics or help(\"topics\")\n\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("* Examples:\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  omc Model.mo             will produce flattened Model on standard output.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  omc -s Model.mo          will produce simulation code for the model:\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("                            * Model.c           The model C code.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("                            * Model_functions.c The model functions C code.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("                            * Model.makefile    The makefile to compile the model.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("                            * Model_init.xml    The initial values.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  omc Script.mos           will run the commands from Script.mos.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  omc Model.mo Modelica    will first load the Modelica library and then produce\n                            flattened Model on standard output.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  omc Model1.mo Model2.mo  will load both Model1.mo and Model2.mo, and produce\n                            flattened Model1 on standard output.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  *.mo (Modelica files)\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("  *.mos (Modelica Script files)\n\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("For available simulation flags, use --help=simulation.\n\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("Documentation is available in the built-in package OpenModelica.Scripting or\nonline <https://build.openmodelica.org/Documentation/OpenModelica.Scripting.html>.\n")).clone())).clone())?;
    usage = (Print::getString()?).clone();
    Print::clearBuf();
    Ok(usage)
}

pub fn printUsageSphinxAll() -> Result<ArcStr> {
    let mut usage: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    Print::clearBuf();
    s = (literal!("OpenModelica Compiler Flags")).clone();
    Print::printBuf((literal!("\n.. _openmodelica-compiler-flags :\n\n")).clone())?;
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf(({
        let mut __acc = String::new();
        for mut e in (1..=((s.clone()).clone().len() as i32)).into_iter() {
            let __x = literal!("=");
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }).clone())?;
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf((System::gettext((literal!("Usage: omc [Options] (Model.mo | Script.mos) [Libraries | .mo-files]\n\n* Libraries: Fully qualified names of libraries to load before processing Model or Script.\n  The libraries should be separated by spaces: Lib1 Lib2 ... LibN.\n\n")).clone())).clone())?;
    Print::printBuf((literal!("\n.. _omcflags-options :\n\n")).clone())?;
    s = (System::gettext((literal!("Options")).clone())).clone();
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf(({
        let mut __acc = String::new();
        for mut e in (1..=((s.clone()).clone().len() as i32)).into_iter() {
            let __x = literal!("-");
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    for mut flag in &*allConfigFlags.clone() {
        let mut flag = flag.clone();
        Print::printBuf((printConfigFlagSphinx(flag.clone())?).clone())?;
    }
    Print::printBuf((literal!("\n.. _omcflag-debug-section:\n\n")).clone())?;
    s = (System::gettext((literal!("Debug flags")).clone())).clone();
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf(({
        let mut __acc = String::new();
        for mut e in (1..=((s.clone()).clone().len() as i32)).into_iter() {
            let __x = literal!("-");
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    Print::printBuf((System::gettext((literal!("The debug flag takes a comma-separated list of flags which are used by the\ncompiler for debugging or experimental purposes.\nFlags prefixed with \"-\" or \"no\" will be disabled.\n")).clone())).clone())?;
    Print::printBuf((System::gettext((literal!("The available flags are (+ are enabled by default, - are disabled):\n\n")).clone())).clone())?;
    for mut flag in &*List::sort(allDebugFlags.clone(), (std::sync::Arc::new(compareDebugFlags) as std::sync::Arc<dyn ::std::ops::Fn(Flags::DebugFlag, Flags::DebugFlag) -> Result<bool> + 'static>))? {
        let mut flag = flag.clone();
        Print::printBuf((printDebugFlag(flag.clone(), true)?).clone())?;
    }
    Print::printBuf((literal!("\n.. _omcflag-optmodules-section:\n\n")).clone())?;
    s = (System::gettext((literal!("Flags for Optimization Modules")).clone())).clone();
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("\n")).clone())?;
    Print::printBuf(({
        let mut __acc = String::new();
        for mut e in (1..=((s.clone()).clone().len() as i32)).into_iter() {
            let __x = literal!("-");
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    Print::printBuf((literal!("Flags that determine which symbolic methods are used to produce the causalized equation system.\n\n")).clone())?;
    Print::printBuf((System::gettext((literal!("The :ref:`--preOptModules <omcflag-preOptModules>` flag sets the optimization modules which are used before the\nmatching and index reduction in the back end. These modules are specified as a comma-separated list.")).clone())).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    Print::printBuf((System::gettext((literal!("The :ref:`--matchingAlgorithm <omcflag-matchingAlgorithm>` sets the method that is used for the matching algorithm, after the pre optimization modules.")).clone())).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    Print::printBuf((System::gettext((literal!("The :ref:`--indexReductionMethod <omcflag-indexReductionMethod>` sets the method that is used for the index reduction, after the pre optimization modules.")).clone())).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    Print::printBuf((System::gettext((literal!("The :ref:`--initOptModules <omcflag-initOptModules>` then sets the optimization modules which are used after the index reduction to optimize the system for initialization, specified as a comma-separated list.")).clone())).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    Print::printBuf((System::gettext((literal!("The :ref:`--postOptModules <omcflag-postOptModules>` then sets the optimization modules which are used after the index reduction to optimize the system for simulation, specified as a comma-separated list.")).clone())).clone())?;
    Print::printBuf((literal!("\n\n")).clone())?;
    usage = (Print::getString()?).clone();
    Print::clearBuf();
    Ok(usage)
}

pub fn printAllConfigFlags() -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringAppendList(List::map(allConfigFlags.clone(), (std::sync::Arc::new(printConfigFlag) as std::sync::Arc<dyn ::std::ops::Fn(Flags::ConfigFlag) -> Result<ArcStr> + 'static>)));
    outString
}

fn printConfigFlag(mut inFlag: Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFlag.clone() {
        Flags::ConfigFlag { visibility: Flags::FlagVisibility::INTERNAL, .. } => {
            literal!("")
        },
        Flags::ConfigFlag { description: mut desc, .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut desc_str: ArcStr = arcstr::literal!("");
            let mut flag_str: ArcStr = arcstr::literal!("");
            let mut delim_str: ArcStr = arcstr::literal!("");
            let mut opt_str: ArcStr = arcstr::literal!("");
            let mut wrapped_str: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            desc_str = (Gettext::translateContent(desc.clone())?).clone();
            name = (Util::stringPadRight(((printConfigFlagName(inFlag.clone(), false)?).0).clone(), 28, (literal!(" ")).clone())).clone();
            flag_str = stringAppendList(list![(name.clone()).clone(), (literal!(" ")).clone(), (desc_str.clone()).clone()]);
            delim_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(descriptionIndent)); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone();
            wrapped_str = StringUtil::wordWrap((flag_str.clone()).clone(), System::getTerminalWidth(), (delim_str.clone()).clone(), metamodelica::OrderedFloat(0.3_f64))?;
            opt_str = (printValidOptions(inFlag.clone())?).clone();
            flag_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(wrapped_str.clone(), (literal!("\n")).clone())); __mm_s.push_str(&*opt_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            flag_str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn printConfigFlagSphinx(mut inFlag: Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFlag.clone() {
        Flags::ConfigFlag { visibility: Flags::FlagVisibility::INTERNAL, .. } => {
            literal!("")
        },
        Flags::ConfigFlag { description: mut desc, .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut longName: ArcStr = arcstr::literal!("");
            let mut desc_str: ArcStr = arcstr::literal!("");
            let mut flag_str: ArcStr = arcstr::literal!("");
            let mut opt_str: ArcStr = arcstr::literal!("");
            desc_str = (Gettext::translateContent(desc.clone())?).clone();
            desc_str = (System::stringReplace((desc_str.clone()).clone(), (literal!("--help=debug")).clone(), (literal!(":ref:`--help=debug <omcflag-debug-section>`")).clone())?).clone();
            desc_str = (System::stringReplace((desc_str.clone()).clone(), (literal!("--help=optmodules")).clone(), (literal!(":ref:`--help=optmodules <omcflag-optmodules-section>`")).clone())?).clone();
            (name, longName) = printConfigFlagName(inFlag.clone(), true)?;
            opt_str = (printValidOptionsSphinx(inFlag.clone())?).clone();
            flag_str = stringAppendList(list![(literal!(".. _omcflag-")).clone(), (longName.clone()).clone(), (literal!(":\n\n:ref:`")).clone(), (name.clone()).clone(), (literal!("<omcflag-")).clone(), (longName.clone()).clone(), (literal!(">`\n\n")).clone(), (desc_str.clone()).clone(), (literal!("\n")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*opt_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()]);
            flag_str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn printConfigFlagName(mut inFlag: Flags::ConfigFlag, mut sphinx: bool) -> Result<(ArcStr, ArcStr)> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut longName: ArcStr = arcstr::literal!("");
    (outString, longName) = (match inFlag.clone() {
        Flags::ConfigFlag { shortname: Some(mut shortname), name: mut name, .. } => {
            shortname = (if (sphinx.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*shortname.clone()); ArcStr::from(__mm_s) }} else {Util::stringPadLeft(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*shortname.clone()); ArcStr::from(__mm_s) }).clone(), 4, (literal!(" ")).clone())}).clone();
            (stringAppendList(list![(shortname.clone()).clone(), (literal!(", --")).clone(), (name.clone()).clone()]), name.clone())
        },
        Flags::ConfigFlag { shortname: None, name: mut name, .. } => {
            ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (sphinx.clone()) {literal!("--")} else {literal!("      --")}); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }, name.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outString, longName))
}

fn printValidOptions(mut inFlag: Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFlag.clone() {
        Flags::ConfigFlag { validOptions: None, .. } => {
            literal!("")
        },
        Flags::ConfigFlag { validOptions: Some(Flags::ValidOptions::STRING_OPTION { options: ref strl }), .. } => {
            let mut opt_str: ArcStr = arcstr::literal!("");
            let mut strl = strl.clone();
            opt_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(descriptionIndent)); __mm_s.push_str(&*literal!("   ")); __mm_s.push_str(&*System::gettext((literal!("Valid options:")).clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(strl.clone(), (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone();
            strl = StringUtil::wordWrap((opt_str.clone()).clone(), System::getTerminalWidth(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(descriptionIndent)); __mm_s.push_str(&*literal!("     ")); ArcStr::from(__mm_s) }).clone(), metamodelica::OrderedFloat(0.3_f64))?;
            opt_str = stringDelimitList(strl.clone(), (literal!("\n")).clone());
            opt_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*opt_str.clone()); ArcStr::from(__mm_s) }).clone();
            opt_str.clone()
        },
        Flags::ConfigFlag { validOptions: Some(Flags::ValidOptions::STRING_DESC_OPTION { options: ref descl }), .. } => {
            let mut opt_str: ArcStr = arcstr::literal!("");
            opt_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(descriptionIndent)); __mm_s.push_str(&*literal!("   ")); __mm_s.push_str(&*System::gettext((literal!("Valid options:")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut d in (descl.clone()).into_iter().cloned() {
            let __x = printFlagOptionDescShort(d.clone(), false);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })); ArcStr::from(__mm_s) }).clone();
            opt_str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn printValidOptionsSphinx(mut inFlag: Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFlag.clone() {
        Flags::ConfigFlag { validOptions: None, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*defaultFlagSphinx(inFlag.defaultValue.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Flags::ConfigFlag { validOptions: Some(Flags::ValidOptions::STRING_OPTION { options: ref strl }), .. } => {
            let mut opt_str: ArcStr = arcstr::literal!("");
            opt_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*defaultFlagSphinx(inFlag.defaultValue.clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::gettext((literal!("Valid options")).clone())); __mm_s.push_str(&*literal!(":\n\n")); __mm_s.push_str(&*{
        let mut __acc = String::new();
        for mut s in (strl.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }); ArcStr::from(__mm_s) }).clone();
            opt_str.clone()
        },
        Flags::ConfigFlag { validOptions: Some(Flags::ValidOptions::STRING_DESC_OPTION { options: ref descl }), .. } => {
            let mut opt_str: ArcStr = arcstr::literal!("");
            opt_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*defaultFlagSphinx(inFlag.defaultValue.clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::gettext((literal!("Valid options")).clone())); __mm_s.push_str(&*literal!(":\n\n")); __mm_s.push_str(&*{
        let mut __acc = String::new();
        for mut s in (descl.clone()).into_iter().cloned() {
            let __x = printFlagOptionDesc(s.clone(), true)?;
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }); ArcStr::from(__mm_s) }).clone();
            opt_str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn defaultFlagSphinx(mut flag: Flags::FlagData) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(flag.clone()) {
        Flags::FlagData::BOOL_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("Boolean (default")).clone())); __mm_s.push_str(&*literal!(" ``")); __mm_s.push_str(&*boolString(var_field!(flag.data, Flags::FlagData::BOOL_FLAG).clone())); __mm_s.push_str(&*literal!("``).")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::INT_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("Integer (default")).clone())); __mm_s.push_str(&*literal!(" ``")); __mm_s.push_str(&*intString(var_field!(flag.data, Flags::FlagData::INT_FLAG).clone())); __mm_s.push_str(&*literal!("``).")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::REAL_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("Real (default")).clone())); __mm_s.push_str(&*literal!(" ``")); __mm_s.push_str(&*realString(var_field!(flag.data, Flags::FlagData::REAL_FLAG).clone())); __mm_s.push_str(&*literal!("``).")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::STRING_FLAG { data: Deref @ "" } => {
            System::gettext((literal!("String (default *empty*).")).clone())
        },
        Flags::FlagData::STRING_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("String (default")).clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*var_field!(flag.data, Flags::FlagData::STRING_FLAG).clone()); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::STRING_LIST_FLAG { data: Deref @ metamodelica::List::Nil } => {
            System::gettext((literal!("String list (default *empty*).")).clone())
        },
        Flags::FlagData::STRING_LIST_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("String list (default")).clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(var_field!(flag.data, Flags::FlagData::STRING_LIST_FLAG).clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::ENUM_FLAG { .. } => {
            let mut i: i32 = 0;
            for mut f in &*var_field!(flag.validValues, Flags::FlagData::ENUM_FLAG).clone() {
                let mut f = f.clone();
                (r#str, i) = f.clone();
                if i.clone() == var_field!(flag.data, Flags::FlagData::ENUM_FLAG).clone() {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::gettext((literal!("String (default ")).clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }).clone();
                    return r#str;
                }
            }
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#ENUM_FLAG Failed#")); __mm_s.push_str(&*anyString(flag.clone())); ArcStr::from(__mm_s) }
        },
        _ => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown default value")); __mm_s.push_str(&*anyString(flag.clone())); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    r#str
}

fn printFlagOptionDescShort(mut inOption: (ArcStr, Gettext::TranslatableContent), mut sphinx: bool) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    (name, _) = inOption.clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (sphinx.clone()) {literal!("* ")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(descriptionIndent)); __mm_s.push_str(&*literal!("    * ")); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outString
}

fn printFlagValidOptionsDesc(mut inFlag: Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut options: Arc<metamodelica::List<(ArcStr, Gettext::TranslatableContent)>> = metamodelica::nil();
    let Flags::CONFIG_FLAG { validOptions: Some(Flags::STRING_DESC_OPTION { options: __pa0 }), .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    options = __pa0.clone();
    outString = ({
        let mut __acc = String::new();
        for mut o in (options.clone()).into_iter().cloned() {
            let __x = printFlagOptionDesc(o.clone(), false)?;
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }).clone();
    Ok(outString)
}

fn sphinxMathMode(mut s: ArcStr) -> Result<ArcStr> {
    let mut o: ArcStr = s.clone();
    let mut i: i32 = 0;
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    (i, strs) = System::regex((o.clone()).clone(), (literal!("^(.*)[$]([^$]*)[$](.*)$")).clone(), 4, true, false);
    if i.clone() == 4 {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(strs.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        s1 = __pa0.clone();
        s2 = __pa1.clone();
        s3 = __pa2.clone();
        o = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" :math:`")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("` ")); __mm_s.push_str(&*s3.clone()); ArcStr::from(__mm_s) }).clone();
    }
    Ok(o)
}

fn removeSphinxMathMode(mut s: ArcStr) -> Result<ArcStr> {
    let mut o: ArcStr = s.clone();
    let mut i: i32 = 0;
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    (i, strs) = System::regex((o.clone()).clone(), (literal!("^(.*):math:`([^`]*)[`](.*)$")).clone(), 4, true, false);
    if i.clone() == 4 {
        o = (removeSphinxMathMode(stringAppendList(listRest(strs.clone())?))?).clone();
    }
    Ok(o)
}

fn printFlagOptionDesc(mut inOption: (ArcStr, Gettext::TranslatableContent), mut sphinx: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut desc: Gettext::TranslatableContent;
    let mut name: ArcStr = arcstr::literal!("");
    let mut desc_str: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    (name, desc) = inOption.clone();
    desc_str = (Gettext::translateContent(desc.clone())?).clone();
    if sphinx.clone() {
        desc_str = ({
        let mut __acc = String::new();
        for mut s in (System::strtok((desc_str.clone()).clone(), (literal!("\n")).clone())).into_iter().cloned() {
            let __x = System::trim((s.clone()).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone());
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    }).clone();
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("* ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*desc_str.clone()); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
    } else {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::stringPadRight(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" * ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone(), 30, (literal!(" ")).clone())); __mm_s.push_str(&*removeSphinxMathMode((desc_str.clone()).clone())?); ArcStr::from(__mm_s) }).clone();
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(StringUtil::wordWrap((r#str.clone()).clone(), System::getTerminalWidth(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(descriptionIndent)); __mm_s.push_str(&*literal!("    ")); ArcStr::from(__mm_s) }).clone(), metamodelica::OrderedFloat(0.3_f64))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

fn printDebugFlag(mut inFlag: Flags::DebugFlag, mut sphinx: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut desc: Gettext::TranslatableContent;
    let mut name: ArcStr = arcstr::literal!("");
    let mut desc_str: ArcStr = arcstr::literal!("");
    let mut default: bool = false;
    let Flags::DEBUG_FLAG { description: __pa0, name: __pa1, default: __pa2, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    desc = __pa0.clone();
    name = __pa1.clone();
    default = __pa2.clone();
    desc_str = (Gettext::translateContent(desc.clone())?).clone();
    if sphinx.clone() {
        desc_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut s in (System::strtok((desc_str.clone()).clone(), (literal!("\n")).clone())).into_iter().cloned() {
            let __x = System::trim((s.clone()).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n  ")).clone());
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n.. _omcflag-debug-")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(":\n\n")); __mm_s.push_str(&*literal!(":ref:`")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" <omcflag-debug-")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(">`")); __mm_s.push_str(&*literal!(" (default: ")); __mm_s.push_str(&*if (default.clone()) {literal!("on")} else {literal!("off")}); __mm_s.push_str(&*literal!(")\n  ")); __mm_s.push_str(&*desc_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::stringPadRight(({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (default.clone()) {literal!(" + ")} else {literal!(" - ")}); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone(), 26, (literal!(" ")).clone())); __mm_s.push_str(&*removeSphinxMathMode((desc_str.clone()).clone())?); ArcStr::from(__mm_s) }).clone();
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(StringUtil::wordWrap((outString.clone()).clone(), System::getTerminalWidth(), (arcstr::literal!(descriptionIndent)).clone(), metamodelica::OrderedFloat(0.3_f64))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub fn debugFlagName(mut inFlag: Flags::DebugFlag) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let Flags::DEBUG_FLAG { name: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    Ok(name)
}

pub fn configFlagName(mut inFlag: Flags::ConfigFlag) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let Flags::CONFIG_FLAG { name: __pa0, .. } = (inFlag.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    Ok(name)
}

fn getValidStringOptions(mut inOptions: Flags::ValidOptions) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut validOptions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    validOptions = (match inOptions.clone() {
        Flags::ValidOptions::STRING_OPTION { options: ref validOptions } => {
            validOptions.clone()
        },
        Flags::ValidOptions::STRING_DESC_OPTION { options: mut options } => {
            List::map(options.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(validOptions)
}

pub fn flagDataEq(mut data1: Flags::FlagData, mut data2: Flags::FlagData) -> bool {
    let mut eq: bool = false;
    eq = (match (data1.clone(), data2.clone()) {
        (Flags::FlagData::EMPTY_FLAG, Flags::FlagData::EMPTY_FLAG) => true,
        (Flags::FlagData::BOOL_FLAG { .. }, Flags::FlagData::BOOL_FLAG { .. }) => var_field!(data1.data, Flags::FlagData::BOOL_FLAG).clone() == var_field!(data2.data, Flags::FlagData::BOOL_FLAG).clone(),
        (Flags::FlagData::INT_FLAG { .. }, Flags::FlagData::INT_FLAG { .. }) => var_field!(data1.data, Flags::FlagData::INT_FLAG).clone() == var_field!(data2.data, Flags::FlagData::INT_FLAG).clone(),
        (Flags::FlagData::INT_LIST_FLAG { .. }, Flags::FlagData::INT_LIST_FLAG { .. }) => List::isEqualOnTrue(var_field!(data1.data, Flags::FlagData::INT_LIST_FLAG).clone(), var_field!(data2.data, Flags::FlagData::INT_LIST_FLAG).clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)),
        (Flags::FlagData::REAL_FLAG { .. }, Flags::FlagData::REAL_FLAG { .. }) => var_field!(data1.data, Flags::FlagData::REAL_FLAG).clone() == var_field!(data2.data, Flags::FlagData::REAL_FLAG).clone(),
        (Flags::FlagData::STRING_FLAG { .. }, Flags::FlagData::STRING_FLAG { .. }) => var_field!(data1.data, Flags::FlagData::STRING_FLAG).clone() == var_field!(data2.data, Flags::FlagData::STRING_FLAG).clone(),
        (Flags::FlagData::STRING_LIST_FLAG { .. }, Flags::FlagData::STRING_LIST_FLAG { .. }) => List::isEqualOnTrue(var_field!(data1.data, Flags::FlagData::STRING_LIST_FLAG).clone(), var_field!(data2.data, Flags::FlagData::STRING_LIST_FLAG).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)),
        (Flags::FlagData::ENUM_FLAG { .. }, Flags::FlagData::ENUM_FLAG { .. }) => referenceEq(&var_field!(data1.validValues, Flags::FlagData::ENUM_FLAG).clone(),&var_field!(data2.validValues, Flags::FlagData::ENUM_FLAG).clone()) && var_field!(data1.data, Flags::FlagData::ENUM_FLAG).clone() == var_field!(data2.data, Flags::FlagData::ENUM_FLAG).clone(),
        _ => false,
    });
    eq
}

pub fn flagDataString(mut flagData: Flags::FlagData) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match flagData.clone() {
        Flags::FlagData::BOOL_FLAG { .. } => {
            boolString(var_field!(flagData.data, Flags::FlagData::BOOL_FLAG).clone())
        },
        Flags::FlagData::INT_FLAG { .. } => {
            intString(var_field!(flagData.data, Flags::FlagData::INT_FLAG).clone())
        },
        Flags::FlagData::INT_LIST_FLAG { .. } => {
            List::toString(var_field!(flagData.data, Flags::FlagData::INT_LIST_FLAG).clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(",")).clone(), (literal!("")).clone(), false, 0)?
        },
        Flags::FlagData::REAL_FLAG { .. } => {
            realString(var_field!(flagData.data, Flags::FlagData::REAL_FLAG).clone())
        },
        Flags::FlagData::STRING_FLAG { .. } => {
            var_field!(flagData.data, Flags::FlagData::STRING_FLAG).clone()
        },
        Flags::FlagData::STRING_LIST_FLAG { .. } => {
            stringDelimitList(var_field!(flagData.data, Flags::FlagData::STRING_LIST_FLAG).clone(), (literal!(",")).clone())
        },
        Flags::FlagData::ENUM_FLAG { .. } => {
            let mut v: i32 = 0;
            for mut vt in &*var_field!(flagData.validValues, Flags::FlagData::ENUM_FLAG).clone() {
                let mut vt = vt.clone();
                (r#str, v) = vt.clone();
                if v.clone() == var_field!(flagData.data, Flags::FlagData::ENUM_FLAG).clone() {
                    return Ok(r#str);
                }
            }
            literal!("")
        },
        _ => {
            literal!("")
        },
    })).clone();
    Ok(r#str)
}

pub fn unparseFlags() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut flagStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fvalue: bool = false;
    if let Ok(Flags::FLAGS { configFlags: __pa0, debugFlags: __pa1 }) = loadFlags(false) {
        config_flags = __pa0.clone();
        debug_flags = __pa1.clone();
    } else {
        return Ok(flagStrings);
    }
    for mut f in &*allConfigFlags.clone() {
        let mut f = f.clone();
        if !(flagDataEq(f.defaultValue.clone(), config_flags.borrow()[(f.index.clone()-1) as usize].clone())) {
            name = ((match f.shortname.clone() {
        Some(mut name) => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) },
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--")); __mm_s.push_str(&*f.name.clone()); ArcStr::from(__mm_s) },
    })).clone();
            flagStrings = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*flagDataString(config_flags.borrow()[(f.index.clone()-1) as usize].clone())?); ArcStr::from(__mm_s) }).clone(), flagStrings.clone());
        }
    }
    for mut f in &*allDebugFlags.clone() {
        let mut f = f.clone();
        fvalue = debug_flags.borrow()[(f.index.clone()-1) as usize].clone();
        if f.default.clone() != fvalue.clone() {
            name = (if (fvalue.clone()) {f.name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("no")); __mm_s.push_str(&*f.name.clone()); ArcStr::from(__mm_s) }}).clone();
            strl = cons((name.clone()).clone(), strl.clone());
        }
    }
    if !(strl.clone().is_empty()) {
        flagStrings = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-d=")); __mm_s.push_str(&*stringDelimitList(strl.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone(), flagStrings.clone());
    }
    Ok(flagStrings)
}

pub fn splitCSV(mut value: ArcStr) -> Arc<metamodelica::List<ArcStr>> {
    let mut outValues: Arc<metamodelica::List<ArcStr>> = System::strtok((value.clone()).clone(), (literal!(",")).clone());
    outValues
}

