#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_poly_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_poly_mat_struct {
    pub entries: *mut fmpz_poly_struct,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fmpz_poly_struct,
}

pub type fmpz_poly_mat_t = [fmpz_poly_mat_struct; 1usize];

extern "C" {
    pub fn fmpz_poly_mat_entry(
        mat: *const fmpz_poly_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpz_poly_struct;
    pub fn fmpz_poly_mat_init(
        mat: *mut fmpz_poly_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mat_init_set(mat: *mut fmpz_poly_mat_struct, src: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_swap(mat1: *mut fmpz_poly_mat_struct, mat2: *mut fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_set(mat1: *mut fmpz_poly_mat_struct, mat2: *mut fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_clear(mat: *mut fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_nrows(mat: *const fmpz_poly_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_ncols(mat: *const fmpz_poly_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_equal(
        mat1: *const fmpz_poly_mat_struct,
        mat2: *const fmpz_poly_mat_struct,
    ) -> c_int;
    pub fn fmpz_poly_mat_is_zero(mat: *const fmpz_poly_mat_struct) -> c_int;
    pub fn fmpz_poly_mat_is_one(mat: *const fmpz_poly_mat_struct) -> c_int;
    pub fn fmpz_poly_mat_is_empty(mat: *const fmpz_poly_mat_struct) -> c_int;
    pub fn fmpz_poly_mat_is_square(mat: *const fmpz_poly_mat_struct) -> c_int;
    pub fn fmpz_poly_mat_zero(mat: *mut fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_one(mat: *mut fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_randtest(
        mat: *mut fmpz_poly_mat_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_poly_mat_randtest_unsigned(
        mat: *mut fmpz_poly_mat_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_poly_mat_randtest_sparse(
        A: *mut fmpz_poly_mat_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        density: f32,
    );
    pub fn fmpz_poly_mat_window_init(
        window: *mut fmpz_poly_mat_struct,
        mat: *const fmpz_poly_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mat_window_clear(window: *mut fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_concat_horizontal(
        res: *mut fmpz_poly_mat_struct,
        mat1: *const fmpz_poly_mat_struct,
        mat2: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_concat_vertical(
        res: *mut fmpz_poly_mat_struct,
        mat1: *const fmpz_poly_mat_struct,
        mat2: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_print(mat: *const fmpz_poly_mat_struct, x: *const c_char);
    pub fn fmpz_poly_mat_max_bits(A: *const fmpz_poly_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_max_length(A: *const fmpz_poly_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_transpose(B: *mut fmpz_poly_mat_struct, A: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_truncate(A: *mut fmpz_poly_mat_struct, len: mp_limb_signed_t);
    pub fn fmpz_poly_mat_scalar_mul_fmpz_poly(
        B: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        c: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_mat_scalar_mul_fmpz(
        B: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        c: *const fmpz,
    );
    pub fn fmpz_poly_mat_add(
        C: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_sub(
        C: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_neg(B: *mut fmpz_poly_mat_struct, A: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_mul(
        C: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_mul_classical(
        C: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_mul_KS(
        C: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_mullow(
        C: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
        len: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mat_sqr(B: *mut fmpz_poly_mat_struct, A: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_sqr_classical(
        B: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_sqr_KS(B: *mut fmpz_poly_mat_struct, A: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_sqrlow(
        B: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        len: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mat_pow(
        B: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_poly_mat_pow_trunc(
        B: *mut fmpz_poly_mat_struct,
        A: *const fmpz_poly_mat_struct,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mat_prod(
        res: *mut fmpz_poly_mat_struct,
        factors: *const fmpz_poly_mat_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mat_evaluate_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_poly_mat_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_mat_find_pivot_any(
        mat: *const fmpz_poly_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_find_pivot_partial(
        mat: *const fmpz_poly_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_fflu(
        B: *mut fmpz_poly_mat_struct,
        den: *mut fmpz_poly_struct,
        perm: *const mp_limb_signed_t,
        A: *const fmpz_poly_mat_struct,
        rank_check: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_rref(
        B: *mut fmpz_poly_mat_struct,
        den: *mut fmpz_poly_struct,
        A: *const fmpz_poly_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_trace(trace: *mut fmpz_poly_struct, mat: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_det(det: *mut fmpz_poly_struct, A: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_det_fflu(det: *mut fmpz_poly_struct, A: *const fmpz_poly_mat_struct);
    pub fn fmpz_poly_mat_det_interpolate(
        det: *mut fmpz_poly_struct,
        A: *const fmpz_poly_mat_struct,
    );
    pub fn fmpz_poly_mat_rank(A: *const fmpz_poly_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_inv(
        Ainv: *mut fmpz_poly_mat_struct,
        den: *mut fmpz_poly_struct,
        A: *const fmpz_poly_mat_struct,
    ) -> c_int;
    pub fn fmpz_poly_mat_nullspace(
        res: *mut fmpz_poly_mat_struct,
        mat: *const fmpz_poly_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_mat_solve(
        X: *mut fmpz_poly_mat_struct,
        den: *mut fmpz_poly_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    ) -> c_int;
    pub fn fmpz_poly_mat_solve_fflu(
        X: *mut fmpz_poly_mat_struct,
        den: *mut fmpz_poly_struct,
        A: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    ) -> c_int;
    pub fn fmpz_poly_mat_solve_fflu_precomp(
        X: *mut fmpz_poly_mat_struct,
        perm: *const mp_limb_signed_t,
        FFLU: *const fmpz_poly_mat_struct,
        B: *const fmpz_poly_mat_struct,
    );
}
