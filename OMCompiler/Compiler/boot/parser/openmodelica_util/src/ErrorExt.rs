// Manually written file.
//
// Rust port of `OMCompiler/Compiler/Util/ErrorExt.mo`'s `external "C"`
// declarations. The MetaModelica source defines this module purely as
// FFI shims into `OMCompiler/Compiler/runtime/errorext.cpp`, so the
// auto-generator emits `todo!()` stubs; we replace those with a real
// implementation here.
//
// The C++ side keeps one `errorext_members` per thread containing:
//
//   * `errorMessageQueue`        — deque of pending messages
//   * `checkPoints`              — stack of (queue_position, id) pairs
//   * `numErrorMessages`         — running count of ERROR/INTERNAL severities
//   * `numWarningMessages`       — running count of WARNING severities
//   * `lastDeletedCheckpoint`    — last id passed to `delCheckpoint` (used
//                                   to provide diagnostic context for stack
//                                   underflow assertions)
//   * `showErrorMessages`        — when true, every message is also echoed
//                                   to stderr at push time
//   * `pop_more_on_rollback`     — duplicate-suppression flag used by
//                                   `pop_message` during `rollBack`
//
// We mirror that state in a `thread_local!` `RefCell` so the Rust port
// preserves the per-thread isolation that the bootstrap depends on. The
// generated code never touches `threadData_t` directly (the codegen drops
// the `OpenModelica.threadData()` argument when lowering the external
// call), so the Rust API is parameterless.
//
// Functions that need richer runtime support than the message buffer
// alone (currently `registerModelicaFormatError`, `initAssertionFunctions`,
// and `moveMessagesToParentThread`) are intentionally left as no-ops —
// the bootstrap compiler does not use them, and the doc-comment on each
// records the C++ behavior we are forgoing.

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::sync::Arc;

use arcstr::ArcStr;
use metamodelica::{List, SourceInfo, nil, cons};

use crate::ErrorTypes::{Message, Severity, MessageType, TotalMessage};
use crate::Gettext::TranslatableContent;
use crate::Gettext::TranslatableContent as Trans;

/// One slot in the per-thread error queue.
///
/// Stored separately from the `TotalMessage` we hand out to MetaModelica
/// callers so we can keep the original (untranslated) `tokens` list around
/// for `printMessagesStr` and friends without forcing every reader to
/// reach into the message's source info.
#[derive(Clone, Debug)]
struct QueuedMessage {
    msg: Message,
    tokens: Arc<List<ArcStr>>,
    info: SourceInfo,
}

impl QueuedMessage {
    fn as_total(&self) -> TotalMessage {
        TotalMessage { msg: self.msg.clone(), info: self.info.clone() }
    }

    /// Mirrors `ErrorMessage::getFullMessage()` on the C++ side: the
    /// rendered text without trailing newline. Used by `pop_message`'s
    /// duplicate-suppression and by `printMessagesStr`.
    fn full_message(&self) -> String {
        let body = substitute_tokens(&content_text(&self.msg.message), &self.tokens);
        format!(
            "[{}:{}.{}-{}.{} {}] {}: {}",
            self.info.fileName,
            self.info.lineNumberStart,
            self.info.columnNumberStart,
            self.info.lineNumberEnd,
            self.info.columnNumberEnd,
            if self.info.isReadOnly { "readonly" } else { "writable" },
            severity_label(&self.msg.severity),
            body,
        )
    }
}

fn content_text(c: &TranslatableContent) -> ArcStr {
    match c {
        Trans::gettext { msgid } => msgid.clone(),
        Trans::notrans { r#str } => r#str.clone(),
    }
}

/// Substitute `%s` placeholders with the supplied tokens, in order. Extra
/// tokens are ignored; missing tokens leave the placeholder verbatim.
/// This matches `ErrorMessage::TokenList`-style substitution closely
/// enough for the diagnostic output the bootstrap actually inspects.
fn substitute_tokens(template: &str, tokens: &Arc<List<ArcStr>>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    let mut cur = tokens.clone();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&n) = chars.peek() {
                if n == 's' {
                    chars.next();
                    if let List::Cons { head, tail } = &*cur {
                        out.push_str(head);
                        cur = tail.clone();
                    } else {
                        out.push_str("%s");
                    }
                    continue;
                }
            }
        }
        out.push(c);
    }
    out
}

