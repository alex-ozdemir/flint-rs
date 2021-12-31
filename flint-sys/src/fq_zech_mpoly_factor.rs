#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_zech_mpoly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fq_zech::{fq_zech_ctx_struct, fq_zech_struct, fq_zech_t};
use crate::fq_zech_mpoly::*;
use crate::fq_zech_poly::fq_zech_poly_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpoly_factor_struct {
    pub constant: fq_zech_t,
    pub poly: *mut fq_zech_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type fq_zech_mpoly_factor_t = [fq_zech_mpoly_factor_struct; 1usize];
extern "C" {
    pub fn fq_zech_mpoly_factor_init(
        f: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_realloc(
        f: *mut fq_zech_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_fit_length(
        f: *mut fq_zech_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_clear(
        f: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_set(
        a: *mut fq_zech_mpoly_factor_struct,
        b: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_print_pretty(
        f: *mut fq_zech_mpoly_factor_struct,
        vars: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_append_ui(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        e: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_append_fmpz(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        e: *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_squarefree(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_swap(
        A: *mut fq_zech_mpoly_factor_struct,
        B: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_one(
        a: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_expand(
        A: *mut fq_zech_mpoly_struct,
        f: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_matches(
        a: *mut fq_zech_mpoly_struct,
        f: *mut fq_zech_mpoly_factor_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_get_lead0(
        c: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_set_lead0(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpolyv_struct {
    pub coeffs: *mut fq_zech_mpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fq_zech_mpolyv_t = [fq_zech_mpolyv_struct; 1usize];
extern "C" {
    pub fn fq_zech_mpolyv_init(A: *mut fq_zech_mpolyv_struct, ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpolyv_swap(
        A: *mut fq_zech_mpolyv_struct,
        B: *mut fq_zech_mpolyv_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyv_clear(A: *mut fq_zech_mpolyv_struct, ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpolyv_print_pretty(
        poly: *mut fq_zech_mpolyv_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyv_fit_length(
        A: *mut fq_zech_mpolyv_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyv_set_coeff(
        A: *mut fq_zech_mpolyv_struct,
        i: mp_limb_signed_t,
        c: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_to_mpolyv(
        A: *mut fq_zech_mpolyv_struct,
        B: *mut fq_zech_mpoly_struct,
        xalpha: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_from_mpolyv(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpolyv_struct,
        xalpha: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_content_mpoly(
        g: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_univar_divexact_mpoly(
        A: *mut fq_zech_mpoly_univar_struct,
        b: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_factor_lcc_wang(
        lc_divs: *mut fq_zech_mpoly_struct,
        lcAfac: *mut fq_zech_mpoly_factor_struct,
        Auc: *mut fq_zech_poly_struct,
        Auf: *const fq_zech_bpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fq_zech_poly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_irred_smprime_zassenhaus(
        fac: *mut fq_zech_mpolyv_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_irred_lgprime_zassenhaus(
        fac: *mut fq_zech_mpolyv_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_irred_smprime_wang(
        fac: *mut fq_zech_mpolyv_struct,
        A: *mut fq_zech_mpoly_struct,
        lcAfac: *mut fq_zech_mpoly_factor_struct,
        lcA: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_irred_lgprime_wang(
        Af: *mut fq_zech_mpolyv_struct,
        A: *mut fq_zech_mpoly_struct,
        lcAfac: *mut fq_zech_mpoly_factor_struct,
        lcA: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_irred_smprime_zippel(
        fac: *mut fq_zech_mpolyv_struct,
        A: *mut fq_zech_mpoly_struct,
        lcAfac: *mut fq_zech_mpoly_factor_struct,
        lcA: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_irred_lgprime_zippel(
        Af: *mut fq_zech_mpolyv_struct,
        A: *mut fq_zech_mpoly_struct,
        lcAfac: *mut fq_zech_mpoly_factor_struct,
        lcA: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpoly_pfrac_struct {
    pub bits: mp_limb_t,
    pub w: mp_limb_signed_t,
    pub r: mp_limb_signed_t,
    pub inv_prod_dbetas: *mut fq_zech_poly_struct,
    pub inv_prod_dbetas_mvar: *mut fq_zech_mpoly_struct,
    pub dbetas: *mut fq_zech_poly_struct,
    pub dbetas_mvar: *mut fq_zech_mpoly_struct,
    pub prod_mbetas: *mut fq_zech_mpoly_struct,
    pub prod_mbetas_coeffs: *mut fq_zech_mpolyv_struct,
    pub mbetas: *mut fq_zech_mpoly_struct,
    pub deltas: *mut fq_zech_mpoly_struct,
    pub xalpha: *mut fq_zech_mpoly_struct,
    pub q: *mut fq_zech_mpoly_struct,
    pub qt: *mut fq_zech_mpoly_struct,
    pub newt: *mut fq_zech_mpoly_struct,
    pub delta_coeffs: *mut fq_zech_mpolyv_struct,
    pub T: fq_zech_mpoly_t,
    pub Q: fq_zech_mpoly_t,
    pub R: fq_zech_mpoly_t,
}
pub type fq_zech_mpoly_pfrac_t = [fq_zech_mpoly_pfrac_struct; 1usize];
extern "C" {
    pub fn fq_zech_mpoly_pfrac_init(
        I: *mut fq_zech_mpoly_pfrac_struct,
        bits: mp_limb_t,
        l: mp_limb_signed_t,
        r: mp_limb_signed_t,
        betas: *const fq_zech_mpoly_struct,
        alpha: *const fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_pfrac_clear(
        I: *mut fq_zech_mpoly_pfrac_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_pfrac(
        r: mp_limb_signed_t,
        t: *mut fq_zech_mpoly_struct,
        deg: *const mp_limb_signed_t,
        I: *mut fq_zech_mpoly_pfrac_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_hlift(
        m: mp_limb_signed_t,
        f: *mut fq_zech_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_hlift2(
        A: *mut fq_zech_bpoly_struct,
        B0: *mut fq_zech_bpoly_struct,
        B1: *mut fq_zech_bpoly_struct,
        alpha: *mut fq_zech_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_hlift(
        r: mp_limb_signed_t,
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        alpha: *mut fq_zech_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_polyu3_hlift(
        r: mp_limb_signed_t,
        BB: *mut fq_zech_polyun_struct,
        A: *mut fq_zech_polyu_struct,
        B: *mut fq_zech_polyu_struct,
        beta: *mut fq_zech_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_algo(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
        algo: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_zassenhaus(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_wang(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_factor_zippel(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_poly_product_roots_fq_zech(
        master: *mut fq_zech_poly_struct,
        monomials: *const fq_zech_struct,
        mlength: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_monomial_evals(
        E: *mut fq_zech_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        alpha: *const fq_zech_struct,
        vstart: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_eval_rest_fq_zech_poly(
        E: *mut fq_zech_poly_struct,
        starts: *mut mp_limb_signed_t,
        ends: *mut mp_limb_signed_t,
        stops: *mut mp_limb_signed_t,
        es: *mut mp_limb_t,
        Acoeffs: *const fq_zech_struct,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        alphas: *const fq_zech_poly_struct,
        offsets: *const mp_limb_signed_t,
        shifts: *const mp_limb_signed_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
        nvars: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_eval_to_bpoly(
        E: *mut fq_zech_bpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        alphabetas: *const fq_zech_poly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_set_fq_zech_bpoly_var1_zero(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fq_zech_bpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
