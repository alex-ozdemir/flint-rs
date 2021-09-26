#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_mpoly.html).


use crate::deps::*;
use crate::flint::*;
use crate::n_poly::*;
use crate::mpoly::*;
use crate::nmod_mpoly::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::nmod_vec::nmod_t;
use crate::nmod_mat::nmod_mat_struct;
use crate::fmpz_mod::{fmpz_mod_ctx_struct, fmpz_mod_ctx_t};
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
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
    pub fn fmpz_mod_mpoly_ctx_nvars(ctx: *mut fmpz_mod_mpoly_ctx_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_ord(ctx: *mut fmpz_mod_mpoly_ctx_struct) -> ordering_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_modulus(ctx: *mut fmpz_mod_mpoly_ctx_struct) -> *const fmpz;
}
extern "C" {
    pub fn fmpz_mod_mpoly_ctx_get_modulus(m: *mut fmpz, ctx: *mut fmpz_mod_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpoly_init(A: *mut fmpz_mod_mpoly_struct, ctx: *mut fmpz_mod_mpoly_ctx_struct);
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
    pub fn _fmpz_mod_mpoly_fit_length(
        coeffs: *mut *mut fmpz,
        coeffs_alloc: *mut mp_limb_signed_t,
        exps: *mut *mut mp_limb_t,
        exps_alloc: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        length: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_set_length(
        A: *mut fmpz_mod_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_truncate(
        A: *mut fmpz_mod_mpoly_struct,
        newlen: mp_limb_signed_t,
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
    pub fn fmpz_mod_mpoly_print_pretty(
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
    pub fn fmpz_mod_mpoly_swap(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
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
    pub fn fmpz_mod_mpoly_zero(A: *mut fmpz_mod_mpoly_struct, ctx: *mut fmpz_mod_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpoly_one(A: *mut fmpz_mod_mpoly_struct, ctx: *mut fmpz_mod_mpoly_ctx_struct);
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
    pub fn fmpz_mod_mpoly_is_zero(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_is_one(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_degrees_fit_si(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_degrees_fmpz(
        degs: *mut *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_degree_fmpz(
        deg: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_degree_si(
        A: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_total_degree_fits_si(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_total_degree_fmpz(
        td: *mut fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_total_degree_si(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_used_vars(
        used: *mut ::std::os::raw::c_int,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
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
    pub fn fmpz_mod_mpoly_leadcoeff(A: *mut fmpz_mod_mpoly_struct) -> *mut fmpz;
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
    pub fn fmpz_mod_mpoly_length(
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
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
    pub fn fmpz_mod_mpoly_term_exp_fits_ui(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_term_exp_fits_si(
        A: *mut fmpz_mod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
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
    pub fn fmpz_mod_mpoly_divexact(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
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
    pub fn fmpz_mod_mpoly_sqrt(
        Q: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_is_square(
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
    pub fn fmpz_mod_mpoly_univar_zero(
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
    pub fn fmpz_mod_mpoly_univar_swap(
        A: *mut fmpz_mod_mpoly_univar_struct,
        B: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_degree_fits_si(
        A: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_length(
        A: *mut fmpz_mod_mpoly_univar_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_get_term_exp_si(
        A: *mut fmpz_mod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_get_term_coeff(
        c: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_univar_swap_term_coeff(
        c: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_univar_struct,
        i: mp_limb_signed_t,
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
    pub fn _fmpz_mod_mpoly_clear_dense_mock(D: *mut fmpz_mod_poly_struct);
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
extern "C" {
    pub fn fmpz_mod_mpoly_remainder_strongtest(
        r: *mut fmpz_mod_mpoly_struct,
        g: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_bpoly(
        A: *mut n_bpoly_struct,
        B: *mut nmod_mpoly_struct,
        var0: mp_limb_signed_t,
        var1: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_bpoly(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut n_bpoly_struct,
        var0: mp_limb_signed_t,
        var1: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn n_bpoly_mod_factor_smprime(
        c: *mut n_poly_struct,
        F: *mut n_tpoly_struct,
        B: *mut n_bpoly_struct,
        allow_shift: ::std::os::raw::c_int,
        ctx: nmod_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn n_bpoly_mod_factor_lgprime(
        c: *mut n_poly_struct,
        F: *mut n_tpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_mat_is_reduced(N: *mut nmod_mat_struct) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nmod_mat_init_nullspace_tr(X: *mut nmod_mat_struct, tmp: *mut nmod_mat_struct);
}
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
