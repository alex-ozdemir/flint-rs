#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpq_mpoly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::{fmpq, fmpq_t};
use crate::fmpq_poly::fmpq_poly_struct;
use crate::fmpz::fmpz;
use crate::fmpz_mpoly::{fmpz_mpoly_ctx_t, fmpz_mpoly_struct, fmpz_mpoly_t};
use crate::mpoly::*;
use libc::FILE;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpq_mpoly_ctx_struct {
    pub zctx: fmpz_mpoly_ctx_t,
}
pub type fmpq_mpoly_ctx_t = [fmpq_mpoly_ctx_struct; 1usize];
extern "C" {
    pub fn fmpq_mpoly_ctx_init(
        ctx: *mut fmpq_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
    );
}
extern "C" {
    pub fn fmpq_mpoly_ctx_init_rand(
        ctx: *mut fmpq_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpq_mpoly_ctx_clear(ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_ctx_nvars(ctx: *mut fmpq_mpoly_ctx_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_ctx_ord(ctx: *mut fmpq_mpoly_ctx_struct) -> ordering_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpq_mpoly_struct {
    pub content: fmpq_t,
    pub zpoly: fmpz_mpoly_t,
}
pub type fmpq_mpoly_t = [fmpq_mpoly_struct; 1usize];
extern "C" {
    pub fn fmpq_mpoly_content_ref(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> *mut fmpq;
}
extern "C" {
    pub fn fmpq_mpoly_zpoly_ref(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> *mut fmpz_mpoly_struct;
}
extern "C" {
    pub fn fmpq_mpoly_zpoly_term_coeff_ref(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> *mut fmpz;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpq_mpoly_univar_struct {
    pub coeffs: *mut fmpq_mpoly_struct,
    pub exps: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpq_mpoly_univar_t = [fmpq_mpoly_univar_struct; 1usize];
extern "C" {
    pub fn fmpq_mpoly_init(A: *mut fmpq_mpoly_struct, ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_init2(
        A: *mut fmpq_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_init3(
        A: *mut fmpq_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_realloc(
        A: *mut fmpq_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_fit_length(
        A: *mut fmpq_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_clear(A: *mut fmpq_mpoly_struct, ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_fit_bits(
        A: *mut fmpq_mpoly_struct,
        bits: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_str_pretty(
        A: *mut fmpq_mpoly_struct,
        str_: *const ::std::os::raw::c_char,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_get_str_pretty(
        A: *mut fmpq_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmpq_mpoly_fprint_pretty(
        file: *mut FILE,
        A: *mut fmpq_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_print_pretty(
        A: *mut fmpq_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gen(
        A: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_is_gen(
        A: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_set(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_equal(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_swap(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_is_fmpq(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_get_fmpq(
        c: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_fmpq(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_ui(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_si(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_zero(A: *mut fmpq_mpoly_struct, ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_one(A: *mut fmpq_mpoly_struct, ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_equal_fmpq(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_equal_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_equal_ui(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_equal_si(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_is_zero(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_is_one(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_degrees_fit_si(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_degrees_fmpz(
        degs: *mut *mut fmpz,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_degree_fmpz(
        deg: *mut fmpz,
        A: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_degree_si(
        A: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_total_degree_fits_si(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_total_degree_fmpz(
        tdeg: *mut fmpz,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_total_degree_si(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_used_vars(
        used: *mut ::std::os::raw::c_int,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_denominator(
        d: *mut fmpz,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_is_monic(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_get_coeff_fmpq_monomial(
        c: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        M: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_coeff_fmpq_monomial(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        M: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpq_mpoly_set_coeff_fmpq_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        exp: *const fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_coeff_fmpq_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        exp: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_coeff_fmpq_ui(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        exp: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpq_mpoly_get_coeff_fmpq_fmpz(
        c: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_coeff_fmpq_fmpz(
        c: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_coeff_fmpq_ui(
        c: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_coeff_vars_ui(
        C: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        vars: *const mp_limb_signed_t,
        exps: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_is_fmpq_poly(
        A: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_get_fmpq_poly(
        A: *mut fmpq_poly_struct,
        B: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_set_fmpq_poly(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_cmp(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_is_canonical(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_length(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_resize(
        A: *mut fmpq_mpoly_struct,
        new_length: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_term_coeff_fmpq(
        c: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_term_coeff_fmpq(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_term_exp_fits_ui(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_term_exp_fits_si(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_get_term_exp_fmpz(
        exps: *mut *mut fmpz,
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_term_exp_ui(
        exps: *mut mp_limb_t,
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_term_exp_si(
        exps: *mut mp_limb_signed_t,
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_term_var_exp_ui(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn fmpq_mpoly_get_term_var_exp_si(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_set_term_exp_fmpz(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        exps: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_set_term_exp_ui(
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        exps: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_term(
        M: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_get_term_monomial(
        M: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_fmpq_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        exp: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_fmpz_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        exp: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_ui_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_si_fmpz(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_fmpq_ui(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        exp: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_fmpz_ui(
        A: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        exp: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_ui_ui(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_push_term_si_ui(
        A: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_reduce(A: *mut fmpq_mpoly_struct, ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_reduce_easy(
        A: *mut fmpq_mpoly_struct,
        easy_length: mp_limb_signed_t,
        arg1: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sort_terms(A: *mut fmpq_mpoly_struct, ctx: *mut fmpq_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpq_mpoly_combine_like_terms(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_reverse(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_assert_canonical(
        poly: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpq_mpoly_push_rescale(
        A: *mut fmpq_mpoly_struct,
        C: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_randtest_bounds(
        A: *mut fmpq_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bounds: *mut mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_randtest_bound(
        A: *mut fmpq_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bound: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_randtest_bits(
        A: *mut fmpq_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bits: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_add_fmpq(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_add_fmpz(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_add_ui(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_add_si(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sub_fmpq(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sub_fmpz(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sub_ui(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sub_si(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_add(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        C: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sub(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        C: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_neg(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_mul_fmpq(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_mul_fmpz(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_mul_ui(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_mul_si(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_div_fmpq(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_div_fmpz(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_div_ui(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_scalar_div_si(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_make_monic(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpq_mpoly_make_monic_inplace(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_derivative(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_integral(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpq_mpoly_rescale(
        Acontent: *mut fmpq,
        Acoeff: *mut fmpz,
        B: *mut fmpq_mpoly_struct,
        scales: *const fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_evaluate_all_fmpq(
        ev: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        vals: *const *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_evaluate_one_fmpq(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        val: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_compose_fmpq_poly(
        A: *mut fmpq_poly_struct,
        B: *mut fmpq_mpoly_struct,
        C: *const *mut fmpq_poly_struct,
        ctxB: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_compose_fmpq_mpoly(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        C: *const *mut fmpq_mpoly_struct,
        ctxB: *mut fmpq_mpoly_ctx_struct,
        ctxAC: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_compose_fmpq_mpoly_gen(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        c: *const mp_limb_signed_t,
        ctxB: *mut fmpq_mpoly_ctx_struct,
        ctxAC: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_mul(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        C: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_pow_fmpz(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        k: *mut fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_pow_ui(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_divides(
        poly1: *mut fmpq_mpoly_struct,
        poly2: *mut fmpq_mpoly_struct,
        poly3: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_div(
        q: *mut fmpq_mpoly_struct,
        poly2: *mut fmpq_mpoly_struct,
        poly3: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_divrem(
        q: *mut fmpq_mpoly_struct,
        r: *mut fmpq_mpoly_struct,
        poly2: *mut fmpq_mpoly_struct,
        poly3: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_divrem_ideal(
        q: *mut *mut fmpq_mpoly_struct,
        r: *mut fmpq_mpoly_struct,
        poly2: *mut fmpq_mpoly_struct,
        poly3: *const *mut fmpq_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_sqrt(
        Q: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_is_square(
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_content(
        g: *mut fmpq,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_term_content(
        M: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_content_vars(
        g: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        vars: *mut mp_limb_signed_t,
        vars_length: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gcd(
        G: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_inflate(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_gcd_cofactors(
        G: *mut fmpq_mpoly_struct,
        Abar: *mut fmpq_mpoly_struct,
        Bbar: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gcd_hensel(
        G: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gcd_brown(
        G: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gcd_subresultant(
        G: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gcd_zippel(
        G: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_gcd_zippel2(
        G: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_resultant(
        R: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_discriminant(
        R: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mpoly_void_ring_init_fmpq_mpoly_ctx(
        R: *mut _bindgen_ty_22,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_repack_bits(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_univar_init(
        A: *mut fmpq_mpoly_univar_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_clear(
        A: *mut fmpq_mpoly_univar_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_fit_length(
        A: *mut fmpq_mpoly_univar_struct,
        length: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_print_pretty(
        A: *mut fmpq_mpoly_univar_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_assert_canonical(
        A: *mut fmpq_mpoly_univar_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_to_univar(
        A: *mut fmpq_mpoly_univar_struct,
        B: *mut fmpq_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_from_univar_bits(
        A: *mut fmpq_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fmpq_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_from_univar(
        A: *mut fmpq_mpoly_struct,
        B: *mut fmpq_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_swap(
        A: *mut fmpq_mpoly_univar_struct,
        B: *mut fmpq_mpoly_univar_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_degree_fits_si(
        A: *mut fmpq_mpoly_univar_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_univar_length(
        A: *mut fmpq_mpoly_univar_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_univar_get_term_exp_si(
        A: *mut fmpq_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_univar_get_term_coeff(
        c: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_univar_swap_term_coeff(
        c: *mut fmpq_mpoly_struct,
        A: *mut fmpq_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_remainder_test(
        r: *mut fmpq_mpoly_struct,
        g: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn tuple_print(alpha: *mut fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn tuple_saturate(alpha: *mut fmpz, n: mp_limb_signed_t, m: mp_limb_signed_t);
}
extern "C" {
    pub fn tuple_next(alpha: *mut fmpz, n: mp_limb_signed_t);
}
