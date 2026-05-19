//! Typed global-root variables for the `openmodelica_util` crate.
//!
//! This file is **manually maintained**. The auto-generated code in the
//! other `.rs` files in this crate references these variables via
//! `crate::Globals::NAME` (within the crate) or
//! `openmodelica_util::Globals::NAME` (from outside).
//!
//! # Background
//!
//! MetaModelica uses `setGlobalRoot(Global.INDEX, value)` /
//! `getGlobalRoot(Global.INDEX)` to store and retrieve global state.
//! The original C MMC runtime backs this with two arrays:
//!   * thread-local (indices 0–8):  `threadData->localRoots`
//!   * process-global (indices 9+): `mmc_GC_state->global_roots`
//!
//! In Rust, every named slot is exposed as a typed `thread_local!` variable.
//!
//! # Why `thread_local!` for everything (including "global" slots)?
//!
//! `Array<T> = Rc<RefCell<Vec<T>>>` is **not** `Send`, which prevents the use
//! of `static Mutex<T>` for slots that may store arrays. We therefore use
//! `thread_local!` for *all* named roots for now.
//!
//! TODO: when `Array<T>` is changed to `Arc<Mutex<Vec<T>>>` (making it
//! `Send`), migrate the process-global roots (index 9+) to `static Mutex<T>`
//! and update `global_root_var_path` in `mmtorust/src/codegen.rs` accordingly.
//!
//! # Access pattern
//!
//! *Read:*  `NAME.with(|__root| __root.borrow().clone())`
//! *Write:* `NAME.with(|__root| *__root.borrow_mut() = value)`
//!
//! Both are infallible; no `?` propagation is needed at call sites.
//!
//! # Adding new entries
//!
//! When the compiler reports a missing `Globals::XXX`, find the type from the
//! MetaModelica source (look at the `setGlobalRoot` / `getGlobalRoot` call
//! sites for `Global.XXX`) and add a `thread_local!` declaration here.  Run
//! `cargo run -p mmtorust` afterwards to regenerate the call sites.

#![allow(non_snake_case, non_upper_case_globals)]

use std::cell::RefCell;
use anyhow::Result;
use arcstr::ArcStr;
use metamodelica::{Array, SourceInfo};

// ── Thread-local roots (index 0–8, C: threadData->localRoots) ────────────────

thread_local! {
    /// Index 0 — Whether to force instantiation of functions only.
    ///
    /// Set to `Some(true)` before inst-only runs; `None` otherwise.
    /// Source: `CevalScript.mo` (setter), `Static.mo` (reader).
    ///
    /// Note: `Global.simulationData` also maps to index 0 but is only used in
    /// simulation builds.  The two constants are mutually exclusive; they share
    /// the same underlying slot but the compiler build only ever uses
    /// `instOnlyForcedFunctions`.
    pub static instOnlyForcedFunctions: RefCell<Option<bool>> =
        const { RefCell::new(None) };

    // Index 1 — codegenTryThrowIndex
    // Type: TODO — used in SimCodeFunctionUtil.mo. Determine from usage.

    // Index 2 — codegenFunctionList
    // Type: TODO — used in SimCode.mo / SimCodeFunctionUtil.mo.

    // Index 3 — symbolTable
    // Type: TODO — used in multiple frontend files.

    // Indices 4–8 are unused in the MetaModelica sources seen so far.
}

// ── Process-global roots (index 9+, C: mmc_GC_state->global_roots) ───────────
//
// These SHOULD be `static Mutex<T>` to match C semantics.  Currently using
// `thread_local!` because `Array<T> = Rc<RefCell<Vec<T>>>` is not `Send`.
// See the module-level doc comment for the upgrade path.

thread_local! {
    // Index 9  — instHashIndex
    // Type: lives in openmodelica_frontend; not declared here.

    // Index 10 — instNFInstCacheIndex
    // Type: TODO.

    // Index 11 — instNFNodeCacheIndex
    // Type: TODO.

    // Index 12 — instNFLookupCacheIndex
    // Type: TODO.

    // Index 13 — builtinIndex
    // Type: lives in openmodelica_frontend; not declared here.

    // Index 14 — builtinEnvIndex
    // Type: TODO.

    // Index 15 — profilerTime1Index
    // Type: TODO.

    // Index 16 — profilerTime2Index
    // Type: TODO.

    /// Index 17 — Compiler flags.
    ///
    /// Initialised by `FlagsUtil.loadFlags`; read by `Flags.getFlags`.
    pub static flagsIndex: RefCell<Option<crate::Flags::Flag>> =
        const { RefCell::new(None) };

    // Index 18 — builtinGraphIndex
    // Type: lives in openmodelica_frontend; not declared here.

    // Index 19 — rewriteRulesIndex
    // Type: TODO — used in RewriteRules.mo (Script package).

    /// Index 20 — Stack-overflow sentinel.
    ///
    /// Set to `None` before code that may overflow; set to `Some(())` as a
    /// marker when overflow is detected and caught.
    /// Source: `BackendDAECreate.mo`, `Util.mo`, `DAEMode.mo`.
    ///
    /// TODO: Confirm the stored value is indeed `Option<()>` and not a richer
    /// type by inspecting the `getGlobalRoot` call sites.
    pub static stackoverFlowIndex: RefCell<Option<()>> =
        const { RefCell::new(None) };

    /// Index 21 — GC profiling statistics.
    ///
    /// Stores the GC stats snapshot at the last call to `execStatReset`.
    /// Set and read by `ExecStat.mo`.
    pub static gcProfilingIndex: RefCell<Option<openmodelica_util_datatypes_basic::GCExt::ProfStats>> =
        const { RefCell::new(None) };

    // Index 22 — inlineHashTable
    // Type: TODO — used in Inline.mo (FrontEnd package).

    /// Index 23 — Current component being instantiated.
    ///
    /// A triple of parallel arrays: component name strings, source-location
    /// records, and prefix-to-string functions.  Written by
    /// `Error.updateCurrentComponent`; read by
    /// `Error.getCurrentComponent` / `Error.addMessage`.
    pub static currentInstVar: RefCell<
        Option<(
            Array<ArcStr>,
            Array<SourceInfo>,
            Array<fn(ArcStr) -> Result<ArcStr>>,
        )>
    > = const { RefCell::new(None) };

    // Index 24 — operatorOverloadingCache
    // Type: AVL-tree structures — OperatorOverloading.mo (FrontEnd package).

    // Index 25 — optionSimCode
    // Type: Option<SimCode> — lives in openmodelica_backend (SimCode package).

    // Index 26 — interactiveCache
    // Type: TODO — Interactive.mo (Script package).

    // Index 27 — isInStream
    // Type: TODO — NFConnectEquations.mo / BackendDAEUtil.mo.

    // Index 28 — MMToJLListIndex
    // Type: TODO — JuliaLink list.

    // Index 29 — packageIndexCacheIndex
    // Type: TODO — PackageManagement.mo (Script package).

    // Index 30 — sharedLibraryCacheIndex
    // Type: TODO — NFEvalFunction.mo.

    // Index 31 — backendInterface
    // Type: TODO — BackendInterface.mo (FrontEnd package).
}
