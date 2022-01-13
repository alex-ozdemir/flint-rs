#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_nmod_embed.html).

use crate::deps::*;
use crate::fq_nmod::fq_nmod_ctx_struct;
use crate::nmod_mat::nmod_mat_struct;
use crate::nmod_poly::nmod_poly_struct;

extern "C" {
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
    pub fn fq_nmod_embed_composition_matrix(
        matrix: *mut nmod_mat_struct,
        gen: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
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
    pub fn fq_nmod_modulus_pow_series_inv(
        res: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_nmod_modulus_derivative_inv(
        m_prime: *mut nmod_poly_struct,
        m_prime_inv: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
