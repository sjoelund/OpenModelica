//! Translation of Util/Lapack.mo
//!
//! This module provides bindings to LAPACK linear algebra routines via the
//! `omcruntime`/`Lapack` C library. It exposes external function declarations
//! for operations including eigenvalue computation, linear system solving,
//! SVD, QR factorization, and more.
//!
//! All functions are external "C" calls linked to the `omcruntime` LAPACK runtime.
//! The C functions are named `LapackImpl__<funcname>` and accept C arrays with
//! leading dimensions, matching the LAPACK Fortran calling convention.

use std::ffi::{c_char, c_int};

// ============================================================================
// Extern "C" declarations (C API from omcruntime/Lapack)
// ============================================================================

unsafe extern "C" {
    /// External "C" call - LapackImpl__dgeev
    /// Computes eigenvalues and optionally eigenvectors of a real square matrix.
    fn LapackImpl__dgeev(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: c_int,
        a: *const f64,
        lda: c_int,
        ldvl: c_int,
        ldvr: c_int,
        work: *const f64,
        lwork: c_int,
        _a: *mut f64,
        wr: *mut f64,
        wi: *mut f64,
        vl: *mut f64,
        vr: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgegv
    /// Computes eigenvalues and optionally eigenvectors of a real generalized n-by-n
    /// matrix pair (A, B) from the QZ decomposition.
    fn LapackImpl__dgegv(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: c_int,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        ldvl: c_int,
        ldvr: c_int,
        work: *const f64,
        lwork: c_int,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vl: *mut f64,
        vr: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgels
    /// Solves overdetermined or underdetermined real linear systems.
    fn LapackImpl__dgels(
        trans: *const c_char,
        m: c_int,
        n: c_int,
        nrhs: c_int,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        work: *const f64,
        lwork: c_int,
        a_out: *mut f64,
        b_out: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgelsx
    /// Solves a real linear least squares problem using the QR factorization with
    /// pivoting.
    fn LapackImpl__dgelsx(
        m: c_int,
        n: c_int,
        nrhs: c_int,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        jpvt: *const c_int,
        rcond: f64,
        work: *const f64,
        a_out: *mut f64,
        b_out: *mut f64,
        jpvt_out: *mut c_int,
        rank: *mut c_int,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgelsy
    /// Solves a real linear least squares problem using the QR factorization with
    /// column pivoting.
    fn LapackImpl__dgelsy(
        m: c_int,
        n: c_int,
        nrhs: c_int,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        jpvt: *const c_int,
        rcond: f64,
        work: *const f64,
        lwork: c_int,
        a_out: *mut f64,
        b_out: *mut f64,
        jpvt_out: *mut c_int,
        rank: *mut c_int,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgesv
    /// Solves a system of linear equations A*X = B.
    fn LapackImpl__dgesv(
        n: c_int,
        nrhs: c_int,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        a_out: *mut f64,
        ipiv: *mut c_int,
        b_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgglse
    /// Solves a constrained least-squares problem (the Gauss-Markov problem).
    fn LapackImpl__dgglse(
        m: c_int,
        n: c_int,
        p: c_int,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        c: *const f64,
        d: *const f64,
        work: *const f64,
        lwork: c_int,
        a_out: *mut f64,
        b_out: *mut f64,
        c_out: *mut f64,
        d_out: *mut f64,
        x: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgtsv
    /// Solves a system of linear equations with a tridiagonal coefficient matrix.
    fn LapackImpl__dgtsv(
        n: c_int,
        nrhs: c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        b: *const f64,
        ldb: c_int,
        dl_out: *mut f64,
        d_out: *mut f64,
        du_out: *mut f64,
        b_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgbsv
    /// Solves a system of linear equations with a banded coefficient matrix.
    fn LapackImpl__dgbsv(
        n: c_int,
        kl: c_int,
        ku: c_int,
        nrhs: c_int,
        ab: *const f64,
        ldab: c_int,
        b: *const f64,
        ldb: c_int,
        ab_out: *mut f64,
        ipiv: *mut c_int,
        b_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgesvd
    /// Computes the singular value decomposition (SVD) of a real M-by-N matrix.
    fn LapackImpl__dgesvd(
        jobu: *const c_char,
        jobvt: *const c_char,
        m: c_int,
        n: c_int,
        a: *const f64,
        lda: c_int,
        ldu: c_int,
        ldvt: c_int,
        work: *const f64,
        lwork: c_int,
        a_out: *mut f64,
        s: *mut f64,
        u: *mut f64,
        vt: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgetrf
    /// Computes the LU factorization of a general M-by-N matrix.
    fn LapackImpl__dgetrf(
        m: c_int,
        n: c_int,
        a: *const f64,
        lda: c_int,
        a_out: *mut f64,
        ipiv: *mut c_int,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgetrs
    /// Solves a system of linear equations using the LU factorization computed by
    /// dgetrf.
    fn LapackImpl__dgetrs(
        trans: *const c_char,
        n: c_int,
        nrhs: c_int,
        a: *const f64,
        lda: c_int,
        ipiv: *const c_int,
        b: *const f64,
        ldb: c_int,
        b_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgetri
    /// Computes the inverse of a matrix using the LU factorization from dgetrf.
    fn LapackImpl__dgetri(
        n: c_int,
        a: *const f64,
        lda: c_int,
        ipiv: *const c_int,
        work: *const f64,
        lwork: c_int,
        a_out: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dgeqpf
    /// Computes the QR factorization of a real M-by-N matrix with pivoting.
    fn LapackImpl__dgeqpf(
        m: c_int,
        n: c_int,
        a: *const f64,
        lda: c_int,
        jpvt: *const c_int,
        work: *const f64,
        a_out: *mut f64,
        jpvt_out: *mut c_int,
        tau: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dorgqr
    /// Generates the M-by-N orthogonal matrix Q from a QR factorization computed
    /// by dgeqpf.
    fn LapackImpl__dorgqr(
        m: c_int,
        n: c_int,
        k: c_int,
        a: *const f64,
        lda: c_int,
        tau: *const f64,
        work: *const f64,
        lwork: c_int,
        a_out: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );

    /// External "C" call - LapackImpl__dhseqr
    /// Computes eigenvalues of a real upper quasi-triangular matrix (or Hessenberg
    /// matrix), and optionally the Schur factors.
    fn LapackImpl__dhseqr(
        job: *const c_char,
        compz: *const c_char,
        n: c_int,
        ilo: c_int,
        ihi: c_int,
        h: *const f64,
        ldh: c_int,
        z: *const f64,
        ldz: c_int,
        work: *const f64,
        lwork: c_int,
        h_out: *mut f64,
        wr: *mut f64,
        wi: *mut f64,
        z_out: *mut f64,
        work_out: *mut f64,
        info: *mut c_int,
    );
}
