#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_mpoly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fmpz_mod::{fmpz_mod_ctx_struct, fmpz_mod_ctx_t};
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::mpoly::*;
use crate::nmod_mpoly::*;
use libc::FILE;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub ffinfo: fmpz_mod_ctx_t,
}
pub type fmpz_mod_mpoly_ctx_t = [fmpz_mod_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_struct {
    pub coeffs: *mut fmpz,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub coeffs_alloc: mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
}
pub type fmpz_mod_mpoly_t = [fmpz_mod_mpoly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_univar_struct {
    pub coeffs: *mut fmpz_mod_mpoly_struct,
    pub exps: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpz_mod_mpoly_univar_t = [fmpz_mod_mpoly_univar_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_init(
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
        modulus: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_init_rand(
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
        modulus: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_init_rand_bits_prime(
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
        max_bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_init_rand_bits(
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
        max_bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_clear(ctx: *mut fmpz_mod_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpoly_clear(A: *mut fmpz_mod_mpoly_struct, ctx: *mut fmpz_mod_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpoly_init2(
        A: *mut fmpz_mod_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_init3(
        A: *mut fmpz_mod_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_realloc(
        A: *mut fmpz_mod_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_fit_length(
        A: *mut fmpz_mod_mpoly_struct,
        length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_fit_length_fit_bits(
        A: *mut fmpz_mod_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_fit_length_reset_bits(
        A: *mut fmpz_mod_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_str_pretty(
        A: *mut fmpz_mod_mpoly_struct,
        str_: *const ::std::os::raw::c_char,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_str_pretty(
        A: *mut fmpz_mod_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmpz_mod_mpoly_fprint_pretty(
        file: *mut FILE,
        A: *mut fmpz_mod_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gen(
        A: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_is_gen(
        A: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_set(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_equal(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_is_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_fmpz(
        c: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_fmpz_mod(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_si(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_equal_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_equal_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_equal_si(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_coeff_fmpz_monomial(
        c: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        M: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_fmpz_monomial(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        M: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_coeff_fmpz_fmpz(
        c: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_coeff_fmpz_ui(
        c: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_set_coeff_fmpz_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        exp: *const fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_fmpz_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_ui_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_si_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_fmpz_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_ui_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_coeff_si_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_coeff_vars_ui(
        C: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        vars: *const mp_limb_signed_t,
        exps: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_is_fmpz_mod_poly(
        A: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_fmpz_mod_poly(
        A: *mut fmpz_mod_poly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_set_fmpz_mod_poly(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        Bcoeffs: *const fmpz,
        Blen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_fmpz_mod_poly(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_cmp(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_is_canonical(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_resize(
        A: *mut fmpz_mod_mpoly_struct,
        new_length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_coeff_fmpz(
        c: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_term_coeff_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_term_coeff_ui(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_term_coeff_si(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_exp_fmpz(
        exp: *mut *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_exp_ui(
        exp: *mut mp_limb_t,
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_exp_si(
        exp: *mut mp_limb_signed_t,
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_var_exp_ui(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_var_exp_si(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_term_exp_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_term_exp_ui(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term(
        M: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_term_monomial(
        M: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_push_term_fmpz_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_push_term_ui_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_push_term_si_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_push_term_fmpz_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_push_term_ui_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_push_term_si_ui(
        A: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_sort_terms(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_combine_like_terms(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_reverse(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_assert_canonical(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_radix_sort1(
        arg1: *mut fmpz,
        arg2: *mut mp_limb_t,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        cmpmask: mp_limb_t,
        totalmask: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_radix_sort(
        arg1: *mut fmpz,
        arg2: *mut mp_limb_t,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_push_exp_ffmpz(
        A: *mut fmpz_mod_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_push_exp_pfmpz(
        A: *mut fmpz_mod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_push_exp_ui(
        A: *mut fmpz_mod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_randtest_bounds(
        A: *mut fmpz_mod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bounds: *mut mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_randtest_bound(
        A: *mut fmpz_mod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bound: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_randtest_bits(
        A: *mut fmpz_mod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_add_fmpz_mod(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_add_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_add_ui(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_add_si(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_sub_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_sub_ui(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_sub_si(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_add(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_sub(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_neg(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_scalar_mul_fmpz_mod_invertible(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_scalar_mul_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_scalar_mul_ui(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_scalar_mul_si(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_scalar_addmul_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *mut fmpz_mod_mpoly_struct,
        d: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_make_monic(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_derivative(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_eval_all_fmpz_mod(
        eval: *mut fmpz,
        Acoeffs: *const fmpz,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        alphas: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
        fctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_evaluate_all_fmpz(
        eval: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        alphas: *const *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_evaluate_one_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        val: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_compose_mat(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        M: *mut fmpz_mat_struct,
        ctxB: *mut fmpz_mod_mpoly_ctx_struct,
        ctxAC: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_compose_fmpz_mod_mpoly_geobucket(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *const *mut fmpz_mod_mpoly_struct,
        ctxB: *mut fmpz_mod_mpoly_ctx_struct,
        ctxAC: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_compose_fmpz_mod_mpoly(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *const *mut fmpz_mod_mpoly_struct,
        ctxB: *mut fmpz_mod_mpoly_ctx_struct,
        ctxAC: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_mul(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_mul_johnson(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_mul_johnson_maxfields(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut fmpz_mod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_mul_dense(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        C: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_mul_dense_maxfields(
        P: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        maxAfields: *mut fmpz,
        B: *mut fmpz_mod_mpoly_struct,
        maxBfields: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_pow_fmpz(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        k: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_pow_ui(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_pow_rmul(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_divides(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_div(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_divrem(
        Q: *mut fmpz_mod_mpoly_struct,
        R: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_divrem_ideal(
        Q: *mut *mut fmpz_mod_mpoly_struct,
        R: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *const *mut fmpz_mod_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_divides_dense_maxfields(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        maxAfields: *mut fmpz,
        B: *mut fmpz_mod_mpoly_struct,
        maxBfields: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_divides_dense(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_divides_monagan_pearce_maxfields(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        maxAfields: *mut fmpz,
        B: *mut fmpz_mod_mpoly_struct,
        maxBfields: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_divides_monagan_pearce(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_div_monagan_pearce(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_divrem_monagan_pearce(
        Q: *mut fmpz_mod_mpoly_struct,
        R: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_divrem_ideal_monagan_pearce(
        Q: *mut *mut fmpz_mod_mpoly_struct,
        R: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *const *mut fmpz_mod_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_sqrt_heap(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_quadratic_root(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_term_content(
        M: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_content_vars(
        g: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        vars: *mut mp_limb_signed_t,
        vars_length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd(
        G: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd_cofactors(
        G: *mut fmpz_mod_mpoly_struct,
        Abar: *mut fmpz_mod_mpoly_struct,
        Bbar: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd_subresultant(
        G: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd_brown(
        G: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd_hensel(
        G: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd_zippel(
        G: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_gcd_zippel2(
        G: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_deflation(
        shift: *mut fmpz,
        stride: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_deflate(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_inflate(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_init(
        A: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_clear(
        A: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_fit_length(
        A: *mut fmpz_mod_mpoly_univar_struct,
        length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_print_pretty(
        A: *mut fmpz_mod_mpoly_univar_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_assert_canonical(
        A: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_set_coeff_ui(
        A: *mut fmpz_mod_mpoly_univar_struct,
        e: mp_limb_t,
        c: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_to_univar(
        A: *mut fmpz_mod_mpoly_univar_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_from_univar(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fmpz_mod_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_from_univar(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_pseudo_gcd(
        Gx: *mut fmpz_mod_mpoly_univar_struct,
        Ax: *mut fmpz_mod_mpoly_univar_struct,
        Bx: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_resultant(
        R: *mut fmpz_mod_mpoly_struct,
        Ax: *mut fmpz_mod_mpoly_univar_struct,
        Bx: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_discriminant(
        D: *mut fmpz_mod_mpoly_struct,
        Fx: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_resultant(
        R: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_discriminant(
        R: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_init_dense_mock(
        D: *mut fmpz_mod_poly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        Adeg_bounds: *const mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_void_ring_init_fmpz_mod_mpoly_ctx(
        R: *mut _bindgen_ty_22,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_geobucket {
    pub polys: [fmpz_mod_mpoly_struct; 32usize],
    pub temps: [fmpz_mod_mpoly_struct; 32usize],
    pub length: mp_limb_signed_t,
}
pub type fmpz_mod_mpoly_geobucket_struct = fmpz_mod_mpoly_geobucket;
pub type fmpz_mod_mpoly_geobucket_t = [fmpz_mod_mpoly_geobucket_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_init(
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_clear(
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_empty(
        p: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_fit_length(
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_set(
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        p: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_add(
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        p: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_geobucket_sub(
        B: *mut fmpz_mod_mpoly_geobucket_struct,
        p: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyl_lead_coeff(
        c: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyl_content(
        g: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_to_fmpz_mod_poly_deflate(
        A: *mut fmpz_mod_poly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        Bshift: *const mp_limb_t,
        Bstride: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_from_fmpz_mod_poly_inflate(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fmpz_mod_poly_struct,
        var: mp_limb_signed_t,
        Ashift: *const mp_limb_t,
        Astride: *const mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_set_nmod_mpoly(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        nA: *mut nmod_mpoly_struct,
        nctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_get_nmod_mpoly(
        nA: *mut nmod_mpoly_struct,
        nctx: *mut nmod_mpoly_ctx_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_repack_bits(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_repack_bits_inplace(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_to_mpolyl_perm_deflate(
        A: *mut fmpz_mod_mpoly_struct,
        lctx: *mut fmpz_mod_mpoly_ctx_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_from_mpolyl_perm_inflate(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        B: *mut fmpz_mod_mpoly_struct,
        lctx: *mut fmpz_mod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}

#[inline]
pub unsafe fn fmpz_mod_mpoly_init(
    A: *mut fmpz_mod_mpoly_struct,
    ctx: *mut fmpz_mod_mpoly_ctx_struct,
) {
    fmpz_mod_mpoly_init2(A, 0, ctx);
}
