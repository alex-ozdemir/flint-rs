#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_nmod_mpoly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fq_nmod::*;
use crate::fq_nmod_mpoly::*;
use crate::fq_nmod_poly::fq_nmod_poly_struct;
use crate::mpoly::*;
use crate::n_poly::*;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_factor_struct {
    pub constant: fq_nmod_t,
    pub poly: *mut fq_nmod_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type fq_nmod_mpoly_factor_t = [fq_nmod_mpoly_factor_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpolyv_struct {
    pub coeffs: *mut fq_nmod_mpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fq_nmod_mpolyv_t = [fq_nmod_mpolyv_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_pfrac_struct {
    pub bits: mp_limb_t,
    pub w: mp_limb_signed_t,
    pub r: mp_limb_signed_t,
    pub inv_prod_dbetas: *mut fq_nmod_poly_struct,
    pub inv_prod_dbetas_mvar: *mut fq_nmod_mpoly_struct,
    pub dbetas: *mut fq_nmod_poly_struct,
    pub dbetas_mvar: *mut fq_nmod_mpoly_struct,
    pub prod_mbetas: *mut fq_nmod_mpoly_struct,
    pub prod_mbetas_coeffs: *mut fq_nmod_mpolyv_struct,
    pub mbetas: *mut fq_nmod_mpoly_struct,
    pub deltas: *mut fq_nmod_mpoly_struct,
    pub xalpha: *mut fq_nmod_mpoly_struct,
    pub q: *mut fq_nmod_mpoly_struct,
    pub G: *mut fq_nmod_mpoly_geobucket_struct,
    pub qt: *mut fq_nmod_mpoly_struct,
    pub newt: *mut fq_nmod_mpoly_struct,
    pub delta_coeffs: *mut fq_nmod_mpolyv_struct,
    pub T: fq_nmod_mpoly_t,
    pub Q: fq_nmod_mpoly_t,
    pub R: fq_nmod_mpoly_t,
}

pub type fq_nmod_mpoly_pfrac_t = [fq_nmod_mpoly_pfrac_struct; 1usize];

extern "C" {
    pub fn fq_nmod_mpoly_factor_init(
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_realloc(
        f: *mut fq_nmod_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_fit_length(
        f: *mut fq_nmod_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_clear(
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_length(
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_factor_get_constant_fq_nmod(
        c: *mut nmod_poly_struct,
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_get_base(
        p: *mut fq_nmod_mpoly_struct,
        f: *mut fq_nmod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_swap_base(
        p: *mut fq_nmod_mpoly_struct,
        f: *mut fq_nmod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_get_exp_si(
        f: *mut fq_nmod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_mpoly_factor_set(
        a: *mut fq_nmod_mpoly_factor_struct,
        b: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_print_pretty(
        f: *mut fq_nmod_mpoly_factor_struct,
        vars: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_append_ui(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        e: mp_limb_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_append_fmpz(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        e: *mut fmpz,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_content(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_squarefree(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_separable(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        sep: c_int,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_sort(
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_cmp(
        A: *mut fq_nmod_mpoly_factor_struct,
        B: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_swap(
        A: *mut fq_nmod_mpoly_factor_struct,
        B: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_one(
        a: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_factor_expand(
        A: *mut fq_nmod_mpoly_struct,
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_matches(
        a: *mut fq_nmod_mpoly_struct,
        f: *mut fq_nmod_mpoly_factor_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_get_lead0(
        c: *mut fq_nmod_mpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_set_lead0(
        A: *mut fq_nmod_mpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn n_fq_bpoly_mul(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_bpoly_mul_series(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_bpoly_add(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_bpoly_sub(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_bpoly_divrem_series(
        Q: *mut n_bpoly_struct,
        R: *mut n_bpoly_struct,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_fq_bpoly_divides(
        Q: *mut n_bpoly_struct,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn n_fq_bpoly_make_primitive(
        g: *mut n_poly_struct,
        A: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mpoly_get_n_fq_bpoly(
        A: *mut n_bpoly_struct,
        B: *mut fq_nmod_mpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_set_n_fq_bpoly(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut n_bpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn n_fq_bpoly_factor_smprime(
        c: *mut n_poly_struct,
        F: *mut n_tpoly_struct,
        B: *mut n_bpoly_struct,
        allow_shift: c_int,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn n_fq_bpoly_factor_lgprime(
        c: *mut n_poly_struct,
        F: *mut n_tpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn n_polyu3_fq_print_pretty(
        A: *mut n_polyu_struct,
        var0: *const c_char,
        var1: *const c_char,
        var2: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_polyu_fq_is_canonical(A: *mut n_polyu_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn n_polyu2n_fq_print_pretty(
        A: *mut n_polyun_struct,
        var0: *const c_char,
        var1: *const c_char,
        varlast: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_polyu3n_fq_print_pretty(
        A: *mut n_polyun_struct,
        var0: *const c_char,
        var1: *const c_char,
        var2: *const c_char,
        varlast: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn n_polyun_fq_is_canonical(A: *mut n_polyun_struct, ctx: *mut fq_nmod_ctx_struct)
        -> c_int;
    pub fn fq_nmod_mpolyv_init(A: *mut fq_nmod_mpolyv_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyv_swap(
        A: *mut fq_nmod_mpolyv_struct,
        B: *mut fq_nmod_mpolyv_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyv_clear(A: *mut fq_nmod_mpolyv_struct, ctx: *mut fq_nmod_mpoly_ctx_struct);
    pub fn fq_nmod_mpolyv_print_pretty(
        poly: *mut fq_nmod_mpolyv_struct,
        x: *mut *const c_char,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyv_fit_length(
        A: *mut fq_nmod_mpolyv_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpolyv_set_coeff(
        A: *mut fq_nmod_mpolyv_struct,
        i: mp_limb_signed_t,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_to_mpolyv(
        A: *mut fq_nmod_mpolyv_struct,
        B: *mut fq_nmod_mpoly_struct,
        xalpha: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_from_mpolyv(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fq_nmod_mpolyv_struct,
        xalpha: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_vec_content_mpoly(
        g: *mut fq_nmod_mpoly_struct,
        A: *const fq_nmod_mpoly_struct,
        Alen: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_vec_divexact_mpoly(
        A: *mut fq_nmod_mpoly_struct,
        Alen: mp_limb_signed_t,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_vec_mul_mpoly(
        A: *mut fq_nmod_mpoly_struct,
        Alen: mp_limb_signed_t,
        c: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_factor_separable(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        sep: c_int,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_lcc_wang(
        lc_divs: *mut fq_nmod_mpoly_struct,
        lcAfac: *mut fq_nmod_mpoly_factor_struct,
        Auc: *mut n_poly_struct,
        Auf: *const n_bpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const n_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_irred_smprime_zassenhaus(
        fac: *mut fq_nmod_mpolyv_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_irred_lgprime_zassenhaus(
        fac: *mut fq_nmod_mpolyv_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_irred_smprime_wang(
        fac: *mut fq_nmod_mpolyv_struct,
        A: *mut fq_nmod_mpoly_struct,
        lcAfac: *mut fq_nmod_mpoly_factor_struct,
        lcA: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_irred_lgprime_wang(
        Af: *mut fq_nmod_mpolyv_struct,
        A: *mut fq_nmod_mpoly_struct,
        lcAfac: *mut fq_nmod_mpoly_factor_struct,
        lcA: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_irred_smprime_zippel(
        fac: *mut fq_nmod_mpolyv_struct,
        A: *mut fq_nmod_mpoly_struct,
        lcAfac: *mut fq_nmod_mpoly_factor_struct,
        lcA: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_irred_lgprime_zippel(
        Af: *mut fq_nmod_mpolyv_struct,
        A: *mut fq_nmod_mpoly_struct,
        lcAfac: *mut fq_nmod_mpoly_factor_struct,
        lcA: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
    pub fn fq_nmod_mpoly_compression_do(
        L: *mut fq_nmod_mpoly_struct,
        Lctx: *mut fq_nmod_mpoly_ctx_struct,
        Acoeffs: *mut mp_limb_t,
        Alen: mp_limb_signed_t,
        M: *mut mpoly_compression_struct,
    );
    pub fn fq_nmod_mpoly_compression_undo(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        Actx: *mut fq_nmod_mpoly_ctx_struct,
        L: *mut fq_nmod_mpoly_struct,
        Lctx: *mut fq_nmod_mpoly_ctx_struct,
        M: *mut mpoly_compression_struct,
    );
    pub fn fq_nmod_mpoly_pfrac_init(
        I: *mut fq_nmod_mpoly_pfrac_struct,
        bits: mp_limb_t,
        l: mp_limb_signed_t,
        r: mp_limb_signed_t,
        betas: *const fq_nmod_mpoly_struct,
        alpha: *const fq_nmod_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_pfrac_clear(
        I: *mut fq_nmod_mpoly_pfrac_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn fq_nmod_mpoly_pfrac(
        r: mp_limb_signed_t,
        t: *mut fq_nmod_mpoly_struct,
        deg: *const mp_limb_signed_t,
        I: *mut fq_nmod_mpoly_pfrac_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_hlift(
        m: mp_limb_signed_t,
        f: *mut fq_nmod_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fq_nmod_struct,
        A: *mut fq_nmod_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn n_fq_bpoly_hlift2_cubic(
        A: *mut n_bpoly_struct,
        B0: *mut n_bpoly_struct,
        B1: *mut n_bpoly_struct,
        alpha_: *mut nmod_poly_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        E: *mut nmod_eval_interp_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
    pub fn n_fq_bpoly_hlift2(
        A: *mut n_bpoly_struct,
        B0: *mut n_bpoly_struct,
        B1: *mut n_bpoly_struct,
        alpha: *mut nmod_poly_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
    pub fn n_fq_bpoly_hlift_cubic(
        A: *mut n_bpoly_struct,
        B0: *mut n_bpoly_struct,
        B1: *mut n_bpoly_struct,
        alpha_: *mut nmod_poly_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        E: *mut nmod_eval_interp_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
    pub fn n_fq_bpoly_hlift(
        r: mp_limb_signed_t,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        alpha: *mut nmod_poly_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
    pub fn n_fq_polyu3_hlift(
        r: mp_limb_signed_t,
        BB: *mut n_polyun_struct,
        A: *mut n_polyu_struct,
        B: *mut n_polyu_struct,
        beta: *mut nmod_poly_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_algo(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
        algo: c_uint,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_wang(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_zassenhaus(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_mpoly_factor_zippel(
        f: *mut fq_nmod_mpoly_factor_struct,
        A: *mut fq_nmod_mpoly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_eval_rest_n_fq_poly(
        E: *mut n_poly_struct,
        starts: *mut mp_limb_signed_t,
        ends: *mut mp_limb_signed_t,
        stops: *mut mp_limb_signed_t,
        es: *mut mp_limb_t,
        Acoeffs: *const mp_limb_t,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        alphas: *const n_fq_poly_struct,
        offsets: *const mp_limb_signed_t,
        shifts: *const mp_limb_signed_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
        nvars: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_mpoly_eval_rest_to_n_fq_bpoly(
        E: *mut n_bpoly_struct,
        A: *mut fq_nmod_mpoly_struct,
        alphabetas: *const n_poly_struct,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
    pub fn _fq_nmod_mpoly_set_n_fq_bpoly_gen1_zero(
        A: *mut fq_nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut n_bpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_nmod_mpoly_ctx_struct,
    );
}
