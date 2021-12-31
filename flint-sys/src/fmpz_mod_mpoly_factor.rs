#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_mpoly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpz_mod::fmpz_mod_ctx_struct;
use crate::fmpz_mod_mat::fmpz_mod_mat_struct;
use crate::fmpz_mod_mpoly::*;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::mpoly::*;
use crate::nmod_mpoly::nmod_mpoly_ctx_struct;
use crate::nmod_mpoly_factor::nmod_mpoly_factor_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_factor_struct {
    pub constant: fmpz_t,
    pub poly: *mut fmpz_mod_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type fmpz_mod_mpoly_factor_t = [fmpz_mod_mpoly_factor_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_mpoly_factor_init(
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_init2(
        f: *mut fmpz_mod_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_realloc(
        f: *mut fmpz_mod_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_fit_length(
        f: *mut fmpz_mod_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_clear(
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_length(
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_get_constant_fmpz(
        c: *mut fmpz,
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_get_base(
        p: *mut fmpz_mod_mpoly_struct,
        f: *mut fmpz_mod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_swap_base(
        p: *mut fmpz_mod_mpoly_struct,
        f: *mut fmpz_mod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_get_exp_si(
        f: *mut fmpz_mod_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_swap(
        f: *mut fmpz_mod_mpoly_factor_struct,
        g: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_set(
        f: *mut fmpz_mod_mpoly_factor_struct,
        g: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_print_pretty(
        f: *mut fmpz_mod_mpoly_factor_struct,
        vars: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_content(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_squarefree(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_sort(
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_cmp(
        A: *mut fmpz_mod_mpoly_factor_struct,
        B: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_poly_degree(a: *mut fmpz_mod_poly_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_poly_scalar_addmul_fmpz_mod(
        A: *mut fmpz_mod_poly_struct,
        B: *mut fmpz_mod_poly_struct,
        C: *mut fmpz_mod_poly_struct,
        d0: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_poly_addmul_linear(
        A: *mut fmpz_mod_poly_struct,
        B: *mut fmpz_mod_poly_struct,
        C: *mut fmpz_mod_poly_struct,
        d1: *mut fmpz,
        d0: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_poly_shift_left_scalar_addmul_fmpz_mod(
        A: *mut fmpz_mod_poly_struct,
        k: mp_limb_signed_t,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_poly_eval_pow(
        eval: *mut fmpz,
        P: *mut fmpz_mod_poly_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_poly_eval2_pow(
        evalp: *mut fmpz,
        evalm: *mut fmpz,
        P: *mut fmpz_mod_poly_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mat_is_reduced(N: *mut fmpz_mod_mat_struct) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mat_init_nullspace_tr(
        X: *mut fmpz_mod_mat_struct,
        tmp: *mut fmpz_mod_mat_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_bpoly_struct {
    pub coeffs: *mut fmpz_mod_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpz_mod_bpoly_t = [fmpz_mod_bpoly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_tpoly_struct {
    pub coeffs: *mut fmpz_mod_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpz_mod_tpoly_t = [fmpz_mod_tpoly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_polyu_struct {
    pub exps: *mut mp_limb_t,
    pub coeffs: *mut fmpz,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type fmpz_mod_polyu_t = [fmpz_mod_polyu_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_polyun_struct {
    pub coeffs: *mut fmpz_mod_poly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpz_mod_polyun_t = [fmpz_mod_polyun_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpolyu_struct {
    pub coeffs: *mut fmpz_mod_mpoly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type fmpz_mod_mpolyu_t = [fmpz_mod_mpolyu_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpolyn_struct {
    pub coeffs: *mut fmpz_mod_poly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_signed_t,
}
pub type fmpz_mod_mpolyn_t = [fmpz_mod_mpolyn_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_mpoly_factor_separable(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        sep: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_expand(
        A: *mut fmpz_mod_mpoly_struct,
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_matches(
        a: *mut fmpz_mod_mpoly_struct,
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_append_fmpz_swap(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        e: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_one(
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_get_lead0(
        c: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_set_lead0(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        c: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_factor_set_nmod_mpoly_factor(
        f: *mut fmpz_mod_mpoly_factor_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        nf: *mut nmod_mpoly_factor_struct,
        nctx: *mut nmod_mpoly_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_stack_struct {
    pub array: *mut *mut fmpz_mod_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
pub type fmpz_mod_poly_stack_t = [fmpz_mod_poly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_bpoly_stack_struct {
    pub array: *mut *mut fmpz_mod_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
pub type fmpz_mod_bpoly_stack_t = [fmpz_mod_bpoly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_polyun_stack_struct {
    pub array: *mut *mut fmpz_mod_polyun_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
pub type fmpz_mod_polyun_stack_t = [fmpz_mod_polyun_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpolyn_stack_struct {
    pub array: *mut *mut fmpz_mod_mpolyn_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
pub type fmpz_mod_mpolyn_stack_t = [fmpz_mod_mpolyn_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_bpoly_stack_struct {
    pub poly_stack: fmpz_mod_poly_stack_t,
    pub bpoly_stack: fmpz_mod_bpoly_stack_t,
}
pub type fmpz_mod_poly_bpoly_stack_t = [fmpz_mod_poly_bpoly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_polyun_stack_struct {
    pub poly_stack: fmpz_mod_poly_stack_t,
    pub polyun_stack: fmpz_mod_polyun_stack_t,
}
pub type fmpz_mod_poly_polyun_stack_t = [fmpz_mod_poly_polyun_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_polyun_mpolyn_stack_struct {
    pub poly_stack: fmpz_mod_poly_stack_t,
    pub polyun_stack: fmpz_mod_polyun_stack_t,
    pub mpolyn_stack: fmpz_mod_mpolyn_stack_t,
}
pub type fmpz_mod_poly_polyun_mpolyn_stack_t = [fmpz_mod_poly_polyun_mpolyn_stack_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_poly_stack_init(S: *mut fmpz_mod_poly_stack_struct);
}
extern "C" {
    pub fn fmpz_mod_poly_stack_clear(S: *mut fmpz_mod_poly_stack_struct);
}
extern "C" {
    pub fn fmpz_mod_poly_stack_fit_request(
        S: *mut fmpz_mod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut fmpz_mod_poly_struct;
}
extern "C" {
    pub fn fmpz_mod_poly_stack_request(
        S: *mut fmpz_mod_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut fmpz_mod_poly_struct;
}
extern "C" {
    pub fn fmpz_mod_poly_stack_take_top(
        S: *mut fmpz_mod_poly_stack_struct,
    ) -> *mut fmpz_mod_poly_struct;
}
extern "C" {
    pub fn fmpz_mod_poly_stack_give_back(S: *mut fmpz_mod_poly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_mod_poly_stack_size(S: *mut fmpz_mod_poly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_init(S: *mut fmpz_mod_bpoly_stack_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_clear(S: *mut fmpz_mod_bpoly_stack_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_fit_request(
        S: *mut fmpz_mod_bpoly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut fmpz_mod_bpoly_struct;
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_request(
        S: *mut fmpz_mod_bpoly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut fmpz_mod_bpoly_struct;
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_take_top(
        S: *mut fmpz_mod_bpoly_stack_struct,
    ) -> *mut fmpz_mod_bpoly_struct;
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_give_back(S: *mut fmpz_mod_bpoly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_mod_bpoly_stack_size(S: *mut fmpz_mod_bpoly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_init(S: *mut fmpz_mod_polyun_stack_struct);
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_clear(S: *mut fmpz_mod_polyun_stack_struct);
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_fit_request(
        S: *mut fmpz_mod_polyun_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut fmpz_mod_polyun_struct;
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_request(
        S: *mut fmpz_mod_polyun_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut fmpz_mod_polyun_struct;
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_take_top(
        S: *mut fmpz_mod_polyun_stack_struct,
    ) -> *mut fmpz_mod_polyun_struct;
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_give_back(
        S: *mut fmpz_mod_polyun_stack_struct,
        k: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mod_polyun_stack_size(S: *mut fmpz_mod_polyun_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_init(
        S: *mut fmpz_mod_mpolyn_stack_struct,
        bits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_clear(
        S: *mut fmpz_mod_mpolyn_stack_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_fit_request(
        S: *mut fmpz_mod_mpolyn_stack_struct,
        k: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> *mut *mut fmpz_mod_mpolyn_struct;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_request(
        S: *mut fmpz_mod_mpolyn_stack_struct,
        k: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> *mut *mut fmpz_mod_mpolyn_struct;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_take_top(
        S: *mut fmpz_mod_mpolyn_stack_struct,
    ) -> *mut fmpz_mod_mpolyn_struct;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_give_back(
        S: *mut fmpz_mod_mpolyn_stack_struct,
        k: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_stack_size(S: *mut fmpz_mod_mpolyn_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _fmpz_mod_poly_vec_max_degree(
        A: *const fmpz_mod_poly_struct,
        Alen: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _fmpz_mod_poly_vec_content(
        g: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        Alen: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_poly_vec_remove_content(
        g: *mut fmpz_mod_poly_struct,
        A: *mut fmpz_mod_poly_struct,
        Alen: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_poly_vec_mul_poly(
        A: *mut fmpz_mod_poly_struct,
        Alen: mp_limb_signed_t,
        g: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_poly_vec_divexact_poly(
        A: *mut fmpz_mod_poly_struct,
        Alen: mp_limb_signed_t,
        g: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_poly_vec_mul_fmpz_mod(
        A: *mut fmpz_mod_poly_struct,
        Alen: mp_limb_signed_t,
        g: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu1n_bidegree(A: *mut fmpz_mod_polyun_struct) -> mp_limb_t;
}
extern "C" {
    pub fn fmpz_mod_polyun_leadcoeff(A: *mut fmpz_mod_polyun_struct) -> *const fmpz;
}
extern "C" {
    pub fn fmpz_mod_polyun_swap(A: *mut fmpz_mod_polyun_struct, B: *mut fmpz_mod_polyun_struct);
}
extern "C" {
    pub fn fmpz_mod_polyun_is_canonical(
        A: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_polyun_init(A: *mut fmpz_mod_polyun_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_polyun_clear(A: *mut fmpz_mod_polyun_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_polyun_realloc(
        A: *mut fmpz_mod_polyun_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu2n_print_pretty(
        A: *mut fmpz_mod_polyun_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        varlast: *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyun_equal(
        A: *mut fmpz_mod_polyun_struct,
        B: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_polyun_set(
        A: *mut fmpz_mod_polyun_struct,
        B: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu3n_print_pretty(
        A: *mut fmpz_mod_polyun_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        var2: *const ::std::os::raw::c_char,
        varlast: *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu1n_print_pretty(
        A: *mut fmpz_mod_polyun_struct,
        var0: *const ::std::os::raw::c_char,
        varlast: *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyun_fit_length(
        A: *mut fmpz_mod_polyun_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyun_one(A: *mut fmpz_mod_polyun_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_polyu1n(
        A: *mut fmpz_mod_polyun_struct,
        B: *mut fmpz_mod_mpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_polyu1n(
        B: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_polyun_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_init(
        A: *mut fmpz_mod_mpolyn_struct,
        bits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_swap(
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_fit_length(
        A: *mut fmpz_mod_mpolyn_struct,
        length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_leadcoeff(A: *mut fmpz_mod_mpolyn_struct) -> *const fmpz;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_is_canonical(
        A: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_lastdeg(
        A: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_clear(
        A: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_one(A: *mut fmpz_mod_mpolyn_struct, ctx: *mut fmpz_mod_mpoly_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpolyn_scalar_mul_fmpz_mod(
        A: *mut fmpz_mod_mpolyn_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_equal(
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_print_pretty(
        poly: *mut fmpz_mod_mpolyn_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_cvtfrom_mpolyn(
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_cvtto_mpolyn(
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_to_mpolyn_perm_deflate(
        A: *mut fmpz_mod_mpolyn_struct,
        nctx: *mut fmpz_mod_mpoly_ctx_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_from_mpolyn_perm_inflate(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        nctx: *mut fmpz_mod_mpoly_ctx_struct,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_set(
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_is_nonzero_fmpz(
        A: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_divides(
        Q: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_polyu1n_interp_reduce_2sm_poly(
        E: *mut fmpz_mod_poly_struct,
        F: *mut fmpz_mod_poly_struct,
        A: *mut fmpz_mod_polyun_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu1n_interp_lift_2sm_poly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fmpz_mod_polyun_struct,
        A: *mut fmpz_mod_poly_struct,
        B: *mut fmpz_mod_poly_struct,
        alpha: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu1n_interp_crt_2sm_poly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fmpz_mod_polyun_struct,
        T: *mut fmpz_mod_polyun_struct,
        A: *mut fmpz_mod_poly_struct,
        B: *mut fmpz_mod_poly_struct,
        modulus: *mut fmpz_mod_poly_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_reduce_sm_poly(
        E: *mut fmpz_mod_poly_struct,
        A: *mut fmpz_mod_mpolyn_struct,
        alpha: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_lift_sm_poly(
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_crt_sm_poly(
        lastdeg_: *mut mp_limb_signed_t,
        F: *mut fmpz_mod_mpolyn_struct,
        T: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_poly_struct,
        modulus: *mut fmpz_mod_poly_struct,
        alpha: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_reduce_2sm_mpolyn(
        E: *mut fmpz_mod_mpolyn_struct,
        F: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpolyn_struct,
        var: mp_limb_signed_t,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_lift_2sm_mpolyn(
        lastdeg: *mut mp_limb_signed_t,
        T: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        var: mp_limb_signed_t,
        alpha: *mut fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_crt_2sm_mpolyn(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fmpz_mod_mpolyn_struct,
        T: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        var: mp_limb_signed_t,
        modulus: *mut fmpz_mod_poly_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_lift_sm_mpoly(
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_crt_sm_mpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fmpz_mod_mpolyn_struct,
        T: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpoly_struct,
        modulus: *mut fmpz_mod_poly_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_interp_mcrt_sm_mpoly(
        lastdeg: *mut mp_limb_signed_t,
        F: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpoly_struct,
        modulus: *mut fmpz_mod_poly_struct,
        alphapow: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_polyu_swap(A: *mut fmpz_mod_polyu_struct, B: *mut fmpz_mod_polyu_struct);
}
extern "C" {
    pub fn fmpz_mod_polyu_init(A: *mut fmpz_mod_polyu_struct);
}
extern "C" {
    pub fn fmpz_mod_polyu_clear(A: *mut fmpz_mod_polyu_struct);
}
extern "C" {
    pub fn fmpz_mod_polyu_realloc(A: *mut fmpz_mod_polyu_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_mod_polyu_fit_length(
        a: *mut fmpz_mod_polyu_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu3_degrees(
        deg0: *mut mp_limb_signed_t,
        deg1: *mut mp_limb_signed_t,
        deg2: *mut mp_limb_signed_t,
        A: *mut fmpz_mod_polyu_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu3_print_pretty(
        A: *mut fmpz_mod_polyu_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        var2: *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyu_is_canonical(
        A: *mut fmpz_mod_mpolyu_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyu3_print_pretty(
        A: *mut fmpz_mod_mpolyu_struct,
        var0: *const ::std::os::raw::c_char,
        var1: *const ::std::os::raw::c_char,
        var2: *const ::std::os::raw::c_char,
        vars: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_is_canonical(
        A: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_init(A: *mut fmpz_mod_bpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_clear(A: *mut fmpz_mod_bpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_swap(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_get_coeff(
        c: *mut fmpz,
        A: *mut fmpz_mod_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_degree0(
        A: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_bpoly_normalise(A: *mut fmpz_mod_bpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_equal(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_set(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_set_poly_gen1(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_set_poly_gen0(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_one(A: *mut fmpz_mod_bpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_is_one(
        A: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_degree1(
        A: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_bpoly_print_pretty(
        A: *mut fmpz_mod_bpoly_struct,
        xvar: *const ::std::os::raw::c_char,
        yvar: *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_fit_length(
        A: *mut fmpz_mod_bpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_set_coeff(
        A: *mut fmpz_mod_bpoly_struct,
        xi: mp_limb_signed_t,
        yi: mp_limb_signed_t,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_zero(A: *mut fmpz_mod_bpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_bpoly_reverse_vars(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_taylor_shift_gen1(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_sub(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        C: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_add(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        C: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_make_primitive(
        g: *mut fmpz_mod_poly_struct,
        A: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_mul(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        C: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_mul_series(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        C: *mut fmpz_mod_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_divrem_series(
        Q: *mut fmpz_mod_bpoly_struct,
        R: *mut fmpz_mod_bpoly_struct,
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_divides(
        Q: *mut fmpz_mod_bpoly_struct,
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_taylor_shift_gen0(
        A: *mut fmpz_mod_bpoly_struct,
        alpha: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_derivative_gen0(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_make_monic_series(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_tpoly_init(A: *mut fmpz_mod_tpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_tpoly_swap(
        A: *mut fmpz_mod_tpoly_struct,
        B: *mut fmpz_mod_tpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_tpoly_fit_length(
        A: *mut fmpz_mod_tpoly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_tpoly_clear(A: *mut fmpz_mod_tpoly_struct, ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_mpoly_get_fmpz_mod_bpoly(
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var0: mp_limb_signed_t,
        var1: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_set_fmpz_mod_bpoly(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fmpz_mod_bpoly_struct,
        var0: mp_limb_signed_t,
        var1: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_bpoly_factor_smprime(
        c: *mut fmpz_mod_poly_struct,
        F: *mut fmpz_mod_tpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        allow_shift: ::std::os::raw::c_int,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_zip_vand_solve(
        coeffs: *mut fmpz,
        monomials: *const fmpz,
        mlength: mp_limb_signed_t,
        evals: *const fmpz,
        elength: mp_limb_signed_t,
        master: *const fmpz,
        scratch: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_zip_eval_step(
        ev: *mut fmpz,
        cur: *mut fmpz,
        inc: *const fmpz,
        coeffs: *const fmpz,
        length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpolyv_struct {
    pub coeffs: *mut fmpz_mod_mpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpz_mod_mpolyv_t = [fmpz_mod_mpolyv_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_mpolyv_init(
        A: *mut fmpz_mod_mpolyv_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyv_swap(
        A: *mut fmpz_mod_mpolyv_struct,
        B: *mut fmpz_mod_mpolyv_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyv_clear(
        A: *mut fmpz_mod_mpolyv_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyv_print_pretty(
        poly: *mut fmpz_mod_mpolyv_struct,
        x: *mut *const ::std::os::raw::c_char,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyv_fit_length(
        A: *mut fmpz_mod_mpolyv_struct,
        length: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpolyv_set_coeff(
        A: *mut fmpz_mod_mpolyv_struct,
        i: mp_limb_signed_t,
        c: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_to_mpolyv(
        A: *mut fmpz_mod_mpolyv_struct,
        B: *mut fmpz_mod_mpoly_struct,
        xalpha: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_from_mpolyv(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fmpz_mod_mpolyv_struct,
        xalpha: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_vec_content_mpoly(
        g: *mut fmpz_mod_mpoly_struct,
        A: *const fmpz_mod_mpoly_struct,
        Alen: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_vec_divexact_mpoly(
        A: *mut fmpz_mod_mpoly_struct,
        Alen: mp_limb_signed_t,
        c: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_vec_mul_mpoly(
        A: *mut fmpz_mod_mpoly_struct,
        Alen: mp_limb_signed_t,
        c: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_factor_separable(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        sep: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_lcc_wang(
        lc_divs: *mut fmpz_mod_mpoly_struct,
        lcAfac: *mut fmpz_mod_mpoly_factor_struct,
        Auc: *mut fmpz_mod_poly_struct,
        Auf: *const fmpz_mod_bpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_irred_smprime_zassenhaus(
        fac: *mut fmpz_mod_mpolyv_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_irred_smprime_wang(
        fac: *mut fmpz_mod_mpolyv_struct,
        A: *mut fmpz_mod_mpoly_struct,
        lcAfac: *mut fmpz_mod_mpoly_factor_struct,
        lcA: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_irred_smprime_zippel(
        fac: *mut fmpz_mod_mpolyv_struct,
        A: *mut fmpz_mod_mpoly_struct,
        lcAfac: *mut fmpz_mod_mpoly_factor_struct,
        lcA: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_compression_do(
        L: *mut fmpz_mod_mpoly_struct,
        Lctx: *mut fmpz_mod_mpoly_ctx_struct,
        Acoeffs: *mut fmpz,
        Alen: mp_limb_signed_t,
        M: *mut mpoly_compression_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_compression_undo(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        Actx: *mut fmpz_mod_mpoly_ctx_struct,
        L: *mut fmpz_mod_mpoly_struct,
        Lctx: *mut fmpz_mod_mpoly_ctx_struct,
        M: *mut mpoly_compression_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_pfrac_struct {
    pub bits: mp_limb_t,
    pub w: mp_limb_signed_t,
    pub r: mp_limb_signed_t,
    pub inv_prod_dbetas: *mut fmpz_mod_poly_struct,
    pub inv_prod_dbetas_mvar: *mut fmpz_mod_mpoly_struct,
    pub dbetas: *mut fmpz_mod_poly_struct,
    pub dbetas_mvar: *mut fmpz_mod_mpoly_struct,
    pub prod_mbetas: *mut fmpz_mod_mpoly_struct,
    pub prod_mbetas_coeffs: *mut fmpz_mod_mpolyv_struct,
    pub mbetas: *mut fmpz_mod_mpoly_struct,
    pub deltas: *mut fmpz_mod_mpoly_struct,
    pub xalpha: *mut fmpz_mod_mpoly_struct,
    pub q: *mut fmpz_mod_mpoly_struct,
    pub G: *mut fmpz_mod_mpoly_geobucket_struct,
    pub qt: *mut fmpz_mod_mpoly_struct,
    pub newt: *mut fmpz_mod_mpoly_struct,
    pub delta_coeffs: *mut fmpz_mod_mpolyv_struct,
    pub T: fmpz_mod_mpoly_t,
    pub Q: fmpz_mod_mpoly_t,
    pub R: fmpz_mod_mpoly_t,
}
pub type fmpz_mod_mpoly_pfrac_t = [fmpz_mod_mpoly_pfrac_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_mpoly_pfrac_init(
        I: *mut fmpz_mod_mpoly_pfrac_struct,
        bits: mp_limb_t,
        l: mp_limb_signed_t,
        r: mp_limb_signed_t,
        betas: *const fmpz_mod_mpoly_struct,
        alpha: *const fmpz,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_pfrac_clear(
        I: *mut fmpz_mod_mpoly_pfrac_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_pfrac(
        r: mp_limb_signed_t,
        t: *mut fmpz_mod_mpoly_struct,
        deg: *const mp_limb_signed_t,
        I: *mut fmpz_mod_mpoly_pfrac_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_hlift(
        m: mp_limb_signed_t,
        f: *mut fmpz_mod_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_pfrac(
        r: mp_limb_signed_t,
        C: *mut fmpz_mod_bpoly_struct,
        C_deg1_bound: *mut mp_limb_signed_t,
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_hlift2(
        A: *mut fmpz_mod_bpoly_struct,
        B0: *mut fmpz_mod_bpoly_struct,
        B1: *mut fmpz_mod_bpoly_struct,
        alpha: *mut fmpz,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
        St: *mut fmpz_mod_poly_bpoly_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_bpoly_hlift(
        r: mp_limb_signed_t,
        A: *mut fmpz_mod_bpoly_struct,
        B: *mut fmpz_mod_bpoly_struct,
        alpha: *mut fmpz,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
        St: *mut fmpz_mod_poly_bpoly_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_polyu3_hlift(
        r: mp_limb_signed_t,
        BB: *mut fmpz_mod_polyun_struct,
        A: *mut fmpz_mod_polyu_struct,
        B: *mut fmpz_mod_polyu_struct,
        beta: *mut fmpz,
        degree_inner: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_hlift_zippel(
        m: mp_limb_signed_t,
        B: *mut fmpz_mod_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fmpz,
        A: *mut fmpz_mod_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_algo(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        algo: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_zassenhaus(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_wang(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpoly_factor_zippel(
        f: *mut fmpz_mod_mpoly_factor_struct,
        A: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_evaluate_rest_fmpz_mod_poly(
        E: *mut fmpz_mod_poly_struct,
        starts: *mut mp_limb_signed_t,
        ends: *mut mp_limb_signed_t,
        stops: *mut mp_limb_signed_t,
        es: *mut mp_limb_t,
        Acoeffs: *const fmpz,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        alphas: *const fmpz_mod_poly_struct,
        offsets: *const mp_limb_signed_t,
        shifts: *const mp_limb_signed_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
        nvars: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_mod_mpoly_eval_rest_to_fmpz_mod_bpoly(
        E: *mut fmpz_mod_bpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        alphabetas: *const fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_set_fmpz_mod_bpoly_var1_zero(
        A: *mut fmpz_mod_mpoly_struct,
        Abits: mp_limb_t,
        B: *mut fmpz_mod_bpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mpoly_gcd_algo(
        G: *mut fmpz_mod_mpoly_struct,
        Abar: *mut fmpz_mod_mpoly_struct,
        Bbar: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        algo: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_polyu1n_gcd_brown_smprime(
        G: *mut fmpz_mod_polyun_struct,
        Abar: *mut fmpz_mod_polyun_struct,
        Bbar: *mut fmpz_mod_polyun_struct,
        A: *mut fmpz_mod_polyun_struct,
        B: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_ctx_struct,
        St_poly: *mut fmpz_mod_poly_stack_struct,
        St_polyun: *mut fmpz_mod_polyun_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyn_gcd_brown_smprime(
        G: *mut fmpz_mod_mpolyn_struct,
        Abar: *mut fmpz_mod_mpolyn_struct,
        Bbar: *mut fmpz_mod_mpolyn_struct,
        A: *mut fmpz_mod_mpolyn_struct,
        B: *mut fmpz_mod_mpolyn_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        I: *mut mpoly_gcd_info_struct,
        St: *mut fmpz_mod_poly_polyun_mpolyn_stack_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyl_gcdp_zippel(
        G: *mut fmpz_mod_mpoly_struct,
        Abar: *mut fmpz_mod_mpoly_struct,
        Bbar: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyl_gcd_zippel2_smprime(
        rG: *mut fmpz_mod_mpoly_struct,
        rGdegs: *const mp_limb_signed_t,
        rAbar: *mut fmpz_mod_mpoly_struct,
        rBbar: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        Adegs: *const mp_limb_signed_t,
        B: *mut fmpz_mod_mpoly_struct,
        Bdegs: *const mp_limb_signed_t,
        gamma: *mut fmpz_mod_mpoly_struct,
        gammadegs: *const mp_limb_signed_t,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_mpolyl_gcd_hensel_smprime(
        G: *mut fmpz_mod_mpoly_struct,
        Gdeg: mp_limb_signed_t,
        Abar: *mut fmpz_mod_mpoly_struct,
        Bbar: *mut fmpz_mod_mpoly_struct,
        A: *mut fmpz_mod_mpoly_struct,
        B: *mut fmpz_mod_mpoly_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_pow_cache_start(
        b: *mut fmpz,
        c: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_pow_cache_mulpow_ui(
        a: *mut fmpz,
        b: *mut fmpz,
        e: mp_limb_t,
        c: *mut fmpz_mod_poly_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomial_evals_fmpz_mod(
        EH: *mut fmpz_mod_poly_struct,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        alpha_caches: *mut fmpz_mod_poly_struct,
        start: mp_limb_signed_t,
        stop: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        fpctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly1_monomial_evals_fmpz_mod(
        EH: *mut fmpz_mod_polyun_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Amarks: *const mp_limb_t,
        Amarkslen: mp_limb_signed_t,
        alpha_caches: *mut fmpz_mod_poly_struct,
        m: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        fpctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly2_monomial_evals_fmpz_mod(
        EH: *mut fmpz_mod_polyun_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Amarks: *mut mp_limb_t,
        Amarkslen: mp_limb_signed_t,
        alpha_caches: *mut fmpz_mod_poly_struct,
        m: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
        fpctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mpoly_mock_eval_coeff(
        mock: *mut fmpz_mod_polyun_struct,
        A: *mut fmpz_mod_mpoly_struct,
        Aeh_inc: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyun_product_roots(
        M: *mut fmpz_mod_polyun_struct,
        H: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mod_polyun_zip_start(
        Z: *mut fmpz_mod_polyun_struct,
        H: *mut fmpz_mod_polyun_struct,
        req_images: mp_limb_signed_t,
        fctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyu2n_zip_eval_cur_inc_coeff(
        E: *mut fmpz_mod_polyun_struct,
        Acur: *mut fmpz_mod_polyun_struct,
        Ainc: *mut fmpz_mod_polyun_struct,
        Acoeff: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_polyun_zip_solve(
        A: *mut fmpz_mod_mpoly_struct,
        Z: *mut fmpz_mod_polyun_struct,
        H: *mut fmpz_mod_polyun_struct,
        M: *mut fmpz_mod_polyun_struct,
        ctx: *mut fmpz_mod_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
