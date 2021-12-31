#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_nmod_mpoly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fq_nmod::{fq_nmod_ctx_struct, fq_nmod_ctx_t, fq_nmod_struct, fq_nmod_t};
use crate::fq_nmod_poly::{fq_nmod_poly_struct, fq_nmod_poly_t};
use crate::mpoly::*;
use crate::n_poly::*;
use crate::nmod_mat::nmod_mat_t;
use crate::nmod_mpoly::*;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, c_uint, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub fqctx: fq_nmod_ctx_t,
}

pub type fq_nmod_mpoly_ctx_t = [fq_nmod_mpoly_ctx_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_struct {
    pub coeffs: *mut mp_limb_t,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub coeffs_alloc: mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
}

pub type fq_nmod_mpoly_t = [fq_nmod_mpoly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_univar_struct {
    pub coeffs: *mut fq_nmod_mpoly_struct,
    pub exps: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fq_nmod_mpoly_univar_t = [fq_nmod_mpoly_univar_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpolyu_struct {
    pub coeffs: *mut fq_nmod_mpoly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}

pub type fq_nmod_mpolyu_t = [fq_nmod_mpolyu_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpolyn_struct {
    pub coeffs: *mut n_fq_poly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_signed_t,
}

pub type fq_nmod_mpolyn_t = [fq_nmod_mpolyn_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpolyun_struct {
    pub coeffs: *mut fq_nmod_mpolyn_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}

pub type fq_nmod_mpolyun_t = [fq_nmod_mpolyun_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bad_fq_nmod_embed {
    pub smctx: *const fq_nmod_ctx_struct,
    pub phi_sm: fq_nmod_poly_t,
    pub h: fq_nmod_poly_t,
    pub h_as_n_fq_poly: n_fq_poly_t,
    pub lgctx: *const fq_nmod_ctx_struct,
    pub theta_lg: fq_nmod_t,
    pub x_lg: fq_nmod_t,
    pub lg_to_sm_mat: nmod_mat_t,
    pub sm_to_lg_mat: nmod_mat_t,
}

pub type bad_fq_nmod_embed_struct = bad_fq_nmod_embed;

pub type bad_fq_nmod_embed_t = [bad_fq_nmod_embed_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bad_fq_nmod_mpoly_embed_chooser {
    pub embed: *mut bad_fq_nmod_embed_struct,
    pub m: mp_limb_signed_t,
    pub n: mp_limb_signed_t,
    pub k: mp_limb_signed_t,
    pub p: mp_limb_t,
}

pub type bad_fq_nmod_mpoly_embed_chooser_struct = bad_fq_nmod_mpoly_embed_chooser;
pub type bad_fq_nmod_mpoly_embed_chooser_t = [bad_fq_nmod_mpoly_embed_chooser_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_geobucket {
    pub polys: [fq_nmod_mpoly_struct; 32usize],
    pub temps: [fq_nmod_mpoly_struct; 32usize],
    pub length: mp_limb_signed_t,
}

pub type fq_nmod_mpoly_geobucket_struct = fq_nmod_mpoly_geobucket;
pub type fq_nmod_mpoly_geobucket_t = [fq_nmod_mpoly_geobucket_struct; 1usize];

extern "C" {
    pub fn bad_fq_nmod_embed_clear(emb: *mut bad_fq_nmod_embed_struct);
    pub fn bad_fq_nmod_embed_array_init(
        emb: *mut bad_fq_nmod_embed_struct,
        bigctx: *mut fq_nmod_ctx_struct,
        smallctx: *mut fq_nmod_ctx_struct,
    );
    pub fn bad_fq_nmod_embed_sm_to_lg(
        out: *mut nmod_poly_struct,
        in_: *mut fq_nmod_poly_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_fq_nmod_embed_lg_to_sm(
        out: *mut fq_nmod_poly_struct,
        in_: *mut nmod_poly_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_n_fq_embed_sm_to_lg(
        out_: *mut mp_limb_t,
        in_: *mut n_poly_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_fq_nmod_embed_n_fq_sm_to_fq_nmod_lg(
        out: *mut nmod_poly_struct,
        in_: *mut n_poly_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_n_fq_embed_lg_to_sm(
        out_: *mut n_poly_struct,
        in_: *const mp_limb_t,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_fq_nmod_embed_fq_nmod_lg_to_n_fq_sm(
        out_: *mut n_poly_struct,
        in_: *mut nmod_poly_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_n_fq_embed_sm_elem_to_lg(
        out: *mut mp_limb_t,
        in_: *const mp_limb_t,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_fq_nmod_embed_sm_elem_to_lg(
        out: *mut nmod_poly_struct,
        in_: *mut nmod_poly_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn bad_fq_nmod_mpoly_embed_chooser_init(
        embc: *mut bad_fq_nmod_mpoly_embed_chooser_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    ) -> *mut bad_fq_nmod_embed_struct;
    pub fn bad_fq_nmod_mpoly_embed_chooser_clear(
        embc: *mut bad_fq_nmod_mpoly_embed_chooser_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    );
    pub fn bad_fq_nmod_mpoly_embed_chooser_next(
        embc: *mut bad_fq_nmod_mpoly_embed_chooser_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    ) -> *mut bad_fq_nmod_embed_struct;
    pub fn fq_nmod_mpoly_ctx_init_deg(
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
        p: mp_limb_t,
        deg: mp_limb_signed_t,
    );
    pub fn fq_nmod_mpoly_ctx_init(
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
        fqctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mpoly_ctx_init_rand(
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
        p_bits: mp_limb_t,
        deg_bound: mp_limb_signed_t,
    );
    pub fn fq_nmod_mpoly_ctx_clear(ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpoly_ctx_nvars(ctx: *mut fq_nmod_mpoly_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_ctx_ord(ctx: *mut fq_nmod_mpoly_ctx_struct) -> ordering_t;
    pub fn fq_nmod_mpoly_init(A: *mut fq_nmod_mpoly_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpoly_clear(A: *mut fq_nmod_mpoly_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpoly_init2(
        A: *mut fq_nmod_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_init3(
        A: *mut fq_nmod_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_realloc(
        A: *mut fq_nmod_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_fit_length(
        A: *mut fq_nmod_mpoly_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_fit_length_fit_bits(
        A: *mut fq_nmod_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_fit_length_reset_bits(
        A: *mut fq_nmod_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_fit_length(
        coeffs: *mut *mut mp_limb_t,
        coeffs_alloc: *mut mp_limb_signed_t,
        d: mp_limb_signed_t,
        exps: *mut *mut mp_limb_t,
        exps_alloc: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        length: mp_limb_signed_t,
    );
    pub fn _fq_nmod_mpoly_set_length(
        A: *mut fq_nmod_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_truncate(
        A: *mut fq_nmod_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_str_pretty(
        A: *mut fq_nmod_mpoly_struct,
        str_: *const c_char,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_get_str_pretty(
        A: *mut fq_nmod_mpoly_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_nmod_mpoly_fprint_pretty(
        file: *mut FILE,
        A: *mut fq_nmod_mpoly_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_print_pretty(
        A: *mut fq_nmod_mpoly_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gen(
        A: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_is_gen(
        A: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_set(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_equal(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_swap(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_nonzero_n_fq(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut mp_limb_t;
    pub fn fq_nmod_mpoly_is_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_get_fq_nmod(
        c: *mut nmod_poly_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_n_fq(
        A: *mut fq_nmod_mpoly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_ui(
        A: *mut fq_nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_fmpz(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_fq_nmod_gen(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_equal_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_zero(A: *mut fq_nmod_mpoly_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpoly_one(A: *mut fq_nmod_mpoly_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpoly_is_zero(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_is_one(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_degrees_fit_si(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_degrees_fmpz(
        degs: *mut *mut fmpz,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_degree_fmpz(
        deg: *mut fmpz,
        A: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_degree_si(
        A: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_total_degree_fits_si(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_total_degree_fmpz(
        td: *mut fmpz,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_total_degree_si(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_used_vars(
        used: *mut c_int,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_coeff_fq_nmod_monomial(
        c: *mut nmod_poly_struct,
        A: *mut fq_nmod_mpoly_struct,
        M: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_coeff_fq_nmod_monomial(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        M: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_coeff_fq_nmod_fmpz(
        c: *mut nmod_poly_struct,
        A: *mut fq_nmod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_coeff_fq_nmod_ui(
        c: *mut nmod_poly_struct,
        A: *mut fq_nmod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_set_coeff_fq_nmod_fmpz(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        exp: *const fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_coeff_fq_nmod_fmpz(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_coeff_fq_nmod_ui(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_coeff_vars_ui(
        C: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        vars: *const mp_limb_signed_t,
        exps: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_leadcoeff(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut mp_limb_t;
    pub fn fq_nmod_mpoly_is_monic(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_is_fq_nmod_poly(
        A: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_get_fq_nmod_poly(
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_set_fq_nmod_poly(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        Bcoeffs: *const fq_nmod_struct,
        Blen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_fq_nmod_poly(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_cmp(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_is_canonical(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_length(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_resize(
        A: *mut fq_nmod_mpoly_struct,
        new_length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_term_coeff_fq_nmod(
        c: *mut nmod_poly_struct,
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_term_coeff_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_term_exp_fits_ui(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_term_exp_fits_si(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_get_term_exp_fmpz(
        exp: *mut *mut fmpz,
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_term_exp_ui(
        exp: *mut mp_limb_t,
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_term_exp_si(
        exp: *mut mp_limb_signed_t,
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_term_var_exp_ui(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
    pub fn fq_nmod_mpoly_get_term_var_exp_si(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_set_term_exp_fmpz(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_term_exp_ui(
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_term(
        M: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_term_monomial(
        M: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_push_term_fq_nmod_fmpz(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_push_term_fq_nmod_ui(
        A: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_sort_terms(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_combine_like_terms(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_reverse(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_assert_canonical(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_radix_sort1(
        A: *mut fq_nmod_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        cmpmask: mp_limb_t,
        totalmask: mp_limb_t,
        d: mp_limb_signed_t,
    );
    pub fn _fq_nmod_mpoly_radix_sort(
        A: *mut fq_nmod_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *mut mp_limb_t,
        d: mp_limb_signed_t,
    );
    pub fn _fq_nmod_mpoly_push_exp_ffmpz(
        A: *mut fq_nmod_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_push_exp_pfmpz(
        A: *mut fq_nmod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_push_exp_ui(
        A: *mut fq_nmod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_randtest_bound(
        A: *mut fq_nmod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bound: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_randtest_bounds(
        A: *mut fq_nmod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bounds: *mut mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_randtest_bits(
        A: *mut fq_nmod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_add(
        coeff1: *mut mp_limb_t,
        exp1: *mut mp_limb_t,
        coeff2: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *mut mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fqctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_add_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_add_n_fq(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_sub_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_add(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_sub(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_neg(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_scalar_mul_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_scalar_mul_n_fq(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_make_monic(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_scalar_addmul_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *mut fq_nmod_mpoly_struct,
        e: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_derivative(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_evaluate_one_fq_nmod(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        val: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_eval_all_fq_nmod(
        ev: *mut nmod_poly_struct,
        Acoeffs: *const mp_limb_t,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        alphas: *const *mut fq_nmod_struct,
        mctx: *mut mpoly_ctx_struct,
        fqctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mpoly_evaluate_all_fq_nmod(
        ev: *mut nmod_poly_struct,
        A: *mut fq_nmod_mpoly_struct,
        vals: *const *mut fq_nmod_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_compose_fq_nmod_poly(
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *const *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_compose_mat(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        M: *mut fmpz_mat_struct,
        ctxB: *mut fq_nmod_mpoly_ctx_struct,
        ctxAC: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_compose_fq_nmod_mpoly_geobucket(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *const *mut fq_nmod_mpoly_struct,
        ctxB: *mut fq_nmod_mpoly_ctx_struct,
        ctxAC: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_compose_fq_nmod_mpoly_horner(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *const *mut fq_nmod_mpoly_struct,
        ctxB: *mut fq_nmod_mpoly_ctx_struct,
        ctxAC: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_compose_fq_nmod_mpoly(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *const *mut fq_nmod_mpoly_struct,
        ctxB: *mut fq_nmod_mpoly_ctx_struct,
        ctxAC: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_compose_fq_nmod_mpoly_gen(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        c: *const mp_limb_signed_t,
        ctxB: *mut fq_nmod_mpoly_ctx_struct,
        ctxAC: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_mul(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        C: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_mul_johnson(
        poly1: *mut fq_nmod_mpoly_struct,
        poly2: *mut fq_nmod_mpoly_struct,
        poly3: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_mul_johnson(
        A: *mut fq_nmod_mpoly_struct,
        Bcoeffs: *const mp_limb_t,
        Bexps: *const mp_limb_t,
        Blen: mp_limb_signed_t,
        Ccoeffs: *const mp_limb_t,
        Cexps: *const mp_limb_t,
        Clen: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mpoly_pow_fmpz(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        k: *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_pow_ui(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_divides(
        Q: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_div(
        Q: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_divrem(
        Q: *mut fq_nmod_mpoly_struct,
        R: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_divrem_ideal(
        Q: *mut *mut fq_nmod_mpoly_struct,
        R: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *const *mut fq_nmod_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_divexact(
        Q: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_divides_monagan_pearce(
        poly1: *mut fq_nmod_mpoly_struct,
        poly2: *mut fq_nmod_mpoly_struct,
        poly3: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_div_monagan_pearce(
        q: *mut fq_nmod_mpoly_struct,
        poly2: *mut fq_nmod_mpoly_struct,
        poly3: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_divrem_monagan_pearce(
        q: *mut fq_nmod_mpoly_struct,
        r: *mut fq_nmod_mpoly_struct,
        poly2: *mut fq_nmod_mpoly_struct,
        poly3: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_divrem_ideal_monagan_pearce(
        q: *mut *mut fq_nmod_mpoly_struct,
        r: *mut fq_nmod_mpoly_struct,
        poly2: *mut fq_nmod_mpoly_struct,
        poly3: *const *mut fq_nmod_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_divides_monagan_pearce(
        A: *mut fq_nmod_mpoly_struct,
        coeff2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fqctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_sqrt_heap(
        Q: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_sqrt(
        Q: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_is_square(
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_quadratic_root(
        Q: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_term_content(
        M: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_content_vars(
        g: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        vars: *mut mp_limb_signed_t,
        vars_length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gcd(
        G: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_gcd_algo(
        G: *mut fq_nmod_mpoly_struct,
        Abar: *mut fq_nmod_mpoly_struct,
        Bbar: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        algo: c_uint,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gcd_cofactors(
        G: *mut fq_nmod_mpoly_struct,
        Abar: *mut fq_nmod_mpoly_struct,
        Bbar: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gcd_brown(
        G: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gcd_hensel(
        G: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gcd_zippel(
        G: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_gcd_zippel2(
        G: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_deflation(
        shift: *mut fmpz,
        stride: *mut fmpz,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_deflate(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_inflate(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn mpoly_void_ring_init_fq_nmod_mpoly_ctx(
        R: *mut _bindgen_ty_22,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyl_lead_coeff(
        c: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_to_mpolyl_perm_deflate(
        A: *mut fq_nmod_mpoly_struct,
        lctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fq_nmod_mpoly_from_mpolyl_perm_inflate(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpoly_struct,
        lctx: *mut fq_nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fq_nmod_mpolyl_gcd_zippel_smprime(
        rG: *mut fq_nmod_mpoly_struct,
        rGdegs: *const mp_limb_signed_t,
        rAbar: *mut fq_nmod_mpoly_struct,
        rBbar: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        Adegs: *const mp_limb_signed_t,
        B: *mut fq_nmod_mpoly_struct,
        Bdegs: *const mp_limb_signed_t,
        gamma: *mut fq_nmod_mpoly_struct,
        gammadegs: *const mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyl_gcd_zippel_lgprime(
        rG: *mut fq_nmod_mpoly_struct,
        rGdegs: *const mp_limb_signed_t,
        rAbar: *mut fq_nmod_mpoly_struct,
        rBbar: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        Adegs: *const mp_limb_signed_t,
        B: *mut fq_nmod_mpoly_struct,
        Bdegs: *const mp_limb_signed_t,
        gamma: *mut fq_nmod_mpoly_struct,
        gammadegs: *const mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyl_gcd_hensel_smprime(
        G: *mut fq_nmod_mpoly_struct,
        Gdeg: mp_limb_signed_t,
        Abar: *mut fq_nmod_mpoly_struct,
        Bbar: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyl_content(
        g: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_pow_rmul(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_to_fq_nmod_poly_deflate(
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        Bshift: *const mp_limb_t,
        Bstride: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_from_fq_nmod_poly_inflate(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fq_nmod_poly_struct,
        var: mp_limb_signed_t,
        Ashift: *const mp_limb_t,
        Astride: *const mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_repack_bits(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_repack_bits_inplace(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_ctx_change_modulus(
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        deg: mp_limb_signed_t,
    );
    pub fn fq_nmod_mpoly_univar_init(
        A: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_clear(
        A: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_fit_length(
        A: *mut fq_nmod_mpoly_univar_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_print_pretty(
        A: *mut fq_nmod_mpoly_univar_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_assert_canonical(
        A: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_zero(
        A: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_set_coeff_ui(
        A: *mut fq_nmod_mpoly_univar_struct,
        e: mp_limb_t,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_to_univar(
        A: *mut fq_nmod_mpoly_univar_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_from_univar(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fq_nmod_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_from_univar(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_swap(
        A: *mut fq_nmod_mpoly_univar_struct,
        B: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_degree_fits_si(
        A: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_univar_length(
        A: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_univar_get_term_exp_si(
        A: *mut fq_nmod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_univar_get_term_coeff(
        c: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_swap_term_coeff(
        c: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_univar_pseudo_gcd(
        Gx: *mut fq_nmod_mpoly_univar_struct,
        Ax: *mut fq_nmod_mpoly_univar_struct,
        Bx: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_univar_resultant(
        R: *mut fq_nmod_mpoly_struct,
        Ax: *mut fq_nmod_mpoly_univar_struct,
        Bx: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_univar_discriminant(
        D: *mut fq_nmod_mpoly_struct,
        Fx: *mut fq_nmod_mpoly_univar_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_resultant(
        R: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_discriminant(
        R: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_is_canonical(
        poly: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_init(
        A: *mut fq_nmod_mpolyu_struct,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_clear(A: *mut fq_nmod_mpolyu_struct, uctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyu_swap(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_zero(A: *mut fq_nmod_mpolyu_struct, uctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyu_is_one(
        A: *mut fq_nmod_mpolyu_struct,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_print_pretty(
        poly: *mut fq_nmod_mpolyu_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_fit_length(
        A: *mut fq_nmod_mpolyu_struct,
        length: mp_limb_signed_t,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_one(A: *mut fq_nmod_mpolyu_struct, uctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyu_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_repack_bits_inplace(
        A: *mut fq_nmod_mpolyu_struct,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_shift_right(A: *mut fq_nmod_mpolyu_struct, s: mp_limb_t);
    pub fn fq_nmod_mpolyu_shift_left(A: *mut fq_nmod_mpolyu_struct, s: mp_limb_t);
    pub fn fq_nmod_mpolyu_content_mpoly(
        g: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_scalar_mul_fq_nmod(
        A: *mut fq_nmod_mpolyu_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_set(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_evaluate_one_fq_nmod(
        E: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        var: mp_limb_signed_t,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_setform(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpolyu_get_coeff(
        A: *mut fq_nmod_mpolyu_struct,
        pow: mp_limb_t,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut fq_nmod_mpoly_struct;
    pub fn fq_nmod_mpoly_to_mpolyu_perm_deflate(
        A: *mut fq_nmod_mpolyu_struct,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fq_nmod_mpoly_from_mpolyu_perm_inflate(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpolyu_struct,
        uctx: *mut fq_nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fq_nmod_mpolyuu_divides(
        Q: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        nmainvars: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_divexact_mpoly_inplace(
        A: *mut fq_nmod_mpolyu_struct,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_mul_mpoly(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_mul_mpoly_inplace(
        A: *mut fq_nmod_mpolyu_struct,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_gcdm_zippel(
        G: *mut fq_nmod_mpolyu_struct,
        Abar: *mut fq_nmod_mpolyu_struct,
        Bbar: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_leadcoeff(
        A: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut mp_limb_t;
    pub fn fq_nmod_mpolyn_init(
        A: *mut fq_nmod_mpolyn_struct,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_clear(A: *mut fq_nmod_mpolyn_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyn_swap(A: *mut fq_nmod_mpolyn_struct, B: *mut fq_nmod_mpolyn_struct);
    pub fn fq_nmod_mpolyn_is_canonical(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_zero(A: *mut fq_nmod_mpolyn_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyn_one(A: *mut fq_nmod_mpolyn_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyn_is_zero(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_print_pretty(
        A: *mut fq_nmod_mpolyn_struct,
        x_in: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_fit_length(
        A: *mut fq_nmod_mpolyn_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_fit_bits(
        A: *mut fq_nmod_mpolyn_struct,
        bits: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_set(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_leadcoeff(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut mp_limb_t;
    pub fn fq_nmod_mpolyn_leadcoeff_poly(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut n_poly_struct;
    pub fn fq_nmod_mpoly_to_mpolyn_perm_deflate(
        A: *mut fq_nmod_mpolyn_struct,
        nctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fq_nmod_mpoly_from_mpolyn_perm_inflate(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpolyn_struct,
        nctx: *mut fq_nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fq_nmod_mpolyun_init(
        A: *mut fq_nmod_mpolyun_struct,
        bits: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_clear(
        A: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_is_canonical(
        A: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyun_print_pretty(
        poly: *mut fq_nmod_mpolyun_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_swap(A: *mut fq_nmod_mpolyun_struct, B: *mut fq_nmod_mpolyun_struct);
    pub fn fq_nmod_mpolyun_zero(A: *mut fq_nmod_mpolyun_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyun_fit_length(
        A: *mut fq_nmod_mpolyun_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_one(A: *mut fq_nmod_mpolyun_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyn_is_nonzero_fq_nmod(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyun_is_nonzero_fq_nmod(
        A: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_scalar_mul_fq_nmod(
        A: *mut fq_nmod_mpolyn_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_scalar_mul_fq_nmod(
        A: *mut fq_nmod_mpolyun_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_shift_right(A: *mut fq_nmod_mpolyun_struct, s: mp_limb_t);
    pub fn fq_nmod_mpolyun_shift_left(A: *mut fq_nmod_mpolyun_struct, s: mp_limb_t);
    pub fn fq_nmod_mpolyn_mul_poly(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpolyn_struct,
        c: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        t: *mut fq_nmod_poly_struct,
    );
    pub fn fq_nmod_mpolyun_mul_poly(
        A: *mut fq_nmod_mpolyun_struct,
        B: *mut fq_nmod_mpolyun_struct,
        c: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_divexact_poly(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpolyn_struct,
        c: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        q: *mut fq_nmod_poly_struct,
        r: *mut fq_nmod_poly_struct,
    );
    pub fn fq_nmod_mpolyun_divexact_poly(
        A: *mut fq_nmod_mpolyun_struct,
        B: *mut fq_nmod_mpolyun_struct,
        c: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_content_poly(
        g: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_content_poly(
        g: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_lastdeg(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpolyun_lastdeg(
        A: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpolyun_set(
        A: *mut fq_nmod_mpolyun_struct,
        B: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_cvtto_mpolyn(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_cvtto_mpolyun(
        A: *mut fq_nmod_mpolyun_struct,
        B: *mut fq_nmod_mpolyu_struct,
        k: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_cvtfrom_mpolyn(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyu_cvtfrom_mpolyun(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyun_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_leadcoeff_poly(
        A: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> *mut n_poly_struct;
    pub fn fq_nmod_next(alpha: *mut nmod_poly_struct, fqctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn fq_nmod_next_not_zero(alpha: *mut nmod_poly_struct, fqctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_mpolyu_gcds_zippel(
        G: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        f: *mut fq_nmod_mpolyu_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
        degbound: *mut mp_limb_signed_t,
    ) -> nmod_gcds_ret_t;
    pub fn fq_nmod_mpolyu_gcdp_zippel_univar(
        G: *mut fq_nmod_mpolyu_struct,
        Abar: *mut fq_nmod_mpolyu_struct,
        Bbar: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_gcdp_zippel_univar_no_cofactors(
        G: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyu_gcdp_zippel(
        G: *mut fq_nmod_mpolyu_struct,
        Abar: *mut fq_nmod_mpolyu_struct,
        Bbar: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyu_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_gcd_brown_smprime(
        G: *mut fq_nmod_mpolyn_struct,
        Abar: *mut fq_nmod_mpolyn_struct,
        Bbar: *mut fq_nmod_mpolyn_struct,
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_gcd_brown_lgprime(
        G: *mut fq_nmod_mpolyn_struct,
        Abar: *mut fq_nmod_mpolyn_struct,
        Bbar: *mut fq_nmod_mpolyn_struct,
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_monomial_evals2_cache(
        E: *mut n_polyun_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        betas: *const fq_nmod_struct,
        m: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_monomial_evals_cache(
        E: *mut n_poly_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        betas: *const fq_nmod_struct,
        start: mp_limb_signed_t,
        stop: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn n_fq_bpoly_eval_step_sep(
        E: *mut n_bpoly_struct,
        cur: *mut n_polyun_struct,
        inc: *mut n_polyun_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_polyun_zip_start(
        Z: *mut n_polyun_struct,
        H: *mut n_polyun_struct,
        req_images: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_polyu2n_add_zip_must_match(
        Z: *mut n_polyun_struct,
        A: *mut n_bpoly_struct,
        cur_length: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn n_fq_polyun_zip_solve(
        A: *mut fq_nmod_mpoly_struct,
        Z: *mut n_polyun_struct,
        H: *mut n_polyun_struct,
        M: *mut n_polyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn nmod_mpolyn_interp_reduce_lg_poly(
        E: *mut fq_nmod_poly_struct,
        fqctx: *mut fq_nmod_ctx_struct,
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyn_interp_lift_lg_poly(
        lastdeg_: *mut mp_limb_signed_t,
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_poly_struct,
        fqctx: *mut fq_nmod_ctx_struct,
    );
    pub fn nmod_mpolyn_interp_crt_lg_poly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        modulus: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_poly_struct,
        fqctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn nmod_mpolyn_interp_lift_lg_bpoly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        smctx: *mut nmod_mpoly_ctx_struct,
        A: *mut n_bpoly_struct,
        lgctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyn_interp_crt_lg_bpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        modulus: *mut n_poly_struct,
        smctx: *mut nmod_mpoly_ctx_struct,
        A: *mut n_bpoly_struct,
        lgctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn nmod_mpolyn_interp_reduce_lg_mpolyn(
        E: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyn_interp_lift_lg_mpolyn(
        lastdeg: *mut mp_limb_signed_t,
        A: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyn_interp_crt_lg_mpolyn(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        modulus: *mut n_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn nmod_mpolyn_interp_reduce_lg_mpoly(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut nmod_mpolyn_struct,
        ffctx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyun_interp_reduce_lg_mpolyu(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut nmod_mpolyun_struct,
        ffctx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyn_interp_lift_lg_mpoly(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        Ap: *mut fq_nmod_mpoly_struct,
        ctxp: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyun_interp_lift_lg_mpolyu(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        Ap: *mut fq_nmod_mpolyu_struct,
        ctxp: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn nmod_mpolyun_interp_crt_lg_mpolyu(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyun_struct,
        T: *mut nmod_mpolyun_struct,
        m: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_mpolyu_struct,
        ffctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn nmod_mpolyn_interp_mcrt_lg_mpoly(
        lastdeg_: *mut mp_limb_signed_t,
        H: *mut nmod_mpolyn_struct,
        smctx: *mut nmod_mpoly_ctx_struct,
        m: *mut n_poly_struct,
        inv_m_eval: *const mp_limb_t,
        A: *mut fq_nmod_mpoly_struct,
        lgctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn nmod_mpolyun_interp_mcrt_lg_mpolyu(
        lastdeg: *mut mp_limb_signed_t,
        H: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        m: *mut n_poly_struct,
        A: *mut fq_nmod_mpolyu_struct,
        ctxp: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_reduce_sm_poly(
        E: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_mpolyn_struct,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_interp_lift_sm_poly(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_interp_crt_sm_poly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        T: *mut fq_nmod_mpolyn_struct,
        A: *mut fq_nmod_poly_struct,
        modulus: *mut fq_nmod_poly_struct,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_lift_sm_bpoly(
        F: *mut fq_nmod_mpolyn_struct,
        A: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_interp_crt_sm_bpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        T: *mut fq_nmod_mpolyn_struct,
        A: *mut n_bpoly_struct,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_reduce_sm_mpolyn(
        E: *mut fq_nmod_mpolyn_struct,
        A: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_interp_lift_sm_mpolyn(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_interp_mcrt_sm_mpoly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        A: *mut fq_nmod_mpoly_struct,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_crt_sm_mpolyn(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        T: *mut fq_nmod_mpolyn_struct,
        A: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        modulus: *mut fq_nmod_poly_struct,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_reduce_lg_poly(
        E: *mut fq_nmod_poly_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyn_interp_lift_lg_poly(
        lastdeg_: *mut mp_limb_signed_t,
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_poly_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyn_interp_crt_lg_poly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        T: *mut fq_nmod_mpolyn_struct,
        modulus: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_poly_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_lift_lg_bpoly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        smctx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut n_bpoly_struct,
        lgctx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyn_interp_crt_lg_bpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        T: *mut fq_nmod_mpolyn_struct,
        modulus: *mut n_poly_struct,
        smctx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut n_bpoly_struct,
        lgctx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_reduce_lg_mpolyn(
        E: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyn_interp_lift_lg_mpolyn(
        lastdeg_: *mut mp_limb_signed_t,
        A: *mut fq_nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyn_interp_crt_lg_mpolyn(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyn_struct,
        T: *mut fq_nmod_mpolyn_struct,
        modulus: *mut fq_nmod_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyun_interp_reduce_sm_mpolyu(
        B: *mut fq_nmod_mpolyu_struct,
        A: *mut fq_nmod_mpolyun_struct,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyn_interp_lift_sm_mpoly(
        A: *mut fq_nmod_mpolyn_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_interp_lift_sm_mpolyu(
        A: *mut fq_nmod_mpolyun_struct,
        B: *mut fq_nmod_mpolyu_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyun_interp_crt_sm_mpolyu(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyun_struct,
        T: *mut fq_nmod_mpolyun_struct,
        A: *mut fq_nmod_mpolyu_struct,
        modulus: *mut fq_nmod_poly_struct,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyn_interp_reduce_lg_mpoly(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpolyn_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyun_interp_reduce_lg_mpolyu(
        A: *mut fq_nmod_mpolyu_struct,
        B: *mut fq_nmod_mpolyun_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyn_interp_lift_lg_mpoly(
        A: *mut fq_nmod_mpolyn_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpoly_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyun_interp_lift_lg_mpolyu(
        A: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_nmod_mpolyu_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    );
    pub fn fq_nmod_mpolyun_interp_crt_lg_mpolyu(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fq_nmod_mpolyun_struct,
        T: *mut fq_nmod_mpolyun_struct,
        m: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        A: *mut fq_nmod_mpolyu_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    ) -> c_int;
    pub fn fq_nmod_mpolyun_interp_mcrt_lg_mpolyu(
        lastdeg: *mut mp_limb_signed_t,
        H: *mut fq_nmod_mpolyun_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        m: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_mpolyu_struct,
        ectx: *mut fq_nmod_mpoly_ctx_struct,
        emb: *mut bad_fq_nmod_embed_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_geobucket_init(
        B: *mut fq_nmod_mpoly_geobucket_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_geobucket_clear(
        B: *mut fq_nmod_mpoly_geobucket_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_geobucket_empty(
        p: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_geobucket_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_geobucket_fit_length(
        B: *mut fq_nmod_mpoly_geobucket_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_geobucket_set(
        B: *mut fq_nmod_mpoly_geobucket_struct,
        p: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_geobucket_add(
        B: *mut fq_nmod_mpoly_geobucket_struct,
        p: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_geobucket_sub(
        B: *mut fq_nmod_mpoly_geobucket_struct,
        p: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_remainder_strongtest(
        r: *mut fq_nmod_mpoly_struct,
        g: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
}
