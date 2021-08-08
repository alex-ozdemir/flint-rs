#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_mpoly.html).


use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::mpoly::{ordering_t, mpoly_ctx_struct, mpoly_ctx_t, mpoly_gcd_info_struct, mpoly_zipinfo_struct};
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmpz_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
}

pub type fmpz_mpoly_ctx_t = [fmpz_mpoly_ctx_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_struct {
    pub coeffs: *mut fmpz,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}

pub type fmpz_mpoly_t = [fmpz_mpoly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_univar_struct {
    pub coeffs: *mut fmpz_mpoly_struct,
    pub exps: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_mpoly_univar_t = [fmpz_mpoly_univar_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpolyu_struct {
    pub coeffs: *mut fmpz_mpoly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type fmpz_mpolyu_t = [fmpz_mpolyu_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpolyd_struct {
    pub nvars: mp_limb_signed_t,
    pub degb_alloc: mp_limb_signed_t,
    pub deg_bounds: *mut mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub coeff_alloc: mp_limb_signed_t,
    pub coeffs: *mut fmpz,
}

pub type fmpz_mpolyd_t = [fmpz_mpolyd_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _fmpz_mpoly_stripe_struct {
    pub big_mem: *mut c_char,
    pub big_mem_alloc: mp_limb_signed_t,
    pub N: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub cmpmask: *const mp_limb_t,
    pub startidx: *mut mp_limb_signed_t,
    pub endidx: *mut mp_limb_signed_t,
    pub emin: *mut mp_limb_t,
    pub emax: *mut mp_limb_t,
    pub coeff_bits: mp_limb_t,
    pub upperclosed: c_int,
    pub flint_small: c_int,
}

pub type fmpz_mpoly_stripe_struct = _fmpz_mpoly_stripe_struct;
pub type fmpz_mpoly_stripe_t = [fmpz_mpoly_stripe_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpolyd_ctx_struct {
    pub nvars: mp_limb_signed_t,
    pub perm: *mut mp_limb_signed_t,
}

pub type fmpz_mpolyd_ctx_t = [fmpz_mpolyd_ctx_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_geobucket {
    pub polys: [fmpz_mpoly_struct; 32usize],
    pub temps: [fmpz_mpoly_struct; 32usize],
    pub length: mp_limb_signed_t,
}

pub type fmpz_mpoly_geobucket_struct = fmpz_mpoly_geobucket;
pub type fmpz_mpoly_geobucket_t = [fmpz_mpoly_geobucket_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpolyc_struct {
    pub coeffs: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_mpolyc_t = [fmpz_mpolyc_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpolycu_struct {
    pub coeffs: *mut fmpz_mpolyc_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_mpolycu_t = [fmpz_mpolycu_struct; 1usize];

extern "C" {
    pub fn fmpz_mpoly_term_coeff_ref(
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> *mut fmpz;
    pub fn fmpz_mpoly_ctx_init(
        ctx: *mut fmpz_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
    );
    pub fn fmpz_mpoly_ctx_init_rand(
        mctx: *mut fmpz_mpoly_ctx_struct,
        state: *const flint_rand_s,
        max_nvars: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_ctx_clear(ctx: *mut fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_ctx_nvars(ctx: *const fmpz_mpoly_ctx_struct) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_ctx_ord(ctx: *const fmpz_mpoly_ctx_struct) -> ordering_t;
    pub fn fmpz_mpoly_init(A: *mut fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_init2(
        A: *mut fmpz_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_init3(
        A: *mut fmpz_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_realloc(
        Acoeff: *mut *mut fmpz,
        Aexp: *mut *mut mp_limb_t,
        Aalloc: *mut mp_limb_signed_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_realloc(
        A: *mut fmpz_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_fit_length(
        Acoeff: *mut *mut fmpz,
        Aexp: *mut *mut mp_limb_t,
        Aalloc: *mut mp_limb_signed_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_fit_length(
        A: *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_fit_length_reset_bits(
        A: *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_clear(A: *mut fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn _fmpz_mpoly_set_length(
        A: *mut fmpz_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_truncate(
        A: *mut fmpz_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_fit_bits(
        A: *mut fmpz_mpoly_struct,
        bits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_str_pretty(
        A: *mut fmpz_mpoly_struct,
        str_: *const c_char,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_get_str_pretty(
        poly: *const fmpz,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        x: *const *const c_char,
        bits: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
    ) -> *mut c_char;
    pub fn fmpz_mpoly_get_str_pretty(
        A: *const fmpz_mpoly_struct,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> *mut c_char;
    pub fn _fmpz_mpoly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpz,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        x_in: *const *const c_char,
        bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_fprint_pretty(
        file: *mut FILE,
        A: *const fmpz_mpoly_struct,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_print_pretty(
        poly: *const fmpz,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        x: *const *const c_char,
        bits: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_print_pretty(
        A: *const fmpz_mpoly_struct,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gen(
        poly: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_is_gen(
        poly: *const fmpz_mpoly_struct,
        k: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_set(
        poly1: *mut fmpz,
        exps1: *mut mp_limb_t,
        poly2: *const fmpz,
        exps2: *const mp_limb_t,
        n: mp_limb_signed_t,
        N: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_set(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_equal(
        poly1: *const fmpz,
        exps1: *const mp_limb_t,
        poly2: *const fmpz,
        exps2: *const mp_limb_t,
        n: mp_limb_signed_t,
        N: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_equal(
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_swap(
        A: *mut fmpz_mpoly_struct,
        B: *mut fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_fits_small(
        poly: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_max_bits(A: *const fmpz_mpoly_struct) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_is_fmpz(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_get_fmpz(
        c: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_ui(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_si(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_zero(A: *mut fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_one(A: *mut fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_equal_fmpz(
        A: *const fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_equal_ui(
        A: *const fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_equal_si(
        A: *const fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_is_zero(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_is_one(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_degrees_fit_si(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_degrees_fmpz(
        degs: *mut *mut fmpz,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_degree_fmpz(
        deg: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_degree_si(
        A: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_total_degree_fits_si(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_total_degree_fmpz(
        td: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_total_degree_si(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_get_coeff_fmpz_monomial(
        c: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        M: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_fmpz_monomial(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        M: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_coeff_fmpz_fmpz(
        c: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_coeff_ui_fmpz(
        A: *const fmpz_mpoly_struct,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_t;
    pub fn fmpz_mpoly_get_coeff_si_fmpz(
        A: *const fmpz_mpoly_struct,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_get_coeff_fmpz_ui(
        c: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_coeff_ui_ui(
        A: *const fmpz_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_t;
    pub fn fmpz_mpoly_get_coeff_si_ui(
        A: *const fmpz_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_mpoly_set_coeff_fmpz_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        exp: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_fmpz_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        exp: *const *mut fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_ui_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_t,
        exp: *const *mut fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_si_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_fmpz_ui(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_ui_ui(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_coeff_si_ui(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_coeff_vars_ui(
        C: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        vars: *const mp_limb_signed_t,
        exps: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_cmp(
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_is_canonical(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_length(
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_resize(
        A: *mut fmpz_mpoly_struct,
        new_length: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term_coeff_fmpz(
        c: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term_coeff_ui(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_t;
    pub fn fmpz_mpoly_get_term_coeff_si(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_set_term_coeff_fmpz(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_term_coeff_ui(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_term_coeff_si(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_term_exp_fits_ui(
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_term_exp_fits_si(
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_get_term_exp_fmpz(
        exp: *mut *mut fmpz,
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term_exp_ui(
        exp: *mut mp_limb_t,
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term_exp_si(
        exp: *mut mp_limb_signed_t,
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term_var_exp_ui(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_t;
    pub fn fmpz_mpoly_get_term_var_exp_si(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_set_term_exp_fmpz(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_set_term_exp_ui(
        A: *mut fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term(
        M: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_get_term_monomial(
        M: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_push_term_fmpz_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_push_term_ui_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_t,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_push_term_si_fmpz(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_push_term_fmpz_ui(
        A: *mut fmpz_mpoly_struct,
        c: *const fmpz,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_push_term_ui_ui(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_push_term_si_ui(
        A: *mut fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_sort_terms(A: *mut fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_combine_like_terms(
        A: *mut fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_reverse(
        A: *mut fmpz_mpoly_struct,
        B: *mut fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_assert_canonical(A: *const fmpz_mpoly_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn _fmpz_mpoly_radix_sort1(
        A: *mut fmpz_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        cmpmask: mp_limb_t,
        totalmask: mp_limb_t,
    );
    pub fn _fmpz_mpoly_radix_sort(
        A: *mut fmpz_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *mut mp_limb_t,
    );
    pub fn _fmpz_mpoly_push_exp_ffmpz(
        A: *mut fmpz_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_push_exp_pfmpz(
        A: *mut fmpz_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_push_exp_ui(
        A: *mut fmpz_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_randtest_bound(
        A: *mut fmpz_mpoly_struct,
        state: *const flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bound: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_randtest_bounds(
        A: *mut fmpz_mpoly_struct,
        state: *const flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bounds: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_randtest_bits(
        A: *mut fmpz_mpoly_struct,
        state: *const flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_add_fmpz(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_add_ui(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_add_si(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_sub_fmpz(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_sub_ui(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_sub_si(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_add(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_add(
        poly1: *mut fmpz,
        exps1: *mut mp_limb_t,
        poly2: *const fmpz,
        exps2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exps3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_sub(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_sub(
        poly1: *mut fmpz,
        exps1: *mut mp_limb_t,
        poly2: *const fmpz,
        exps2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exps3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_neg(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_mul_fmpz(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_mul_si(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_mul_ui(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_fmma(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz,
        D: *const fmpz_mpoly_struct,
        e: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_divexact_fmpz(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_divexact_si(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_divexact_ui(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_scalar_divides_fmpz(
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_scalar_divides_si(
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_scalar_divides_ui(
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_derivative(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_integral(
        A: *mut fmpz_mpoly_struct,
        scale: *const fmpz,
        B: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_pow_ui_is_not_feasible(bbits: mp_limb_t, e: mp_limb_t) -> c_int;
    pub fn _fmpz_pow_fmpz_is_not_feasible(bbits: mp_limb_t, e: *const fmpz) -> c_int;
    pub fn fmpz_mpoly_evaluate_all_fmpz(
        ev: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        vals: *const *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_evaluate_one_fmpz(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        val: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_compose_fmpz_poly(
        A: *mut fmpz_poly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const *const fmpz_poly_struct,
        ctxB: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_compose_mat(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        M: *const fmpz_mat_struct,
        ctxB: *const fmpz_mpoly_ctx_struct,
        ctxAC: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_compose_fmpz_mpoly_geobucket(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const *const fmpz_mpoly_struct,
        ctxB: *const fmpz_mpoly_ctx_struct,
        ctxAC: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_compose_fmpz_mpoly_horner(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const *const fmpz_mpoly_struct,
        ctxB: *const fmpz_mpoly_ctx_struct,
        ctxAC: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_compose_fmpz_mpoly(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const *const fmpz_mpoly_struct,
        ctxB: *const fmpz_mpoly_ctx_struct,
        ctxAC: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_compose_fmpz_mpoly_gen(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        c: *const mp_limb_signed_t,
        ctxB: *const fmpz_mpoly_ctx_struct,
        ctxAC: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_mul(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_mul_johnson(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_mul_heap_threaded(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_mul_array(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_mul_array_threaded(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_mul_dense(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        C: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_mul_johnson(
        poly1: *mut *const fmpz,
        exp1: *mut *mut mp_limb_t,
        alloc: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_mpoly_mul_johnson_maxfields(
        A: *mut fmpz_mpoly_struct,
        B: *mut fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        C: *const fmpz_mpoly_struct,
        maxCfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_mul_heap_threaded_pool_maxfields(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        C: *const fmpz_mpoly_struct,
        maxCfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
    pub fn _fmpz_mpoly_mul_array_DEG(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        C: *const fmpz_mpoly_struct,
        maxCfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_mul_array_LEX(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        C: *const fmpz_mpoly_struct,
        maxCfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_mul_array_threaded_pool_DEG(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        C: *const fmpz_mpoly_struct,
        maxCfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_mpoly_mul_array_threaded_pool_LEX(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        C: *const fmpz_mpoly_struct,
        maxCfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_mpoly_mul_dense(
        P: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        maxAfields: *const fmpz,
        B: *const fmpz_mpoly_struct,
        maxBfields: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_pow_fmpz(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        k: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_pow_ui(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        k: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_pow_fps(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        k: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_pow_fps(
        poly1: *mut *mut fmpz,
        exp1: *mut *mut mp_limb_t,
        alloc: *const mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        k: mp_limb_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_divides(
        Q: *const fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_divides_monagan_pearce(
        Q: *const fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_divides_heap_threaded(
        Q: *const fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_divides_heap_threaded_pool(
        Q: *const fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_mpoly_divides_array(
        poly1: *const *const fmpz,
        exp1: *const *const mp_limb_t,
        alloc: *const mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        mults: *const mp_limb_signed_t,
        num: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_divides_array(
        poly1: *const fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn mpoly_divides_select_exps(
        S: *const fmpz_mpoly_struct,
        zctx: *const fmpz_mpoly_ctx_struct,
        nworkers: mp_limb_signed_t,
        Aexp: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Bexp: *const mp_limb_t,
        Blen: mp_limb_signed_t,
        bits: mp_limb_t,
    ) -> c_int;
    pub fn _fmpz_mpoly_divides_monagan_pearce(
        poly1: *mut *mut fmpz,
        exp1: *mut *mut mp_limb_t,
        alloc: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_divrem(
        Q: *mut fmpz_mpoly_struct,
        R: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_quasidivrem(
        scale: *mut fmpz,
        Q: *mut fmpz_mpoly_struct,
        R: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_div(
        Q: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_quasidiv(
        scale: *mut fmpz,
        Q: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_divrem_ideal(
        Q: *mut *mut fmpz_mpoly_struct,
        R: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_quasidivrem_ideal(
        scale: *mut fmpz,
        Q: *mut *mut fmpz_mpoly_struct,
        R: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_div_monagan_pearce(
        polyq: *mut *mut fmpz,
        expq: *mut *mut mp_limb_t,
        allocq: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_div_monagan_pearce(
        q: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_divrem_monagan_pearce(
        lenr: *mut mp_limb_signed_t,
        polyq: *mut *mut fmpz,
        expq: *mut *mut mp_limb_t,
        allocq: *mut mp_limb_signed_t,
        polyr: *mut *mut fmpz,
        expr: *mut *mut mp_limb_t,
        allocr: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_divrem_monagan_pearce(
        q: *mut fmpz_mpoly_struct,
        r: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_divrem_array(
        lenr: *mut mp_limb_signed_t,
        polyq: *mut *mut fmpz,
        expq: *mut *mut mp_limb_t,
        allocq: *mut mp_limb_signed_t,
        polyr: *mut *mut fmpz,
        expr: *mut *mut mp_limb_t,
        allocr: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const fmpz,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        mults: *const mp_limb_signed_t,
        num: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_divrem_array(
        q: *mut fmpz_mpoly_struct,
        r: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_quasidivrem_heap(
        scale: *mut fmpz,
        q: *mut fmpz_mpoly_struct,
        r: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_quasidiv_heap(
        scale: *mut fmpz,
        q: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_divrem_ideal_monagan_pearce(
        polyq: *mut *mut fmpz_mpoly_struct,
        polyr: *mut *mut fmpz,
        expr: *mut *mut mp_limb_t,
        allocr: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const *mut fmpz_mpoly_struct,
        exp3: *const *mut mp_limb_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        ctx: *mut fmpz_mpoly_ctx_struct,
        cmpmask: *const mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_divrem_ideal_monagan_pearce(
        q: *mut *mut fmpz_mpoly_struct,
        r: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_quasidivrem_ideal_heap(
        scale: *mut fmpz,
        q: *mut *mut fmpz_mpoly_struct,
        r: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        poly3: *const *mut fmpz_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_sqrt_heap(
        polyq: *mut *mut fmpz,
        expq: *mut *mut mp_limb_t,
        allocq: *mut mp_limb_signed_t,
        poly2: *const fmpz,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        check: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_sqrt_heap(
        q: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        check: c_int,
    ) -> c_int;
    pub fn fmpz_mpoly_sqrt(
        q: *mut fmpz_mpoly_struct,
        poly2: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_is_square(
        poly2: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_term_content(
        M: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_content_vars(
        g: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        vars: *const mp_limb_signed_t,
        vars_length: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_gcd_threaded_pool(
        G: *mut fmpz_mpoly_struct,
        Gbits: mp_limb_t,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd_cofactors(
        G: *mut fmpz_mpoly_struct,
        Abar: *mut fmpz_mpoly_struct,
        Bbar: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_gcd_cofactors_threaded_pool(
        G: *mut fmpz_mpoly_struct,
        Gbits: mp_limb_t,
        Abar: *mut fmpz_mpoly_struct,
        Abarbits: mp_limb_t,
        Bbar: *mut fmpz_mpoly_struct,
        Bbarbits: mp_limb_t,
        A: *mut fmpz_mpoly_struct,
        B: *mut fmpz_mpoly_struct,
        ctx: *mut fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_deflation(
        shift: *mut fmpz,
        stride: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_deflate(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_inflate(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_gcd_prs(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd_brown(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd_zippel(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd_berlekamp_massey(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd_brown_threaded(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_gcd_berlekamp_massey_threaded(
        G: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_univar_init(
        A: *mut fmpz_mpoly_univar_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_clear(
        A: *mut fmpz_mpoly_univar_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_fit_length(
        A: *mut fmpz_mpoly_univar_struct,
        length: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_print_pretty(
        A: *const fmpz_mpoly_univar_struct,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_assert_canonical(
        A: *const fmpz_mpoly_univar_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_to_univar(
        A: *mut fmpz_mpoly_univar_struct,
        B: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_from_univar_bits(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        B: *const fmpz_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_from_univar(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_swap(
        A: *mut fmpz_mpoly_univar_struct,
        B: *mut fmpz_mpoly_univar_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_degree_fits_si(
        A: *const fmpz_mpoly_univar_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_univar_length(
        A: *const fmpz_mpoly_univar_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_univar_get_term_exp_si(
        A: *const fmpz_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mpoly_univar_get_term_coeff(
        c: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_univar_swap_term_coeff(
        c: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyl_lead_coeff(
        c: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyl_content(
        g: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mpoly_to_fmpz_poly_deflate(
        A: *mut fmpz_poly_struct,
        B: *const fmpz_mpoly_struct,
        var: mp_limb_signed_t,
        Bshift: *const mp_limb_t,
        Bstride: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_from_fmpz_poly_inflate(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        B: *const fmpz_poly_struct,
        var: mp_limb_signed_t,
        Ashift: *const mp_limb_t,
        Astride: *const mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_repack_bits(
        A: *mut fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpolyd_init(poly: *mut fmpz_mpolyd_struct, nvars: mp_limb_signed_t);
    pub fn fmpz_mpolyd_fit_length(poly: *mut fmpz_mpolyd_struct, len: mp_limb_signed_t);
    pub fn fmpz_mpolyd_clear(poly: *mut fmpz_mpolyd_struct);
    pub fn fmpz_mpolyu_is_canonical(
        A: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpolyu_init(
        A: *mut fmpz_mpolyu_struct,
        bits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_clear(A: *mut fmpz_mpolyu_struct, uctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpolyu_swap(
        A: *mut fmpz_mpolyu_struct,
        B: *mut fmpz_mpolyu_struct,
        uctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_zero(A: *mut fmpz_mpolyu_struct, uctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpolyu_print_pretty(
        poly: *const fmpz_mpolyu_struct,
        x: *const *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyuu_print_pretty(
        poly: *const fmpz_mpolyu_struct,
        x: *const *const c_char,
        nmainvars: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_fit_length(
        A: *mut fmpz_mpolyu_struct,
        length: mp_limb_signed_t,
        uctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_one(A: *mut fmpz_mpolyu_struct, uctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpolyu_equal_upto_unit(
        A: *const fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpolyu_inner_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_set(
        A: *mut fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        uctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_to_mpoly_perm_deflate_threaded_pool(
        A: *mut fmpz_mpoly_struct,
        lctx: *const fmpz_mpoly_ctx_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_from_mpoly_perm_inflate(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
        B: *const fmpz_mpoly_struct,
        lctx: *const fmpz_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fmpz_mpoly_to_mpolyu_perm_deflate_threaded_pool(
        A: *mut fmpz_mpolyu_struct,
        uctx: *mut fmpz_mpoly_ctx_struct,
        B: *mut fmpz_mpoly_struct,
        ctx: *mut fmpz_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
        maxexps: *const mp_limb_t,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_from_mpolyu_perm_inflate(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
        B: *const fmpz_mpolyu_struct,
        uctx: *const fmpz_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fmpz_mpoly_to_mpolyuu_perm_deflate_threaded_pool(
        A: *mut fmpz_mpolyu_struct,
        uctx: *const fmpz_mpoly_ctx_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
        maxexps: *const mp_limb_t,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
    pub fn fmpz_mpoly_from_mpolyuu_perm_inflate(
        A: *mut fmpz_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
        B: *const fmpz_mpolyu_struct,
        uctx: *const fmpz_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
    pub fn fmpz_mpolyuu_divides(
        Q: *mut fmpz_mpolyu_struct,
        A: *const fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        main_nvars: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpolyuu_divides_threaded_pool(
        Q: *mut fmpz_mpolyu_struct,
        A: *const fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        main_nvars: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpolyu_fmpz_content(
        c: *mut fmpz,
        A: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_mul_fmpz(
        A: *mut fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_divexact_fmpz(
        A: *mut fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_divexact_mpoly(
        A: *mut fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        saveB: c_int,
        c: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_divexact_mpoly_inplace(
        A: *mut fmpz_mpolyu_struct,
        c: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_mul_mpoly(
        A: *mut fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        c: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_mul_mpoly_inplace(
        A: *mut fmpz_mpolyu_struct,
        c: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_shift_right(A: *mut fmpz_mpolyu_struct, s: mp_limb_t);
    pub fn fmpz_mpolyu_shift_left(A: *mut fmpz_mpolyu_struct, s: mp_limb_t);
    pub fn fmpz_mpolyu_content_fmpz(
        g: *mut fmpz,
        A: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_content_mpoly_threaded_pool(
        g: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_height(
        max: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_heights(
        max: *mut fmpz,
        sum: *mut fmpz,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyl_gcd_brown(
        G: *mut fmpz_mpoly_struct,
        Abar: *mut fmpz_mpoly_struct,
        Bbar: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        I: *const mpoly_gcd_info_struct,
    ) -> c_int;
    pub fn fmpz_mpolyl_gcd_brown_threaded_pool(
        G: *mut fmpz_mpoly_struct,
        Abar: *mut fmpz_mpoly_struct,
        Bbar: *mut fmpz_mpoly_struct,
        A: *const fmpz_mpoly_struct,
        B: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        I: *const mpoly_gcd_info_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpolyu_gcdm_zippel(
        G: *mut fmpz_mpolyu_struct,
        Abar: *mut fmpz_mpolyu_struct,
        Bbar: *mut fmpz_mpolyu_struct,
        A: *const fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        zinfo: *const mpoly_zipinfo_struct,
        randstate: *const flint_rand_s,
    ) -> c_int;
    pub fn fmpz_mpolyuu_gcd_berlekamp_massey(
        G: *mut fmpz_mpolyu_struct,
        Abar: *mut fmpz_mpolyu_struct,
        Bbar: *mut fmpz_mpolyu_struct,
        A: *const fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        Gamma: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpolyuu_gcd_berlekamp_massey_threaded_pool(
        G: *mut fmpz_mpolyu_struct,
        Abar: *mut fmpz_mpolyu_struct,
        Bbar: *mut fmpz_mpolyu_struct,
        A: *const fmpz_mpolyu_struct,
        B: *const fmpz_mpolyu_struct,
        Gamma: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mpoly_leadcoeff(A: *const fmpz_mpoly_struct) -> *mut fmpz;
    pub fn fmpz_mpolyu_leadcoeff(A: *const fmpz_mpolyu_struct) -> *mut fmpz;
    /*
    pub fn fmpz_mpolyu_interp_reduce_p(
        Ap: *mut nmod_mpolyu_struct,
        ctxp: *mut nmod_mpoly_ctx_struct,
        A: *const fmpz_mpolyu_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_interp_lift_p(
        A: *mut fmpz_mpolyu_struct,
        ctx: *mut fmpz_mpoly_ctx_struct,
        Ap: *const nmod_mpolyu_struct,
        ctxp: *const nmod_mpoly_ctx_struct,
    );
    pub fn fmpz_mpolyu_interp_mcrt_p(
        coeffbits: *mut mp_limb_t,
        H: *mut fmpz_mpolyu_struct,
        ctx: *mut fmpz_mpoly_ctx_struct,
        m: *const fmpz,
        A: *const nmod_mpolyu_struct,
        ctxp: *const nmod_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_interp_reduce_p_mpolyn(
        E: *mut nmod_mpolyn_struct,
        pctx: *mut nmod_mpoly_ctx_struct,
        A: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_interp_lift_p_mpolyn(
        A: *mut fmpz_mpoly_struct,
        ctx: *mut fmpz_mpoly_ctx_struct,
        B: *const nmod_mpolyn_struct,
        pctx: *const nmod_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_interp_crt_p_mpolyn(
        F: *mut fmpz_mpoly_struct,
        T: *mut fmpz_mpoly_struct,
        ctx: *mut fmpz_mpoly_ctx_struct,
        modulus: *const fmpz,
        A: *const nmod_mpolyn_struct,
        pctx: *const nmod_mpoly_ctx_struct,
    ) -> c_int;
    */
    pub fn fmpz_mpoly_geobucket_init(
        B: *mut fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_clear(
        B: *mut fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_empty(
        p: *mut fmpz_mpoly_struct,
        B: *mut fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_fit_length(
        B: *mut fmpz_mpoly_geobucket_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_set(
        B: *mut fmpz_mpoly_geobucket_struct,
        p: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_add(
        B: *mut fmpz_mpoly_geobucket_struct,
        p: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_sub(
        B: *mut fmpz_mpoly_geobucket_struct,
        p: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_set_fmpz(
        B: *mut fmpz_mpoly_geobucket_struct,
        c: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_gen(
        B: *mut fmpz_mpoly_geobucket_struct,
        var: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_add_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        B2: *const fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_sub_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        B2: *const fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_neg_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_mul_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        B2: *const fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_pow_ui_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        k: mp_limb_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_pow_fmpz_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        k: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_geobucket_divides_inplace(
        B1: *mut fmpz_mpoly_geobucket_struct,
        B2: *const fmpz_mpoly_geobucket_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpolyc_init(A: *mut fmpz_mpolyc_struct);
    pub fn fmpz_mpolyc_clear(A: *mut fmpz_mpolyc_struct);
    pub fn fmpz_mpolyc_fit_length(A: *mut fmpz_mpolyc_struct, length: mp_limb_signed_t);
    pub fn fmpz_mpolycu_init(A: *mut fmpz_mpolycu_struct);
    pub fn fmpz_mpolycu_clear(A: *mut fmpz_mpolycu_struct);
    pub fn fmpz_mpolycu_fit_length(A: *mut fmpz_mpolycu_struct, length: mp_limb_signed_t);
}
