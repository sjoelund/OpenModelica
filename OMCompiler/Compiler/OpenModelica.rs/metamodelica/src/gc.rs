//! Cycle collection for MetaModelica values: a trial-deletion collector over
//! the existing `Arc`/`Rc` representation.
//!
//! Immutable MetaModelica values are constructed bottom-up and can never be
//! cyclic on their own; reference cycles only arise when a mutable cell
//! (`Mutable.update` / `Pointer.update`) is updated with a value that
//! transitively contains the cell. Plain `Arc` leaks such cycles.
//!
//! # Why not an off-the-shelf tracing pointer (`dumpster`, `bacon-rajan-cc`)
//!
//! Tracing collectors of that family require every traced handle to live
//! *inside* an allocation the collector owns: the counting pass visits each
//! allocation once and assumes each handle it sees is owned by the allocation
//! being traversed, and the drop pass dismantles handle-containing memory it
//! believes is garbage. MetaModelica values are shared `Arc` graphs — a
//! cycle-closing value is routinely also referenced from the stack or from
//! another value (shared list tails, caches). Tracing *through* an `Arc`
//! breaks both assumptions: a handle inside a doubly-shared `Arc` is counted
//! twice (frees live data), and the drop pass kills handles inside `Arc`s
//! that outlive the garbage (dead-handle panics). This was demonstrated with
//! a real use-after-collect against `dumpster` before this design was chosen
//! (see `mutable_cycle_drop_tests.rs::content_shared_with_stack_survives_collect`).
//!
//! # The design
//!
//! Instead of changing value representation, the collector works with the
//! `Arc` graph the way CPython's cycle detector works with refcounted
//! objects: compare, per shared allocation, the number of in-graph handles
//! against the allocation's actual strong count.
//!
//! * Every mutable cell registers a weak handle in a thread-local registry
//!   at creation (cells are the only places a cycle can be closed, hence
//!   the only candidate roots worth scanning from).
//! * [`collect`] snapshots the registry and traverses each cell's content
//!   once, structurally, via [`MMTrace`]. Each shared allocation (`Arc`,
//!   `Rc`, cell) is reported to the visitor with its address and strong
//!   count and traversed at most once, so each physical handle slot is
//!   counted at most once.
//! * An allocation whose counted in-graph slots are fewer than its strong
//!   count has a handle somewhere the traversal cannot see — the stack, a
//!   global, a closure capture, an unregistered structure. Such allocations
//!   are *external roots*: everything reachable from them is marked live.
//! * A snapshot cell not reachable from any root is garbage: every handle to
//!   it (and to everything it keeps alive) lies inside the dead subgraph.
//!   The collector breaks the cycle by *poisoning* the cell — replacing its
//!   content with the collected sentinel — and lets ordinary `Arc`/`Rc`
//!   drops cascade. No memory is freed behind anyone's back: a bug in the
//!   analysis can manifest only as a leak or as a "accessed a collected
//!   cell" panic, never as a use-after-free.
//!
//! Anything the traversal cannot see is automatically safe: an `Arc<dyn Fn>`
//! capture holding a handle is never counted, so the handle's target counts
//! as externally referenced and is kept (a leak, not a correctness issue).
//!
//! # Threads
//!
//! The registry is thread-local: cells are candidates only on the thread
//! that created them, and [`collect`] only collects that thread's cells
//! (cells created inside `System.launchParallelTasks` workers are never
//! collected — a bounded leak). Concurrent threads cannot break the
//! analysis: a thread can only reach a value through a handle it already
//! holds, and that handle is invisible to the traversal, which makes the
//! value an external root. Cells locked by another thread mid-traversal
//! abort the whole collection (`Err` propagation through [`MMTrace`]) — a
//! conservative no-op.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Weak};

use arcstr::ArcStr;

use crate::List;

// ── tracing ───────────────────────────────────────────────────────────────────

