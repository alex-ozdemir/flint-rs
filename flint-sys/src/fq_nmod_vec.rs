#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_nmod_vec.html).

use crate::deps::*;
use crate::flint::*;
use crate::fq_nmod::{fq_nmod_ctx_struct, fq_nmod_struct};
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_int, FILE};

extern "C" {
    pub fn _fq_nmod_vec_init(
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut fq_nmod_struct;
    pub fn _fq_nmod_vec_clear(
        vec: *mut fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_randtest(
        f: *mut fq_nmod_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_fprint(
        file: *mut FILE,
        vec: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_vec_print(
        vec: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_vec_set(
        v: *mut fq_nmod_struct,
        f: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_swap(
        vec1: *mut fq_nmod_struct,
        vec2: *mut fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_zero(
        v: *mut fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_neg(
        vec1: *mut fq_nmod_struct,
        vec2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_is_zero(
        vec: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_vec_equal(
        vec1: *const fq_nmod_struct,
        vec2: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_vec_add(
        res: *mut fq_nmod_struct,
        vec1: *const fq_nmod_struct,
        vec2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_sub(
        res: *mut fq_nmod_struct,
        vec1: *const fq_nmod_struct,
        vec2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_scalar_addmul_fq_nmod(
        poly1: *mut fq_nmod_struct,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_scalar_submul_fq_nmod(
        poly1: *mut fq_nmod_struct,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_scalar_mul_fq_nmod(
        poly1: *mut fq_nmod_struct,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_vec_dot(
        res: *mut nmod_poly_struct,
        vec1: *const fq_nmod_struct,
        vec2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
