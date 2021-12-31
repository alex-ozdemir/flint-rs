#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_zech_vec.html).

use crate::deps::*;
use crate::flint::*;
use crate::fq_zech::{fq_zech_ctx_struct, fq_zech_struct};
use libc::{c_int, FILE};

extern "C" {
    pub fn _fq_zech_vec_init(
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> *mut fq_zech_struct;
    pub fn _fq_zech_vec_clear(
        vec: *mut fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_randtest(
        f: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_fprint(
        file: *mut FILE,
        vec: *const fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn _fq_zech_vec_print(
        vec: *const fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn _fq_zech_vec_set(
        v: *mut fq_zech_struct,
        f: *const fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_swap(
        vec1: *mut fq_zech_struct,
        vec2: *mut fq_zech_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_zero(
        v: *mut fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_neg(
        vec1: *mut fq_zech_struct,
        vec2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_is_zero(
        vec: *const fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn _fq_zech_vec_equal(
        vec1: *const fq_zech_struct,
        vec2: *const fq_zech_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn _fq_zech_vec_add(
        res: *mut fq_zech_struct,
        vec1: *const fq_zech_struct,
        vec2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_sub(
        res: *mut fq_zech_struct,
        vec1: *const fq_zech_struct,
        vec2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_scalar_addmul_fq_zech(
        poly1: *mut fq_zech_struct,
        poly2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        x: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_scalar_submul_fq_zech(
        poly1: *mut fq_zech_struct,
        poly2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        x: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_scalar_mul_fq_zech(
        poly1: *mut fq_zech_struct,
        poly2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        x: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_vec_dot(
        res: *mut fq_zech_struct,
        vec1: *const fq_zech_struct,
        vec2: *const fq_zech_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
}