fn severity_label(s: &Severity) -> &'static str {
    match s {
        Severity::INTERNAL => "Internal error",
        Severity::ERROR => "Error",
        Severity::WARNING => "Warning",
        Severity::NOTIFICATION => "Notification",
    }
}

#[derive(Default)]
struct State {
    queue: Vec<QueuedMessage>,
    /// Stack of (queue_length_at_set_time, id) pairs.
    check_points: Vec<(usize, ArcStr)>,
    num_errors: i32,
    num_warnings: i32,
    last_deleted_checkpoint: ArcStr,
    show_messages: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

fn bump_counters(state: &mut State, severity: &Severity, delta: i32) {
    match severity {
        Severity::ERROR | Severity::INTERNAL => state.num_errors += delta,
        Severity::WARNING => state.num_warnings += delta,
        Severity::NOTIFICATION => {}
    }
}

// ---------------------------------------------------------------------------
// Public API — matches the signatures the auto-generated stub used to have.
// All return types stay `()` / plain primitives because the upstream
// MetaModelica declarations are `external "C"` with no `failure` clause.
// ---------------------------------------------------------------------------

/// Push a new diagnostic onto the per-thread queue.
pub fn addSourceMessage(
    id: i32,
    msg_type: MessageType,
    msg_severity: Severity,
    sline: i32,
    scol: i32,
    eline: i32,
    ecol: i32,
    read_only: bool,
    filename: ArcStr,
    msg: ArcStr,
    tokens: Arc<List<ArcStr>>,
) {
    let entry = QueuedMessage {
        msg: Message {
            id,
            ty: msg_type,
            severity: msg_severity.clone(),
            // The C++ side stores the rendered message verbatim, so we
            // wrap the raw string in `notrans` rather than re-running it
            // through gettext at every read.
            message: Trans::notrans { r#str: msg.clone() },
        },
        tokens,
        info: SourceInfo {
            fileName: filename,
            isReadOnly: read_only,
            lineNumberStart: sline,
            columnNumberStart: scol,
            lineNumberEnd: eline,
            columnNumberEnd: ecol,
            lastModification: metamodelica::OrderedFloat(0.0),
        },
    };
    with_state(|s| {
        if s.show_messages {
            eprintln!("{}", entry.full_message());
        }
        bump_counters(s, &msg_severity, 1);
        s.queue.push(entry);
    });
}

pub fn clearMessages() {
    with_state(|s| {
        s.queue.clear();
        s.check_points.clear();
        s.num_errors = 0;
        s.num_warnings = 0;
    });
}

/// Pop the topmost checkpoint without affecting messages added after it.
///
/// `id` is recorded in `last_deleted_checkpoint` so that subsequent stack
/// underflow can produce a helpful diagnostic — the C++ side does the
/// same dance.
pub fn delCheckpoint(id: ArcStr) {
    with_state(|s| {
        if s.check_points.pop().is_none() {
            // Stack underflow — match C++ by printing to stderr instead of
            // panicking so the surrounding compilation continues.
            eprintln!("ErrorExt.delCheckpoint: no checkpoint to delete (id={id})");
        }
        s.last_deleted_checkpoint = id;
    });
}

/// Pop the topmost `n` checkpoints. Used to unwind after a stack-overflow
/// exception where the matching `delCheckpoint`s were skipped.
pub fn deleteNumCheckpoints(n: i32) {
    with_state(|s| {
        for _ in 0..n.max(0) {
            if s.check_points.pop().is_none() {
                break;
            }
        }
    });
}

/// Free a previously `popCheckPoint`-saved list of message handles.
///
/// In the C++ runtime each handle is a raw `ErrorMessage*` heap pointer
/// that must be `delete`d. In Rust the `QueuedMessage` lives by value, so
/// there is nothing to free — the handle list is purely an opaque
/// MetaModelica value we no longer reference once it is dropped.
pub fn freeMessages(_handles: Arc<List<i32>>) {
    // Intentionally empty — see doc comment.
}

/// Return the messages added since the most recent checkpoint, in queue
/// order. Used by callers that want to inspect a transactional batch
/// before deciding whether to `rollBack` or `delCheckpoint`.
pub fn getCheckpointMessages() -> Arc<List<TotalMessage>> {
    with_state(|s| {
        let start = s.check_points.last().map(|(p, _)| *p).unwrap_or(0);
        list_from_slice(&s.queue[start..])
    })
}

pub fn getMessages() -> Arc<List<TotalMessage>> {
    with_state(|s| list_from_slice(&s.queue))
}

fn list_from_slice(slice: &[QueuedMessage]) -> Arc<List<TotalMessage>> {
    // MetaModelica lists are head-first: the most recently added message
    // becomes the head of the returned list. Mirrors the C++ iteration
    // order in `Error_getMessages`.
    let mut out = nil::<TotalMessage>();
    for m in slice.iter() {
        out = cons(m.as_total(), out);
    }
    out
}

pub fn getNumCheckpoints() -> i32 {
    with_state(|s| s.check_points.len() as i32)
}

pub fn getNumErrorMessages() -> i32 {
    with_state(|s| s.num_errors)
}

pub fn getNumMessages() -> i32 {
    with_state(|s| s.queue.len() as i32)
}

pub fn getNumWarningMessages() -> i32 {
    with_state(|s| s.num_warnings)
}

/// Register OMC's `assert(...)` family to route output through the
/// error buffer instead of stdout. The bootstrap compiler never relies
/// on this redirection (it always reads errors back through the queue
/// directly), so we leave it as a no-op.
pub fn initAssertionFunctions() {
    // Intentional no-op — see doc comment.
}

pub fn isTopCheckpoint(id: ArcStr) -> bool {
    with_state(|s| s.check_points.last().map(|(_, cid)| cid == &id).unwrap_or(false))
}

/// Hand off pending messages to the parent thread's queue when a worker
/// thread terminates. The bootstrap is single-threaded with respect to
/// the error buffer, so there is no parent to merge into.
pub fn moveMessagesToParentThread() {
    // Intentional no-op — see doc comment.
}

/// Roll back the messages added since the most recent checkpoint and
/// return their ids in a list so the caller can re-push them later
/// via [`pushMessages`].
///
/// The returned list of "handles" mirrors the C++ runtime's `void*` queue
/// of detached `ErrorMessage*`s; on the Rust side it is stored in a
/// thread-local side table keyed by an opaque integer.
pub fn popCheckPoint(id: ArcStr) -> Arc<List<i32>> {
    with_state(|s| {
        let start = s.check_points.last().map(|(p, _)| *p).unwrap_or(0);
        if !s.check_points.last().map(|(_, cid)| cid == &id).unwrap_or(false) {
            eprintln!(
                "ErrorExt.popCheckPoint: id mismatch (expected {:?}, got {id:?})",
                s.check_points.last().map(|(_, cid)| cid.as_str()),
            );
        }
        let detached: Vec<QueuedMessage> = s.queue.drain(start..).collect();
        for d in &detached {
            bump_counters(s, &d.msg.severity, -1);
        }
        s.check_points.pop();
        let mut handles = nil::<i32>();
        for d in detached.into_iter().rev() {
            let h = store_detached(d);
            handles = cons(h, handles);
        }
        handles
    })
}

pub fn printCheckpointMessagesStr(warningsAsErrors: bool) -> ArcStr {
    with_state(|s| {
        let start = s.check_points.last().map(|(p, _)| *p).unwrap_or(0);
        render_messages(&s.queue[start..], warningsAsErrors)
    })
}

pub fn printErrorsNoWarning() -> ArcStr {
    with_state(|s| {
        let mut out = String::new();
        for m in &s.queue {
            if matches!(m.msg.severity, Severity::ERROR | Severity::INTERNAL) {
                out.push_str(&m.full_message());
                out.push('\n');
            }
        }
        ArcStr::from(out)
    })
}

pub fn printMessagesStr(warningsAsErrors: bool) -> ArcStr {
    with_state(|s| render_messages(&s.queue, warningsAsErrors))
}

fn render_messages(slice: &[QueuedMessage], warnings_as_errors: bool) -> ArcStr {
    let mut out = String::new();
    for m in slice {
        let promoted = warnings_as_errors && matches!(m.msg.severity, Severity::WARNING);
        if promoted {
            // Show the original severity replaced with Error so callers
            // see the promotion in the rendered output.
            let mut m2 = m.clone();
            m2.msg.severity = Severity::ERROR;
            out.push_str(&m2.full_message());
        } else {
            out.push_str(&m.full_message());
        }
        out.push('\n');
    }
    ArcStr::from(out)
}

/// Push previously [`popCheckPoint`]-detached handles back onto the queue.
pub fn pushMessages(handles: Arc<List<i32>>) {
    let mut cur = handles;
    let mut batch = Vec::new();
    while let List::Cons { head, tail } = &*cur {
        if let Some(d) = take_detached(*head) {
            batch.push(d);
        }
        cur = tail.clone();
    }
    with_state(|s| {
        for d in batch {
            bump_counters(s, &d.msg.severity, 1);
            s.queue.push(d);
        }
    });
}

/// Register the runtime's `ModelicaFormatError` hook. Unused by the
/// bootstrap — see comments at the top of this file.
pub fn registerModelicaFormatError() {
    // Intentional no-op — see doc comment.
}

/// Roll back messages added since the most recent checkpoint and discard
/// them. The checkpoint itself is removed.
pub fn rollBack(_id: ArcStr) {
    with_state(|s| {
        if let Some((start, _)) = s.check_points.pop() {
            let drained = s.queue.split_off(start);
            for d in &drained {
                bump_counters(s, &d.msg.severity, -1);
            }
        }
    });
}

pub fn rollbackNumCheckpoints(n: i32) {
    for _ in 0..n.max(0) {
        with_state(|s| {
            if let Some((start, _)) = s.check_points.pop() {
                let drained = s.queue.split_off(start);
                for d in &drained {
                    bump_counters(s, &d.msg.severity, -1);
                }
            }
        });
    }
}

pub fn setCheckpoint(id: ArcStr) {
    with_state(|s| {
        let pos = s.queue.len();
        s.check_points.push((pos, id));
    });
}

pub fn setShowErrorMessages(inShow: bool) {
    with_state(|s| s.show_messages = inShow);
}

// ---------------------------------------------------------------------------
// Detached-message handle table.
//
// `popCheckPoint` hands MetaModelica callers an opaque integer handle for
// each detached message; `pushMessages` later trades that handle back for
// the original `QueuedMessage`. The C++ runtime stores raw heap pointers
// here, but Rust forbids transmuting an owned value through an `i32`, so
// we use a thread-local sparse table instead. Handles are monotonically
// increasing within a thread so there is no aliasing across detach/attach
// cycles even if a caller leaks them.
// ---------------------------------------------------------------------------

thread_local! {
    static DETACHED: RefCell<DetachedTable> = RefCell::new(DetachedTable::default());
}

#[derive(Default)]
struct DetachedTable {
    next_id: i32,
    slots: std::collections::HashMap<i32, QueuedMessage>,
}

fn store_detached(m: QueuedMessage) -> i32 {
    DETACHED.with(|d| {
        let mut d = d.borrow_mut();
        // Skip 0 so the value never collides with the C runtime's NULL
        // sentinel — callers that round-trip through C-style code paths
        // (none in the bootstrap, but cheap to preserve) can treat 0 as
        // "no handle".
        d.next_id = d.next_id.wrapping_add(1);
        if d.next_id == 0 {
            d.next_id = 1;
        }
        let id = d.next_id;
        d.slots.insert(id, m);
        id
    })
}

fn take_detached(id: i32) -> Option<QueuedMessage> {
    DETACHED.with(|d| d.borrow_mut().slots.remove(&id))
}
