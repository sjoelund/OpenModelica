//! Translation of HpcOmSchedulerExt.mo
//!
//! This module provides bindings to the HpcOmSchedulerExt adapter package
//! used for reading schedule information from external files and Metis-based
//! graph partitioning. The implementation relies on the `omcruntime` C library.
//!
//! All functions are external C calls - no pure Rust implementation provided.
//! The `extern "C"` declarations mirror the C API from omcruntime.

use std::ffi::CString;

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

unsafe extern "C" {
    fn HpcOmSchedulerExt_readScheduleFromGraphMl(fileName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn HpcOmSchedulerExt_scheduleMetis(
        xadj: *mut std::ffi::c_void,
        adjncy: *mut std::ffi::c_void,
        vwgt: *mut std::ffi::c_void,
        adjwgt: *mut std::ffi::c_void,
        nparts: i32,
    ) -> *mut std::ffi::c_void;
    fn HpcOmSchedulerExt_schedulehMetis(
        vwgts: *mut std::ffi::c_void,
        eptr: *mut std::ffi::c_void,
        eint: *mut std::ffi::c_void,
        hewgts: *mut std::ffi::c_void,
        nparts: i32,
    ) -> *mut std::ffi::c_void;
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - HpcOmSchedulerExt_readScheduleFromGraphMl
/// Reads schedule information from a GraphML file.
/// `fileName`: path to the GraphML file.
/// Returns a pointer to a linked list of integers (schedule).
/// Deprecated: external C dependency on omcruntime.
pub fn read_schedule_from_graph_ml(file_name: &str) -> *mut std::ffi::c_void {
    let c_name = CString::new(file_name).expect("fileName contained NUL bytes");
    unsafe { HpcOmSchedulerExt_readScheduleFromGraphMl(c_name.as_ptr()) }
}

/// External "C" call - HpcOmSchedulerExt_scheduleMetis
/// Runs Metis-based graph partitioning for scheduling.
/// `xadj`: adjacency list offsets.
/// `adjncy`: adjacency list indices.
/// `vwgt`: vertex weights.
/// `adjwgt`: edge weights.
/// `nparts`: number of partitions.
/// Returns a pointer to a linked list of integers (schedule).
/// Deprecated: external C dependency on omcruntime.
pub fn schedule_metis(
    xadj: *mut std::ffi::c_void,
    adjncy: *mut std::ffi::c_void,
    vwgt: *mut std::ffi::c_void,
    adjwgt: *mut std::ffi::c_void,
    nparts: i32,
) -> *mut std::ffi::c_void {
    unsafe {
        HpcOmSchedulerExt_scheduleMetis(xadj, adjncy, vwgt, adjwgt, nparts)
    }
}

/// External "C" call - HpcOmSchedulerExt_schedulehMetis
/// Runs hMetis-based hypergraph partitioning for scheduling.
/// `vwgts`: vertex weights.
/// `eptr`: hyperedge vertex list offsets.
/// `eint`: hyperedge vertex list.
/// `hewgts`: hyperedge weights.
/// `nparts`: number of partitions.
/// Returns a pointer to a linked list of integers (schedule).
/// Deprecated: external C dependency on omcruntime.
pub fn schedule_h_metis(
    vwgts: *mut std::ffi::c_void,
    eptr: *mut std::ffi::c_void,
    eint: *mut std::ffi::c_void,
    hewgts: *mut std::ffi::c_void,
    nparts: i32,
) -> *mut std::ffi::c_void {
    unsafe {
        HpcOmSchedulerExt_schedulehMetis(vwgts, eptr, eint, hewgts, nparts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_schedule_from_graph_ml_signature() {
        let _ = || -> *mut std::ffi::c_void { read_schedule_from_graph_ml("test.graphml") };
    }

    #[test]
    fn test_schedule_metis_signature() {
        let _ = || -> *mut std::ffi::c_void {
            schedule_metis(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                1,
            )
        };
    }

    #[test]
    fn test_schedule_h_metis_signature() {
        let _ = || -> *mut std::ffi::c_void {
            schedule_h_metis(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                1,
            )
        };
    }
}