/// Driver side of a traversal over the shared-allocation graph.
///
/// [`MMTrace`] impls report every *handle slot* they own that designates a
/// shared allocation (`Arc`, `Rc`, mutable cell) via [`visit_shared`], and
/// descend into the allocation's interior exactly when it returns `true`
/// (the visitor returns `true` only on the first visit of each allocation).
/// After a descent the impl must call [`leave_shared`] — the visitor
/// maintains the traversal source for edge recording with that pair.
///
/// [`visit_shared`]: MMVisitor::visit_shared
/// [`leave_shared`]: MMVisitor::leave_shared
pub trait MMVisitor {
    /// Report one handle slot referencing the shared allocation at `ptr`
    /// (its data address) whose current strong count is `strong`. Returns
    /// whether the caller should descend into the allocation's interior.
    /// `type_name` is the payload type (for collector diagnostics only).
    fn visit_shared(&mut self, ptr: *const (), strong: usize, type_name: &'static str) -> bool;
    /// Close the descent opened by the matching `visit_shared` that
    /// returned `true`. Must not be called otherwise.
    fn leave_shared(&mut self);
}

/// Structural tracing of MetaModelica values: report every shared handle
/// slot, delegate into every field that can transitively contain one, skip
/// scalar leaves.
///
/// Generated types receive an impl emitted by mmtorust; runtime container
/// types delegate below. `Err(())` means the traversal hit interior state it
/// could not read (a locked cell, a borrowed `RefCell`) — the collector
/// aborts the whole collection in response, since partial traversals cannot
/// be trusted for the mark phase.
pub trait MMTrace {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()>;
}

// ── scalar leaves ─────────────────────────────────────────────────────────────

macro_rules! mm_trace_leaf {
    ($($t:ty),* $(,)?) => {$(
        impl MMTrace for $t {
            #[inline]
            fn mm_accept(&self, _visitor: &mut dyn MMVisitor) -> Result<(), ()> {
                Ok(())
            }
        }
    )*};
}

mm_trace_leaf!(
    (), bool, char,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    String,
    // `ArcStr` is shared, but a string can never contain a handle; reporting
    // it would only bloat the collector's maps.
    ArcStr,
);

impl MMTrace for ordered_float::OrderedFloat<f64> {
    #[inline]
    fn mm_accept(&self, _visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

/// Strings, flags and line numbers only.
impl MMTrace for crate::SourceInfo {
    #[inline]
    fn mm_accept(&self, _visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

/// References are opaque leaves. A borrowed value can only originate on some
/// stack frame, and whatever owns it keeps everything beneath it externally
/// rooted, so hiding it from the traversal is safe (it can only make the
/// collector more conservative). This impl exists so generic instantiations
/// that flow borrows (callback forwarding) satisfy the blanket `T: MMTrace`
/// bounds on generated signatures.
impl<T: ?Sized> MMTrace for &T {
    #[inline]
    fn mm_accept(&self, _visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

// ── shared handles: report, then delegate to the payload once ────────────────

impl<T: MMTrace + ?Sized> MMTrace for Arc<T> {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        if visitor.visit_shared(
            Arc::as_ptr(self) as *const (),
            Arc::strong_count(self),
            std::any::type_name::<T>(),
        ) {
            let r = (**self).mm_accept(visitor);
            visitor.leave_shared();
            r
        } else {
            Ok(())
        }
    }
}

impl<T: MMTrace + ?Sized> MMTrace for Rc<T> {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        if visitor.visit_shared(
            Rc::as_ptr(self) as *const (),
            Rc::strong_count(self),
            std::any::type_name::<T>(),
        ) {
            let r = (**self).mm_accept(visitor);
            visitor.leave_shared();
            r
        } else {
            Ok(())
        }
    }
}

// ── owned containers: delegate structurally ───────────────────────────────────

impl<T: MMTrace + ?Sized> MMTrace for Box<T> {
    #[inline]
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        (**self).mm_accept(visitor)
    }
}

impl<T: MMTrace> MMTrace for Option<T> {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        match self {
            Some(x) => x.mm_accept(visitor),
            None => Ok(()),
        }
    }
}

impl<T: MMTrace> MMTrace for Vec<T> {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        for x in self {
            x.mm_accept(visitor)?;
        }
        Ok(())
    }
}

/// A `RefCell` that is currently borrowed cannot be read; report `Err` and
/// let the collector abort. At the manual collection points the generated
/// compiler uses (between phases, no MM code on the stack) no borrow can be
/// active, so this is defensive.
impl<T: MMTrace + ?Sized> MMTrace for RefCell<T> {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        self.try_borrow().map_err(|_| ())?.mm_accept(visitor)
    }
}

/// Iterative, not recursive: lists are routinely tens of thousands of
/// elements long and a recursive traversal would overflow the stack. Every
/// spine cell is itself a shared allocation (tail sharing is pervasive), so
/// each cell is reported like any other `Arc`.
impl<T: MMTrace + Clone> MMTrace for List<T> {
    fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
        let mut depth = 0usize;
        let mut cur = self;
        let r = loop {
            let Some(cell) = &cur.0 else { break Ok(()) };
            if !visitor.visit_shared(
                Arc::as_ptr(cell) as *const (),
                Arc::strong_count(cell),
                std::any::type_name::<crate::ListNode<T>>(),
            ) {
                break Ok(());
            }
            depth += 1;
            let crate::ListNode::Cons { head, tail } = &**cell else { break Ok(()) };
            if let e @ Err(()) = head.mm_accept(visitor) {
                break e;
            }
            cur = tail;
        };
        for _ in 0..depth {
            visitor.leave_shared();
        }
        r
    }
}

