//! Registry of external C runtime calls used by MetaModelica sources.
//!
//! Every `external "C"` function referenced from the MetaModelica source must
//! be listed here together with its fallibility classification. The classifier
//! is consulted by [`crate::fallibility`] when building the call-graph: an
//! external function that *can* fail (e.g. throws via `MMC_THROW()` in the
//! OpenModelica runtime, returns a status code, or otherwise reports errors)
//! propagates `Result<_>` to its callers; an infallible one does not.
//!
//! ## Why a hand-curated table?
//!
//! The OpenModelica C runtime expresses errors through several mechanisms
//! (`MMC_THROW()`, `c_add_message(...)`, return codes, `errno`), and the
//! MetaModelica binding signature alone does not say which mechanism applies.
//! There is no machine-readable manifest on the C side either. So the only
//! safe approach is to enumerate the externals manually after inspecting the
//! corresponding `.c` source under `OMCompiler/Compiler/runtime/`.
//!
//! ## Strict mode
//!
//! [`lookup_or_panic`] panics if the external name is not listed. That makes
//! the table self-policing: adding a new `external "C"` declaration in the MM
//! source forces a compile-time-equivalent decision before code generation
//! runs.  We deliberately do *not* default to "fallible" — silently
//! over-approximating would defeat the whole point of the analysis.
//!
//! ## Adding entries
//!
//! For each new external, read the C implementation under
//! `OMCompiler/Compiler/runtime/` and check whether it:
//!   * calls `MMC_THROW()` / `MMC_THROW_INTERNAL()` / similar long-jump exits,
//!   * calls `c_add_message(..., ErrorLevel_error, ...)` followed by `MMC_THROW`,
//!   * returns a status code that the MM wrapper checks,
//!   * may set `errno` / abort.
//! Any of those → `Fallibility::Fallible`. Otherwise → `Fallibility::Infallible`.

use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Whether an external C function can fail (and therefore returns `Result<T>`
/// in our lowering) or is provably infallible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fallibility {
    /// Function never fails: it has no `MMC_THROW`, no error reporting,
    /// no error-returning side channel. The Rust lowering returns the
    /// raw output type without a `Result` wrapper.
    Infallible,
    /// Function can fail: it propagates errors back to the MetaModelica
    /// runtime via `MMC_THROW` / `c_add_message` / status return. The
    /// Rust lowering wraps the output in `Result<T>`.
    Fallible,
}

/// Static registry of known external "C" calls.
///
/// Built lazily on first use; the table is intentionally append-only — once
/// a classification is committed it should only change if the underlying C
/// behavior changes. Each entry must be justified by an inspection of the
/// matching C source.
fn registry() -> &'static BTreeMap<&'static str, Fallibility> {
    static REGISTRY: OnceLock<BTreeMap<&'static str, Fallibility>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        use Fallibility::*;
        let mut m: BTreeMap<&'static str, Fallibility> = BTreeMap::new();

        // ── ErrorExt — error/diagnostic infrastructure ────────────────────────
        // These manipulate the message stack; none of them throw on their own.
        // Source: OMCompiler/Compiler/runtime/errorext.cpp (no MMC_THROW).
        m.insert("ErrorImpl__setCheckpoint", Infallible);
        m.insert("ErrorImpl__rollBack", Infallible);
        m.insert("ErrorImpl__delCheckpoint", Infallible);
        m.insert("ErrorImpl__rollbackNumCheckpoint", Infallible);
        m.insert("ErrorImpl__getNumErrorMessages", Infallible);
        m.insert("ErrorImpl__getNumWarningMessages", Infallible);
        m.insert("ErrorImpl__getNumMessages", Infallible);
        m.insert("ErrorImpl__clearMessages", Infallible);
        m.insert("ErrorImpl__getMessages", Infallible);

        // ── System — process / filesystem / string utilities ──────────────────
        // Most of these wrap libc; the wrappers that allocate or open files
        // throw on failure.
        // Source: OMCompiler/Compiler/runtime/systemimpl.c.
        m.insert("SystemImpl__regex", Fallible);   // compiles a regex; throws on bad pattern
        m.insert("SystemImpl__getuid", Infallible);
        m.insert("SystemImpl__readFile", Fallible);
        m.insert("SystemImpl__writeFile", Fallible);
        m.insert("SystemImpl__realpath", Fallible);
        m.insert("SystemImpl__dirname", Infallible);
        m.insert("SystemImpl__basename", Infallible);
        m.insert("System_realtimeClock", Infallible);
        m.insert("System_realtimeTick", Infallible);
        m.insert("System_realtimeTock", Infallible);

        // ── Print — output buffer manipulation, no failure path ───────────────
        // Source: OMCompiler/Compiler/runtime/printimpl.c.
        m.insert("PrintImpl__printBufSpace", Infallible);
        m.insert("PrintImpl__printBufNewLine", Infallible);
        m.insert("PrintImpl__getString", Infallible);
        m.insert("PrintImpl__clearBuf", Infallible);

        // NOTE: the above is a deliberately small seed set, just enough for the
        // fallibility analyser to start with a known-good corpus. The complete
        // population — projected at ~467 entries based on a `grep -c` over the
        // MetaModelica sources — is left as follow-up work; the analyser will
        // panic on the first unlisted external it encounters, which forces the
        // table to be filled in lockstep with the call-graph walk.

        m
    })
}

