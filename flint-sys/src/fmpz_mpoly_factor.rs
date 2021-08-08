#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_mpoly_factor.html).


use crate::deps::*;
use crate::flint::*;
use crate::mpoly::mpoly_compression_struct;
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpq::fmpq;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fmpq_poly::{fmpq_poly_struct, fmpq_poly_t};
use crate::fmpz_poly_factor::{fmpz_poly_factor_struct, zassenhaus_prune_struct};
use crate::fmpz_mpoly::{fmpz_mpoly_struct, fmpz_mpoly_ctx_struct, fmpz_mpoly_univar_struct, fmpz_mpoly_geobucket_struct};
use libc::{c_char, c_int, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_factor_struct {
    pub constant: fmpz_t,
    pub constant_den: fmpz_t,
    pub poly: *mut fmpz_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type fmpz_mpoly_factor_t = [fmpz_mpoly_factor_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpolyv_struct {
    pub coeffs: *mut fmpz_mpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_mpolyv_t = [fmpz_mpolyv_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_pfrac_struct {
    pub bits: mp_limb_t,
    pub w: mp_limb_signed_t,
    pub r: mp_limb_signed_t,
    pub inv_prod_dbetas: *mut fmpq_poly_struct,
    pub dbetas: *mut fmpq_poly_struct,
    pub prod_mbetas: *mut fmpz_mpoly_struct,
    pub prod_mbetas_coeffs: *mut fmpz_mpolyv_struct,
    pub mbetas: *mut fmpz_mpoly_struct,
    pub deltas: *mut fmpz_mpoly_struct,
    pub xalpha: *mut fmpz_mpoly_struct,
    pub q: *mut fmpz_mpoly_struct,
    pub U: *mut fmpz_mpoly_univar_struct,
    pub G: *mut fmpz_mpoly_geobucket_struct,
    pub qt: *mut fmpz_mpoly_struct,
    pub newt: *mut fmpz_mpoly_struct,
    pub delta_coeffs: *mut fmpz_mpolyv_struct,
    pub dtq: fmpq_poly_t,
    pub S: fmpq_poly_t,
    pub R: fmpq_poly_t,
}

pub type fmpz_mpoly_pfrac_t = [fmpz_mpoly_pfrac_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_bpoly_struct {
    pub coeffs: *mut fmpz_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_bpoly_t = [fmpz_bpoly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_tpoly_struct {
    pub coeffs: *mut fmpz_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_tpoly_t = [fmpz_tpoly_struct; 1usize];

extern "C" {
    pub fn fmpz_mpoly_fit_length_set_bits(
        A: *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_is_fmpz_poly(
        A: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_get_fmpz_poly(
        A: *mut fmpz_poly_struct,
        B: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_set_fmpz_poly(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        Bcoeffs: *const fmpz,
        Blen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_fmpz_poly(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_poly_struct,
        v: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn tuple_print(alpha: *mut fmpz, n: mp_limb_signed_t);
    pub fn tuple_saturate(alpha: *mut fmpz, n: mp_limb_signed_t, m: mp_limb_signed_t);
    pub fn tuple_next(alpha: *mut fmpz, n: mp_limb_signed_t);
    pub fn fmpz_mpoly_factor_init(
        f: *mut fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_init2(
        f: *mut fmpz_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_realloc(
        f: *mut fmpz_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_fit_length(
        f: *mut fmpz_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_clear(
        f: *mut fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_length(
        f: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_factor_get_constant_fmpz(
        c: *mut fmpz,
        f: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_get_constant_fmpq(
        c: *mut fmpq,
        f: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_get_base(
        p: *mut fmpz_mpoly_struct,
        f: *const fmpz_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_swap_base(
        p: *mut fmpz_mpoly_struct,
        f: *const fmpz_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_get_exp_si(
        f: *const fmpz_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_factor_set(
        f: *mut fmpz_mpoly_factor_struct,
        g: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_cmp(
        f: *const fmpz_mpoly_factor_struct,
        g: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_print_pretty(
        f: *const fmpz_mpoly_factor_struct,
        vars: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_content(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_squarefree(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_swap(
        f: *mut fmpz_mpoly_factor_struct,
        g: *mut fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_set_fmpz(
        f: *mut fmpz_mpoly_factor_struct,
        a: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_zero(
        f: *mut fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_one(f: *mut fmpz_mpoly_factor_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_factor_sort(
        f: *mut fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_expand(
        A: *mut fmpz_mpoly_struct,
        f: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_bound_si(
        B: *mut fmpz,
        A: *const fmpz,
        degs: *const mp_limb_signed_t,
        nvars: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_matches(
        A: *const fmpz_mpoly_struct,
        f: *const fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_append_fmpz_swap(
        f: *mut fmpz_mpoly_factor_struct,
        A: *mut fmpz_mpoly_struct,
        e: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_append_ui(
        f: *mut fmpz_mpoly_factor_struct,
        A: *mut fmpz_mpoly_struct,
        e: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyv_init(A: *mut fmpz_mpolyv_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpolyv_swap(
        A: *mut fmpz_mpolyv_struct,
        B: *mut fmpz_mpolyv_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyv_clear(A: *mut fmpz_mpolyv_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpolyv_print_pretty(
        poly: *const fmpz_mpolyv_struct,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyv_fit_length(
        A: *mut fmpz_mpolyv_struct,
        length: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyv_set_coeff(
        A: *mut fmpz_mpolyv_struct,
        i: mp_limb_signed_t,
        c: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_to_mpolyv(
        A: *mut fmpz_mpolyv_struct,
        B: *const fmpz_mpoly_struct,
        xalpha: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_from_mpolyv(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        B: *const fmpz_mpolyv_struct,
        xalpha: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_vec_content_mpoly(
        g: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        Alen: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_pfrac_init(
        I: *mut fmpz_mpoly_pfrac_struct,
        bits: mp_limb_t,
        r: mp_limb_signed_t,
        w: mp_limb_signed_t,
        betas: *const fmpz_mpoly_struct,
        alpha: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_pfrac_clear(I: *mut fmpz_mpoly_pfrac_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_pfrac(
        l: mp_limb_signed_t,
        t: *mut fmpz_mpoly_struct,
        degs: *const mp_limb_signed_t,
        I: *const fmpz_mpoly_pfrac_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_hlift(
        m: mp_limb_signed_t,
        f: *mut fmpz_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fmpz,
        A: *const fmpz_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_get_lead0(
        c: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_set_lead0(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_bpoly_init(A: *mut fmpz_bpoly_struct);
    pub fn fmpz_bpoly_swap(A: *mut fmpz_bpoly_struct, B: *mut fmpz_bpoly_struct);
    pub fn fmpz_bpoly_clear(A: *mut fmpz_bpoly_struct);
    pub fn fmpz_bpoly_realloc(A: *mut fmpz_bpoly_struct, len: mp_limb_signed_t);
    pub fn fmpz_bpoly_fit_length(A: *mut fmpz_bpoly_struct, len: mp_limb_signed_t);
    pub fn fmpz_bpoly_print_pretty(
        A: *const fmpz_bpoly_struct,
        var0: *const c_char,
        var1: *const c_char,
    );
    pub fn fmpz_bpoly_lead(A: *const fmpz_bpoly_struct) -> *mut fmpz_poly_struct;
    pub fn fmpz_bpoly_zero(A: *mut fmpz_bpoly_struct);
    pub fn fmpz_bpoly_degree0(A: *const fmpz_bpoly_struct) -> mp_limb_signed_t;
    pub fn fmpz_bpoly_degree1(A: *const fmpz_bpoly_struct) -> mp_limb_signed_t;
    pub fn fmpz_bpoly_set_coeff(
        A: *mut fmpz_bpoly_struct,
        exp0: mp_limb_signed_t,
        exp1: mp_limb_signed_t,
        c: *const fmpz,
    );
    pub fn fmpz_mpoly_set_fmpz_bpoly(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        B: *const fmpz_bpoly_struct,
        var0: mp_limb_signed_t,
        var1: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_bpoly(
        A: *mut fmpz_bpoly_struct,
        B: *const fmpz_mpoly_struct,
        var0: mp_limb_signed_t,
        var1: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_tpoly_init(A: *mut fmpz_tpoly_struct);
    pub fn fmpz_tpoly_swap(A: *mut fmpz_tpoly_struct, B: *mut fmpz_tpoly_struct);
    pub fn fmpz_tpoly_fit_length(A: *mut fmpz_tpoly_struct, len: mp_limb_signed_t);
    pub fn fmpz_tpoly_clear(A: *mut fmpz_tpoly_struct);
    pub fn fmpz_bpoly_factor(
        c: *mut fmpz_poly_struct,
        F: *mut fmpz_tpoly_struct,
        B: *const fmpz_bpoly_struct,
    );
    pub fn fmpz_bpoly_factor_ordered(
        c: *mut fmpz_poly_struct,
        F: *mut fmpz_tpoly_struct,
        B: *const fmpz_bpoly_struct,
        alpha: *const fmpz,
        Bevalf: *const fmpz_poly_factor_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_unit_normalize(A: *mut fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn _fmpz_mpoly_factor_squarefree(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        e: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_lcc_wang(
        lc_divs: *mut fmpz_mpoly_struct,
        lcAfac: *mut fmpz_mpoly_factor_struct,
        Auc: *const fmpz,
        Auf: *const fmpz_poly_struct,
        r: mp_limb_signed_t,
        alpha: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_irred_zassenhaus(
        fac: *mut fmpz_mpolyv_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        Z: *const zassenhaus_prune_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_irred_wang(
        fac: *mut fmpz_mpolyv_struct,
        A: *const fmpz_mpoly_struct,
        lcAfac: *const fmpz_mpoly_factor_struct,
        lcAfac_irred: c_int,
        lcA: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        state: *const flint_rand_s,
        Z: *const zassenhaus_prune_struct,
        allow_shift: c_int,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_irred_zippel(
        fac: *mut fmpz_mpolyv_struct,
        A: *const fmpz_mpoly_struct,
        lcAfac: *const fmpz_mpoly_factor_struct,
        lcAfac_irred: c_int,
        lcA: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        state: *const flint_rand_s,
        Z: *const zassenhaus_prune_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_irred(
        f: *mut fmpz_mpoly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        algo: c_uint,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_zassenhaus(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_wang(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_zippel(
        f: *mut fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_evaluate_rest_fmpz(
        E: *mut fmpz,
        starts: *mut mp_limb_signed_t,
        ends: *mut mp_limb_signed_t,
        stops: *mut mp_limb_signed_t,
        es: *mut mp_limb_t,
        Acoeffs: *const fmpz,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        alphas: *const fmpz,
        offsets: *const mp_limb_signed_t,
        shifts: *const mp_limb_signed_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
        nvars: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_mpoly_eval_rest_to_poly(
        E: *mut fmpz_poly_struct,
        A: *const fmpz_mpoly_struct,
        alphas: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_factor_lcc_kaltofen_step(
        divs: *mut fmpz_mpoly_struct,
        r: mp_limb_signed_t,
        Af: *mut fmpz_mpoly_factor_struct,
        Au: *const fmpz_poly_struct,
        v: mp_limb_signed_t,
        alphas: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_factor_lcc_kaltofen(
        divs: *mut fmpz_mpoly_struct,
        lcAf_: *const fmpz_mpoly_factor_struct,
        A: *const fmpz_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fmpz,
        degs: *const mp_limb_signed_t,
        uf: *const fmpz_poly_factor_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_evaluate_rest_except_one(
        e: *mut fmpz_poly_struct,
        A: *const fmpz_mpoly_struct,
        alphas: *const fmpz,
        v: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_compression_do(
        L: *mut fmpz_mpoly_struct,
        Lctx: *const fmpz_mpoly_ctx_struct,
        Acoeffs: *const fmpz,
        Alen: mp_limb_signed_t,
        M: *const mpoly_compression_struct,
    );
    pub fn fmpz_mpoly_compression_undo(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        Actx: *const fmpz_mpoly_ctx_struct,
        L: *const fmpz_mpoly_struct,
        Lctx: *const fmpz_mpoly_ctx_struct,
        M: *const mpoly_compression_struct,
    );
}
