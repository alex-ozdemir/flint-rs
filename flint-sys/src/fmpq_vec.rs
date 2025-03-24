/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::flint::*;


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
    pub fn _fmpq_vec_get_fmpz_vec_fmpz(
        num: *mut fmpz,
        den: *mut fmpz,
        a: *const fmpq,
        len: mp_limb_signed_t,
    );
    pub fn _fmpq_vec_dot(
        res: *mut fmpq,
        vec1: *const fmpq,
        vec2: *const fmpq,
        len: mp_limb_signed_t,
    );
    pub fn _fmpq_vec_fprint(
        file: *mut FILE,
        vec: *const fmpq,
        len: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn _fmpq_vec_print(vec: *const fmpq, len: mp_limb_signed_t) -> libc::c_int;
}
