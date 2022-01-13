#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_default_poly_factor.html).

use crate::deps::*;
use crate::fq_default::{fq_default_ctx_struct, fq_default_struct};
use crate::fq_default_poly::fq_default_poly_struct;
use crate::fq_nmod_poly_factor::fq_nmod_poly_factor_t;
use crate::fq_poly_factor::fq_poly_factor_t;
use crate::fq_zech_poly_factor::fq_zech_poly_factor_t;
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Copy, Clone)]
pub union fq_default_poly_factor_struct {
    pub fq: fq_poly_factor_t,
    pub fq_nmod: fq_nmod_poly_factor_t,
    pub fq_zech: fq_zech_poly_factor_t,
}

pub type fq_default_poly_factor_t = [fq_default_poly_factor_struct; 1usize];

extern "C" {
    pub fn fq_default_poly_factor_init(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_clear(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_realloc(
        fac: *mut fq_default_poly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_fit_length(
        fac: *mut fq_default_poly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_length(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *mut fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_poly_factor_exp(
        fac: *mut fq_default_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_poly_factor_set(
        res: *mut fq_default_poly_factor_struct,
        fac: *mut fq_default_poly_factor_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_insert(
        fac: *mut fq_default_poly_factor_struct,
        poly: *mut fq_default_poly_struct,
        exp: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_get_poly(
        poly: *mut fq_default_poly_struct,
        fac: *mut fq_default_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_print(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_print_pretty(
        fac: *mut fq_default_poly_factor_struct,
        var: *const c_char,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_concat(
        res: *mut fq_default_poly_factor_struct,
        fac: *mut fq_default_poly_factor_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_pow(
        fac: *mut fq_default_poly_factor_struct,
        exp: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_is_squarefree(
        f: *mut fq_default_poly_struct,
        ctx: *mut fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_factor_squarefree(
        res: *mut fq_default_poly_factor_struct,
        f: *mut fq_default_poly_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_is_irreducible(
        f: *mut fq_default_poly_struct,
        ctx: *mut fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_factor_distinct_deg(
        res: *mut fq_default_poly_factor_struct,
        poly: *mut fq_default_poly_struct,
        degs: *const *mut mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_equal_deg(
        factors: *mut fq_default_poly_factor_struct,
        pol: *mut fq_default_poly_struct,
        d: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor(
        result: *mut fq_default_poly_factor_struct,
        leading_coeff: *mut fq_default_struct,
        input: *mut fq_default_poly_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_factor_split_single(
        linfactor: *mut fq_default_poly_struct,
        input: *mut fq_default_poly_struct,
        ctx: *mut fq_default_ctx_struct,
    );
    pub fn fq_default_poly_roots(
        r: *mut fq_default_poly_factor_struct,
        f: *mut fq_default_poly_struct,
        with_multiplicity: c_int,
        ctx: *mut fq_default_ctx_struct,
    );
}