macro_rules! mm_trace_tuple {
    ($($name:ident : $idx:tt),+) => {
        impl<$($name: MMTrace),+> MMTrace for ($($name,)+) {
            fn mm_accept(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()> {
                $(self.$idx.mm_accept(visitor)?;)+
                Ok(())
            }
        }
    };
}

mm_trace_tuple!(A: 0);
mm_trace_tuple!(A: 0, B: 1);
mm_trace_tuple!(A: 0, B: 1, C: 2);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15);

// `Array<T>` (= `Rc<RefCell<Vec<T>>>`) is covered by composing the `Rc`,
// `RefCell` and `Vec` impls above.

// ── function values: opaque leaves, automatically safe ───────────────────────
//
// Codegen lowers MetaModelica function values to `Arc<dyn Fn(..) -> .. +
// 'static>`. A closure's captures cannot be traversed, so these impls accept
// trivially. Unlike a collector that frees what it traces, hiding captures
// here is *safe by construction*: a handle inside a capture is simply never
// counted, so its target always looks externally referenced and is kept
// alive (the cycle leaks, nothing dangles).
//
// One impl per arity; extend the list if codegen ever emits a higher arity.

macro_rules! mm_trace_dyn_fn {
    ($($args:ident),*) => {
        impl<Ret $(,$args)*> MMTrace for dyn Fn($($args),*) -> Ret + 'static {
            #[inline]
            fn mm_accept(&self, _visitor: &mut dyn MMVisitor) -> Result<(), ()> {
                Ok(())
            }
        }
    };
}

/// Plain function pointers carry no captures at all; trivially accepted.
/// (They appear in cell contents, e.g. SimCodeUtil's tuple-shaped caches.)
macro_rules! mm_trace_fn_ptr {
    ($($args:ident),*) => {
        impl<Ret $(,$args)*> MMTrace for fn($($args),*) -> Ret {
            #[inline]
            fn mm_accept(&self, _visitor: &mut dyn MMVisitor) -> Result<(), ()> {
                Ok(())
            }
        }
    };
}

mm_trace_fn_ptr!();
mm_trace_fn_ptr!(A);
mm_trace_fn_ptr!(A, B);
mm_trace_fn_ptr!(A, B, C);
mm_trace_fn_ptr!(A, B, C, D);
mm_trace_fn_ptr!(A, B, C, D, E);
mm_trace_fn_ptr!(A, B, C, D, E, F);
mm_trace_fn_ptr!(A, B, C, D, E, F, G);
mm_trace_fn_ptr!(A, B, C, D, E, F, G, H);

