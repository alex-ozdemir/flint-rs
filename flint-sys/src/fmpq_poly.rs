#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpq_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::{fmpz, fmpz_preinvn_struct, fmpz_t};
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fmpq_poly_struct {
    pub coeffs: *mut fmpz,
    pub den: fmpz_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpq_poly_t = [fmpq_poly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpq_poly_powers_precomp_struct {
    pub powers: *mut fmpq_poly_struct,
    pub len: mp_limb_signed_t,
}

pub type fmpq_poly_powers_precomp_t = [fmpq_poly_powers_precomp_struct; 1usize];

extern "C" {
    pub fn fmpq_poly_init(poly: *mut fmpq_poly_struct);
    pub fn fmpq_poly_init2(poly: *mut fmpq_poly_struct, alloc: mp_limb_signed_t);
    pub fn fmpq_poly_realloc(poly: *mut fmpq_poly_struct, alloc: mp_limb_signed_t);
    pub fn fmpq_poly_fit_length(poly: *mut fmpq_poly_struct, len: mp_limb_signed_t);
    pub fn _fmpq_poly_set_length(poly: *mut fmpq_poly_struct, len: mp_limb_signed_t);
    pub fn fmpq_poly_clear(poly: *mut fmpq_poly_struct);
    pub fn _fmpq_poly_normalise(poly: *mut fmpq_poly_struct);
    pub fn fmpq_poly_get_numerator(res: *mut fmpz_poly_struct, poly: *const fmpq_poly_struct);
    pub fn fmpq_poly_get_denominator(den: *mut fmpz, poly: *const fmpq_poly_struct);
    pub fn _fmpq_poly_canonicalise(rpoly: *mut fmpz, den: *mut fmpz, len: mp_limb_signed_t);
    pub fn fmpq_poly_canonicalise(poly: *mut fmpq_poly_struct);
    pub fn _fmpq_poly_is_canonical(
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_is_canonical(poly: *const fmpq_poly_struct) -> c_int;
    pub fn fmpq_poly_degree(poly: *const fmpq_poly_struct) -> mp_limb_signed_t;
    pub fn fmpq_poly_length(poly: *const fmpq_poly_struct) -> mp_limb_signed_t;
    pub fn fmpq_poly_randtest(
        f: *mut fmpq_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits_in: mp_limb_t,
    );
    pub fn fmpq_poly_randtest_unsigned(
        f: *mut fmpq_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits_in: mp_limb_t,
    );
    pub fn fmpq_poly_randtest_not_zero(
        f: *mut fmpq_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits_in: mp_limb_t,
    );
    pub fn fmpq_poly_set(poly1: *mut fmpq_poly_struct, poly2: *const fmpq_poly_struct);
    pub fn fmpq_poly_set_si(poly: *mut fmpq_poly_struct, x: mp_limb_signed_t);
    pub fn fmpq_poly_set_ui(poly: *mut fmpq_poly_struct, x: mp_limb_t);
    pub fn fmpq_poly_set_fmpz(poly: *mut fmpq_poly_struct, x: *const fmpz);
    pub fn fmpq_poly_set_fmpq(poly: *mut fmpq_poly_struct, x: *const fmpq);
    pub fn fmpq_poly_set_mpz(poly: *mut fmpq_poly_struct, x: *const __mpz_struct);
    pub fn fmpq_poly_set_mpq(poly: *mut fmpq_poly_struct, x: *const __mpq_struct);
    pub fn fmpq_poly_set_fmpz_poly(rop: *mut fmpq_poly_struct, op: *const fmpz_poly_struct);
    //pub fn _fmpq_poly_get_nmod_poly(rop: *mut nmod_poly_struct, op: *const fmpq_poly_struct);
    /*
    pub fn fmpq_poly_get_nmod_poly_den(
        rop: *mut nmod_poly_struct,
        op: *const fmpq_poly_struct,
        den: c_int,
    );
    pub fn fmpq_poly_get_nmod_poly(rop: *mut nmod_poly_struct, op: *const fmpq_poly_struct);
    pub fn fmpq_poly_set_nmod_poly(rop: *mut fmpq_poly_struct, op: *const nmod_poly_struct);
    */
    pub fn _fmpq_poly_set_array_mpq(
        poly: *mut fmpz,
        den: *mut fmpz,
        a: *const mpq_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_set_array_mpq(
        poly: *mut fmpq_poly_struct,
        a: *const mpq_t,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_set_str(
        poly: *mut fmpz,
        den: *mut fmpz,
        str_: *const c_char,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_set_str(poly: *mut fmpq_poly_struct, str_: *const c_char) -> c_int;
    pub fn fmpq_poly_get_str(poly: *const fmpq_poly_struct) -> *mut c_char;
    pub fn fmpq_poly_get_str_pretty(
        poly: *const fmpq_poly_struct,
        var: *const c_char,
    ) -> *mut c_char;
    pub fn _fmpq_poly_get_str_pretty(
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        var: *const c_char,
    ) -> *mut c_char;
    pub fn fmpq_poly_zero(poly: *mut fmpq_poly_struct);
    pub fn fmpq_poly_one(poly: *mut fmpq_poly_struct);
    pub fn fmpq_poly_neg(poly1: *mut fmpq_poly_struct, poly2: *const fmpq_poly_struct);
    pub fn fmpq_poly_inv(poly1: *mut fmpq_poly_struct, poly2: *const fmpq_poly_struct);
    pub fn fmpq_poly_swap(poly1: *mut fmpq_poly_struct, poly2: *mut fmpq_poly_struct);
    pub fn fmpq_poly_truncate(poly: *mut fmpq_poly_struct, n: mp_limb_signed_t);
    pub fn fmpq_poly_set_trunc(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_get_slice(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    );
    pub fn fmpq_poly_reverse(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_get_coeff_fmpz(
        x: *mut fmpz,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_get_coeff_fmpq(
        x: *mut fmpq,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_get_coeff_mpq(
        x: *mut __mpq_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_set_coeff_si(
        poly: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_signed_t,
    );
    pub fn fmpq_poly_set_coeff_ui(poly: *mut fmpq_poly_struct, n: mp_limb_signed_t, x: mp_limb_t);
    pub fn fmpq_poly_set_coeff_fmpz(
        poly: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
        x: *const fmpz,
    );
    pub fn fmpq_poly_set_coeff_fmpq(
        poly: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
        x: *const fmpq,
    );
    pub fn fmpq_poly_set_coeff_mpz(
        poly: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
        x: *const __mpz_struct,
    );
    pub fn fmpq_poly_set_coeff_mpq(
        poly: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
        x: *const __mpq_struct,
    );
    pub fn fmpq_poly_equal(poly1: *const fmpq_poly_struct, poly2: *const fmpq_poly_struct)
        -> c_int;
    pub fn _fmpq_poly_cmp(
        lpoly: *const fmpz,
        lden: *const fmpz,
        rpoly: *const fmpz,
        rden: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_cmp(left: *const fmpq_poly_struct, right: *const fmpq_poly_struct) -> c_int;
    pub fn _fmpq_poly_equal_trunc(
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_equal_trunc(
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_is_zero(poly: *const fmpq_poly_struct) -> c_int;
    pub fn fmpq_poly_is_one(poly: *const fmpq_poly_struct) -> c_int;
    pub fn fmpq_poly_is_gen(op: *const fmpq_poly_struct) -> c_int;
    pub fn fmpq_poly_add_si(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sub_si(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpq_poly_si_sub(
        res: *mut fmpq_poly_struct,
        c: mp_limb_signed_t,
        poly: *const fmpq_poly_struct,
    );
    pub fn fmpq_poly_add_fmpz(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        c: *const fmpz,
    );
    pub fn fmpq_poly_sub_fmpz(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        c: *const fmpz,
    );
    pub fn fmpq_poly_fmpz_sub(
        res: *mut fmpq_poly_struct,
        c: *const fmpz,
        poly: *const fmpq_poly_struct,
    );
    pub fn fmpq_poly_add_fmpq(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        c: *const fmpq,
    );
    pub fn fmpq_poly_sub_fmpq(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        c: *const fmpq,
    );
    pub fn fmpq_poly_fmpq_sub(
        res: *mut fmpq_poly_struct,
        c: *const fmpq,
        poly: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_add(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpq_poly_add(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_add_can(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        can: c_int,
    );
    pub fn fmpq_poly_add_can(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        can: c_int,
    );
    pub fn _fmpq_poly_add_series(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_add_series(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_add_series_can(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        can: c_int,
    );
    pub fn fmpq_poly_add_series_can(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
        can: c_int,
    );
    pub fn _fmpq_poly_sub(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sub(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_sub_can(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        can: c_int,
    );
    pub fn fmpq_poly_sub_can(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        can: c_int,
    );
    pub fn _fmpq_poly_sub_series(
        rpoly: *mut fmpz,
        rden: *const fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sub_series(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_sub_series_can(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        can: c_int,
    );
    pub fn fmpq_poly_sub_series_can(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
        can: c_int,
    );
    pub fn _fmpq_poly_scalar_mul_si(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_scalar_mul_ui(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        c: mp_limb_t,
    );
    pub fn _fmpq_poly_scalar_mul_fmpz(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        c: *const fmpz,
    );
    pub fn _fmpq_poly_scalar_mul_fmpq(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        r: *const fmpz,
        s: *const fmpz,
    );
    pub fn fmpq_poly_scalar_mul_si(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpq_poly_scalar_mul_ui(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: mp_limb_t,
    );
    pub fn fmpq_poly_scalar_mul_fmpz(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const fmpz,
    );
    pub fn fmpq_poly_scalar_mul_fmpq(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const fmpq,
    );
    pub fn fmpq_poly_scalar_mul_mpz(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const __mpz_struct,
    );
    pub fn fmpq_poly_scalar_mul_mpq(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const __mpq_struct,
    );
    pub fn _fmpq_poly_scalar_div_si(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_scalar_div_ui(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        c: mp_limb_t,
    );
    pub fn _fmpq_poly_scalar_div_fmpz(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        c: *const fmpz,
    );
    pub fn _fmpq_poly_scalar_div_fmpq(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        r: *const fmpz,
        s: *const fmpz,
    );
    pub fn fmpq_poly_scalar_div_si(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpq_poly_scalar_div_ui(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: mp_limb_t,
    );
    pub fn fmpq_poly_scalar_div_fmpz(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const fmpz,
    );
    pub fn fmpq_poly_scalar_div_fmpq(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const fmpq,
    );
    pub fn fmpq_poly_scalar_div_mpz(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const __mpz_struct,
    );
    pub fn fmpq_poly_scalar_div_mpq(
        rop: *mut fmpq_poly_struct,
        op: *const fmpq_poly_struct,
        c: *const __mpq_struct,
    );
    pub fn _fmpq_poly_mul(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpq_poly_mul(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_mullow(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_mullow(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_addmul(
        rop: *mut fmpq_poly_struct,
        op1: *const fmpq_poly_struct,
        op2: *const fmpq_poly_struct,
    );
    pub fn fmpq_poly_submul(
        rop: *mut fmpq_poly_struct,
        op1: *const fmpq_poly_struct,
        op2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_pow(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
    pub fn fmpq_poly_pow(rpoly: *mut fmpq_poly_struct, poly: *const fmpq_poly_struct, e: mp_limb_t);
    pub fn _fmpq_poly_pow_trunc(
        res: *mut fmpz,
        resden: *mut fmpz,
        f: *const fmpz,
        fden: *const fmpz,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_pow_trunc(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_shift_left(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_shift_right(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_divrem(
        Q: *mut fmpz,
        q: *mut fmpz,
        R: *const fmpz,
        r: *const fmpz,
        A: *const fmpz,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpq_poly_divrem(
        Q: *mut fmpq_poly_struct,
        R: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_div(
        Q: *mut fmpz,
        q: *mut fmpz,
        A: *const fmpz,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpq_poly_div(
        Q: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_rem(
        R: *mut fmpz,
        r: *mut fmpz,
        A: *const fmpz,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpq_poly_rem(
        R: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_powers_precompute(
        B: *const fmpz,
        denB: *mut fmpz,
        len: mp_limb_signed_t,
    ) -> *mut fmpq_poly_struct;
    pub fn fmpq_poly_powers_precompute(
        pinv: *mut fmpq_poly_powers_precomp_struct,
        poly: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_powers_clear(powers: *const fmpq_poly_struct, len: mp_limb_signed_t);
    pub fn fmpq_poly_powers_clear(pinv: *const fmpq_poly_powers_precomp_struct);
    pub fn _fmpq_poly_rem_powers_precomp(
        A: *mut fmpz,
        denA: *mut fmpz,
        m: mp_limb_signed_t,
        B: *const fmpz,
        denB: *const fmpz,
        n: mp_limb_signed_t,
        powers: *const fmpq_poly_struct,
    );
    pub fn fmpq_poly_rem_powers_precomp(
        R: *mut fmpq_poly_struct,
        A: *const fmpq_poly_struct,
        B: *const fmpq_poly_struct,
        B_inv: *const fmpq_poly_powers_precomp_struct,
    );
    pub fn _fmpq_poly_divides(
        qpoly: *mut fmpz,
        qden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_divides(
        q: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    ) -> c_int;
    pub fn fmpq_poly_remove(
        q: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    ) -> mp_limb_signed_t;
    pub fn _fmpq_poly_inv_series_newton(
        Qinv: *mut fmpz,
        Qinvden: *mut fmpz,
        Q: *const fmpz,
        Qden: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_inv_series_newton(
        Qinv: *mut fmpq_poly_struct,
        Q: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_inv_series(
        Qinv: *mut fmpz,
        Qinvden: *mut fmpz,
        Q: *const fmpz,
        Qden: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_inv_series(
        Qinv: *mut fmpq_poly_struct,
        Q: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_div_series(
        Q: *mut fmpz,
        denQ: *mut fmpz,
        A: *const fmpz,
        denA: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        denB: *const fmpz,
        lenB: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_div_series(
        Q: *mut fmpq_poly_struct,
        A: *const fmpq_poly_struct,
        B: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_gcd(
        G: *mut fmpz,
        denG: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpq_poly_gcd(
        G: *mut fmpq_poly_struct,
        A: *const fmpq_poly_struct,
        B: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_xgcd(
        G: *mut fmpz,
        denG: *mut fmpz,
        S: *mut fmpz,
        denS: *mut fmpz,
        T: *mut fmpz,
        denT: *mut fmpz,
        A: *const fmpz,
        denA: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        denB: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpq_poly_xgcd(
        G: *mut fmpq_poly_struct,
        S: *mut fmpq_poly_struct,
        T: *mut fmpq_poly_struct,
        A: *const fmpq_poly_struct,
        B: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_lcm(
        G: *mut fmpz,
        denG: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpq_poly_lcm(
        L: *mut fmpq_poly_struct,
        A: *const fmpq_poly_struct,
        B: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_resultant(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpq_poly_resultant(
        r: *mut fmpq,
        f: *const fmpq_poly_struct,
        g: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_resultant_div(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        divisor: *const fmpz,
        nbits: mp_limb_signed_t,
    );
    pub fn fmpq_poly_resultant_div(
        r: *mut fmpq,
        f: *const fmpq_poly_struct,
        g: *const fmpq_poly_struct,
        divisor: *const fmpz,
        nbits: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_derivative(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_derivative(res: *mut fmpq_poly_struct, poly: *const fmpq_poly_struct);
    pub fn _fmpq_poly_integral(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_integral(res: *mut fmpq_poly_struct, poly: *const fmpq_poly_struct);
    pub fn _fmpq_poly_invsqrt_series(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_invsqrt_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_sqrt_series(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sqrt_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_power_sums(
        res: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_power_sums(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_power_sums_to_poly(
        res: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_power_sums_to_fmpz_poly(
        res: *mut fmpz_poly_struct,
        Q: *const fmpq_poly_struct,
    );
    pub fn fmpq_poly_power_sums_to_poly(res: *mut fmpq_poly_struct, Q: *const fmpq_poly_struct);
    pub fn _fmpq_poly_log_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        f: *const fmpz,
        fden: *const fmpz,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_log_series(
        res: *mut fmpq_poly_struct,
        f: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_exp_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_exp_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_atan_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_atan_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_atanh_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_atanh_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_asin_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_asin_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_asinh_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_asinh_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_tan_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_tan_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_sin_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sin_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_cos_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_cos_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_sin_cos_series(
        s: *mut fmpz,
        sden: *mut fmpz,
        c: *mut fmpz,
        cden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sin_cos_series(
        res1: *mut fmpq_poly_struct,
        res2: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_sinh_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_sinh_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_cosh_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_cosh_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_tanh_series(
        g: *mut fmpz,
        gden: *mut fmpz,
        h: *const fmpz,
        hden: *const fmpz,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_tanh_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_legendre_p(coeffs: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn fmpq_poly_legendre_p(poly: *mut fmpq_poly_struct, n: mp_limb_t);
    pub fn _fmpq_poly_laguerre_l(coeffs: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn fmpq_poly_laguerre_l(poly: *mut fmpq_poly_struct, n: mp_limb_t);
    pub fn _fmpq_poly_gegenbauer_c(coeffs: *mut fmpz, den: *mut fmpz, n: mp_limb_t, a: *const fmpq);
    pub fn fmpq_poly_gegenbauer_c(poly: *mut fmpq_poly_struct, n: mp_limb_t, a: *const fmpq);
    pub fn _fmpq_poly_evaluate_fmpz(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
    );
    pub fn fmpq_poly_evaluate_fmpz(res: *mut fmpq, poly: *const fmpq_poly_struct, a: *const fmpz);
    pub fn _fmpq_poly_evaluate_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        anum: *const fmpz,
        aden: *const fmpz,
    );
    pub fn fmpq_poly_evaluate_fmpq(res: *mut fmpq, poly: *const fmpq_poly_struct, a: *const fmpq);
    pub fn fmpq_poly_evaluate_mpz(
        res: *mut __mpq_struct,
        poly: *const fmpq_poly_struct,
        a: *const __mpz_struct,
    );
    pub fn fmpq_poly_evaluate_mpq(
        res: *mut __mpq_struct,
        poly: *const fmpq_poly_struct,
        a: *const __mpq_struct,
    );
    pub fn _fmpq_poly_interpolate_fmpz_vec(
        poly: *mut fmpz,
        den: *mut fmpz,
        xs: *const fmpz,
        ys: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_interpolate_fmpz_vec(
        poly: *mut fmpq_poly_struct,
        xs: *const fmpz,
        ys: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_compose(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpq_poly_compose(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
    );
    pub fn _fmpq_poly_rescale(
        res: *mut fmpz,
        denr: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        xnum: *const fmpz,
        xden: *const fmpz,
    );
    pub fn fmpq_poly_rescale(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        x: *const fmpq,
    );
    pub fn _fmpq_poly_compose_series_horner(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_compose_series_horner(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_compose_series_brent_kung(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_compose_series_brent_kung(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_compose_series(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        den2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_compose_series(
        res: *mut fmpq_poly_struct,
        poly1: *const fmpq_poly_struct,
        poly2: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_revert_series_lagrange(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_revert_series_lagrange(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_revert_series_lagrange_fast(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_revert_series_lagrange_fast(
        res: *mut fmpq_poly_struct,
        poly: *mut fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_revert_series_newton(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_revert_series_newton(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_revert_series(
        res: *mut fmpz,
        den: *mut fmpz,
        poly1: *const fmpz,
        den1: *const fmpz,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpq_poly_revert_series(
        res: *mut fmpq_poly_struct,
        poly: *const fmpq_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpq_poly_content(
        res: *mut fmpq,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_content(res: *mut fmpq, poly: *const fmpq_poly_struct);
    pub fn _fmpq_poly_primitive_part(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_primitive_part(res: *mut fmpq_poly_struct, poly: *const fmpq_poly_struct);
    pub fn _fmpq_poly_is_monic(poly: *const fmpz, den: *const fmpz, len: mp_limb_signed_t)
        -> c_int;
    pub fn fmpq_poly_is_monic(poly: *const fmpq_poly_struct) -> c_int;
    pub fn _fmpq_poly_make_monic(
        rpoly: *mut fmpz,
        rden: *mut fmpz,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpq_poly_make_monic(res: *mut fmpq_poly_struct, poly: *const fmpq_poly_struct);
    pub fn fmpq_poly_is_squarefree(poly: *const fmpq_poly_struct) -> c_int;
    pub fn fmpq_poly_debug(poly: *const fmpq_poly_struct) -> c_int;
    pub fn _fmpq_poly_fprint(
        file: *mut FILE,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_poly_fprint(file: *mut FILE, poly: *const fmpq_poly_struct) -> c_int;
    pub fn _fmpq_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        x: *const c_char,
    ) -> c_int;
    pub fn fmpq_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpq_poly_struct,
        var: *const c_char,
    ) -> c_int;
    pub fn _fmpq_poly_print(poly: *const fmpz, den: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn fmpq_poly_print(poly: *const fmpq_poly_struct) -> c_int;
    pub fn _fmpq_poly_print_pretty(
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        x: *const c_char,
    ) -> c_int;
    pub fn fmpq_poly_print_pretty(poly: *const fmpq_poly_struct, var: *const c_char) -> c_int;
    pub fn fmpq_poly_fread(file: *const FILE, poly: *mut fmpq_poly_struct) -> c_int;
    pub fn fmpq_poly_read(poly: *mut fmpq_poly_struct) -> c_int;
}
