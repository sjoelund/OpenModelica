//! Translation of GCExt.mo
//!
//! This module provides bindings to the garbage collector (GC) via the `omcgc` C library.
//! It exposes GC control functions, profiling statistics, and heap management utilities.
//!
//! All external C functions link against the `omcgc` library.

use std::ffi::c_int;

// ============================================================================
// Extern "C" declarations (C API from omcgc)
// ============================================================================

unsafe extern "C" {
    fn GC_gcollect();
    fn GC_gcollect_and_unmap();
    fn GC_enable();
    fn GC_disable();
    fn GC_expand_hp(sz: f64) -> c_int;
    fn GC_set_free_space_divisor(divisor: c_int);
    fn GC_get_force_unmap_on_gcollect() -> c_int;
    fn GC_set_force_unmap_on_gcollect(force_unmap: c_int);
    fn omc_GC_set_max_heap_size(size: usize);
}

// ============================================================================
// ProfStats record type (from ProfStats uniontype)
// ============================================================================

/// Represents GC profiling statistics.
/// Mirrors the PROFSTATS record from GCExt.
#[derive(Debug, Clone)]
pub struct ProfStats {
    pub heapsize_full: i64,
    pub free_bytes_full: i64,
    pub unmapped_bytes: i64,
    pub bytes_allocd_since_gc: i64,
    pub allocd_bytes_before_gc: i64,
    pub non_gc_bytes: i64,
    pub gc_no: i64,
    pub markers_m1: i64,
    pub bytes_reclaimed_since_gc: i64,
    pub reclaimed_bytes_before_gc: i64,
}

