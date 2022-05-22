#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpz_mat::fmpz_mat_t;
use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mat_struct {
    pub mat: fmpz_mat_t,
    pub mod_: fmpz_t,
}

pub type fmpz_mod_mat_t = [fmpz_mod_mat_struct; 1usize];

extern "C" {
    pub fn fmpz_mod_mat_entry(
        mat: *const fmpz_mod_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *const fmpz;
    pub fn fmpz_mod_mat_set_entry(
        mat: *mut fmpz_mod_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        val: *const fmpz,
    );
    pub fn fmpz_mod_mat_init(
        mat: *mut fmpz_mod_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
        n: *const fmpz,
    );
    pub fn fmpz_mod_mat_init_set(mat: *mut fmpz_mod_mat_struct, src: *const fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_clear(mat: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_nrows(mat: *const fmpz_mod_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mod_mat_ncols(mat: *const fmpz_mod_mat_struct) -> mp_limb_signed_t;
    pub fn _fmpz_mod_mat_set_mod(mat: *mut fmpz_mod_mat_struct, n: *const fmpz);
    pub fn fmpz_mod_mat_one(mat: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_zero(mat: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_is_empty(mat: *const fmpz_mod_mat_struct) -> c_int;
    pub fn fmpz_mod_mat_is_square(mat: *const fmpz_mod_mat_struct) -> c_int;
    pub fn fmpz_mod_mat_swap(mat1: *mut fmpz_mod_mat_struct, mat2: *mut fmpz_mod_mat_struct);
    pub fn _fmpz_mod_mat_reduce(mat: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_randtest(mat: *mut fmpz_mod_mat_struct, state: *const flint_rand_s);
    pub fn fmpz_mod_mat_window_init(
        window: *mut fmpz_mod_mat_struct,
        mat: *const fmpz_mod_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn fmpz_mod_mat_window_clear(window: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_concat_horizontal(
        res: *mut fmpz_mod_mat_struct,
        mat1: *const fmpz_mod_mat_struct,
        mat2: *const fmpz_mod_mat_struct,
    );
    pub fn fmpz_mod_mat_concat_vertical(
        res: *mut fmpz_mod_mat_struct,
        mat1: *const fmpz_mod_mat_struct,
        mat2: *const fmpz_mod_mat_struct,
    );
    pub fn fmpz_mod_mat_print_pretty(mat: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_equal(
        mat1: *const fmpz_mod_mat_struct,
        mat2: *const fmpz_mod_mat_struct,
    ) -> c_int;
    pub fn fmpz_mod_mat_is_zero(mat: *const fmpz_mod_mat_struct) -> c_int;
    pub fn fmpz_mod_mat_set(B: *mut fmpz_mod_mat_struct, A: *const fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_transpose(B: *mut fmpz_mod_mat_struct, A: *const fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_add(
        C: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        B: *const fmpz_mod_mat_struct,
    );
    pub fn fmpz_mod_mat_sub(
        C: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        B: *const fmpz_mod_mat_struct,
    );
    pub fn fmpz_mod_mat_neg(B: *mut fmpz_mod_mat_struct, A: *const fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_scalar_mul_si(
        B: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_mod_mat_scalar_mul_ui(
        B: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        c: mp_limb_t,
    );
    pub fn fmpz_mod_mat_scalar_mul_fmpz(
        B: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        c: *const fmpz,
    );
    pub fn fmpz_mod_mat_mul(
        C: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        B: *const fmpz_mod_mat_struct,
    );
    pub fn _fmpz_mod_mat_mul_classical_threaded_pool_op(
        D: *mut fmpz_mod_mat_struct,
        C: *const fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        B: *const fmpz_mod_mat_struct,
        op: c_int,
        threads: *const thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn fmpz_mod_mat_mul_classical_threaded(
        C: *mut fmpz_mod_mat_struct,
        A: *const fmpz_mod_mat_struct,
        B: *const fmpz_mod_mat_struct,
    );
    pub fn fmpz_mod_mat_sqr(B: *mut fmpz_mod_mat_struct, A: *const fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_trace(trace: *mut fmpz, mat: *const fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_rref(
        perm: *mut mp_limb_signed_t,
        mat: *const fmpz_mod_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_mat_howell_form(mat: *mut fmpz_mod_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mod_mat_strong_echelon_form(mat: *mut fmpz_mod_mat_struct);
    pub fn fmpz_mod_mat_get_entry(
        x: *mut fmpz,
        mat: *const fmpz_mod_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    );
}
