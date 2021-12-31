#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpq_mpoly_factor.html).

use crate::deps::*;
use crate::fmpq::{fmpq, fmpq_t};
use crate::fmpq_mpoly::*;
use crate::fmpz::fmpz;
use crate::fmpz_mpoly_factor::fmpz_mpoly_factor_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpq_mpoly_factor_struct {
    pub constant: fmpq_t,
    pub poly: *mut fmpq_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type fmpq_mpoly_factor_t = [fmpq_mpoly_factor_struct; 1usize];
extern "C" {
    pub fn fmpq_mpoly_factor_init(
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_realloc(
        f: *mut fmpq_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_fit_length(
        f: *mut fmpq_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_clear(
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_length(
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_factor_get_constant_fmpq(
        c: *mut fmpq,
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_get_base(
        p: *mut fmpq_mpoly_struct,
        f: *mut fmpq_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_swap_base(
        p: *mut fmpq_mpoly_struct,
        f: *mut fmpq_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_get_exp_si(
        f: *mut fmpq_mpoly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpq_mpoly_factor_sort(
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_make_monic(
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_factor_make_integral(
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_factor_squarefree(
        f: *mut fmpq_mpoly_factor_struct,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpq_mpoly_factor(
        f: *mut fmpq_mpoly_factor_struct,
        A: *mut fmpq_mpoly_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpq_mpoly_factor_swap_fmpz_mpoly_factor(
        f: *mut fmpq_mpoly_factor_struct,
        g: *mut fmpz_mpoly_factor_struct,
        c: *mut fmpq,
        ctx: *mut fmpq_mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn fmpq_mpoly_factor_expand(
        A: *mut fmpq_mpoly_struct,
        f: *mut fmpq_mpoly_factor_struct,
        ctx: *mut fmpq_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
