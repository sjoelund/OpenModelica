//! Typed global-root variables for the `openmodelica_frontend_dump` crate.
//!
//! This file is **manually maintained**. It declares the `thread_local!`
//! statics for global roots whose value types are defined in this crate
//! (and therefore cannot be declared in `openmodelica_util::Globals` without
//! creating a circular dependency).
//!
//! See `openmodelica_util/src/Globals.rs` for the full design rationale.

#![allow(non_snake_case, non_upper_case_globals, type_complexity)]

use std::cell::RefCell;

// ── Thread-local roots (process-global by MetaModelica semantics) ─────────────

thread_local! {
    // Index 31 — backendInterface
    //
    // Function table populated by backend registration (frontend_dump side).
    // Uses todo!() since there is no meaningful empty state; always set
    // before first use. See FrontEnd/BackendInterface.mo (upstream).
    pub static backendInterface: RefCell<crate::BackendInterface::BackendInterfaceFunctions> =
        RefCell::new(todo!("backendInterface must be registered before first use"));
}