mm_trace_dyn_fn!();
mm_trace_dyn_fn!(A);
mm_trace_dyn_fn!(A, B);
mm_trace_dyn_fn!(A, B, C);
mm_trace_dyn_fn!(A, B, C, D);
mm_trace_dyn_fn!(A, B, C, D, E);
mm_trace_dyn_fn!(A, B, C, D, E, F);
mm_trace_dyn_fn!(A, B, C, D, E, F, G);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I, J);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I, J, K);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I, J, K, L);

// ── the cell registry ─────────────────────────────────────────────────────────

/// Collector-facing view of a mutable cell (`Mutable`/`Pointer`). The cell
/// types live in `openmodelica_util_datatypes_basic`; this trait is what
/// they register here so [`collect`] can drive them without knowing their
/// content type.
pub trait TraceableCell {
    /// Trace the cell's current content (without reporting the cell itself —
    /// the collector accounts for the snapshot handle separately).
    /// `Err` means the content is unreadable right now (locked elsewhere).
    fn trace_content(&self, visitor: &mut dyn MMVisitor) -> Result<(), ()>;
    /// Drop the cell's content, leaving the "collected" sentinel behind.
    /// Only called on cells proven unreachable; a subsequent access (which
    /// would be a collector bug) panics rather than reading freed memory.
    fn poison(&self);
}

/// A point in the registration order, from [`mark`]. Cells registered at or
/// after it are the candidates of a scoped collection.
pub type Mark = u64;

thread_local! {
    /// All mutable cells created on this thread that are still alive, each
    /// tagged with the [`Mark`] it was registered at (weak handles; dead
    /// entries are purged on every collection and, amortized, on
    /// registration). Only ever pushed to, so it stays sorted by mark and a
    /// scoped collection can binary-search its starting point — purging
    /// compacts the `Vec` but preserves the order.
    static CELL_REGISTRY: RefCell<Vec<(Mark, Weak<dyn TraceableCell>)>> =
        const { RefCell::new(Vec::new()) };
    /// The mark the next registration gets. Monotonic, never reset: an index
    /// into the registry would not survive a purge, a mark does.
    static NEXT_MARK: std::cell::Cell<Mark> = const { std::cell::Cell::new(1) };
    /// Registry length right after the last purge. When the registry grows
    /// to twice this, dead entries are purged inline — without this, a
    /// program that never calls [`collect`] would accumulate one `Weak`
    /// entry *and* one live control block (the `Weak` pins it) per dead
    /// cell. The 2× rule makes registration amortized O(1).
    static REGISTRY_WATERMARK: std::cell::Cell<usize> = const { std::cell::Cell::new(64) };
    /// Where the last [`collect_new`] stopped, i.e. the start of the next
    /// scope.
    static LAST_SCOPE: std::cell::Cell<Mark> = const { std::cell::Cell::new(0) };
}

/// The mark a cell registered right now would get. Everything registered
/// from here on is in scope for `collect_since(mark())`.
pub fn mark() -> Mark {
    NEXT_MARK.with(std::cell::Cell::get)
}

/// Register a freshly created cell as a collection candidate. Called by the
/// cell constructors in `openmodelica_util_datatypes_basic`.
pub fn register_cell(cell: Weak<dyn TraceableCell>) {
    let m = NEXT_MARK.with(|n| {
        let m = n.get();
        n.set(m + 1);
        m
    });
    CELL_REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        reg.push((m, cell));
        let watermark = REGISTRY_WATERMARK.with(std::cell::Cell::get);
        if reg.len() >= watermark.saturating_mul(2) {
            reg.retain(|(_, w)| w.strong_count() > 0);
            REGISTRY_WATERMARK.with(|m| m.set(reg.len().max(64)));
        }
    });
}

// ── the collector ─────────────────────────────────────────────────────────────

/// Outcome of one [`collect`] call, for logging and tests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CollectStats {
    /// Live cells considered as candidates.
    pub candidate_cells: usize,
    /// Shared allocations visited by the counting traversal.
    pub traced_allocations: usize,
    /// Cells proven unreachable and poisoned.
    pub collected_cells: usize,
    /// True if the collection was abandoned because some interior state was
    /// unreadable (a cell locked by another thread, a borrowed `RefCell`).
    /// Nothing was poisoned in that case.
    pub aborted: bool,
}

