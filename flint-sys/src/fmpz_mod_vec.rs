#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_vec.html).

use crate::deps::*;
use crate::fmpz::fmpz;
use crate::fmpz_mod::fmpz_mod_ctx_struct;

extern "C" {
    pub fn _fmpz_mod_vec_set_fmpz_vec(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_neg(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_sub(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        n: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_scalar_mul_fmpz_mod(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_mul(
        A: *mut fmpz,
        B: *const fmpz,
        C: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_scalar_div_fmpz_mod(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        c: *mut fmpz,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_dot(
        d: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_vec_dot_rev(
        r: *mut fmpz,
        a: *const fmpz,
        b: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut fmpz_mod_ctx_struct,
    );
}
