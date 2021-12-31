#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/long_extras.html).

use crate::deps::*;
use crate::flint::*;
use libc::{c_int, size_t};

extern "C" {
    pub fn z_sizeinbase(n: mp_limb_signed_t, b: ::std::os::raw::c_int) -> size_t;
    pub fn z_mul_checked(
        a: *mut mp_limb_signed_t,
        b: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> c_int;
    pub fn z_add_checked(
        a: *mut mp_limb_signed_t,
        b: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> c_int;
    pub fn z_mat22_det_is_negative(
        m11: mp_limb_signed_t,
        m12: mp_limb_signed_t,
        m21: mp_limb_signed_t,
        m22: mp_limb_signed_t,
    ) -> c_int;
    pub fn z_randtest(state: *mut flint_rand_s) -> mp_limb_signed_t;
    pub fn z_randtest_not_zero(state: *mut flint_rand_s) -> mp_limb_signed_t;
    pub fn z_randint(state: *mut flint_rand_s, limit: mp_limb_t) -> mp_limb_signed_t;
}