/// Look up an external C function by its `funcName` (the symbol referenced
/// from the `external "C" ...` clause). Returns `None` if not registered.
///
/// Codegen will use this once it learns to emit external bindings; for
/// the analysis-phase consumer, see [`lookup_or_panic`].
#[allow(dead_code)]
pub fn lookup(name: &str) -> Option<Fallibility> {
    registry().get(name).copied()
}

/// Strict variant of [`lookup`] — panics with an explanatory message if the
/// external is not yet listed.  Use from analysis-phase code where the table
/// is required to be exhaustive.
///
/// `mm_qname` is the dotted MM-side name of the wrapper function (used only
/// for the panic diagnostic, so a missing entry can be traced back to the
/// MetaModelica declaration).
///
/// ## Lenient escape hatch
///
/// Setting `MMTORUST_LENIENT_EXTERNALS=1` in the environment downgrades the
/// panic to a one-shot stderr warning and returns [`Fallibility::Fallible`]
/// (the conservative classification: any call site keeps its `?`). This
/// exists *only* to unblock development while the 400+ entry registry is
/// being populated; CI and release builds should leave it unset so that the
/// strict invariant is enforced.
pub fn lookup_or_panic(c_name: &str, mm_qname: &str) -> Fallibility {
    if let Some(f) = registry().get(c_name) {
        return *f;
    }
    if lenient_mode() {
        record_lenient_miss(c_name);
        return Fallibility::Fallible;
    }
    panic!(
        "external_c_calls: no fallibility entry for external \"C\" function `{c_name}` \
         (used by MetaModelica function `{mm_qname}`).\n\
         Add an entry to mmtorust/src/external_c_calls.rs after inspecting the \
         corresponding C source under OMCompiler/Compiler/runtime/.\n\
         To bypass during bulk registry population, set MMTORUST_LENIENT_EXTERNALS=1 \
         — but only as a temporary measure; missing entries default to `Fallible` \
         and silently bloat the generated code."
    );
}

fn lenient_mode() -> bool {
    static LENIENT: OnceLock<bool> = OnceLock::new();
    *LENIENT.get_or_init(|| matches!(
        std::env::var("MMTORUST_LENIENT_EXTERNALS").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes")
    ))
}

/// Track misses in lenient mode so we can emit one consolidated warning per
/// distinct symbol — flooding stderr with a line per call site would obscure
/// the real signal.
fn record_lenient_miss(c_name: &str) {
    use std::sync::Mutex;
    static MISSES: OnceLock<Mutex<std::collections::BTreeSet<String>>> = OnceLock::new();
    let set = MISSES.get_or_init(|| Mutex::new(std::collections::BTreeSet::new()));
    let mut guard = set.lock().expect("MISSES mutex");
    if guard.insert(c_name.to_owned()) {
        eprintln!("warning: external_c_calls: unlisted external `{c_name}` (assuming Fallible — lenient mode)");
    }
}

/// Number of registered externals — diagnostic aid for the analysis summary.
pub fn registered_count() -> usize {
    registry().len()
}
