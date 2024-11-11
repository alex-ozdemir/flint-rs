#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::acb::{acb_ptr, acb_srcptr, acb_struct};
use crate::acb_poly::acb_poly_struct;
use crate::arb::arb_struct;
use crate::arb_mat::arb_mat_struct;
use crate::mag::mag_struct;
use crate::deps::*;
use crate::flint::*;
use crate::fmpq_mat::fmpq_mat_struct;
use crate::fmpz_mat::fmpz_mat_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_mat_struct {
    pub entries: acb_ptr,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut acb_ptr,
}

pub type acb_mat_t = [acb_mat_struct; 1usize];

extern "C" {
    pub fn acb_mat_init(mat: *mut acb_mat_struct, r: mp_limb_signed_t, c: mp_limb_signed_t);
    pub fn acb_mat_clear(mat: *mut acb_mat_struct);
    pub fn acb_mat_window_init(
        window: *mut acb_mat_struct,
        mat: *mut acb_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn acb_mat_set(dest: *mut acb_mat_struct, src: *mut acb_mat_struct);
    pub fn acb_mat_set_fmpz_mat(dest: *mut acb_mat_struct, src: *mut fmpz_mat_struct);
    pub fn acb_mat_set_round_fmpz_mat(
        dest: *mut acb_mat_struct,
        src: *mut fmpz_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_set_fmpq_mat(
        dest: *mut acb_mat_struct,
        src: *mut fmpq_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_set_arb_mat(dest: *mut acb_mat_struct, src: *mut arb_mat_struct);
    pub fn acb_mat_set_round_arb_mat(
        dest: *mut acb_mat_struct,
        src: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_randtest(
        mat: *mut acb_mat_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_mat_randtest_eig(
        A: *mut acb_mat_struct,
        state: *mut flint_rand_s,
        E: acb_srcptr,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_fprintd(file: *mut FILE, mat: *mut acb_mat_struct, digits: mp_limb_signed_t);
    pub fn acb_mat_eq(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_ne(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_equal(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_overlaps(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_contains(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_contains_fmpq_mat(
        mat1: *mut acb_mat_struct,
        mat2: *mut fmpq_mat_struct,
    ) -> c_int;
    pub fn acb_mat_contains_fmpz_mat(
        mat1: *mut acb_mat_struct,
        mat2: *mut fmpz_mat_struct,
    ) -> c_int;
    pub fn acb_mat_is_real(mat: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_is_exact(mat: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_is_zero(mat: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_is_finite(mat: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_is_triu(mat: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_is_tril(mat: *mut acb_mat_struct) -> c_int;
    pub fn acb_mat_zero(mat: *mut acb_mat_struct);
    pub fn acb_mat_one(mat: *mut acb_mat_struct);
    pub fn acb_mat_ones(mat: *mut acb_mat_struct);
    pub fn acb_mat_indeterminate(mat: *mut acb_mat_struct);
    pub fn acb_mat_dft(res: *mut acb_mat_struct, kind: c_int, prec: mp_limb_signed_t);
    pub fn acb_mat_transpose(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct);
    pub fn acb_mat_conjugate(mat1: *mut acb_mat_struct, mat2: *mut acb_mat_struct);
    pub fn acb_mat_bound_inf_norm(b: *mut mag_struct, A: *mut acb_mat_struct);
    pub fn acb_mat_frobenius_norm(
        res: *mut arb_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_bound_frobenius_norm(b: *mut mag_struct, A: *mut acb_mat_struct);
    pub fn acb_mat_neg(dest: *mut acb_mat_struct, src: *mut acb_mat_struct);
    pub fn acb_mat_add(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_sub(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_mul_classical(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_mul_threaded(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_mul_reorder(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_mul(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_mul_entrywise(
        res: *mut acb_mat_struct,
        mat1: *mut acb_mat_struct,
        mat2: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_sqr_classical(
        res: *mut acb_mat_struct,
        mat: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_sqr(res: *mut acb_mat_struct, mat: *mut acb_mat_struct, prec: mp_limb_signed_t);
    pub fn acb_mat_pow_ui(
        B: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        exp: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_find_pivot_partial(
        mat: *mut acb_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_mat_solve_tril_classical(
        X: *mut acb_mat_struct,
        L: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_solve_tril_recursive(
        X: *mut acb_mat_struct,
        L: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_solve_tril(
        X: *mut acb_mat_struct,
        L: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_solve_triu_classical(
        X: *mut acb_mat_struct,
        U: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_solve_triu_recursive(
        X: *mut acb_mat_struct,
        U: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_solve_triu(
        X: *mut acb_mat_struct,
        U: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_lu_classical(
        P: *mut mp_limb_signed_t,
        LU: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_lu_recursive(
        P: *mut mp_limb_signed_t,
        LU: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_lu(
        P: *mut mp_limb_signed_t,
        LU: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_solve_lu_precomp(
        X: *mut acb_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_solve_lu(
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_solve(
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_solve_precond(
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_approx_mul(
        C: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_approx_solve_triu(
        X: *mut acb_mat_struct,
        U: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_approx_solve_tril(
        X: *mut acb_mat_struct,
        L: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_approx_lu(
        P: *mut mp_limb_signed_t,
        LU: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_approx_solve_lu_precomp(
        X: *mut acb_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_approx_solve(
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        B: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_approx_inv(
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_inv(
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_det_lu(det: *mut acb_struct, A: *mut acb_mat_struct, prec: mp_limb_signed_t);
    pub fn acb_mat_det_precond(
        det: *mut acb_struct,
        A: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_det(det: *mut acb_struct, A: *mut acb_mat_struct, prec: mp_limb_signed_t);
    pub fn acb_mat_approx_eig_qr(
        E: acb_ptr,
        L: *mut acb_mat_struct,
        R: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        tol: *mut mag_struct,
        maxiter: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_eig_global_enclosure(
        eps: *mut mag_struct,
        A: *mut acb_mat_struct,
        E: acb_srcptr,
        R: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_eig_enclosure_rump(
        lambda: *mut acb_struct,
        J: *mut acb_mat_struct,
        X: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        lambda_approx: *mut acb_struct,
        X_approx: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_eig_simple_rump(
        E: acb_ptr,
        L: *mut acb_mat_struct,
        R: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        E_approx: acb_srcptr,
        R_approx: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_eig_simple_vdhoeven_mourrain(
        E: acb_ptr,
        L: *mut acb_mat_struct,
        R: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        E_approx: acb_srcptr,
        R_approx: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_eig_simple(
        E: acb_ptr,
        L: *mut acb_mat_struct,
        R: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        E_approx: acb_srcptr,
        R_approx: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_eig_multiple_rump(
        E: acb_ptr,
        A: *mut acb_mat_struct,
        E_approx: acb_srcptr,
        R_approx: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_eig_multiple(
        E: acb_ptr,
        A: *mut acb_mat_struct,
        E_approx: acb_srcptr,
        R_approx: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_mat_exp_taylor_sum(
        S: *mut acb_mat_struct,
        A: *mut acb_mat_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_exp(B: *mut acb_mat_struct, A: *mut acb_mat_struct, prec: mp_limb_signed_t);
    pub fn _acb_mat_charpoly(poly: acb_ptr, mat: *mut acb_mat_struct, prec: mp_limb_signed_t);
    pub fn acb_mat_charpoly(
        poly: *mut acb_poly_struct,
        mat: *mut acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_mat_companion(mat: *mut acb_mat_struct, poly: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_mat_companion(
        mat: *mut acb_mat_struct,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_trace(trace: *mut acb_struct, mat: *mut acb_mat_struct, prec: mp_limb_signed_t);
    pub fn _acb_mat_diag_prod(
        res: *mut acb_struct,
        A: *mut acb_mat_struct,
        a: mp_limb_signed_t,
        b: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mat_diag_prod(res: *mut acb_struct, A: *mut acb_mat_struct, prec: mp_limb_signed_t);
}
