//! Translation of BackendDAEEXT.mo
//!
//! This module provides bindings to the BackendDAEEXT C runtime library
//! used for BLT and index reduction algorithms in BackendDAE.
//! The implementation relies on several bitvectors (std::vector<bool>)
//! not available natively in MetaModelica.
//!
//! All functions are external C calls - no pure Rust implementation provided.
//! The `extern "C"` declarations mirror the C API from BackendDAEEXT.h.

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

unsafe extern "C" {
    fn BackendDAEEXT_initMarks(inInteger1: i32, inInteger2: i32);
    fn BackendDAEEXT_eMark(inInteger: i32);
    fn BackendDAEEXT_vMark(inInteger: i32);
    fn BackendDAEEXT_getVMark(inInteger: i32) -> i32;
    fn BackendDAEEXT_getMarkedEqns() -> *mut std::ffi::c_void;
    fn BackendDAEEXT_getDifferentiatedEqns() -> *mut std::ffi::c_void;
    fn BackendDAEEXT_clearDifferentiated();
    fn BackendDAEEXT_markDifferentiated(inInteger: i32);
    fn BackendDAEEXT_getMarkedVariables() -> *mut std::ffi::c_void;
    fn BackendDAEEXT_initLowLink(inInteger: i32);
    fn BackendDAEEXT_initNumber(inInteger: i32);
    fn BackendDAEEXT_setLowLink(inInteger1: i32, inInteger2: i32);
    fn BackendDAEEXT_getLowLink(inInteger: i32) -> i32;
    fn BackendDAEEXT_setNumber(inInteger1: i32, inInteger2: i32);
    fn BackendDAEEXT_getNumber(inInteger: i32) -> i32;
    fn BackendDAEEXT_setAdjacencyMatrix(
        nv: i32,
        ne: i32,
        nz: i32,
        m: *mut std::ffi::c_void,
    );
    fn BackendDAEEXT_matching(
        nv: i32,
        ne: i32,
        matching_id: i32,
        cheap_id: i32,
        relabel_period: f64,
        clear_match: i32,
    );
    fn BackendDAEEXT_getAssignment(ass1: *mut std::ffi::c_void, ass2: *mut std::ffi::c_void);
    fn BackendDAEEXT_setAssignment(
        lenass1: i32,
        lenass2: i32,
        ass1: *mut std::ffi::c_void,
        ass2: *mut std::ffi::c_void,
    ) -> i32;
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - BackendDAEEXT_initMarks
/// Initializes the mark sets (e_mark and v_mark).
/// Deprecated: external C dependency; consider in-memory initialization.
pub fn init_marks(in_integer1: i32, in_integer2: i32) {
    unsafe {
        BackendDAEEXT_initMarks(in_integer1, in_integer2);
    }
}

/// External "C" call - BackendDAEEXT_eMark
/// Marks an equation by index.
/// Deprecated: external C dependency.
pub fn e_mark(in_integer: i32) {
    unsafe {
        BackendDAEEXT_eMark(in_integer);
    }
}

/// External "C" call - BackendDAEEXT_vMark
/// Marks a variable by index.
/// Deprecated: external C dependency.
pub fn v_mark(in_integer: i32) {
    unsafe {
        BackendDAEEXT_vMark(in_integer);
    }
}

/// External "C" call - BackendDAEEXT_getVMark
/// Returns whether a variable is marked.
/// Deprecated: external C dependency.
pub fn get_v_mark(in_integer: i32) -> bool {
    unsafe { BackendDAEEXT_getVMark(in_integer) != 0 }
}

/// External "C" call - BackendDAEEXT_getMarkedEqns
/// Returns the list of marked equations.
/// Deprecated: external C dependency.
/// Returns a raw pointer to a C linked list (mmc_mk_cons / mmc_mk_nil).
/// Converting to a Rust list requires MMC runtime bindings.
pub fn get_marked_eqns() -> *mut std::ffi::c_void {
    unsafe { BackendDAEEXT_getMarkedEqns() }
}

/// External "C" call - BackendDAEEXT_getDifferentiatedEqns
/// Returns the list of differentiated equations.
/// Deprecated: external C dependency.
/// Returns a raw pointer to a C linked list.
pub fn get_differentiated_eqns() -> *mut std::ffi::c_void {
    unsafe { BackendDAEEXT_getDifferentiatedEqns() }
}

/// External "C" call - BackendDAEEXT_clearDifferentiated
/// Clears the differentiated equation marks.
/// Deprecated: external C dependency.
pub fn clear_differentiated() {
    unsafe {
        BackendDAEEXT_clearDifferentiated();
    }
}

/// External "C" call - BackendDAEEXT_markDifferentiated
/// Marks an equation as differentiated.
/// Deprecated: external C dependency.
pub fn mark_differentiated(in_integer: i32) {
    unsafe {
        BackendDAEEXT_markDifferentiated(in_integer);
    }
}

/// External "C" call - BackendDAEEXT_getMarkedVariables
/// Returns the list of marked variables.
/// Deprecated: external C dependency.
/// Returns a raw pointer to a C linked list.
pub fn get_marked_variables() -> *mut std::ffi::c_void {
    unsafe { BackendDAEEXT_getMarkedVariables() }
}

/// External "C" call - BackendDAEEXT_initLowLink
/// Initializes the lowlink array for the given number of variables.
/// Deprecated: external C dependency.
pub fn init_low_link(in_integer: i32) {
    unsafe {
        BackendDAEEXT_initLowLink(in_integer);
    }
}

/// External "C" call - BackendDAEEXT_initNumber
/// Initializes the number array for the given number of variables.
/// Deprecated: external C dependency.
pub fn init_number(in_integer: i32) {
    unsafe {
        BackendDAEEXT_initNumber(in_integer);
    }
}

/// External "C" call - BackendDAEEXT_setLowLink
/// Sets the lowlink value for a given index.
/// Deprecated: external C dependency.
pub fn set_low_link(in_integer1: i32, in_integer2: i32) {
    unsafe {
        BackendDAEEXT_setLowLink(in_integer1, in_integer2);
    }
}

/// External "C" call - BackendDAEEXT_getLowLink
/// Gets the lowlink value for a given index.
/// Deprecated: external C dependency.
pub fn get_low_link(in_integer: i32) -> i32 {
    unsafe { BackendDAEEXT_getLowLink(in_integer) }
}

/// External "C" call - BackendDAEEXT_setNumber
/// Sets the number value for a given index.
/// Deprecated: external C dependency.
pub fn set_number(in_integer1: i32, in_integer2: i32) {
    unsafe {
        BackendDAEEXT_setNumber(in_integer1, in_integer2);
    }
}

/// External "C" call - BackendDAEEXT_getNumber
/// Gets the number value for a given index.
/// Deprecated: external C dependency.
pub fn get_number(in_integer: i32) -> i32 {
    unsafe { BackendDAEEXT_getNumber(in_integer) }
}

/// External "C" call - BackendDAEEXT_setAdjacencyMatrix
/// Sets the adjacency matrix for the matching algorithms.
/// `m` is an array of lists of integers representing the adjacency structure.
/// Deprecated: external C dependency; `m` parameter is untyped (void pointer).
pub fn set_adjacency_matrix(nv: i32, ne: i32, nz: i32, m: *mut std::ffi::c_void) {
    unsafe {
        BackendDAEEXT_setAdjacencyMatrix(nv, ne, nz, m);
    }
}

/// External "C" call - BackendDAEEXT_matching
/// Calls matching algorithms.
///
/// matching_id: id of match algorithm (1-10)
///   1: DFS based
///   2: BFS based
///   3: MC21 (DFS + lookahead)
///   4: PF (Pothen and Fan' algorithm)
///   5: PF+ (PF + fairness)
///   6: HK (Hopcroft and Karp's algorithm)
///   7: HK-DW (Duff-Wiberg implementation of HK)
///   8: ABMP (Alt et al.'s algorithm)
///   9: ABMP-BFS (ABMP + BFS)
///  10: PR-FIFO-FAIR (DEFAULT)
///
/// cheap_id: id of cheap algo (0-4)
///   0: No Cheap Matching
///   1: Simple Greedy
///   2: Karp-Sipser
///   3: Random Karp-Sipser (DEFAULT)
///   4: Minimum Degree (two-sided)
///
/// relabel_period: used only when matching_id = 10.
///   -1: for a global relabeling after every m pushes
///   -2: for a global relabeling after every n pushes
///   Other non-positive values are not allowed.
/// Deprecated: external C dependency.
pub fn matching(
    nv: i32,
    ne: i32,
    matching_id: i32,
    cheap_id: i32,
    relabel_period: f64,
    clear_match: i32,
) {
    unsafe {
        BackendDAEEXT_matching(nv, ne, matching_id, cheap_id, relabel_period, clear_match);
    }
}

/// External "C" call - BackendDAEEXT_getAssignment
/// Gets the assignment results into two arrays.
/// Deprecated: external C dependency; arrays are passed as raw pointers.
pub fn get_assignment(ass1: *mut std::ffi::c_void, ass2: *mut std::ffi::c_void) {
    unsafe {
        BackendDAEEXT_getAssignment(ass1, ass2);
    }
}

/// External "C" call - BackendDAEEXT_setAssignment
/// Sets the assignment from two arrays. Returns true if successful.
/// Deprecated: external C dependency; arrays are passed as raw pointers.
pub fn set_assignment(
    lenass1: i32,
    lenass2: i32,
    ass1: *mut std::ffi::c_void,
    ass2: *mut std::ffi::c_void,
) -> bool {
    unsafe { BackendDAEEXT_setAssignment(lenass1, lenass2, ass1, ass2) != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_v_mark_signature() {
        // Just verifies the function compiles and returns bool
        // Does not test actual C behavior (requires runtime linkage)
        let _ = || -> bool { get_v_mark(0) };
    }

    #[test]
    fn test_get_number_signature() {
        // Just verifies the function compiles and returns i32
        let _ = || -> i32 { get_number(0) };
    }

    #[test]
    fn test_set_assignment_signature() {
        // Just verifies the function compiles and returns bool
        let _ = || -> bool { set_assignment(0, 0, std::ptr::null_mut(), std::ptr::null_mut()) };
    }
}
