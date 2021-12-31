#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpq_vec.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::fmpz;
use libc::{c_int, FILE};

extern "C" {
    pub fn _fmpq_vec_init(len: mp_limb_signed_t) -> *mut fmpq;
    pub fn _fmpq_vec_clear(vec: *mut fmpq, len: mp_limb_signed_t);

    pub fn _fmpq_vec_randtest(
        f: *mut fmpq,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn _fmpq_vec_randtest_uniq_sorted(
        vec: *mut fmpq,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn _fmpq_vec_sort(vec: *mut fmpq, len: mp_limb_signed_t);
    pub fn _fmpq_vec_set_fmpz_vec(res: *mut fmpq, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpq_vec_dot(
        res: *mut fmpq,
        vec1: *const fmpq,
        vec2: *const fmpq,
        len: mp_limb_signed_t,
    );
    pub fn _fmpq_vec_fprint(file: *mut FILE, vec: *const fmpq, len: mp_limb_signed_t) -> c_int;
    pub fn _fmpq_vec_print(vec: *const fmpq, len: mp_limb_signed_t) -> c_int;
}
