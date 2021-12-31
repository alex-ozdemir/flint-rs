#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq::{fq_ctx_struct, fq_struct};
use crate::fq_nmod::fq_nmod_ctx_struct;
use crate::nmod_mat::nmod_mat_struct;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fq_mat_struct {
    pub entries: *mut fq_struct,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fq_struct,
}

pub type fq_mat_t = [fq_mat_struct; 1usize];

extern "C" {
    pub fn FQ_MAT_MUL_KS_CUTOFF(
        r: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_init(
        mat: *mut fq_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_init_set(
        mat: *mut fq_mat_struct,
        src: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_swap(mat1: *mut fq_mat_struct, mat2: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_set(mat1: *mut fq_mat_struct, mat2: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_clear(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_equal(
        mat1: *mut fq_mat_struct,
        mat2: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_is_zero(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_mat_is_empty(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_mat_is_square(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_mat_entry(
        mat: *mut fq_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fq_struct;
    pub fn fq_mat_entry_set(
        mat: *mut fq_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_nrows(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_mat_ncols(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_mat_swap_rows(
        mat: *mut fq_mat_struct,
        perm: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_invert_rows(
        mat: *mut fq_mat_struct,
        perm: *mut mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_swap_cols(
        mat: *mut fq_mat_struct,
        perm: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_invert_cols(
        mat: *mut fq_mat_struct,
        perm: *mut mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_zero(A: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_window_init(
        window: *mut fq_mat_struct,
        mat: *mut fq_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_window_clear(window: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_concat_horizontal(
        res: *mut fq_mat_struct,
        mat1: *mut fq_mat_struct,
        mat2: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_concat_vertical(
        res: *mut fq_mat_struct,
        mat1: *mut fq_mat_struct,
        mat2: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_fprint(
        file: *mut FILE,
        mat: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_fprint_pretty(
        file: *mut FILE,
        mat: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_print(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_mat_print_pretty(mat: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_mat_randtest(
        mat: *mut fq_mat_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_randrank(
        mat: *mut fq_mat_struct,
        state: *mut flint_rand_s,
        rank: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_randpermdiag(
        mat: *mut fq_mat_struct,
        state: *mut flint_rand_s,
        diag: *mut fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_randops(
        mat: *mut fq_mat_struct,
        count: mp_limb_signed_t,
        state: *mut flint_rand_s,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_randtril(
        mat: *mut fq_mat_struct,
        state: *mut flint_rand_s,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_randtriu(
        mat: *mut fq_mat_struct,
        state: *mut flint_rand_s,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_add(
        C: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_sub(
        C: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_neg(B: *mut fq_mat_struct, A: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_submul(
        D: *mut fq_mat_struct,
        C: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_mul(
        C: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_mul_classical(
        C: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_mul_KS(
        C: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_lu(
        P: *mut mp_limb_signed_t,
        A: *mut fq_mat_struct,
        rank_check: c_int,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_mat_lu_recursive(
        P: *mut mp_limb_signed_t,
        A: *mut fq_mat_struct,
        rank_check: c_int,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_mat_lu_classical(
        P: *mut mp_limb_signed_t,
        A: *mut fq_mat_struct,
        rank_check: c_int,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_mat_inv(
        B: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_rref(A: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_mat_reduce_row(
        A: *mut fq_mat_struct,
        P: *mut mp_limb_signed_t,
        L: *mut mp_limb_signed_t,
        m: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_mat_nullspace(
        X: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_mat_rank(A: *mut fq_mat_struct, ctx: *mut fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_mat_solve_tril(
        X: *mut fq_mat_struct,
        L: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_solve_tril_classical(
        X: *mut fq_mat_struct,
        L: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_solve_tril_recursive(
        X: *mut fq_mat_struct,
        L: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_solve_triu(
        X: *mut fq_mat_struct,
        U: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_solve_triu_classical(
        X: *mut fq_mat_struct,
        U: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_solve_triu_recursive(
        X: *mut fq_mat_struct,
        U: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        unit: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_solve(
        X: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        C: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_can_solve(
        X: *mut fq_mat_struct,
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_mat_similarity(
        A: *mut fq_mat_struct,
        r: mp_limb_signed_t,
        d: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_nmod_embed_gens(
        gen_sub: *mut nmod_poly_struct,
        gen_sup: *mut nmod_poly_struct,
        minpoly: *mut nmod_poly_struct,
        sub_ctx: *mut fq_nmod_ctx_struct,
        sup_ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_embed_gens_naive(
        gen_sub: *mut nmod_poly_struct,
        gen_sup: *mut nmod_poly_struct,
        minpoly: *mut nmod_poly_struct,
        sub_ctx: *mut fq_nmod_ctx_struct,
        sup_ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_embed_gens_allombert(
        gen_sub: *mut nmod_poly_struct,
        gen_sup: *mut nmod_poly_struct,
        minpoly: *mut nmod_poly_struct,
        sub_ctx: *mut fq_nmod_ctx_struct,
        sup_ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_embed_matrices(
        embed: *mut nmod_mat_struct,
        project: *mut nmod_mat_struct,
        gen_sub: *mut nmod_poly_struct,
        sub_ctx: *mut fq_nmod_ctx_struct,
        gen_sup: *mut nmod_poly_struct,
        sup_ctx: *mut fq_nmod_ctx_struct,
        gen_minpoly: *mut nmod_poly_struct,
    );
    pub fn fq_nmod_embed_trace_matrix(
        res: *mut nmod_mat_struct,
        basis: *mut nmod_mat_struct,
        sub_ctx: *mut fq_nmod_ctx_struct,
        sup_ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_embed_composition_matrix_sub(
        matrix: *mut nmod_mat_struct,
        gen: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_nmod_embed_mul_matrix(
        matrix: *mut nmod_mat_struct,
        gen: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_embed_mono_to_dual_matrix(
        res: *mut nmod_mat_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_embed_dual_to_mono_matrix(
        res: *mut nmod_mat_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
