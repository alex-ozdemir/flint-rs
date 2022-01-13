#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_zech_mpoly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fq_nmod_mpoly::{fq_nmod_mpoly_ctx_struct, fq_nmod_mpoly_struct};
use crate::fq_zech::*;
use crate::fq_zech_poly::fq_zech_poly_struct;
use crate::mpoly::*;
use crate::nmod_mpoly::{nmod_mpoly_ctx_struct, nmod_mpoly_struct};
use crate::nmod_vec::nmod_t;
use libc::FILE;

extern "C" {
    pub fn fq_zech_ctx_mod(ctx: *mut fq_zech_ctx_struct) -> nmod_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub fqctx: fq_zech_ctx_t,
}
pub type fq_zech_mpoly_ctx_t = [fq_zech_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpoly_struct {
    pub coeffs: *mut fq_zech_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type fq_zech_mpoly_t = [fq_zech_mpoly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpoly_univar_struct {
    pub coeffs: *mut fq_zech_mpoly_struct,
    pub exps: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fq_zech_mpoly_univar_t = [fq_zech_mpoly_univar_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpolyu_struct {
    pub coeffs: *mut fq_zech_mpoly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type fq_zech_mpolyu_t = [fq_zech_mpolyu_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpolyn_struct {
    pub coeffs: *mut fq_zech_poly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_signed_t,
}
pub type fq_zech_mpolyn_t = [fq_zech_mpolyn_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpolyun_struct {
    pub coeffs: *mut fq_zech_mpolyn_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type fq_zech_mpolyun_t = [fq_zech_mpolyun_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mpoly_geobucket {
    pub polys: [fq_zech_mpoly_struct; 32usize],
    pub length: mp_limb_signed_t,
}
pub type fq_zech_mpoly_geobucket_struct = fq_zech_mpoly_geobucket;
pub type fq_zech_mpoly_geobucket_t = [fq_zech_mpoly_geobucket_struct; 1usize];
extern "C" {
    pub fn fq_zech_mpoly_ctx_init_deg(
        ctx: *mut fq_zech_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
        p: mp_limb_t,
        deg: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_ctx_init(
        ctx: *mut fq_zech_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
        fqctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_ctx_clear(ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpoly_ctx_nvars(ctx: *mut fq_zech_mpoly_ctx_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_ctx_ord(ctx: *mut fq_zech_mpoly_ctx_struct) -> ordering_t;
}
extern "C" {
    pub fn fq_zech_mpoly_init(A: *mut fq_zech_mpoly_struct, ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpoly_init2(
        A: *mut fq_zech_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_init3(
        A: *mut fq_zech_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_realloc(
        A: *mut fq_zech_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_fit_length(
        A: *mut fq_zech_mpoly_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_fit_length(
        coeff: *mut *mut fq_zech_struct,
        exps: *mut *mut mp_limb_t,
        alloc: *mut mp_limb_signed_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
        fqctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_fit_length_reset_bits(
        A: *mut fq_zech_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_clear(A: *mut fq_zech_mpoly_struct, ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn _fq_zech_mpoly_set_length(
        A: *mut fq_zech_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_truncate(
        A: *mut fq_zech_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_fit_bits(
        A: *mut fq_zech_mpoly_struct,
        bits: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_str_pretty(
        A: *mut fq_zech_mpoly_struct,
        str_: *const ::std::os::raw::c_char,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_get_str_pretty(
        A: *mut fq_zech_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fq_zech_mpoly_fprint_pretty(
        file: *mut FILE,
        A: *mut fq_zech_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_print_pretty(
        A: *mut fq_zech_mpoly_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_gen(
        A: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_is_gen(
        A: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_set(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_equal(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_swap(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_is_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_get_fq_zech(
        c: *mut fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_ui(
        A: *mut fq_zech_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_fq_zech_gen(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_equal_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_zero(A: *mut fq_zech_mpoly_struct, ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpoly_one(A: *mut fq_zech_mpoly_struct, ctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpoly_is_zero(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_is_one(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_degrees_fit_si(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_degrees_fmpz(
        degs: *mut *mut fmpz,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_degree_fmpz(
        deg: *mut fmpz,
        A: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_degree_si(
        A: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_total_degree_fits_si(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_total_degree_fmpz(
        td: *mut fmpz,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_total_degree_si(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_get_coeff_fq_zech_monomial(
        c: *mut fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        M: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_coeff_fq_zech_monomial(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        M: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_coeff_fq_zech_fmpz(
        c: *mut fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_coeff_fq_zech_ui(
        c: *mut fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_set_coeff_fq_zech_fmpz(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        exp: *const fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_coeff_fq_zech_fmpz(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_coeff_fq_zech_ui(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_coeff_vars_ui(
        C: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        vars: *const mp_limb_signed_t,
        exps: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_leadcoeff(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> *mut fq_zech_struct;
}
extern "C" {
    pub fn fq_zech_mpoly_cmp(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_is_canonical(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_length(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_resize(
        A: *mut fq_zech_mpoly_struct,
        new_length: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_coeff_fq_zech(
        c: *mut fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_term_coeff_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        c: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_term_exp_fits_ui(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_term_exp_fits_si(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_exp_fmpz(
        exp: *mut *mut fmpz,
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_exp_ui(
        exp: *mut mp_limb_t,
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_exp_si(
        exp: *mut mp_limb_signed_t,
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_var_exp_ui(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_var_exp_si(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_set_term_exp_fmpz(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_term_exp_ui(
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_term(
        M: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_term_monomial(
        M: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_push_term_fq_zech_fmpz(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_push_term_fq_zech_ui(
        A: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_sort_terms(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_combine_like_terms(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_reverse(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_assert_canonical(
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_radix_sort1(
        A: *mut fq_zech_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        cmpmask: mp_limb_t,
        totalmask: mp_limb_t,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_radix_sort(
        A: *mut fq_zech_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_push_exp_ffmpz(
        A: *mut fq_zech_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_push_exp_pfmpz(
        A: *mut fq_zech_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_push_exp_ui(
        A: *mut fq_zech_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_randtest_bound(
        A: *mut fq_zech_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bound: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_randtest_bounds(
        A: *mut fq_zech_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bounds: *mut mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_randtest_bits(
        A: *mut fq_zech_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bits: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_add(
        coeff1: *mut fq_zech_struct,
        exp1: *mut mp_limb_t,
        coeff2: *mut fq_zech_struct,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *mut fq_zech_struct,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fqctx: *mut fq_zech_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_add_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_sub_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_add(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_sub(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_neg(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_scalar_mul_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        c: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_make_monic(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_scalar_addmul_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *mut fq_zech_mpoly_struct,
        d: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_derivative(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_evaluate_one_fq_zech(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        val: *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_eval_all_fq_zech(
        eval: *mut fq_zech_struct,
        Acoeffs: *const fq_zech_struct,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        alphas: *const *mut fq_zech_struct,
        mctx: *mut mpoly_ctx_struct,
        fqctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_evaluate_all_fq_zech(
        ev: *mut fq_zech_struct,
        A: *mut fq_zech_mpoly_struct,
        vals: *const *mut fq_zech_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_compose_fq_zech_poly(
        A: *mut fq_zech_poly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *const *mut fq_zech_poly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_compose_mat(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        M: *mut fmpz_mat_struct,
        ctxB: *mut fq_zech_mpoly_ctx_struct,
        ctxAC: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_compose_fq_zech_mpoly_geobucket(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *const *mut fq_zech_mpoly_struct,
        ctxB: *mut fq_zech_mpoly_ctx_struct,
        ctxAC: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_compose_fq_zech_mpoly_horner(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *const *mut fq_zech_mpoly_struct,
        ctxB: *mut fq_zech_mpoly_ctx_struct,
        ctxAC: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_compose_fq_zech_mpoly(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *const *mut fq_zech_mpoly_struct,
        ctxB: *mut fq_zech_mpoly_ctx_struct,
        ctxAC: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_compose_fq_zech_mpoly_gen(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        c: *const mp_limb_signed_t,
        ctxB: *mut fq_zech_mpoly_ctx_struct,
        ctxAC: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_mul(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        C: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_mul_johnson(
        poly1: *mut fq_zech_mpoly_struct,
        poly2: *mut fq_zech_mpoly_struct,
        poly3: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_mul_johnson(
        coeff1: *mut *mut fq_zech_struct,
        exp1: *mut *mut mp_limb_t,
        alloc: *mut mp_limb_signed_t,
        coeff2: *const fq_zech_struct,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *const fq_zech_struct,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fqctx: *mut fq_zech_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_pow_fmpz(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        k: *mut fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_pow_ui(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_divides(
        Q: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_div(
        Q: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_divrem(
        Q: *mut fq_zech_mpoly_struct,
        R: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_divrem_ideal(
        Q: *mut *mut fq_zech_mpoly_struct,
        R: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *const *mut fq_zech_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_divides_monagan_pearce(
        poly1: *mut fq_zech_mpoly_struct,
        poly2: *mut fq_zech_mpoly_struct,
        poly3: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_div_monagan_pearce(
        q: *mut fq_zech_mpoly_struct,
        poly2: *mut fq_zech_mpoly_struct,
        poly3: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_divrem_monagan_pearce(
        q: *mut fq_zech_mpoly_struct,
        r: *mut fq_zech_mpoly_struct,
        poly2: *mut fq_zech_mpoly_struct,
        poly3: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_divrem_ideal_monagan_pearce(
        q: *mut *mut fq_zech_mpoly_struct,
        r: *mut fq_zech_mpoly_struct,
        poly2: *mut fq_zech_mpoly_struct,
        poly3: *const *mut fq_zech_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_gcd(
        G: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_gcd(
        G: *mut fq_zech_mpoly_struct,
        Gbits: mp_limb_t,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_gcd_cofactors(
        G: *mut fq_zech_mpoly_struct,
        Abar: *mut fq_zech_mpoly_struct,
        Bbar: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_gcd_cofactors(
        G: *mut fq_zech_mpoly_struct,
        Gbits: mp_limb_t,
        Abar: *mut fq_zech_mpoly_struct,
        Abarbits: mp_limb_t,
        Bbar: *mut fq_zech_mpoly_struct,
        Bbarbits: mp_limb_t,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_gcd_brown(
        G: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_gcd_zippel(
        G: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_deflation(
        shift: *mut fmpz,
        stride: *mut fmpz,
        A: *mut fq_zech_mpoly_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_deflate(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_inflate(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_init(
        A: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_clear(
        A: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_fit_length(
        A: *mut fq_zech_mpoly_univar_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_print_pretty(
        A: *mut fq_zech_mpoly_univar_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_assert_canonical(
        A: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_to_univar(
        A: *mut fq_zech_mpoly_univar_struct,
        B: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_from_univar_bits(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fq_zech_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_from_univar(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_swap(
        A: *mut fq_zech_mpoly_univar_struct,
        B: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_degree_fits_si(
        A: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_univar_length(
        A: *mut fq_zech_mpoly_univar_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_univar_get_term_exp_si(
        A: *mut fq_zech_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_mpoly_univar_get_term_coeff(
        c: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_univar_swap_term_coeff(
        c: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_get_nmod_mpoly(
        s: *mut nmod_mpoly_struct,
        sctx: *mut nmod_mpoly_ctx_struct,
        t: *mut fq_zech_mpoly_struct,
        tctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_set_nmod_mpoly(
        A: *mut fq_zech_mpoly_struct,
        Actx: *mut fq_zech_mpoly_ctx_struct,
        B: *mut nmod_mpoly_struct,
        Bctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyl_lead_coeff(
        c: *mut fq_zech_mpoly_struct,
        A: *mut fq_zech_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_pow_rmul(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_repack_bits_inplace(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_ctx_change_modulus(
        ctx: *mut fq_zech_mpoly_ctx_struct,
        deg: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_get_fq_nmod_mpoly(
        A: *mut fq_nmod_mpoly_struct,
        ctxA: *mut fq_nmod_mpoly_ctx_struct,
        B: *mut fq_zech_mpoly_struct,
        ctxB: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fq_zech_mpoly_set_fq_nmod_mpoly(
        A: *mut fq_zech_mpoly_struct,
        ctxA: *mut fq_zech_mpoly_ctx_struct,
        B: *mut fq_nmod_mpoly_struct,
        ctxB: *mut fq_nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyu_is_canonical(
        poly: *mut fq_zech_mpolyu_struct,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpolyu_init(
        A: *mut fq_zech_mpolyu_struct,
        bits: mp_limb_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyu_clear(A: *mut fq_zech_mpolyu_struct, uctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpolyu_swap(A: *mut fq_zech_mpolyu_struct, B: *mut fq_zech_mpolyu_struct);
}
extern "C" {
    pub fn fq_zech_mpolyu_zero(A: *mut fq_zech_mpolyu_struct, uctx: *mut fq_zech_mpoly_ctx_struct);
}
extern "C" {
    pub fn fq_zech_mpolyu_is_one(
        A: *mut fq_zech_mpolyu_struct,
        uctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpolyu_print_pretty(
        poly: *mut fq_zech_mpolyu_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyu_fit_length(
        A: *mut fq_zech_mpolyu_struct,
        length: mp_limb_signed_t,
        uctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpolyu_one(A: *mut fq_zech_mpolyu_struct, uctx: *mut fq_zech_mpoly_ctx_struct);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_bpoly_struct {
    pub coeffs: *mut fq_zech_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fq_zech_bpoly_t = [fq_zech_bpoly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_tpoly_struct {
    pub coeffs: *mut fq_zech_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fq_zech_tpoly_t = [fq_zech_tpoly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_polyu_struct {
    pub exps: *mut mp_limb_t,
    pub coeffs: *mut fq_zech_struct,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type fq_zech_polyu_t = [fq_zech_polyu_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_polyun_struct {
    pub coeffs: *mut fq_zech_poly_struct,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type fq_zech_polyun_t = [fq_zech_polyun_struct; 1usize];
extern "C" {
    pub fn fq_zech_bpoly_init(A: *mut fq_zech_bpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_bpoly_clear(A: *mut fq_zech_bpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_bpoly_swap(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_normalise(A: *mut fq_zech_bpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_bpoly_realloc(
        A: *mut fq_zech_bpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_fit_length(
        A: *mut fq_zech_bpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_zero(A: *mut fq_zech_bpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_bpoly_is_zero(
        A: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_equal(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_get_coeff(
        c: *mut fq_zech_struct,
        A: *mut fq_zech_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_degree0(
        A: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_bpoly_degree1(
        A: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fq_zech_bpoly_set_poly_var1(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_set_poly_var0(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_print_pretty(
        A: *mut fq_zech_bpoly_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_is_canonical(
        A: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_fq_equal(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_set_coeff_fq_zech(
        A: *mut fq_zech_bpoly_struct,
        xi: mp_limb_signed_t,
        yi: mp_limb_signed_t,
        c: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_set_fq_zech_poly_var0(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_set_fq_zech_poly_var1(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_make_monic(
        A: *mut fq_zech_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_mul(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        C: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_mul_series(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        C: *mut fq_zech_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_add(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        C: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_one(A: *mut fq_zech_bpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_bpoly_sub(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        C: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_derivative(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_divrem_series(
        Q: *mut fq_zech_bpoly_struct,
        R: *mut fq_zech_bpoly_struct,
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_divides(
        Q: *mut fq_zech_bpoly_struct,
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_set(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_make_primitive(
        g: *mut fq_zech_poly_struct,
        A: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_taylor_shift_var1(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        c_: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_taylor_shift_var0(
        A: *mut fq_zech_bpoly_struct,
        alpha: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_get_fq_zech_bpoly(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_mpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_fq_zech_bpoly(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fq_zech_bpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_bpoly_factor_smprime(
        c: *mut fq_zech_poly_struct,
        F: *mut fq_zech_tpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        allow_shift: ::std::os::raw::c_int,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_bpoly_factor_lgprime(
        c: *mut fq_zech_poly_struct,
        F: *mut fq_zech_tpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_tpoly_init(A: *mut fq_zech_tpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_tpoly_swap(
        A: *mut fq_zech_tpoly_struct,
        B: *mut fq_zech_tpoly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_tpoly_fit_length(
        A: *mut fq_zech_tpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_tpoly_clear(A: *mut fq_zech_tpoly_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_polyu_init(A: *mut fq_zech_polyu_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_polyu_clear(A: *mut fq_zech_polyu_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_polyu_realloc(
        A: *mut fq_zech_polyu_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu_fit_length(
        A: *mut fq_zech_polyu_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu_swap(
        A: *mut fq_zech_polyu_struct,
        B: *mut fq_zech_polyu_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu3_print_pretty(
        A: *mut fq_zech_polyu_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        var2: *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu3_degrees(
        deg0: *mut mp_limb_signed_t,
        deg1: *mut mp_limb_signed_t,
        deg2: *mut mp_limb_signed_t,
        A: *mut fq_zech_polyu_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu_is_canonical(
        A: *mut fq_zech_polyu_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_polyun_init(A: *mut fq_zech_polyun_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_polyun_clear(A: *mut fq_zech_polyun_struct, ctx: *mut fq_zech_ctx_struct);
}
extern "C" {
    pub fn fq_zech_polyun_realloc(
        A: *mut fq_zech_polyun_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyun_fit_length(
        A: *mut fq_zech_polyun_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyun_swap(
        A: *mut fq_zech_polyun_struct,
        B: *mut fq_zech_polyun_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu2n_print_pretty(
        A: *mut fq_zech_polyun_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        varlast: *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyu3n_print_pretty(
        A: *mut fq_zech_polyun_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        var2: *const ::std::os::raw::c_char,
        varlast: *const ::std::os::raw::c_char,
        ctx: *mut fq_zech_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_polyun_is_canonical(
        A: *mut fq_zech_polyun_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_is_fq_zech_poly(
        A: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fq_zech_mpoly_get_fq_zech_poly(
        A: *mut fq_zech_poly_struct,
        B: *mut fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fq_zech_mpoly_set_fq_zech_poly(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        Bcoeffs: *const fq_zech_struct,
        Blen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fq_zech_mpoly_set_fq_zech_poly(
        A: *mut fq_zech_mpoly_struct,
        B: *mut fq_zech_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fq_zech_mpoly_ctx_struct,
    );
}
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
