#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_vec.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq::{fq_ctx_struct, fq_struct};
use libc::{c_int, FILE};

extern "C" {
    pub fn _fq_vec_init(len: mp_limb_signed_t, ctx: *mut fq_ctx_struct) -> *mut fq_struct;
    pub fn _fq_vec_clear(vec: *mut fq_struct, len: mp_limb_signed_t, ctx: *mut fq_ctx_struct);
    pub fn _fq_vec_randtest(
        f: *mut fq_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_fprint(
        file: *mut FILE,
        vec: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_vec_print(
        vec: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_vec_set(
        v: *mut fq_struct,
        f: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_swap(
        vec1: *mut fq_struct,
        vec2: *mut fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_zero(v: *mut fq_struct, len: mp_limb_signed_t, ctx: *mut fq_ctx_struct);
    pub fn _fq_vec_neg(
        vec1: *mut fq_struct,
        vec2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_is_zero(
        vec: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_vec_equal(
        vec1: *const fq_struct,
        vec2: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_vec_add(
        res: *mut fq_struct,
        vec1: *const fq_struct,
        vec2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_sub(
        res: *mut fq_struct,
        vec1: *const fq_struct,
        vec2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_scalar_addmul_fq(
        poly1: *mut fq_struct,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_scalar_submul_fq(
        poly1: *mut fq_struct,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_scalar_mul_fq(
        poly1: *mut fq_struct,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_vec_dot(
        res: *mut fmpz_poly_struct,
        vec1: *const fq_struct,
        vec2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
}
