#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! *See the [FLINT documentation](http://flintlib.org/doc/nmod_mpoly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::mpoly::*;
use crate::n_poly::*;
use crate::nmod_mpoly::*;
use crate::nmod_vec::nmod_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_factor_struct {
    pub constant: mp_limb_t,
    pub poly: *mut nmod_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type nmod_mpoly_factor_t = [nmod_mpoly_factor_struct; 1usize];
extern "C" {
    pub fn nmod_mpoly_factor_init(
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_init2(
        f: *mut nmod_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_realloc(
        f: *mut nmod_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_fit_length(
        f: *mut nmod_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_clear(
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_length(
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_factor_get_constant_ui(
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_factor_get_base(
        p: *mut nmod_mpoly_struct,
        f: *mut nmod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_swap_base(
        p: *mut nmod_mpoly_struct,
        f: *mut nmod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_get_exp_si(
        f: *mut nmod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_factor_append_ui(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        e: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_append_fmpz(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        e: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_set(
        f: *mut nmod_mpoly_factor_struct,
        g: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_print_pretty(
        f: *mut nmod_mpoly_factor_struct,
        vars: *mut *const ::std::os::raw::c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_content(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_squarefree(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_separable(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        sep: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_sort(
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_cmp(
        A: *mut nmod_mpoly_factor_struct,
        B: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_expand(
        A: *mut nmod_mpoly_struct,
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_matches(
        a: *mut nmod_mpoly_struct,
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_fix_units(
        f: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_swap(
        f: *mut nmod_mpoly_factor_struct,
        g: *mut nmod_mpoly_factor_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_factor_one(f: *mut nmod_mpoly_factor_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn _nmod_mpoly_get_lead0(
        c: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_set_lead0(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _n_poly_vec_max_degree(
        A: *const n_poly_struct,
        Alen: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _n_poly_vec_mul_nmod_intertible(
        A: *mut n_poly_struct,
        Alen: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_vec_mod_mul_poly(
        A: *mut n_poly_struct,
        Alen: mp_limb_signed_t,
        g: *mut n_poly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_vec_mod_divexact_poly(
        A: *mut n_poly_struct,
        Alen: mp_limb_signed_t,
        g: *mut n_poly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_vec_mod_content(
        g: *mut n_poly_struct,
        A: *const n_poly_struct,
        Alen: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_vec_mod_remove_content(
        g: *mut n_poly_struct,
        A: *mut n_poly_struct,
        Alen: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_polyu1n(
        A: *mut n_polyun_struct,
        B: *mut nmod_mpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_polyu1n(
        B: *mut nmod_mpoly_struct,
        A: *mut n_polyun_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyv_struct {
    pub coeffs: *mut nmod_mpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type nmod_mpolyv_t = [nmod_mpolyv_struct; 1usize];
extern "C" {
    pub fn nmod_mpolyv_init(A: *mut nmod_mpolyv_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyv_swap(
        A: *mut nmod_mpolyv_struct,
        B: *mut nmod_mpolyv_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyv_clear(A: *mut nmod_mpolyv_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyv_print_pretty(
        poly: *mut nmod_mpolyv_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyv_fit_length(
        A: *mut nmod_mpolyv_struct,
        length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyv_set_coeff(
        A: *mut nmod_mpolyv_struct,
        i: mp_limb_signed_t,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_to_mpolyv(
        A: *mut nmod_mpolyv_struct,
        B: *mut nmod_mpoly_struct,
        xalpha: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_from_mpolyv(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut nmod_mpolyv_struct,
        xalpha: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_vec_content_mpoly(
        g: *mut nmod_mpoly_struct,
        A: *const nmod_mpoly_struct,
        Alen: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _nmod_mpoly_vec_divexact_mpoly(
        A: *mut nmod_mpoly_struct,
        Alen: mp_limb_signed_t,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_vec_mul_mpoly(
        A: *mut nmod_mpoly_struct,
        Alen: mp_limb_signed_t,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_factor_separable(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        sep: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_lcc_wang(
        lc_divs: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        Auc: *mut n_poly_struct,
        Auf: *const n_bpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_smprime_zassenhaus(
        fac: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_medprime_zassenhaus(
        fac: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_lgprime_zassenhaus(
        fac: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_smprime_wang(
        fac: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        lcA: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_medprime_wang(
        Af: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        lcA: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_lgprime_wang(
        Af: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        lcA: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_smprime_zippel(
        fac: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        lcA: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_medprime_zippel(
        Af: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        lcA: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_irred_lgprime_zippel(
        Af: *mut nmod_mpolyv_struct,
        A: *mut nmod_mpoly_struct,
        lcAfac: *mut nmod_mpoly_factor_struct,
        lcA: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_compression_do(
        L: *mut nmod_mpoly_struct,
        Lctx: *mut nmod_mpoly_ctx_struct,
        Acoeffs: *mut mp_limb_t,
        Alen: mp_limb_signed_t,
        M: *mut mpoly_compression_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_compression_undo(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        Actx: *mut nmod_mpoly_ctx_struct,
        L: *mut nmod_mpoly_struct,
        Lctx: *mut nmod_mpoly_ctx_struct,
        M: *mut mpoly_compression_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_is_canonical(
        A: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpolyu3_print_pretty(
        A: *mut nmod_mpolyu_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        var2: *const ::std::os::raw::c_char,
        vars: *mut *const ::std::os::raw::c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_pfrac_struct {
    pub bits: mp_limb_t,
    pub w: mp_limb_signed_t,
    pub r: mp_limb_signed_t,
    pub inv_prod_dbetas: *mut n_poly_struct,
    pub inv_prod_dbetas_mvar: *mut nmod_mpoly_struct,
    pub dbetas: *mut n_poly_struct,
    pub dbetas_mvar: *mut nmod_mpoly_struct,
    pub prod_mbetas: *mut nmod_mpoly_struct,
    pub prod_mbetas_coeffs: *mut nmod_mpolyv_struct,
    pub mbetas: *mut nmod_mpoly_struct,
    pub deltas: *mut nmod_mpoly_struct,
    pub xalpha: *mut nmod_mpoly_struct,
    pub q: *mut nmod_mpoly_struct,
    pub G: *mut nmod_mpoly_geobucket_struct,
    pub qt: *mut nmod_mpoly_struct,
    pub newt: *mut nmod_mpoly_struct,
    pub delta_coeffs: *mut nmod_mpolyv_struct,
    pub T: nmod_mpoly_t,
    pub Q: nmod_mpoly_t,
    pub R: nmod_mpoly_t,
}
pub type nmod_mpoly_pfrac_t = [nmod_mpoly_pfrac_struct; 1usize];
extern "C" {
    pub fn nmod_mpoly_pfrac_init(
        I: *mut nmod_mpoly_pfrac_struct,
        bits: mp_limb_t,
        l: mp_limb_signed_t,
        r: mp_limb_signed_t,
        betas: *const nmod_mpoly_struct,
        alpha: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_pfrac_clear(I: *mut nmod_mpoly_pfrac_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_pfrac(
        r: mp_limb_signed_t,
        t: *mut nmod_mpoly_struct,
        deg: *const mp_limb_signed_t,
        I: *mut nmod_mpoly_pfrac_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_hlift(
        m: mp_limb_signed_t,
        f: *mut nmod_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const mp_limb_t,
        A: *mut nmod_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_bpoly_mod_pfrac(
        r: mp_limb_signed_t,
        C: *mut n_bpoly_struct,
        C_deg1_bound: *mut mp_limb_signed_t,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        mod_: nmod_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_bpoly_mod_hlift2(
        A: *mut n_bpoly_struct,
        B0: *mut n_bpoly_struct,
        B1: *mut n_bpoly_struct,
        alpha: mp_limb_t,
        degree_inner: mp_limb_signed_t,
        mod_: nmod_t,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_bpoly_mod_hlift2_cubic(
        A: *mut n_bpoly_struct,
        B0: *mut n_bpoly_struct,
        B1: *mut n_bpoly_struct,
        alpha: mp_limb_t,
        degree_inner: mp_limb_signed_t,
        ctx: nmod_t,
        E: *mut nmod_eval_interp_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_bpoly_mod_hlift(
        r: mp_limb_signed_t,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        alpha: mp_limb_t,
        degree_inner: mp_limb_signed_t,
        mod_: nmod_t,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_bpoly_mod_hlift_cubic(
        r: mp_limb_signed_t,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        alpha: mp_limb_t,
        degree_inner: mp_limb_signed_t,
        mod_: nmod_t,
        E: *mut nmod_eval_interp_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_polyu3_mod_hlift(
        r: mp_limb_signed_t,
        BB: *mut n_polyun_struct,
        A: *mut n_polyu_struct,
        B: *mut n_polyu_struct,
        beta: mp_limb_t,
        degree_inner: mp_limb_signed_t,
        ctx: nmod_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_hlift_zippel(
        m: mp_limb_signed_t,
        B: *mut nmod_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const mp_limb_t,
        A: *mut nmod_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_algo(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        algo: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_zassenhaus(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_wang(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpoly_factor_zippel(
        f: *mut nmod_mpoly_factor_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _nmod_mpoly_evaluate_rest_n_poly(
        E: *mut n_poly_struct,
        starts: *mut mp_limb_signed_t,
        ends: *mut mp_limb_signed_t,
        stops: *mut mp_limb_signed_t,
        es: *mut mp_limb_t,
        Acoeffs: *const mp_limb_t,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        alphas: *const n_poly_struct,
        offsets: *const mp_limb_signed_t,
        shifts: *const mp_limb_signed_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
        nvars: mp_limb_signed_t,
        ctx: nmod_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _nmod_mpoly_eval_rest_to_n_bpoly(
        E: *mut n_bpoly_struct,
        A: *mut nmod_mpoly_struct,
        alphabetas: *const n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_set_n_bpoly_var1_zero(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut n_bpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyl_gcdp_zippel_smprime(
        G: *mut nmod_mpoly_struct,
        Abar: *mut nmod_mpoly_struct,
        Bbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mpolyl_gcds_zippel(
        G: *mut nmod_mpoly_struct,
        Gmarks: *const mp_limb_t,
        Gmarkslen: mp_limb_signed_t,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        perm: *mut mp_limb_signed_t,
        l: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        Gdegbound: *mut mp_limb_signed_t,
        Amarks: *mut n_poly_struct,
        Bmarks: *mut n_poly_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mpoly_monomial_evals_nmod(
        EH: *mut n_poly_struct,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        alpha_caches: *mut n_poly_struct,
        start: mp_limb_signed_t,
        stop: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        fpctx: nmod_t,
    );
}
extern "C" {
    pub fn mpoly1_monomial_evals_nmod(
        EH: *mut n_polyun_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Amarks: *const mp_limb_t,
        Amarkslen: mp_limb_signed_t,
        alpha_caches: *mut n_poly_struct,
        m: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        fpctx: nmod_t,
    );
}
extern "C" {
    pub fn mpoly2_monomial_evals_nmod(
        EH: *mut n_polyun_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Amarks: *mut mp_limb_t,
        Amarkslen: mp_limb_signed_t,
        alpha_caches: *mut n_poly_struct,
        m: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        fpctx: nmod_t,
    );
}
extern "C" {
    pub fn n_polyun_zip_start(
        Z: *mut n_polyun_struct,
        H: *mut n_polyun_struct,
        req_images: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn n_polyu2n_add_zip_must_match(
        Z: *mut n_polyun_struct,
        A: *mut n_bpoly_struct,
        cur_length: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_polyun_zip_solve(
        A: *mut nmod_mpoly_struct,
        Z: *mut n_polyun_struct,
        H: *mut n_polyun_struct,
        M: *mut n_polyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
