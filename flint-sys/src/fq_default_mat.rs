#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_default_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fmpz_mod_mat::fmpz_mod_mat_struct;
use crate::fq_default::{fq_default_ctx_struct, fq_default_struct};
use crate::fq_mat::fq_mat_t;
use crate::fq_nmod_mat::fq_nmod_mat_t;
use crate::fq_zech_mat::fq_zech_mat_t;
use crate::nmod_mat::nmod_mat_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Copy, Clone)]
pub union fq_default_mat_struct {
    pub fq: fq_mat_t,
    pub fq_nmod: fq_nmod_mat_t,
    pub fq_zech: fq_zech_mat_t,
}

pub type fq_default_mat_t = [fq_default_mat_struct; 1usize];

extern "C" {
    pub fn fq_default_mat_init(
        mat: *mut fq_default_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_init_set(
        mat: *mut fq_default_mat_struct,
        src: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_swap(
        mat1: *mut fq_default_mat_struct,
        mat2: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_set(
        mat1: *mut fq_default_mat_struct,
        mat2: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_clear(mat: *mut fq_default_mat_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_mat_equal(
        mat1: *mut fq_default_mat_struct,
        mat2: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_is_zero(
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_is_one(
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_is_empty(
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_is_square(
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_entry(
        val: *mut fq_default_struct,
        mat: *const fq_default_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_entry_set(
        mat: *mut fq_default_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_entry_set_fmpz(
        mat: *mut fq_default_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_nrows(
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_mat_ncols(
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_mat_swap_rows(
        mat: *mut fq_default_mat_struct,
        perm: *const mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_invert_rows(
        mat: *mut fq_default_mat_struct,
        perm: *const mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_swap_cols(
        mat: *mut fq_default_mat_struct,
        perm: *const mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_invert_cols(
        mat: *mut fq_default_mat_struct,
        perm: *const mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_zero(A: *mut fq_default_mat_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_mat_one(A: *mut fq_default_mat_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_mat_set_nmod_mat(
        mat1: *mut fq_default_mat_struct,
        mat2: *const nmod_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_set_fmpz_mod_mat(
        mat1: *mut fq_default_mat_struct,
        mat2: *const fmpz_mod_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_set_fmpz_mat(
        mat1: *mut fq_default_mat_struct,
        mat2: *const fmpz_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_window_init(
        window: *mut fq_default_mat_struct,
        mat: *const fq_default_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_window_clear(
        window: *mut fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_concat_horizontal(
        res: *mut fq_default_mat_struct,
        mat1: *const fq_default_mat_struct,
        mat2: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_concat_vertical(
        res: *mut fq_default_mat_struct,
        mat1: *const fq_default_mat_struct,
        mat2: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_fprint(
        file: *mut FILE,
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_fprint_pretty(
        file: *mut FILE,
        mat: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_print(
        mat: *mut fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_print_pretty(
        mat: *mut fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_randtest(
        mat: *mut fq_default_mat_struct,
        state: *const flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_randrank(
        mat: *mut fq_default_mat_struct,
        state: *const flint_rand_s,
        rank: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_randops(
        mat: *mut fq_default_mat_struct,
        count: mp_limb_signed_t,
        state: *const flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_randtril(
        mat: *mut fq_default_mat_struct,
        state: *const flint_rand_s,
        unit: c_int,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_randtriu(
        mat: *mut fq_default_mat_struct,
        state: *const flint_rand_s,
        unit: c_int,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_add(
        C: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_sub(
        C: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_neg(
        B: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_submul(
        D: *mut fq_default_mat_struct,
        C: *const fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_mul(
        C: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_lu(
        P: *mut mp_limb_signed_t,
        A: *const fq_default_mat_struct,
        rank_check: c_int,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_mat_inv(
        B: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_rref(
        A: *mut fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_mat_nullspace(
        X: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_mat_rank(
        A: *mut fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_mat_solve_tril(
        X: *mut fq_default_mat_struct,
        L: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        unit: c_int,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_solve_triu(
        X: *mut fq_default_mat_struct,
        U: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        unit: c_int,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_solve(
        X: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        C: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_can_solve(
        X: *mut fq_default_mat_struct,
        A: *const fq_default_mat_struct,
        B: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_mat_similarity(
        A: *mut fq_default_mat_struct,
        r: mp_limb_signed_t,
        d: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
}
