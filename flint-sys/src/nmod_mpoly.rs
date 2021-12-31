#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! *See the [FLINT documentation](http://flintlib.org/doc/nmod_mpoly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::mpoly::*;
use crate::n_poly::{n_bpoly_struct, n_poly_struct, n_polyun_struct};
use crate::nmod_poly::nmod_poly_struct;
use crate::nmod_vec::nmod_t;
use libc::{c_char, c_int, c_uint, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub mod_: nmod_t,
}
pub type nmod_mpoly_ctx_t = [nmod_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_struct {
    pub coeffs: *mut mp_limb_t,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub coeffs_alloc: mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
}
pub type nmod_mpoly_t = [nmod_mpoly_struct; 1usize];
extern "C" {
    pub fn nmod_mpoly_term_coeff_ref(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> *mut mp_limb_t;
}
extern "C" {
    pub fn evil_cast_nmod_poly_to_n_poly(a: *mut nmod_poly_struct) -> *mut n_poly_struct;
}
extern "C" {
    pub fn evil_const_cast_nmod_poly_to_n_poly(a: *const nmod_poly_struct) -> *const n_poly_struct;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_univar_struct {
    pub coeffs: *mut nmod_mpoly_struct,
    pub exps: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type nmod_mpoly_univar_t = [nmod_mpoly_univar_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyu_struct {
    pub coeffs: *mut nmod_mpoly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type nmod_mpolyu_t = [nmod_mpolyu_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyn_struct {
    pub coeffs: *mut n_poly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_signed_t,
}
pub type nmod_mpolyn_t = [nmod_mpolyn_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyun_struct {
    pub coeffs: *mut nmod_mpolyn_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type nmod_mpolyun_t = [nmod_mpolyun_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyd_struct {
    pub nvars: mp_limb_signed_t,
    pub degb_alloc: mp_limb_signed_t,
    pub deg_bounds: *mut mp_limb_signed_t,
    pub coeff_alloc: mp_limb_signed_t,
    pub coeffs: *mut mp_limb_t,
}
pub type nmod_mpolyd_t = [nmod_mpolyd_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_stack_struct {
    pub poly_array: *mut *mut n_poly_struct,
    pub poly_alloc: mp_limb_signed_t,
    pub poly_top: mp_limb_signed_t,
    pub mpolyun_array: *mut *mut nmod_mpolyun_struct,
    pub mpolyun_alloc: mp_limb_signed_t,
    pub mpolyun_top: mp_limb_signed_t,
    pub mpolyn_array: *mut *mut nmod_mpolyn_struct,
    pub mpolyn_alloc: mp_limb_signed_t,
    pub mpolyn_top: mp_limb_signed_t,
    pub ctx: *const nmod_mpoly_ctx_struct,
    pub bits: mp_limb_t,
}
pub type nmod_poly_stack_t = [nmod_poly_stack_struct; 1usize];
extern "C" {
    pub fn nmod_poly_stack_init(
        S: *mut nmod_poly_stack_struct,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_poly_stack_clear(S: *mut nmod_poly_stack_struct);
}
extern "C" {
    pub fn nmod_poly_stack_set_ctx(S: *mut nmod_poly_stack_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_poly_stack_fit_request_poly(
        S: *mut nmod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_poly_struct;
}
extern "C" {
    pub fn nmod_poly_stack_fit_request_mpolyun(
        S: *mut nmod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut nmod_mpolyun_struct;
}
extern "C" {
    pub fn nmod_poly_stack_fit_request_mpolyn(
        S: *mut nmod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut nmod_mpolyn_struct;
}
extern "C" {
    pub fn nmod_poly_stack_request_poly(
        S: *mut nmod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_poly_struct;
}
extern "C" {
    pub fn nmod_poly_stack_take_top_poly(S: *mut nmod_poly_stack_struct) -> *mut n_poly_struct;
}
extern "C" {
    pub fn nmod_poly_stack_give_back_poly(S: *mut nmod_poly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_poly_stack_size_poly(S: *mut nmod_poly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_poly_stack_request_mpolyun(
        S: *mut nmod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut nmod_mpolyun_struct;
}
extern "C" {
    pub fn nmod_poly_stack_take_top_mpolyun(
        S: *mut nmod_poly_stack_struct,
    ) -> *mut nmod_mpolyun_struct;
}
extern "C" {
    pub fn nmod_poly_stack_give_back_mpolyun(S: *mut nmod_poly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_poly_stack_size_mpolyun(S: *mut nmod_poly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_poly_stack_request_mpolyn(
        S: *mut nmod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut nmod_mpolyn_struct;
}
extern "C" {
    pub fn nmod_poly_stack_take_top_mpolyn(
        S: *mut nmod_poly_stack_struct,
    ) -> *mut nmod_mpolyn_struct;
}
extern "C" {
    pub fn nmod_poly_stack_give_back_mpolyn(S: *mut nmod_poly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_poly_stack_size_mpolyn(S: *mut nmod_poly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_ctx_init(
        ctx: *mut nmod_mpoly_ctx_struct,
        nvars: mp_limb_signed_t,
        ord: ordering_t,
        modulus: mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_ctx_init_rand(
        ctx: *mut nmod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
        modulus: mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_ctx_clear(ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_ctx_set_modulus(ctx: *mut nmod_mpoly_ctx_struct, p: mp_limb_t);
}
extern "C" {
    pub fn nmod_mpoly_ctx_nvars(ctx: *mut nmod_mpoly_ctx_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_ctx_ord(ctx: *mut nmod_mpoly_ctx_struct) -> ordering_t;
}
extern "C" {
    pub fn nmod_mpoly_ctx_modulus(ctx: *mut nmod_mpoly_ctx_struct) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_ctx_change_modulus(ctx: *mut nmod_mpoly_ctx_struct, modulus: mp_limb_t);
}
extern "C" {
    pub fn nmod_mpoly_init(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_clear(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_init2(
        A: *mut nmod_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_init3(
        A: *mut nmod_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_realloc(
        A: *mut nmod_mpoly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_fit_length(
        A: *mut nmod_mpoly_struct,
        length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_fit_length_fit_bits(
        A: *mut nmod_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_fit_length_reset_bits(
        A: *mut nmod_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_fit_length(
        coeffs: *mut *mut mp_limb_t,
        coeffs_alloc: *mut mp_limb_signed_t,
        exps: *mut *mut mp_limb_t,
        exps_alloc: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        length: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_set_length(
        A: *mut nmod_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_truncate(
        A: *mut nmod_mpoly_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_str_pretty(
        A: *mut nmod_mpoly_struct,
        str_: *const c_char,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_get_str_pretty(
        A: *mut nmod_mpoly_struct,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> *mut c_char;
}
extern "C" {
    pub fn nmod_mpoly_fprint_pretty(
        file: *mut FILE,
        A: *mut nmod_mpoly_struct,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_print_pretty(
        A: *mut nmod_mpoly_struct,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gen(
        A: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_is_gen(
        A: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_set(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_equal(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_swap(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_is_ui(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_get_ui(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_set_ui(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_fmpz(
        A: *mut nmod_mpoly_struct,
        c: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_zero(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_one(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_equal_ui(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_is_zero(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_is_one(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_degrees_fit_si(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_degrees_fmpz(
        degs: *mut *mut fmpz,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_degree_fmpz(
        deg: *mut fmpz,
        A: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_degree_si(
        A: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_total_degree_fits_si(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_total_degree_fmpz(
        td: *mut fmpz,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_total_degree_si(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_used_vars(
        used: *mut c_int,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_coeff_ui_monomial(
        A: *mut nmod_mpoly_struct,
        M: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_set_coeff_ui_monomial(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        M: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_coeff_ui_fmpz(
        A: *mut nmod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_get_coeff_ui_ui(
        A: *mut nmod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn _nmod_mpoly_set_coeff_ui_fmpz(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        exp: *const fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_coeff_ui_fmpz(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        exp: *const *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_coeff_ui_ui(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_coeff_vars_ui(
        C: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        vars: *const mp_limb_signed_t,
        exps: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_leadcoeff(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_is_nmod_poly(
        A: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_get_n_poly(
        A: *mut n_poly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_get_nmod_poly(
        A: *mut nmod_poly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_set_nmod_poly(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        Bcoeffs: *const mp_limb_t,
        Blen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_n_poly_mod(
        A: *mut nmod_mpoly_struct,
        B: *mut n_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_nmod_poly(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_cmp(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_is_canonical(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_length(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_resize(
        A: *mut nmod_mpoly_struct,
        new_length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_term_coeff_ui(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_set_term_coeff_ui(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_term_exp_fits_ui(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_term_exp_fits_si(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_get_term_exp_fmpz(
        exp: *mut *mut fmpz,
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_term_exp_ui(
        exp: *mut mp_limb_t,
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_term_exp_si(
        exp: *mut mp_limb_signed_t,
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_term_var_exp_ui(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_get_term_var_exp_si(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_set_term_exp_fmpz(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_term_exp_ui(
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        exp: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_term(
        M: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_term_monomial(
        M: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_push_term_ui_fmpz(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        exp: *const *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_push_term_ui_ui(
        A: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_sort_terms(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpoly_combine_like_terms(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_reverse(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_assert_canonical(A: *mut nmod_mpoly_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn _nmod_mpoly_radix_sort1(
        A: *mut nmod_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        cmpmask: mp_limb_t,
        totalmask: mp_limb_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_radix_sort(
        A: *mut nmod_mpoly_struct,
        left: mp_limb_signed_t,
        right: mp_limb_signed_t,
        pos: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_push_exp_ffmpz(
        A: *mut nmod_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_push_exp_pfmpz(
        A: *mut nmod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_push_exp_ui(
        A: *mut nmod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_randtest_bounds(
        A: *mut nmod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bounds: *mut mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_randtest_bound(
        A: *mut nmod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bound: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_randtest_bits(
        A: *mut nmod_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_get_term_ui_fmpz(
        poly: *mut nmod_mpoly_struct,
        exp: *const fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_get_term_ui_fmpz(
        poly: *mut nmod_mpoly_struct,
        exp: *const *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_get_term_ui_ui(
        poly: *mut nmod_mpoly_struct,
        exp: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn _nmod_mpoly_max_degrees(
        max_degs: *mut mp_limb_t,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        n: mp_limb_signed_t,
        deg: c_int,
        rev: c_int,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_max_degrees(
        max_degs: *mut mp_limb_t,
        poly: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_nmod(
        poly: *mut nmod_mpoly_struct,
        c: nmod_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_coeff_ui(
        x: nmod_t,
        poly: *mut nmod_mpoly_struct,
        n: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_set_coeff_ui(
        poly: *mut nmod_mpoly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_get_monomial(
        exps: *mut mp_limb_t,
        poly: *mut nmod_mpoly_struct,
        n: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_set_monomial(
        poly: *mut nmod_mpoly_struct,
        n: mp_limb_signed_t,
        exps: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_add_ui(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_sub_ui(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_add(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_sub(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_add(
        coeff1: *mut mp_limb_t,
        exp1: *mut mp_limb_t,
        coeff2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fctx: nmod_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _nmod_mpoly_sub(
        coeff1: *mut mp_limb_t,
        exp1: *mut mp_limb_t,
        coeff2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fctx: nmod_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_neg(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_scalar_mul_ui(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_make_monic(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_scalar_mul_nmod_invertible(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_scalar_addmul_ui(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        d: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_derivative(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _ff_poly_pow_fmpz_is_not_feasible(length: mp_limb_signed_t, e: *mut fmpz) -> c_int;
}
extern "C" {
    pub fn _ff_poly_pow_ui_is_not_feasible(length: mp_limb_signed_t, e: mp_limb_t) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_eval_all_ui(
        Acoeffs: *const mp_limb_t,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        alphas: *const mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
        mod_: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_evaluate_all_ui(
        A: *mut nmod_mpoly_struct,
        vals: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpoly_evaluate_one_ui(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        val: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_compose_nmod_poly(
        A: *mut nmod_poly_struct,
        B: *mut nmod_mpoly_struct,
        C: *const *mut nmod_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_compose_mat(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        M: *mut fmpz_mat_struct,
        ctxB: *mut nmod_mpoly_ctx_struct,
        ctxAC: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_compose_nmod_mpoly_geobucket(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *const *mut nmod_mpoly_struct,
        ctxB: *mut nmod_mpoly_ctx_struct,
        ctxAC: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_compose_nmod_mpoly_horner(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *const *mut nmod_mpoly_struct,
        ctxB: *mut nmod_mpoly_ctx_struct,
        ctxAC: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_compose_nmod_mpoly(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *const *mut nmod_mpoly_struct,
        ctxB: *mut nmod_mpoly_ctx_struct,
        ctxAC: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_compose_nmod_mpoly_gen(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        c: *const mp_limb_signed_t,
        ctxB: *mut nmod_mpoly_ctx_struct,
        ctxAC: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_mul(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_mul_johnson(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_mul_heap_threaded(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_mul_array(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_mul_array_threaded(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_mul_dense(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        C: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_mul_johnson(
        A: *mut nmod_mpoly_struct,
        coeff2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fctx: nmod_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _nmod_mpoly_mul_johnson_maxfields(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut nmod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_mul_heap_threaded_pool_maxfields(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut nmod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_mul_array_DEG(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut nmod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_mul_array_LEX(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut nmod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_mul_array_threaded_pool_DEG(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut nmod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_mul_array_threaded_pool_LEX(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        C: *mut nmod_mpoly_struct,
        maxCfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_mul_dense(
        P: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        maxAfields: *mut fmpz,
        B: *mut nmod_mpoly_struct,
        maxBfields: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_pow_rmul(
        A: *mut nmod_mpoly_struct,
        Bcoeffs: *const mp_limb_t,
        Bexps: *const mp_limb_t,
        Blen: mp_limb_signed_t,
        k: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        mod_: nmod_t,
        T: *mut nmod_mpoly_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_pow_rmul(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_pow_fmpz(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        k: *mut fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_pow_ui(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        k: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_divides(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_divides_threaded_pool(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_divides_monagan_pearce(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_divides_heap_threaded(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_divides_heap_threaded_pool(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_divides_dense(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_div(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_divrem(
        Q: *mut nmod_mpoly_struct,
        R: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_divrem_ideal(
        Q: *mut *mut nmod_mpoly_struct,
        R: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *const *mut nmod_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_divexact(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_divides_monagan_pearce(
        Q: *mut nmod_mpoly_struct,
        coeff2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        coeff3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
        fctx: nmod_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_div_monagan_pearce(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_divrem_monagan_pearce(
        q: *mut nmod_mpoly_struct,
        r: *mut nmod_mpoly_struct,
        poly2: *mut nmod_mpoly_struct,
        poly3: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_divrem_ideal_monagan_pearce(
        Q: *mut *mut nmod_mpoly_struct,
        R: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *const *mut nmod_mpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_sqrt_heap(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_sqrt(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_is_square(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_quadratic_root(
        Q: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_term_content(
        M: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_content_vars(
        g: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        vars: *mut mp_limb_signed_t,
        vars_length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd(
        G: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_gcd_algo_small(
        G: *mut nmod_mpoly_struct,
        Abar: *mut nmod_mpoly_struct,
        Bbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        algo: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_gcd_algo(
        G: *mut nmod_mpoly_struct,
        Abar: *mut nmod_mpoly_struct,
        Bbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        algo: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd_cofactors(
        G: *mut nmod_mpoly_struct,
        Abar: *mut nmod_mpoly_struct,
        Bbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd_brown(
        G: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd_hensel(
        G: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd_zippel(
        G: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd_zippel2(
        G: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_deflation(
        shift: *mut fmpz,
        stride: *mut fmpz,
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_deflate(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_inflate(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        shift: *const fmpz,
        stride: *const fmpz,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_void_ring_init_nmod_mpoly_ctx(
        R: *mut _bindgen_ty_22,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyl_lead_coeff(
        c: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyl_content(
        g: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        num_vars: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_to_nmod_poly_deflate(
        A: *mut nmod_poly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        Bshift: *const mp_limb_t,
        Bstride: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_from_nmod_poly_inflate(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut nmod_poly_struct,
        var: mp_limb_signed_t,
        Ashift: *const mp_limb_t,
        Astride: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_repack_bits(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_repack_bits_inplace(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyd_ctx_struct {
    pub nvars: mp_limb_signed_t,
    pub perm: *mut mp_limb_signed_t,
}
pub type nmod_mpolyd_ctx_t = [nmod_mpolyd_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _nmod_mpoly_stripe_struct {
    pub big_mem: *mut c_char,
    pub big_mem_alloc: mp_limb_signed_t,
    pub ctx: *const nmod_mpoly_ctx_struct,
    pub N: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub mod_: nmod_t,
    pub lc_minus_inv: mp_limb_t,
    pub cmpmask: *const mp_limb_t,
    pub startidx: *mut mp_limb_signed_t,
    pub endidx: *mut mp_limb_signed_t,
    pub emin: *mut mp_limb_t,
    pub emax: *mut mp_limb_t,
    pub upperclosed: c_int,
}
pub type nmod_mpoly_stripe_struct = _nmod_mpoly_stripe_struct;
pub type nmod_mpoly_stripe_t = [nmod_mpoly_stripe_struct; 1usize];
extern "C" {
    pub fn nmod_mpoly_univar_init(
        A: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_clear(
        A: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_fit_length(
        A: *mut nmod_mpoly_univar_struct,
        length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_print_pretty(
        A: *mut nmod_mpoly_univar_struct,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_assert_canonical(
        A: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_zero(
        A: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_set_coeff_ui(
        A: *mut nmod_mpoly_univar_struct,
        e: mp_limb_t,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_to_univar(
        A: *mut nmod_mpoly_univar_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_from_univar(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut nmod_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_from_univar(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_univar_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_swap(
        A: *mut nmod_mpoly_univar_struct,
        B: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_degree_fits_si(
        A: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_univar_length(
        A: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_univar_get_term_exp_si(
        A: *mut nmod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_univar_get_term_coeff(
        c: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_swap_term_coeff(
        c: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_univar_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_univar_pseudo_gcd(
        Gx: *mut nmod_mpoly_univar_struct,
        Ax: *mut nmod_mpoly_univar_struct,
        Bx: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_univar_resultant(
        R: *mut nmod_mpoly_struct,
        Ax: *mut nmod_mpoly_univar_struct,
        Bx: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_univar_discriminant(
        D: *mut nmod_mpoly_struct,
        Fx: *mut nmod_mpoly_univar_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_resultant(
        R: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_discriminant(
        R: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_mul_array_chunked_LEX(
        P: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        mults: *const mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_mul_array_chunked_DEG(
        P: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        degb: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpoly_addmul_array1_ulong1(
        poly1: *mut mp_limb_t,
        poly2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_addmul_array1_ulong2(
        poly1: *mut mp_limb_t,
        poly2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_addmul_array1_ulong3(
        poly1: *mut mp_limb_t,
        poly2: *const mp_limb_t,
        exp2: *const mp_limb_t,
        len2: mp_limb_signed_t,
        poly3: *const mp_limb_t,
        exp3: *const mp_limb_t,
        len3: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm1_LEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        mults: *const mp_limb_t,
        num: mp_limb_signed_t,
        array_size: mp_limb_signed_t,
        top: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm2_LEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        mults: *const mp_limb_t,
        num: mp_limb_signed_t,
        array_size: mp_limb_signed_t,
        top: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm3_LEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        mults: *const mp_limb_t,
        num: mp_limb_signed_t,
        array_size: mp_limb_signed_t,
        top: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm1_DEGLEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        top: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        degb: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm2_DEGLEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        top: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        degb: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm3_DEGLEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        top: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        degb: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm1_DEGREVLEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        top: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        degb: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm2_DEGREVLEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        top: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        degb: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpoly_append_array_sm3_DEGREVLEX(
        P: *mut nmod_mpoly_struct,
        Plen: mp_limb_signed_t,
        coeff_array: *mut mp_limb_t,
        top: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        degb: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpolyd_ctx_init(dctx: *mut nmod_mpolyd_ctx_struct, nvars: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_mpolyd_ctx_clear(dctx: *mut nmod_mpolyd_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyd_swap(poly1: *mut nmod_mpolyd_struct, poly2: *mut nmod_mpolyd_struct);
}
extern "C" {
    pub fn nmod_mpolyd_set_degbounds(
        A: *mut nmod_mpolyd_struct,
        bounds: *mut mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyd_set_degbounds_perm(
        A: *mut nmod_mpolyd_struct,
        dctx: *mut nmod_mpolyd_ctx_struct,
        bounds: *mut mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_convert_to_nmod_mpolyd(
        A: *mut nmod_mpolyd_struct,
        dctx: *mut nmod_mpolyd_ctx_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_convert_to_nmod_mpolyd_degbound(
        A: *mut nmod_mpolyd_struct,
        dctx: *mut nmod_mpolyd_ctx_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_convert_from_nmod_mpolyd(
        A: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpolyd_struct,
        dctx: *mut nmod_mpolyd_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyd_init(poly: *mut nmod_mpolyd_struct, nvars: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_mpolyd_fit_length(poly: *mut nmod_mpolyd_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_mpolyd_zero(poly: *mut nmod_mpolyd_struct);
}
extern "C" {
    pub fn nmod_mpolyd_set_nvars(poly: *mut nmod_mpolyd_struct, nvars: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_mpolyd_set(A: *mut nmod_mpolyd_struct, B: *mut nmod_mpolyd_struct);
}
extern "C" {
    pub fn nmod_mpolyd_clear(poly: *mut nmod_mpolyd_struct);
}
extern "C" {
    pub fn nmod_mpolyd_print(poly: *mut nmod_mpolyd_struct);
}
extern "C" {
    pub fn nmod_mpolyd_length(A: *mut nmod_mpolyd_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpolyu_init(
        A: *mut nmod_mpolyu_struct,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_clear(A: *mut nmod_mpolyu_struct, uctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyu_swap(
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_zero(A: *mut nmod_mpolyu_struct, uctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyu_is_one(
        A: *mut nmod_mpolyu_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyu_print_pretty(
        poly: *mut nmod_mpolyu_struct,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_fit_length(
        A: *mut nmod_mpolyu_struct,
        length: mp_limb_signed_t,
        uctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_one(A: *mut nmod_mpolyu_struct, uctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyu_degrees_si(
        degs: *mut mp_limb_signed_t,
        A: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_repack_bits_inplace(
        A: *mut nmod_mpolyu_struct,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _nmod_mpolyu_get_coeff(
        A: *mut nmod_mpolyu_struct,
        pow: mp_limb_t,
        uctx: *mut nmod_mpoly_ctx_struct,
    ) -> *mut nmod_mpoly_struct;
}
extern "C" {
    pub fn nmod_mpolyu_shift_right(A: *mut nmod_mpolyu_struct, s: mp_limb_t);
}
extern "C" {
    pub fn nmod_mpolyu_shift_left(A: *mut nmod_mpolyu_struct, s: mp_limb_t);
}
extern "C" {
    pub fn nmod_mpolyu_content_mpoly(
        g: *mut nmod_mpoly_struct,
        A: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyu_scalar_mul_nmod(
        A: *mut nmod_mpolyu_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_set(
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_cvtto_poly(
        a: *mut nmod_poly_struct,
        A: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_cvtfrom_poly(
        A: *mut nmod_mpolyu_struct,
        a: *mut nmod_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_cvtfrom_poly_notmain(
        A: *mut nmod_mpolyu_struct,
        a: *mut nmod_poly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_to_mpolyu_perm_deflate_threaded_pool(
        A: *mut nmod_mpolyu_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_from_mpolyu_perm_inflate(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpolyu_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpolyuu_divides(
        Q: *mut nmod_mpolyu_struct,
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        nmainvars: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyu_divexact_mpoly_inplace(
        A: *mut nmod_mpolyu_struct,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_mul_mpoly(
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_mul_mpoly_inplace(
        A: *mut nmod_mpolyu_struct,
        c: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_setform(
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_gcdm_zippel(
        G: *mut nmod_mpolyu_struct,
        Abar: *mut nmod_mpolyu_struct,
        Bbar: *mut nmod_mpolyu_struct,
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyu_leadcoeff(
        A: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpolyn_init(
        A: *mut nmod_mpolyn_struct,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_clear(A: *mut nmod_mpolyn_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyn_swap(A: *mut nmod_mpolyn_struct, B: *mut nmod_mpolyn_struct);
}
extern "C" {
    pub fn nmod_mpolyn_zero(A: *mut nmod_mpolyn_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyn_is_zero(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_print_pretty(
        A: *mut nmod_mpolyn_struct,
        x_in: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_fit_length(
        A: *mut nmod_mpolyn_struct,
        length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_set_length(
        A: *mut nmod_mpolyn_struct,
        newlen: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_fit_bits(
        A: *mut nmod_mpolyn_struct,
        bits: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_is_canonical(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_set(
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_set_mpoly(
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_cvtfrom_mpolyn(
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_mul_poly(
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        c: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_cvtto_mpolyn(
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_leadcoeff(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpolyn_leadcoeff_poly(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> *mut n_poly_struct;
}
extern "C" {
    pub fn nmod_mpolyun_init(
        A: *mut nmod_mpolyun_struct,
        bits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_clear(A: *mut nmod_mpolyun_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyun_swap(A: *mut nmod_mpolyun_struct, B: *mut nmod_mpolyun_struct);
}
extern "C" {
    pub fn nmod_mpolyun_zero(A: *mut nmod_mpolyun_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyun_print_pretty(
        poly: *mut nmod_mpolyun_struct,
        x: *mut *const c_char,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_fit_length(
        A: *mut nmod_mpolyun_struct,
        length: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_is_canonical(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyun_shift_right(A: *mut nmod_mpolyun_struct, s: mp_limb_t);
}
extern "C" {
    pub fn nmod_mpolyun_shift_left(A: *mut nmod_mpolyun_struct, s: mp_limb_t);
}
extern "C" {
    pub fn nmod_mpolyn_lastdeg(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpolyun_lastdeg(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_mpolyun_set(
        A: *mut nmod_mpolyun_struct,
        B: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_one(A: *mut nmod_mpolyn_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyun_one(A: *mut nmod_mpolyun_struct, ctx: *mut nmod_mpoly_ctx_struct);
}
extern "C" {
    pub fn nmod_mpolyun_leadcoeff_last(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpolyn_set_mod(A: *mut nmod_mpolyn_struct, mod_: nmod_t);
}
extern "C" {
    pub fn nmod_mpolyun_set_mod(A: *mut nmod_mpolyun_struct, mod_: nmod_t);
}
extern "C" {
    pub fn nmod_mpolyn_is_nonzero_nmod(
        A: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyun_is_nonzero_nmod(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_scalar_mul_nmod(
        A: *mut nmod_mpolyn_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_scalar_mul_nmod(
        A: *mut nmod_mpolyun_struct,
        c: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_mul_last(
        A: *mut nmod_mpolyn_struct,
        b: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_mul_last(
        A: *mut nmod_mpolyun_struct,
        b: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_equal(
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyun_equal(
        A: *mut nmod_mpolyun_struct,
        B: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyu_cvtto_mpolyun(
        A: *mut nmod_mpolyun_struct,
        B: *mut nmod_mpolyu_struct,
        k: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyu_cvtfrom_mpolyun(
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyun_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_mul_poly(
        A: *mut nmod_mpolyun_struct,
        B: *mut nmod_mpolyun_struct,
        c: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_content_last(
        a: *mut n_poly_struct,
        B: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_content_last(
        a: *mut n_poly_struct,
        B: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_divexact_last(
        A: *mut nmod_mpolyn_struct,
        b: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_divexact_last(
        A: *mut nmod_mpolyun_struct,
        b: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_divides(
        Q: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_divides_threaded_pool(
        Q: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyun_divides(
        Q: *mut nmod_mpolyun_struct,
        A: *mut nmod_mpolyun_struct,
        B: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_to_mpolyun_perm_deflate_threaded_pool(
        A: *mut nmod_mpolyun_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_to_mpolyn_perm_deflate_threaded_pool(
        A: *mut nmod_mpolyn_struct,
        nctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_from_mpolyun_perm_inflate(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpolyun_struct,
        uctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_from_mpolyn_perm_inflate(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpolyn_struct,
        nctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpolyun_leadcoeff(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_mpolyun_leadcoeff_poly(
        A: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> *mut n_poly_struct;
}
extern "C" {
    pub fn mpoly_gcd_get_use_first(
        rGdeg: mp_limb_signed_t,
        Adeg: mp_limb_signed_t,
        Bdeg: mp_limb_signed_t,
        gammadeg: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_gcd_get_use_new(
        rGdeg: mp_limb_signed_t,
        Adeg: mp_limb_signed_t,
        Bdeg: mp_limb_signed_t,
        gammadeg: mp_limb_signed_t,
        degxAB: mp_limb_signed_t,
        degyAB: mp_limb_signed_t,
        numABgamma: mp_limb_signed_t,
        G: *mut n_polyun_struct,
        Abar: *mut n_polyun_struct,
        Bbar: *mut n_polyun_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyu_setform_mpolyun(
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyun_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_gcd_brown_smprime_bivar(
        G: *mut nmod_mpolyn_struct,
        Abar: *mut nmod_mpolyn_struct,
        Bbar: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        Sp: *mut nmod_poly_stack_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_gcd_brown_smprime(
        G: *mut nmod_mpolyn_struct,
        Abar: *mut nmod_mpolyn_struct,
        Bbar: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        I: *mut mpoly_gcd_info_struct,
        Sp: *mut nmod_poly_stack_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_gcd_brown_smprime_threaded_pool(
        G: *mut nmod_mpolyn_struct,
        Abar: *mut nmod_mpolyn_struct,
        Bbar: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        I: *mut mpoly_gcd_info_struct,
        handles: *const thread_pool_handle,
        num_workers: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_gcd_brown_lgprime(
        G: *mut nmod_mpolyn_struct,
        Abar: *mut nmod_mpolyn_struct,
        Bbar: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
pub const nmod_gcds_ret_t_nmod_gcds_success: nmod_gcds_ret_t = 0;
pub const nmod_gcds_ret_t_nmod_gcds_form_main_degree_too_high: nmod_gcds_ret_t = 1;
pub const nmod_gcds_ret_t_nmod_gcds_form_wrong: nmod_gcds_ret_t = 2;
pub const nmod_gcds_ret_t_nmod_gcds_no_solution: nmod_gcds_ret_t = 3;
pub const nmod_gcds_ret_t_nmod_gcds_scales_not_found: nmod_gcds_ret_t = 4;
pub const nmod_gcds_ret_t_nmod_gcds_eval_point_not_found: nmod_gcds_ret_t = 5;
pub const nmod_gcds_ret_t_nmod_gcds_eval_gcd_deg_too_high: nmod_gcds_ret_t = 6;
pub type nmod_gcds_ret_t = c_uint;
extern "C" {
    pub fn nmod_mpolyu_gcds_zippel(
        G: *mut nmod_mpolyu_struct,
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        f: *mut nmod_mpolyu_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
        degbound: *mut mp_limb_signed_t,
    ) -> nmod_gcds_ret_t;
}
extern "C" {
    pub fn nmod_mpolyu_gcdp_zippel(
        G: *mut nmod_mpolyu_struct,
        Abar: *mut nmod_mpolyu_struct,
        Bbar: *mut nmod_mpolyu_struct,
        A: *mut nmod_mpolyu_struct,
        B: *mut nmod_mpolyu_struct,
        var: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        randstate: *mut flint_rand_s,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpoly_to_mpolyl_perm_deflate(
        A: *mut nmod_mpoly_struct,
        lctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpoly_from_mpolyl_perm_inflate(
        A: *mut nmod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
        B: *mut nmod_mpoly_struct,
        lctx: *mut nmod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn nmod_mpolyl_gcd_zippel_smprime(
        rG: *mut nmod_mpoly_struct,
        rGdegs: *const mp_limb_signed_t,
        rAbar: *mut nmod_mpoly_struct,
        rBbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        Adegs: *const mp_limb_signed_t,
        B: *mut nmod_mpoly_struct,
        Bdegs: *const mp_limb_signed_t,
        gamma: *mut nmod_mpoly_struct,
        gammadegs: *const mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyl_gcd_zippel_lgprime(
        rG: *mut nmod_mpoly_struct,
        rGdegs: *const mp_limb_signed_t,
        rAbar: *mut nmod_mpoly_struct,
        rBbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        Adegs: *const mp_limb_signed_t,
        B: *mut nmod_mpoly_struct,
        Bdegs: *const mp_limb_signed_t,
        gamma: *mut nmod_mpoly_struct,
        gammadegs: *const mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyl_gcd_hensel_smprime(
        G: *mut nmod_mpoly_struct,
        Gdeg: mp_limb_signed_t,
        Abar: *mut nmod_mpoly_struct,
        Bbar: *mut nmod_mpoly_struct,
        A: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyl_gcd_hensel_medprime(
        G: *mut nmod_mpoly_struct,
        Gdeg: mp_limb_signed_t,
        Abar: *mut nmod_mpoly_struct,
        Bbar: *mut nmod_mpoly_struct,
        smA: *mut nmod_mpoly_struct,
        smB: *mut nmod_mpoly_struct,
        smctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _nmod_mpoly_monomial_evals_cache(
        E: *mut n_poly_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        betas: *const mp_limb_t,
        start: mp_limb_signed_t,
        stop: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _nmod_mpoly_monomial_evals2_cache(
        E: *mut n_polyun_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        betas: *const mp_limb_t,
        m: mp_limb_signed_t,
        ctx: *mut mpoly_ctx_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _nmod_poly_eval2_pow(
        vp: *mut mp_limb_t,
        vm: *mut mp_limb_t,
        P: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        fctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_reduce_2sm_poly(
        E: *mut n_poly_struct,
        F: *mut n_poly_struct,
        A: *mut nmod_mpolyn_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_lift_2sm_poly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        alpha: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_crt_2sm_poly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_interp_lift_sm_bpoly(
        F: *mut nmod_mpolyn_struct,
        A: *mut n_bpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_crt_sm_bpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        A: *mut n_bpoly_struct,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_interp_reduce_2sm_mpolyn(
        E: *mut nmod_mpolyn_struct,
        F: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        alphapow: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_lift_2sm_mpolyn(
        lastdeg: *mut mp_limb_signed_t,
        T: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        alpha: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_crt_2sm_mpolyn(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpolyn_struct,
        var: mp_limb_signed_t,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyun_interp_reduce_sm_mpolyu(
        B: *mut nmod_mpolyu_struct,
        A: *mut nmod_mpolyun_struct,
        alpha: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_lift_sm_mpoly(
        A: *mut nmod_mpolyn_struct,
        B: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyun_interp_lift_sm_mpolyu(
        A: *mut nmod_mpolyun_struct,
        B: *mut nmod_mpolyu_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpolyn_interp_crt_sm_mpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        T: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpoly_struct,
        modulus: *mut n_poly_struct,
        alpha: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyun_interp_crt_sm_mpolyu(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyun_struct,
        T: *mut nmod_mpolyun_struct,
        A: *mut nmod_mpolyu_struct,
        modulus: *mut n_poly_struct,
        alpha: mp_limb_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_mpolyn_interp_mcrt_sm_mpoly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut nmod_mpolyn_struct,
        A: *mut nmod_mpoly_struct,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_geobucket {
    pub polys: [nmod_mpoly_struct; 32usize],
    pub temps: [nmod_mpoly_struct; 32usize],
    pub length: mp_limb_signed_t,
}
pub type nmod_mpoly_geobucket_struct = nmod_mpoly_geobucket;
pub type nmod_mpoly_geobucket_t = [nmod_mpoly_geobucket_struct; 1usize];
extern "C" {
    pub fn nmod_mpoly_geobucket_init(
        B: *mut nmod_mpoly_geobucket_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_geobucket_clear(
        B: *mut nmod_mpoly_geobucket_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_geobucket_empty(
        p: *mut nmod_mpoly_struct,
        B: *mut nmod_mpoly_geobucket_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_geobucket_fit_length(
        B: *mut nmod_mpoly_geobucket_struct,
        i: mp_limb_signed_t,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_geobucket_set(
        B: *mut nmod_mpoly_geobucket_struct,
        p: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_geobucket_add(
        B: *mut nmod_mpoly_geobucket_struct,
        p: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_geobucket_sub(
        B: *mut nmod_mpoly_geobucket_struct,
        p: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_mpoly_remainder_strongtest(
        r: *mut nmod_mpoly_struct,
        g: *mut nmod_mpoly_struct,
        ctx: *mut nmod_mpoly_ctx_struct,
    );
}
