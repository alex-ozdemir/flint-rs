#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/nmod_poly_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::nmod_mat::nmod_mat_struct;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_mat_struct {
    pub entries: *mut nmod_poly_struct,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut nmod_poly_struct,
    pub modulus: mp_limb_t,
}

pub type nmod_poly_mat_t = [nmod_poly_mat_struct; 1usize];

extern "C" {
    pub fn nmod_poly_mat_entry(
        mat: *mut nmod_poly_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut nmod_poly_struct;
    pub fn nmod_poly_mat_init(
        mat: *mut nmod_poly_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
        n: mp_limb_t,
    );
    pub fn nmod_poly_mat_init_set(mat: *mut nmod_poly_mat_struct, src: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_swap(mat1: *mut nmod_poly_mat_struct, mat2: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_set(mat1: *mut nmod_poly_mat_struct, mat2: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_clear(mat: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_nrows(mat: *mut nmod_poly_mat_struct) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_ncols(mat: *mut nmod_poly_mat_struct) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_modulus(mat: *mut nmod_poly_mat_struct) -> mp_limb_t;
    pub fn nmod_poly_mat_equal(
        mat1: *mut nmod_poly_mat_struct,
        mat2: *mut nmod_poly_mat_struct,
    ) -> c_int;
    pub fn nmod_poly_mat_is_zero(mat: *mut nmod_poly_mat_struct) -> c_int;
    pub fn nmod_poly_mat_is_one(mat: *mut nmod_poly_mat_struct) -> c_int;
    pub fn nmod_poly_mat_is_empty(mat: *mut nmod_poly_mat_struct) -> c_int;
    pub fn nmod_poly_mat_is_square(mat: *mut nmod_poly_mat_struct) -> c_int;
    pub fn nmod_poly_mat_zero(mat: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_one(mat: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_randtest(
        mat: *mut nmod_poly_mat_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_mat_randtest_sparse(
        A: *mut nmod_poly_mat_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        density: f32,
    );
    pub fn nmod_poly_mat_window_init(
        window: *mut nmod_poly_mat_struct,
        mat: *mut nmod_poly_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn nmod_poly_mat_window_clear(window: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_concat_horizontal(
        res: *mut nmod_poly_mat_struct,
        mat1: *mut nmod_poly_mat_struct,
        mat2: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_concat_vertical(
        res: *mut nmod_poly_mat_struct,
        mat1: *mut nmod_poly_mat_struct,
        mat2: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_print(mat: *mut nmod_poly_mat_struct, x: *const c_char);
    pub fn nmod_poly_mat_max_length(A: *mut nmod_poly_mat_struct) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_scalar_mul_nmod_poly(
        B: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        c: *mut nmod_poly_struct,
    );
    pub fn nmod_poly_mat_scalar_mul_nmod(
        B: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        c: mp_limb_t,
    );
    pub fn nmod_poly_mat_add(
        C: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_sub(
        C: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_neg(B: *mut nmod_poly_mat_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_mul(
        C: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_mul_interpolate(
        C: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_mul_classical(
        C: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_mul_KS(
        C: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_sqr(B: *mut nmod_poly_mat_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_sqr_classical(B: *mut nmod_poly_mat_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_sqr_KS(B: *mut nmod_poly_mat_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_sqr_interpolate(
        B: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
    );
    pub fn nmod_poly_mat_pow(
        B: *mut nmod_poly_mat_struct,
        A: *mut nmod_poly_mat_struct,
        exp: mp_limb_t,
    );
    pub fn nmod_poly_mat_evaluate_nmod(
        B: *mut nmod_mat_struct,
        A: *mut nmod_poly_mat_struct,
        x: mp_limb_t,
    );
    pub fn nmod_poly_mat_find_pivot_any(
        mat: *mut nmod_poly_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_find_pivot_partial(
        mat: *mut nmod_poly_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_fflu(
        B: *mut nmod_poly_mat_struct,
        den: *mut nmod_poly_struct,
        perm: *mut mp_limb_signed_t,
        A: *mut nmod_poly_mat_struct,
        rank_check: c_int,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_rref(
        B: *mut nmod_poly_mat_struct,
        den: *mut nmod_poly_struct,
        A: *mut nmod_poly_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_trace(trace: *mut nmod_poly_struct, mat: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_det(det: *mut nmod_poly_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_det_fflu(det: *mut nmod_poly_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_det_interpolate(det: *mut nmod_poly_struct, A: *mut nmod_poly_mat_struct);
    pub fn nmod_poly_mat_rank(A: *mut nmod_poly_mat_struct) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_inv(
        Ainv: *mut nmod_poly_mat_struct,
        den: *mut nmod_poly_struct,
        A: *mut nmod_poly_mat_struct,
    ) -> c_int;
    pub fn nmod_poly_mat_nullspace(
        res: *mut nmod_poly_mat_struct,
        mat: *mut nmod_poly_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_mat_solve(
        X: *mut nmod_poly_mat_struct,
        den: *mut nmod_poly_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    ) -> c_int;
    pub fn nmod_poly_mat_solve_fflu(
        X: *mut nmod_poly_mat_struct,
        den: *mut nmod_poly_struct,
        A: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    ) -> c_int;
    pub fn nmod_poly_mat_solve_fflu_precomp(
        X: *mut nmod_poly_mat_struct,
        perm: *const mp_limb_signed_t,
        FFLU: *mut nmod_poly_mat_struct,
        B: *mut nmod_poly_mat_struct,
    );
}