/// Per shared allocation: the strong count snapshotted at first encounter,
/// the number of in-graph handle slots found so far, and the head of its
/// outgoing-edge chain.
///
/// Allocations are addressed by a dense index, and edges live in one flat
/// arena threaded as a per-node chain, so a collection allocates a handful of
/// growable buffers instead of one `Vec` per traced allocation. That side
/// table is the collector's dominant cost, and a collection runs on every
/// interactive statement.
struct AllocNode {
    strong: u32,
    slots: u32,
    /// Diagnostics only, and 16 bytes on every traced allocation — see the
    /// note above; off unless `gc-diagnostics` is on.
    #[cfg(feature = "gc-diagnostics")]
    type_name: &'static str,
    /// First outgoing edge, or [`NO_EDGE`].
    edge_head: u32,
    /// The collector holds a snapshot handle to this allocation, which no
    /// in-graph slot accounts for.
    is_candidate: bool,
}

const NO_EDGE: u32 = u32::MAX;

/// The counting traversal: builds the allocation graph while ensuring each
/// allocation's interior is entered exactly once. `stack` tracks the
/// allocation whose interior is being traversed, so each reported slot is
/// recorded as an edge from its owner.
struct CountVisitor {
    index: HashMap<*const (), u32>,
    nodes: Vec<AllocNode>,
    /// `(target, next)` — an intrusive chain per owner.
    edges: Vec<(u32, u32)>,
    stack: Vec<u32>,
}

impl CountVisitor {
    fn new() -> Self {
        CountVisitor {
            index: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            stack: Vec::new(),
        }
    }

    /// Index of `ptr`, inserting a fresh node with the given seed values.
    fn intern(&mut self, ptr: *const (), strong: usize, slots: u32, _type_name: &'static str) -> (u32, bool) {
        match self.index.entry(ptr) {
            std::collections::hash_map::Entry::Occupied(o) => (*o.get(), false),
            std::collections::hash_map::Entry::Vacant(v) => {
                let i = self.nodes.len() as u32;
                v.insert(i);
                self.nodes.push(AllocNode {
                    strong: strong.min(u32::MAX as usize) as u32,
                    slots,
                    #[cfg(feature = "gc-diagnostics")]
                    type_name: _type_name,
                    edge_head: NO_EDGE,
                    is_candidate: false,
                });
                (i, true)
            }
        }
    }

    fn add_edge(&mut self, owner: u32, target: u32) {
        let e = self.edges.len() as u32;
        let head = self.nodes[owner as usize].edge_head;
        self.edges.push((target, head));
        self.nodes[owner as usize].edge_head = e;
    }
}

impl MMVisitor for CountVisitor {
    fn visit_shared(&mut self, ptr: *const (), strong: usize, type_name: &'static str) -> bool {
        let (idx, first) = self.intern(ptr, strong, 1, type_name);
        if !first {
            self.nodes[idx as usize].slots += 1;
        }
        if let Some(owner) = self.stack.last().copied() {
            self.add_edge(owner, idx);
        }
        if first {
            self.stack.push(idx);
        }
        first
    }

    fn leave_shared(&mut self) {
        self.stack.pop();
    }
}

/// Run one cycle collection over the cells created on the current thread.
///
/// Call this only from quiescent points — no MetaModelica value borrows on
/// the current thread's stack below the caller (the generated compiler's
/// `GCExt.gcollect` sites satisfy this). Collections are never triggered
/// implicitly; without explicit calls, cycles simply leak as before.
pub fn collect() -> CollectStats {
    collect_impl(0, false)
}

