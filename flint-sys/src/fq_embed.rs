#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_embed.html).

use crate::deps::*;
use crate::fmpz_mod_mat::fmpz_mod_mat_struct;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq::fq_ctx_struct;

extern "C" {
    pub fn fq_embed_gens(
        gen_sub: *mut fmpz_poly_struct,
        gen_sup: *mut fmpz_poly_struct,
        minpoly: *mut fmpz_mod_poly_struct,
        sub_ctx: *mut fq_ctx_struct,
        sup_ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_embed_gens_naive(
        gen_sub: *mut fmpz_poly_struct,
        gen_sup: *mut fmpz_poly_struct,
        minpoly: *mut fmpz_mod_poly_struct,
        sub_ctx: *mut fq_ctx_struct,
        sup_ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_embed_gens_allombert(
        gen_sub: *mut fmpz_poly_struct,
        gen_sup: *mut fmpz_poly_struct,
        minpoly: *mut fmpz_mod_poly_struct,
        sub_ctx: *mut fq_ctx_struct,
        sup_ctx: *mut fq_ctx_struct,
    );
    pub fn fq_embed_matrices(
        embed: *mut fmpz_mod_mat_struct,
        project: *mut fmpz_mod_mat_struct,
        gen_sub: *mut fmpz_poly_struct,
        sub_ctx: *mut fq_ctx_struct,
        gen_sup: *mut fmpz_poly_struct,
        sup_ctx: *mut fq_ctx_struct,
        gen_minpoly: *mut fmpz_mod_poly_struct,
    );
    pub fn fq_embed_trace_matrix(
        res: *mut fmpz_mod_mat_struct,
        basis: *mut fmpz_mod_mat_struct,
        sub_ctx: *mut fq_ctx_struct,
        sup_ctx: *mut fq_ctx_struct,
    );
    pub fn fq_embed_composition_matrix_sub(
        matrix: *mut fmpz_mod_mat_struct,
        gen: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_embed_composition_matrix(
        matrix: *mut fmpz_mod_mat_struct,
        gen: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_embed_mul_matrix(
        matrix: *mut fmpz_mod_mat_struct,
        gen: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_embed_mono_to_dual_matrix(res: *mut fmpz_mod_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_embed_dual_to_mono_matrix(res: *mut fmpz_mod_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_modulus_pow_series_inv(
        res: *mut fmpz_mod_poly_struct,
        ctx: *mut fq_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_modulus_derivative_inv(
        m_prime: *mut fmpz_poly_struct,
        m_prime_inv: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
}