impl ProfStats {
    /// Format profiling stats as a human-readable string.
    /// Mirrors the `profStatsStr` function from GCExt.mo.
    ///
    /// # Parameters
    /// * `head` - Prefix string for the output
    /// * `delimiter` - Delimiter between fields (default: newline + two spaces)
    pub fn to_string_with(&self, head: &str, delimiter: &str) -> String {
        format!(
            "{}{}heapsize_full: {}{}free_bytes_full: {}{}unmapped_bytes: {}{}bytes_allocd_since_gc: {}{}allocd_bytes_before_gc: {}{}total_allocd_bytes: {}{}non_gc_bytes: {}{}gc_no: {}{}markers_m1: {}{}bytes_reclaimed_since_gc: {}{}reclaimed_bytes_before_gc: {}",
            head,
            delimiter,
            self.heapsize_full,
            delimiter,
            self.free_bytes_full,
            delimiter,
            self.unmapped_bytes,
            delimiter,
            self.bytes_allocd_since_gc,
            delimiter,
            self.allocd_bytes_before_gc,
            delimiter,
            self.bytes_allocd_since_gc + self.allocd_bytes_before_gc,
            delimiter,
            self.non_gc_bytes,
            delimiter,
            self.gc_no,
            delimiter,
            self.markers_m1,
            delimiter,
            self.bytes_reclaimed_since_gc,
            delimiter,
            self.reclaimed_bytes_before_gc,
        )
    }
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - GC_gcollect
/// Forces a garbage collection cycle.
pub fn gcollect() {
    unsafe { GC_gcollect() }
}

/// External "C" call - GC_gcollect_and_unmap
/// Forces a garbage collection and unmaps unused memory.
pub fn gcollect_and_unmap() {
    unsafe { GC_gcollect_and_unmap() }
}

/// External "C" call - GC_enable
/// Enables the garbage collector.
pub fn enable() {
    unsafe { GC_enable() }
}

/// External "C" call - GC_disable
/// Disables the garbage collector.
pub fn disable() {
    unsafe { GC_disable() }
}

/// External "C" call - GC_free via omc_GC_free_ext
/// Frees a single object allocated by the GC.
///
/// # Safety
/// The `data` pointer must point to a valid GC-allocated object
/// that was allocated as a single element (not a multi-element
/// allocation via list routines).
///
/// # Warning
/// Calling this is very dangerous. You might be better off trying to
/// set variables to a constant value if you want to GC them. Use this if
/// you are concerned about temporary variables remaining on the stack
/// for a long time.
pub unsafe fn free<T>(data: *mut T) {
    unsafe extern "C" {
        fn omc_GC_free_ext(data: *mut std::ffi::c_void);
    }
    unsafe { omc_GC_free_ext(data as *mut std::ffi::c_void) }
}

/// External "C" call - GC_expand_hp via GC_expand_hp macro.
/// Expands the heap by the given size (in bytes).
/// Returns true if successful, false otherwise.
///
/// # Parameters
/// * `sz` - Size to expand the heap by (f64 to avoid 32-bit signed limit)
pub fn expand_heap(sz: f64) -> bool {
    unsafe { GC_expand_hp(sz) != 0 }
}

/// External "C" call - GC_set_free_space_divisor
/// Sets the free space divisor for the garbage collector.
///
/// # Note
/// Do not set divisor < 3 as that seems to interfere with parallel threads.
///
/// # Parameters
/// * `divisor` - The divisor value (default: 3)
pub fn set_free_space_divisor(divisor: i32) {
    unsafe {
        // The Include macro in MetaModelica defines:
        // #define GC_set_free_space_divisor_int(divisor) GC_set_free_space_divisor(divisor)
        GC_set_free_space_divisor(divisor)
    }
}

/// External "C" call - GC_get_force_unmap_on_gcollect
/// Gets the current force unmap on GC setting.
///
/// # Returns
/// True if unmap is forced on garbage collection.
pub fn get_force_unmap_on_gcollect() -> bool {
    unsafe { GC_get_force_unmap_on_gcollect() != 0 }
}

/// External "C" call - GC_set_force_unmap_on_gcollect
/// Sets whether to force unmap on garbage collection.
///
/// # Parameters
/// * `force_unmap` - Whether to force unmap on GC
pub fn set_force_unmap_on_gcollect(force_unmap: bool) {
    unsafe { GC_set_force_unmap_on_gcollect(if force_unmap { 1 } else { 0 }) }
}

/// External "C" call - omc_GC_set_max_heap_size via GC_set_max_heap_size_dbl macro.
/// Sets the maximum heap size.
///
/// # Parameters
/// * `sz` - Maximum heap size in bytes (f64 to avoid 32-bit signed limit)
pub fn set_max_heap_size(sz: f64) {
    unsafe { omc_GC_set_max_heap_size(sz as usize) }
}

/// Internal C helper - GC_get_prof_stats_modelica
/// Retrieves GC profiling statistics as 10 separate i64 values.
///
/// This mirrors the inner C function from GCExt.mo that uses
/// GC_get_prof_stats to populate the stats structure.
///
/// # Safety
/// This is unsafe because it calls into C code.
/// Requires GC_VERSION_MAJOR == 7 && GC_VERSION_MINOR >= 5, or GC_VERSION_MAJOR >= 8.
unsafe fn gc_get_prof_stats_modelica() -> [i64; 10] {
    // The C code from the original does:
    // struct GC_prof_stats_s info;
    // GC_get_prof_stats(&info, sizeof(struct GC_prof_stats_s));
    // Returns a boxed tuple of 10 i64 values.
    //
    // We replicate this by calling GC_get_prof_stats directly from libgc/omcgc.
    // Since we don't have the C struct available in Rust, we call through
    // the modelica wrapper function via FFI.
    unsafe extern "C" {
        fn GC_get_prof_stats_modelica_rust(
            heapsize_full: *mut i64,
            free_bytes_full: *mut i64,
            unmapped_bytes: *mut i64,
            bytes_allocd_since_gc: *mut i64,
            allocd_bytes_before_gc: *mut i64,
            non_gc_bytes: *mut i64,
            gc_no: *mut i64,
            markers_m1: *mut i64,
            bytes_reclaimed_since_gc: *mut i64,
            reclaimed_bytes_before_gc: *mut i64,
        );
    }
    let mut out = [0i64; 10];
    let [h, f, u, b, a, n, g, m, rsc, rbc] = &mut out;
    // Zero out on fallback (GC_prof_stats_s not available)
    unsafe {
        GC_get_prof_stats_modelica_rust(h, f, u, b, a, n, g, m, rsc, rbc);
    }
    out
}

/// Retrieves GC profiling statistics.
/// Mirrors the `getProfStats` function from GCExt.mo.
///
/// # Returns
/// A `ProfStats` struct containing current GC profiling information.
pub fn get_prof_stats() -> ProfStats {
    let out = unsafe { gc_get_prof_stats_modelica() };
    ProfStats {
        heapsize_full: out[0],
        free_bytes_full: out[1],
        unmapped_bytes: out[2],
        bytes_allocd_since_gc: out[3],
        allocd_bytes_before_gc: out[4],
        non_gc_bytes: out[5],
        gc_no: out[6],
        markers_m1: out[7],
        bytes_reclaimed_since_gc: out[8],
        reclaimed_bytes_before_gc: out[9],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prof_stats_to_string_with() {
        let stats = ProfStats {
            heapsize_full: 1000,
            free_bytes_full: 500,
            unmapped_bytes: 100,
            bytes_allocd_since_gc: 200,
            allocd_bytes_before_gc: 300,
            non_gc_bytes: 50,
            gc_no: 5,
            markers_m1: 4,
            bytes_reclaimed_since_gc: 150,
            reclaimed_bytes_before_gc: 250,
        };
        let result = stats.to_string_with("GC Profiling Stats: ", "\n  ");
        assert!(result.contains("heapsize_full: 1000"));
        assert!(result.contains("free_bytes_full: 500"));
        assert!(result.contains("total_allocd_bytes: 500"));
        assert!(result.contains("gc_no: 5"));
    }

    #[test]
    fn test_prof_stats_default_values() {
        let stats = ProfStats {
            heapsize_full: 0,
            free_bytes_full: 0,
            unmapped_bytes: 0,
            bytes_allocd_since_gc: 0,
            allocd_bytes_before_gc: 0,
            non_gc_bytes: 0,
            gc_no: 0,
            markers_m1: 0,
            bytes_reclaimed_since_gc: 0,
            reclaimed_bytes_before_gc: 0,
        };
        let result = stats.to_string_with("", "");
        assert!(result.contains("0"));
    }

    #[test]
    fn test_prof_stats_derives() {
        let stats1 = ProfStats {
            heapsize_full: 1,
            free_bytes_full: 2,
            unmapped_bytes: 3,
            bytes_allocd_since_gc: 4,
            allocd_bytes_before_gc: 5,
            non_gc_bytes: 6,
            gc_no: 7,
            markers_m1: 8,
            bytes_reclaimed_since_gc: 9,
            reclaimed_bytes_before_gc: 10,
        };
        let stats2 = stats1.clone();
        assert_eq!(stats1.heapsize_full, stats2.heapsize_full);
    }
}
