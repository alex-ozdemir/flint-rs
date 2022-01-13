#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/perm.html).

use crate::deps::*;
use crate::flint::*;
use libc::c_int;

extern "C" {
    pub fn _perm_init(n: mp_limb_signed_t) -> *mut mp_limb_signed_t;
    pub fn _perm_clear(vec: *mut mp_limb_signed_t);
    pub fn _perm_equal(
        vec1: *const mp_limb_signed_t,
        vec2: *const mp_limb_signed_t,
        n: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _perm_set(res: *mut mp_limb_signed_t, vec: *const mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn _perm_set_one(vec: *mut mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn _perm_inv(res: *mut mp_limb_signed_t, vec: *const mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn _perm_compose(
        res: *mut mp_limb_signed_t,
        vec1: *const mp_limb_signed_t,
        vec2: *const mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn _perm_randtest(
        vec: *mut mp_limb_signed_t,
        n: mp_limb_signed_t,
        state: *const flint_rand_s,
    ) -> c_int;
    pub fn _perm_parity(vec: *const mp_limb_signed_t, n: mp_limb_signed_t) -> c_int;
    pub fn _long_vec_print(vec: *const mp_limb_signed_t, len: mp_limb_signed_t) -> c_int;
    pub fn _perm_print(vec: *const mp_limb_signed_t, n: mp_limb_signed_t) -> c_int;
}
