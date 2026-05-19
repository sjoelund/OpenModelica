//! Typed global-root variables for the `openmodelica_backend` crate.
//!
//! This file is **manually maintained**. It declares the `thread_local!`
//! statics for global roots whose value types are defined in this crate
//! (and therefore cannot be declared in `openmodelica_util::Globals` without
//! creating a circular dependency).
//!
//! See `openmodelica_util/src/Globals.rs` for the full design rationale.

#![allow(non_snake_case, non_upper_case_globals, type_complexity)]

use std::cell::RefCell;
use std::sync::Arc;

// ── Thread-local roots (process-global by MetaModelica semantics) ─────────────

thread_local! {
    // Index 3 — symbolTable
    //
    // The compiler symbol table. Initialised by the backend before use.
    // Uses todo!() as placeholder; always set before first read.
    pub static symbolTable: RefCell<Arc<crate::SymbolTable::SymbolTable>> =
        RefCell::new(todo!("symbolTable must be initialised before first use"));

    // Index 19 — rewriteRulesIndex
    //
    // Optional list of active rewrite rules. Set to Some(rules) when a
    // rewrite-rule file is loaded; None otherwise.
    // Source: RewriteRules.mo.
    pub static rewriteRulesIndex: RefCell<Option<Arc<metamodelica::List<crate::RewriteRules::Rule>>>> =
        const { RefCell::new(None) };

    // Index 25 — optionSimCode
    //
    // The current SimCode structure, set during SimCode generation.
    // None when not in a SimCode generation pass.
    pub static optionSimCode: RefCell<Option<crate::SimCode::SimCode>> =
        const { RefCell::new(None) };

    // Index 26 — interactiveCache
    //
    // Cache of interactive lookup results:
    //   list of (program, path, environment).
    // Set to Some(list) when cache is populated; None otherwise.
    pub static interactiveCache: RefCell<Option<Arc<metamodelica::List<(
        openmodelica_frontend::Absyn::Program,
        Arc<openmodelica_frontend::Absyn::Path>,
        crate::Interactive::GraphicEnvCache,
    )>>>> = const { RefCell::new(None) };
}
