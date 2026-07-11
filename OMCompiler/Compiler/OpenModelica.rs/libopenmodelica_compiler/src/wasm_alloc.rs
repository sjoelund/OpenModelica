//! Per-thread talc allocator with segment-tagged cross-thread frees.
//!
//! wasm's default heap serialises every allocation on one global lock, and a
//! single global talc (`TalcLock`) is no better — parsing is net-allocating, so
//! all worker threads contend on the one lock and parallel parsing regresses.
//!
//! Instead each thread owns a `TalcCell` (a real freeing allocator, unlike the
//! old bump), so the hot path takes no lock. Cross-thread frees — a worker's AST
//! outlives the worker and is dropped on the master — are made sound by tagging
//! every segment with its owning heap: memory comes in 4 MiB `SEG`-aligned segments
//! whose base word holds the owner id, so `ptr & !(SEG-1)` recovers the owner in
//! O(1). Freeing a block owned by another thread pushes it onto that thread's
//! lock-free remote-free stack; the owner reclaims it on its next allocation.
//! The rayon pool is persistent, so a heap is never destroyed while its memory
//! is still live elsewhere. Large allocations bypass talc and go straight to the
//! (thread-safe) system heap, tagged so frees route back the same way.

use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use talc::base::Talc;
use talc::base::binning::Binning;
use talc::cell::TalcCell;
use talc::source::Source;
use talc::wasm::WasmBinning;

const SEG: usize = 1 << 22; // 4 MiB per segment
const SEG_MASK: usize = SEG - 1;
const HEADER: usize = 64; // reserved, tag-holding prefix of each segment
const BIG_THRESHOLD: usize = SEG - HEADER - 256; // larger requests bypass talc
const MAX_HEAPS: usize = 128; // pool threads + main; ids beyond this go big-only

const MAGIC_SEG: u32 = 0x5345_474d; // talc segment
const MAGIC_BIG: u32 = 0x4249_474d; // standalone system allocation

#[repr(C)]
struct SegHead {
    magic: u32,
    owner: u32,
    big_size: usize,
    big_off: usize,
}

// Threaded onto a heap's remote-free stack, stored inside the freed block.
// Only `size` is needed to free (talc's dealloc ignores alignment); a block is
// always >= CHUNK_UNIT - TAIL = 12 bytes usable, so the 8-byte node fits.
#[repr(C)]
struct Node {
    next: *mut Node,
    size: usize,
}

struct Remote {
    head: AtomicPtr<Node>,
}

static REMOTE: [Remote; MAX_HEAPS] =
    [const { Remote { head: AtomicPtr::new(null_mut()) } }; MAX_HEAPS];
static NEXT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
struct SegSource {
    id: u32,
}

// SAFETY: `acquire` only touches `System` (a distinct allocator), never the
// parent `TalcCell` or the global allocator, and does not allocate elsewhere.
unsafe impl Source for SegSource {
    fn acquire<B: Binning>(talc: &mut Talc<Self, B>, _layout: Layout) -> Result<(), ()> {
        let id = talc.source.id;
        unsafe {
            let base = System.alloc(Layout::from_size_align_unchecked(SEG, SEG));
            if base.is_null() {
                return Err(());
            }
            let head = base as *mut SegHead;
            (*head).magic = MAGIC_SEG;
            (*head).owner = id;
            talc.claim(base.add(HEADER), SEG - HEADER).map(|_| ()).ok_or(())
        }
    }
}

struct Local {
    id: u32,
    cell: TalcCell<SegSource, WasmBinning>,
}

thread_local! {
    static LOCAL: Local = {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Local { id, cell: TalcCell::new(SegSource { id }) }
    };
}

// Reclaim blocks freed by other threads into this heap.
#[inline]
unsafe fn drain(l: &Local) {
    let slot = &REMOTE[l.id as usize].head;
    // Cheap relaxed check keeps the empty common case off the RMW/hot path.
    if slot.load(Ordering::Relaxed).is_null() {
        return;
    }
    let mut node = slot.swap(null_mut(), Ordering::Acquire);
    while !node.is_null() {
        let size = unsafe { (*node).size };
        let next = unsafe { (*node).next };
        unsafe {
            l.cell
                .dealloc(node as *mut u8, Layout::from_size_align_unchecked(size, 1))
        };
        node = next;
    }
}

// A standalone SEG-aligned system block, tagged so `dealloc` can free it from
// any thread. `off` places the payload past the header while keeping it in the
// base segment (so masking still finds the tag) and honouring `layout.align()`.
#[inline]
unsafe fn big_alloc(layout: Layout) -> *mut u8 {
    let off = HEADER.max(layout.align());
    debug_assert!(off < SEG);
    let total = off + layout.size();
    unsafe {
        let base = System.alloc(Layout::from_size_align_unchecked(total, SEG));
        if base.is_null() {
            return null_mut();
        }
        let head = base as *mut SegHead;
        (*head).magic = MAGIC_BIG;
        (*head).big_size = total;
        (*head).big_off = off;
        base.add(off)
    }
}

pub struct TalcThreadCache;

unsafe impl GlobalAlloc for TalcThreadCache {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > BIG_THRESHOLD {
            return unsafe { big_alloc(layout) };
        }
        LOCAL.with(|l| unsafe {
            if l.id as usize >= MAX_HEAPS {
                return big_alloc(layout);
            }
            drain(l);
            let p = l.cell.alloc(layout);
            if p.is_null() { big_alloc(layout) } else { p }
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let base = ((ptr as usize) & !SEG_MASK) as *mut SegHead;
        match unsafe { (*base).magic } {
            MAGIC_BIG => {
                let total = unsafe { (*base).big_size };
                unsafe {
                    System.dealloc(base as *mut u8, Layout::from_size_align_unchecked(total, SEG))
                };
            }
            MAGIC_SEG => {
                let owner = unsafe { (*base).owner };
                LOCAL.with(|l| unsafe {
                    if owner == l.id {
                        drain(l);
                        l.cell.dealloc(ptr, layout);
                    } else {
                        let node = ptr as *mut Node;
                        (*node).size = layout.size();
                        let slot = &REMOTE[owner as usize].head;
                        let mut cur = slot.load(Ordering::Relaxed);
                        loop {
                            (*node).next = cur;
                            match slot.compare_exchange_weak(
                                cur,
                                node,
                                Ordering::Release,
                                Ordering::Relaxed,
                            ) {
                                Ok(_) => break,
                                Err(e) => cur = e,
                            }
                        }
                    }
                });
            }
            _ => debug_assert!(false, "wasm_alloc: bad segment magic"),
        }
    }
}