/// Collect only over cells registered at or after `since`, bounding the
/// traversal to that scope's own garbage instead of the whole live heap.
///
/// Safe in the same sense a full collection is, and strictly more
/// conservative: handles living outside the traced subgraph — including
/// every handle held by an older cell — are invisible, which makes their
/// targets look externally rooted and keeps them. A scoped pass can
/// therefore only fail to free garbage, never free something live. What it
/// gives up is a cycle closed by updating a cell that predates `since`; that
/// one needs a full [`collect`].
pub fn collect_since(since: Mark) -> CollectStats {
    collect_impl(since, false)
}

/// Collect over everything registered since the previous `collect_new`, and
/// open a new scope. The intended per-statement / per-request trigger: the
/// caller keeps no mark of its own, so an early return or a failed statement
/// cannot leave the scope unbalanced.
///
/// Deliberately not batched. Batching was measured and is *slower*: a scope
/// spanning N statements traverses far more than N single-statement scopes,
/// because the union of their reachable sets overlaps heavily (400 trivial
/// statements: 1.60 s per-statement vs 2.40 s at a 8192-cell threshold, over a
/// 1.30 s no-collection baseline). Keep scopes minimal and make a collection
/// cheap instead.
pub fn collect_new() -> CollectStats {
    let since = LAST_SCOPE.with(std::cell::Cell::get);
    LAST_SCOPE.with(|c| c.set(mark()));
    collect_impl(since, false)
}

/// [`collect`] with diagnostics on stderr: root/allocation counts and, for a
/// few representative kept cells, the type chain from the cell back to the
/// external root that pins it. Use this to answer "why is this not being
/// collected?" — a path ending in an allocation with `slots < strong` shows
/// exactly which untraced handles (globals, stack, closure captures) keep
/// the subgraph alive.
/// Needs the `gc-diagnostics` feature to name the types in a pin path;
/// without it the counts still print but each step is anonymous.
pub fn collect_with_diagnostics() -> CollectStats {
    collect_impl(0, true)
}

