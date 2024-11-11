#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::arb::{arb_ptr, arb_srcptr, arb_struct};
use crate::arb_poly::arb_poly_struct;
use crate::mag::{mag_srcptr, mag_struct};
use crate::deps::*;
use crate::flint::*;
use crate::fmpq_mat::fmpq_mat_struct;
use crate::fmpz_mat::fmpz_mat_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arb_mat_struct {
    pub entries: arb_ptr,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut arb_ptr,
}

pub type arb_mat_t = [arb_mat_struct; 1usize];

extern "C" {
    pub fn arb_mat_init(mat: *mut arb_mat_struct, r: mp_limb_signed_t, c: mp_limb_signed_t);
    pub fn arb_mat_clear(mat: *mut arb_mat_struct);
    pub fn arb_mat_window_init(
        window: *mut arb_mat_struct,
        mat: *mut arb_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn arb_mat_set(dest: *mut arb_mat_struct, src: *mut arb_mat_struct);
    pub fn arb_mat_set_fmpz_mat(dest: *mut arb_mat_struct, src: *mut fmpz_mat_struct);
    pub fn arb_mat_set_round_fmpz_mat(
        dest: *mut arb_mat_struct,
        src: *mut fmpz_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_set_fmpq_mat(
        dest: *mut arb_mat_struct,
        src: *mut fmpq_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_randtest(
        mat: *mut arb_mat_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arb_mat_fprintd(file: *mut FILE, mat: *mut arb_mat_struct, digits: mp_limb_signed_t);
    pub fn arb_mat_eq(mat1: *mut arb_mat_struct, mat2: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_ne(mat1: *mut arb_mat_struct, mat2: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_equal(mat1: *mut arb_mat_struct, mat2: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_overlaps(mat1: *mut arb_mat_struct, mat2: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_contains(mat1: *mut arb_mat_struct, mat2: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_contains_fmpq_mat(
        mat1: *mut arb_mat_struct,
        mat2: *mut fmpq_mat_struct,
    ) -> c_int;
    pub fn arb_mat_contains_fmpz_mat(
        mat1: *mut arb_mat_struct,
        mat2: *mut fmpz_mat_struct,
    ) -> c_int;
    pub fn arb_mat_is_exact(A: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_is_zero(mat: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_is_finite(mat: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_is_triu(mat: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_is_tril(mat: *mut arb_mat_struct) -> c_int;
    pub fn arb_mat_zero(mat: *mut arb_mat_struct);
    pub fn arb_mat_one(mat: *mut arb_mat_struct);
    pub fn arb_mat_ones(mat: *mut arb_mat_struct);
    pub fn arb_mat_indeterminate(mat: *mut arb_mat_struct);
    pub fn arb_mat_hilbert(mat: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn arb_mat_pascal(mat: *mut arb_mat_struct, triangular: c_int, prec: mp_limb_signed_t);
    pub fn arb_mat_stirling(mat: *mut arb_mat_struct, kind: c_int, prec: mp_limb_signed_t);
    pub fn arb_mat_dct(mat: *mut arb_mat_struct, type_: c_int, prec: mp_limb_signed_t);
    pub fn arb_mat_transpose(mat1: *mut arb_mat_struct, mat2: *mut arb_mat_struct);
    pub fn arb_mat_bound_inf_norm(b: *mut mag_struct, A: *mut arb_mat_struct);
    pub fn arb_mat_frobenius_norm(
        res: *mut arb_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_bound_frobenius_norm(b: *mut mag_struct, A: *mut arb_mat_struct);
    pub fn arb_mat_neg(dest: *mut arb_mat_struct, src: *mut arb_mat_struct);
    pub fn arb_mat_add(
        res: *mut arb_mat_struct,
        mat1: *mut arb_mat_struct,
        mat2: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_sub(
        res: *mut arb_mat_struct,
        mat1: *mut arb_mat_struct,
        mat2: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_mul(
        res: *mut arb_mat_struct,
        mat1: *mut arb_mat_struct,
        mat2: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_mul_classical(
        C: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_mul_threaded(
        C: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_mat_addmul_rad_mag_fast(
        C: *mut arb_mat_struct,
        A: mag_srcptr,
        B: mag_srcptr,
        ar: mp_limb_signed_t,
        ac: mp_limb_signed_t,
        bc: mp_limb_signed_t,
    );
    pub fn arb_mat_mul_block(
        C: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_mul_entrywise(
        res: *mut arb_mat_struct,
        mat1: *mut arb_mat_struct,
        mat2: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_sqr_classical(
        B: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_sqr(B: *mut arb_mat_struct, A: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn arb_mat_pow_ui(
        B: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        exp: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_find_pivot_partial(
        mat: *mut arb_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn arb_mat_solve_tril_classical(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_solve_tril_recursive(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_solve_tril(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_solve_triu_classical(
        X: *mut arb_mat_struct,
        U: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_solve_triu_recursive(
        X: *mut arb_mat_struct,
        U: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_solve_triu(
        X: *mut arb_mat_struct,
        U: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_lu_classical(
        P: *mut mp_limb_signed_t,
        LU: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_lu_recursive(
        P: *mut mp_limb_signed_t,
        LU: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_lu(
        P: *mut mp_limb_signed_t,
        LU: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_solve_lu_precomp(
        X: *mut arb_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_solve(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_solve_lu(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_solve_precond(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_solve_preapprox(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        R: *mut arb_mat_struct,
        T: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_approx_mul(
        C: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_approx_solve_triu(
        X: *mut arb_mat_struct,
        U: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_approx_solve_tril(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        unit: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_approx_lu(
        P: *mut mp_limb_signed_t,
        LU: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_approx_solve_lu_precomp(
        X: *mut arb_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_approx_solve(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_approx_inv(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_inv(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_det_lu(det: *mut arb_struct, A: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn arb_mat_det_precond(
        det: *mut arb_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_det(det: *mut arb_struct, A: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn _arb_mat_cholesky_banachiewicz(A: *mut arb_mat_struct, prec: mp_limb_signed_t) -> c_int;
    pub fn arb_mat_cho(
        L: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_solve_cho_precomp(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_inv_cho_precomp(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_spd_solve(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_spd_inv(
        X: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn _arb_mat_ldl_inplace(A: *mut arb_mat_struct, prec: mp_limb_signed_t) -> c_int;
    pub fn _arb_mat_ldl_golub_and_van_loan(A: *mut arb_mat_struct, prec: mp_limb_signed_t)
        -> c_int;
    pub fn arb_mat_ldl(
        L: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_mat_solve_ldl_precomp(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        B: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_inv_ldl_precomp(
        X: *mut arb_mat_struct,
        L: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_exp_taylor_sum(
        S: *mut arb_mat_struct,
        A: *mut arb_mat_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_exp(B: *mut arb_mat_struct, A: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn _arb_mat_charpoly(poly: arb_ptr, mat: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn arb_mat_charpoly(
        poly: *mut arb_poly_struct,
        mat: *mut arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_mat_companion(mat: *mut arb_mat_struct, poly: arb_srcptr, prec: mp_limb_signed_t);
    pub fn arb_mat_companion(
        mat: *mut arb_mat_struct,
        poly: *mut arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_trace(trace: *mut arb_struct, mat: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn _arb_mat_diag_prod(
        res: *mut arb_struct,
        A: *mut arb_mat_struct,
        a: mp_limb_signed_t,
        b: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mat_diag_prod(res: *mut arb_struct, A: *mut arb_mat_struct, prec: mp_limb_signed_t);
    pub fn arb_mat_entrywise_is_zero(dest: *mut fmpz_mat_struct, src: *mut arb_mat_struct);
    pub fn arb_mat_entrywise_not_is_zero(dest: *mut fmpz_mat_struct, src: *mut arb_mat_struct);
    pub fn arb_mat_count_is_zero(mat: *mut arb_mat_struct) -> mp_limb_signed_t;
}
