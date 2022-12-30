#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_poly_q.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fmpz_poly_q_struct {
    pub num: *mut fmpz_poly_struct,
    pub den: *mut fmpz_poly_struct,
}

pub type fmpz_poly_q_t = [fmpz_poly_q_struct; 1usize];

extern "C" {
    pub fn fmpz_poly_q_canonicalise(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_is_canonical(op: *const fmpz_poly_q_struct) -> c_int;
    pub fn fmpz_poly_q_init(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_clear(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_randtest(
        poly: *mut fmpz_poly_q_struct,
        state: *const flint_rand_s,
        len1: mp_limb_signed_t,
        bits1: mp_limb_t,
        len2: mp_limb_signed_t,
        bits2: mp_limb_t,
    );
    pub fn fmpz_poly_q_randtest_not_zero(
        poly: *mut fmpz_poly_q_struct,
        state: *const flint_rand_s,
        len1: mp_limb_signed_t,
        bits1: mp_limb_t,
        len2: mp_limb_signed_t,
        bits2: mp_limb_t,
    );
    pub fn fmpz_poly_q_set(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_set_si(rop: *mut fmpz_poly_q_struct, op: mp_limb_signed_t);
    pub fn fmpz_poly_q_swap(op1: *mut fmpz_poly_q_struct, op2: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_zero(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_one(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_neg(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_inv(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_is_zero(op: *const fmpz_poly_q_struct) -> c_int;
    pub fn fmpz_poly_q_is_one(op: *const fmpz_poly_q_struct) -> c_int;
    pub fn fmpz_poly_q_equal(
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    ) -> c_int;
    pub fn fmpz_poly_q_add_in_place(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_sub_in_place(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_add(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_sub(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_addmul(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_submul(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_scalar_mul_si(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_q_scalar_mul_mpz(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const __mpz_struct,
    );
    pub fn fmpz_poly_q_scalar_mul_mpq(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const __mpq_struct,
    );
    pub fn fmpz_poly_q_scalar_div_si(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_q_scalar_div_mpz(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const __mpz_struct,
    );
    pub fn fmpz_poly_q_scalar_div_mpq(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const __mpq_struct,
    );
    pub fn fmpz_poly_q_mul(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_div(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_pow(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_poly_q_derivative(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_evaluate(
        rop: *mut __mpq_struct,
        f: *const fmpz_poly_q_struct,
        a: *const __mpq_struct,
    ) -> c_int;
    pub fn fmpz_poly_q_set_str(rop: *mut fmpz_poly_q_struct, s: *const c_char) -> c_int;
    pub fn fmpz_poly_q_get_str(op: *const fmpz_poly_q_struct) -> *const c_char;
    pub fn fmpz_poly_q_get_str_pretty(
        op: *const fmpz_poly_q_struct,
        x: *const c_char,
    ) -> *const c_char;
    pub fn fmpz_poly_q_print(op: *const fmpz_poly_q_struct) -> c_int;
    pub fn fmpz_poly_q_print_pretty(op: *const fmpz_poly_q_struct, x: *const c_char) -> c_int;
}