fn collect_impl(since: Mark, diagnose: bool) -> CollectStats {
    let mut stats = CollectStats::default();

    // Snapshot the in-scope live cells, purging dead weak handles in the same
    // pass. Holding strong handles for the duration of the collection keeps
    // every candidate alive until we are done with it: each snapshot handle
    // adds exactly 1 to its cell's strong count, accounted for below.
    let snapshot: Vec<Arc<dyn TraceableCell>> = CELL_REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        // The registry is sorted by mark, so the scope is a suffix.
        let from = reg.partition_point(|(m, _)| *m < since);
        let alive: Vec<Arc<dyn TraceableCell>> =
            reg[from..].iter().filter_map(|(_, w)| w.upgrade()).collect();
        reg.retain(|(_, w)| w.strong_count() > 0);
        alive
    });
    stats.candidate_cells = snapshot.len();

    // The data address of a cell, as its own `MMTrace` impl reports it via
    // `visit_shared` (the `Arc<CellInner<T>>` payload address). Casting the
    // fat trait-object pointer to `*const ()` keeps exactly that address.
    let addr = |cell: &Arc<dyn TraceableCell>| Arc::as_ptr(cell) as *const ();

    // Counting pass: traverse each cell's interior once, building the
    // allocation graph. Cells reached as handle slots first (from another
    // cell's content) are descended into by their own `MMTrace` impl; the
    // explicit loop only enters cells not already visited that way. The
    // node is created with `slots: 1` by `visit_shared`, so entering via
    // the loop pre-creates it with `slots: 0` instead (no handle slot — we
    // got here through the registry).
    let mut count = CountVisitor::new();
    for cell in &snapshot {
        let a = addr(cell);
        if count.index.contains_key(&a) {
            continue; // already traversed via some handle slot
        }
        // Entered from the registry, not through a handle slot, so it starts
        // with no slots of its own.
        let (i, _) = count.intern(a, Arc::strong_count(cell), 0, "<registered cell>");
        count.stack.push(i);
        let traced = cell.trace_content(&mut count);
        count.stack.pop();
        if traced.is_err() {
            stats.aborted = true;
            return stats;
        }
    }
    debug_assert!(count.stack.is_empty(), "unbalanced visit_shared/leave_shared");
    stats.traced_allocations = count.nodes.len();

    // The snapshot handle we hold contributes 1 to each candidate's strong
    // count that no slot accounts for, whether the cell was entered from the
    // registry or first reached through a slot.
    for cell in &snapshot {
        if let Some(&i) = count.index.get(&addr(cell)) {
            count.nodes[i as usize].is_candidate = true;
        }
    }

    // Root determination: any allocation with handles the traversal did not
    // find (stack, globals, closure captures, other threads, unregistered
    // owners) is an external root.
    let mut reachable = vec![false; count.nodes.len()];
    let mut is_root = vec![false; count.nodes.len()];
    let mut work: Vec<u32> = Vec::new();
    for (i, n) in count.nodes.iter().enumerate() {
        if n.slots < n.strong.saturating_sub(u32::from(n.is_candidate)) {
            is_root[i] = true;
            work.push(i as u32);
        }
    }

    if diagnose {
        eprintln!("[gc] roots: {} of {} traced allocations", work.len(), count.nodes.len());
    }
    // Mark pass over the recorded edges. Under diagnostics, remember each
    // allocation's first predecessor so a path root → cell can be
    // reconstructed below.
    let mut pred: Vec<u32> = if diagnose { vec![NO_EDGE; count.nodes.len()] } else { Vec::new() };
    while let Some(p) = work.pop() {
        if reachable[p as usize] {
            continue;
        }
        reachable[p as usize] = true;
        let mut e = count.nodes[p as usize].edge_head;
        while e != NO_EDGE {
            let (target, next) = count.edges[e as usize];
            if diagnose && !reachable[target as usize] && target != p && pred[target as usize] == NO_EDGE {
                pred[target as usize] = p;
            }
            work.push(target);
            e = next;
        }
    }

    // Everything else is only referenced from within the traced graph.
    // Poisoning the unreachable cells drops their contents; ordinary
    // refcounted drops cascade and free the cycles.
    let mut printed = 0;
    for cell in &snapshot {
        let Some(&i) = count.index.get(&addr(cell)) else { continue };
        if !reachable[i as usize] {
            cell.poison();
            stats.collected_cells += 1;
        } else if cfg!(feature = "gc-diagnostics") && diagnose && !is_root[i as usize] && printed < 3 {
            // A kept, non-root cell: show what pins it. The path walks the
            // mark tree from the cell back toward the root that first
            // reached it; the last entry has `slots < strong`, i.e. handles
            // the traversal could not see.
            printed += 1;
            let mut path = vec![i];
            let mut cur = i;
            while pred[cur as usize] != NO_EDGE {
                let q = pred[cur as usize];
                path.push(q);
                if is_root[q as usize] || path.len() > 25 {
                    break;
                }
                cur = q;
            }
            eprintln!("[gc] kept cell, pin path (cell .. root):");
            for &pi in &path {
                let n = &count.nodes[pi as usize];
                #[cfg(feature = "gc-diagnostics")]
                eprintln!("[gc]   slots {}/{} strong  {}", n.slots, n.strong, n.type_name);
                #[cfg(not(feature = "gc-diagnostics"))]
                eprintln!("[gc]   slots {}/{} strong", n.slots, n.strong);
            }
        }
    }
    if diagnose {
        eprintln!(
            "[gc] cells: {} candidates, {} collected",
            stats.candidate_cells, stats.collected_cells
        );
    }

    // The cascade above may have freed cells; purge their registry entries
    // eagerly so repeated collections do not rescan dead weak handles.
    CELL_REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        reg.retain(|(_, w)| w.strong_count() > 0);
        REGISTRY_WATERMARK.with(|m| m.set(reg.len().max(64)));
    });

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time checks that the shapes generated code actually embeds are
    // traceable: Array, List-of-tuple, nested options.
    fn assert_mm_trace<T: MMTrace>() {}

    #[test]
    fn representative_shapes_are_traceable() {
        assert_mm_trace::<crate::Array<i32>>();
        assert_mm_trace::<List<(ArcStr, i32)>>();
        assert_mm_trace::<Option<Box<(String, Vec<f64>)>>>();
    }
}
