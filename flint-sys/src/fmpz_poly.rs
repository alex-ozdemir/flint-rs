#![allow(non_camel_case_types)]
// TODO: fmpq, nmod

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::{fmpz, fmpz_preinvn_struct};
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fmpz_poly_struct {
    pub coeffs: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
/*
impl Drop for fmpz_poly_struct {
    fn drop(&mut self) {
        unsafe { fmpz_poly_clear(self);}
    }
}*/

pub type fmpz_poly_t = [fmpz_poly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_poly_powers_precomp {
    pub powers: *mut *mut fmpz,
    pub len: mp_limb_signed_t,
}

pub type fmpz_poly_powers_precomp_t = [fmpz_poly_powers_precomp; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_poly_factor {
    pub c: fmpz,
    pub p: *mut fmpz_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type fmpz_poly_factor_t = [fmpz_poly_factor; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_poly_mul_precache {
    pub jj: *mut *mut mp_limb_t,
    pub n: mp_limb_signed_t,
    pub len2: mp_limb_signed_t,
    pub loglen: mp_limb_signed_t,
    pub bits2: mp_limb_signed_t,
    pub limbs: mp_limb_signed_t,
    pub poly2: fmpz_poly_t,
}

pub type fmpz_poly_mul_precache_t = [fmpz_poly_mul_precache; 1usize];

extern "C" {
    pub fn fmpz_poly_init(poly: *mut fmpz_poly_struct);
    pub fn fmpz_poly_init2(poly: *mut fmpz_poly_struct, alloc: mp_limb_signed_t);
    pub fn fmpz_poly_realloc(poly: *mut fmpz_poly_struct, alloc: mp_limb_signed_t);
    pub fn fmpz_poly_fit_length(poly: *mut fmpz_poly_struct, len: mp_limb_signed_t);
    pub fn fmpz_poly_clear(poly: *mut fmpz_poly_struct);
    pub fn _fmpz_poly_normalise(poly: *mut fmpz_poly_struct);
    pub fn _fmpz_poly_set_length(poly: *mut fmpz_poly_struct, newlen: mp_limb_signed_t);
    pub fn fmpz_poly_attach_truncate(
        trunc: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_attach_shift(
        trunc: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_length(poly: *const fmpz_poly_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_degree(poly: *const fmpz_poly_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_set(poly1: *mut fmpz_poly_struct, poly2: *const fmpz_poly_struct);
    pub fn fmpz_poly_set_ui(poly: *mut fmpz_poly_struct, c: mp_limb_t);
    pub fn fmpz_poly_set_si(poly: *mut fmpz_poly_struct, c: mp_limb_signed_t);
    pub fn fmpz_poly_set_fmpz(poly: *mut fmpz_poly_struct, c: *const fmpz);
    pub fn fmpz_poly_set_mpz(poly: *mut fmpz_poly_struct, c: *const __mpz_struct);
    pub fn _fmpz_poly_set_str(poly: *mut fmpz, str_: *const c_char) -> c_int;
    pub fn fmpz_poly_set_str(poly: *mut fmpz_poly_struct, str_: *const c_char) -> c_int;
    pub fn _fmpz_poly_get_str(poly: *const fmpz, len: mp_limb_signed_t) -> *mut c_char;
    pub fn fmpz_poly_get_str(poly: *const fmpz_poly_struct) -> *mut c_char;
    pub fn _fmpz_poly_get_str_pretty(
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const c_char,
    ) -> *mut c_char;
    pub fn fmpz_poly_get_str_pretty(poly: *const fmpz_poly_struct, x: *const c_char)
        -> *mut c_char;
    pub fn fmpz_poly_zero(poly: *mut fmpz_poly_struct);
    pub fn fmpz_poly_one(poly: *mut fmpz_poly_struct);
    pub fn fmpz_poly_zero_coeffs(
        poly: *mut fmpz_poly_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    );
    pub fn fmpz_poly_swap(poly1: *mut fmpz_poly_struct, poly2: *mut fmpz_poly_struct);
    pub fn _fmpz_poly_reverse(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_reverse(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_deflation(input: *const fmpz_poly_struct) -> mp_limb_t;
    pub fn fmpz_poly_deflate(
        result: *mut fmpz_poly_struct,
        input: *const fmpz_poly_struct,
        deflation: mp_limb_t,
    );
    pub fn fmpz_poly_inflate(
        result: *mut fmpz_poly_struct,
        input: *const fmpz_poly_struct,
        inflation: mp_limb_t,
    );
    pub fn fmpz_poly_truncate(poly: *mut fmpz_poly_struct, newlen: mp_limb_signed_t);
    pub fn fmpz_poly_set_trunc(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_randtest(
        f: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_poly_randtest_unsigned(
        f: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_poly_randtest_not_zero(
        f: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_poly_randtest_no_real_root(
        p: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_poly_get_coeff_si(
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_set_coeff_si(
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_get_coeff_ui(poly: *const fmpz_poly_struct, n: mp_limb_signed_t) -> mp_limb_t;
    pub fn fmpz_poly_set_coeff_ui(poly: *mut fmpz_poly_struct, n: mp_limb_signed_t, x: mp_limb_t);
    pub fn fmpz_poly_set_coeff_fmpz(
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
        x: *const fmpz,
    );
    pub fn fmpz_poly_get_coeff_fmpz(
        x: *mut fmpz,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_equal(poly1: *const fmpz_poly_struct, poly2: *const fmpz_poly_struct)
        -> c_int;
    pub fn fmpz_poly_equal_trunc(
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_poly_is_one(poly: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn fmpz_poly_is_one(op: *const fmpz_poly_struct) -> c_int;
    pub fn fmpz_poly_is_unit(op: *const fmpz_poly_struct) -> c_int;
    pub fn fmpz_poly_is_gen(op: *const fmpz_poly_struct) -> c_int;
    pub fn fmpz_poly_equal_fmpz(poly: *const fmpz_poly_struct, c: *const fmpz) -> c_int;
    pub fn _fmpz_poly_add(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_add(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_add_series(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_sub(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_sub(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_sub_series(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_neg(res: *mut fmpz_poly_struct, poly: *const fmpz_poly_struct);
    pub fn fmpz_poly_scalar_abs(res: *mut fmpz_poly_struct, poly: *const fmpz_poly_struct);
    pub fn fmpz_poly_scalar_mul_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_mul_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_scalar_mul_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_addmul_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_submul_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_fdiv_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_fdiv_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_scalar_fdiv_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_tdiv_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_tdiv_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_scalar_tdiv_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_divexact_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_divexact_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_scalar_divexact_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_fdiv_2exp(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_tdiv_2exp(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_mul_2exp(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_poly_scalar_mod_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_scalar_smod_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const fmpz,
    );
    pub fn _fmpz_poly_remove_content_2exp(
        pol: *mut fmpz,
        len: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_poly_scale_2exp(pol: *mut fmpz, len: mp_limb_signed_t, k: mp_limb_signed_t);
    pub fn _fmpz_poly_bit_pack(
        arr: mp_ptr,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        bit_size: mp_limb_t,
        negate: c_int,
    );
    pub fn _fmpz_poly_bit_unpack(
        poly: *mut fmpz,
        len: mp_limb_signed_t,
        arr: mp_srcptr,
        bit_size: mp_limb_t,
        negate: c_int,
    ) -> c_int;
    pub fn _fmpz_poly_bit_unpack_unsigned(
        poly: *mut fmpz,
        len: mp_limb_signed_t,
        arr: mp_srcptr,
        bit_size: mp_limb_t,
    );
    pub fn fmpz_poly_bit_pack(f: *mut fmpz, poly: *const fmpz_poly_struct, bit_size: mp_limb_t);
    pub fn fmpz_poly_bit_unpack(poly: *mut fmpz_poly_struct, f: *const fmpz, bit_size: mp_limb_t);
    pub fn fmpz_poly_bit_unpack_unsigned(
        poly: *mut fmpz_poly_struct,
        f: *const fmpz,
        bit_size: mp_limb_t,
    );
    pub fn _fmpz_poly_mul_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mul_classical(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_mullow_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mullow_classical(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mulhigh_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        start: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mulhigh_classical(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        start: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mulmid_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mulmid_classical(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_mul_karatsuba(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_mul_karatsuba(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mullow_karatsuba_n(
        res: *mut fmpz,
        poly1: *const fmpz,
        poly2: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mullow_karatsuba_n(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mulhigh_karatsuba_n(
        res: *mut fmpz,
        poly1: *const fmpz,
        poly2: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mulhigh_karatsuba_n(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        length: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mul_KS(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mul_KS(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_mullow_KS(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mullow_KS(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mul_SS(
        output: *mut fmpz,
        input1: *const fmpz,
        length1: mp_limb_signed_t,
        input2: *const fmpz,
        length2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mul_SS(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_mullow_SS(
        output: *mut fmpz,
        input1: *const fmpz,
        length1: mp_limb_signed_t,
        input2: *const fmpz,
        length2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mullow_SS(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_mul(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mul(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_mullow(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mullow(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mulhigh_n(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mul_SS_precache_init(
        pre: *mut fmpz_poly_mul_precache,
        len1: mp_limb_signed_t,
        bits1: mp_limb_signed_t,
        poly2: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_mul_precache_clear(pre: *mut fmpz_poly_mul_precache);
    pub fn _fmpz_poly_mullow_SS_precache(
        output: *mut fmpz,
        input1: *const fmpz,
        len1: mp_limb_signed_t,
        pre: *const fmpz_poly_mul_precache,
        trunc: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mullow_SS_precache(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        pre: *const fmpz_poly_mul_precache,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_mul_SS_precache(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        pre: *const fmpz_poly_mul_precache,
    );
    pub fn _fmpz_poly_sqr_KS(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_sqr_KS(rop: *mut fmpz_poly_struct, op: *const fmpz_poly_struct);
    pub fn fmpz_poly_sqr_karatsuba(rop: *mut fmpz_poly_struct, op: *const fmpz_poly_struct);
    pub fn _fmpz_poly_sqr_karatsuba(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_poly_sqr_classical(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_sqr_classical(rop: *mut fmpz_poly_struct, op: *const fmpz_poly_struct);
    pub fn _fmpz_poly_sqr(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_sqr(rop: *mut fmpz_poly_struct, op: *const fmpz_poly_struct);
    pub fn _fmpz_poly_sqrlow_KS(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_sqrlow_KS(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_sqrlow_karatsuba_n(res: *mut fmpz, poly: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_sqrlow_karatsuba_n(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_sqrlow_classical(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_sqrlow_classical(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_sqrlow(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_sqrlow(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_pow_multinomial(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
    pub fn fmpz_poly_pow_multinomial(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        e: mp_limb_t,
    );
    pub fn _fmpz_poly_pow_binomial(res: *mut fmpz, poly: *const fmpz, e: mp_limb_t);
    pub fn fmpz_poly_pow_binomial(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        e: mp_limb_t,
    );
    pub fn _fmpz_poly_pow_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
    pub fn fmpz_poly_pow_binexp(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        e: mp_limb_t,
    );
    pub fn _fmpz_poly_pow_addchains(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: *const c_int,
        n: c_int,
    );
    pub fn fmpz_poly_pow_addchains(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        e: mp_limb_t,
    );
    pub fn _fmpz_poly_pow_small(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
    pub fn _fmpz_poly_pow(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t, e: mp_limb_t);
    pub fn fmpz_poly_pow(res: *mut fmpz_poly_struct, poly: *const fmpz_poly_struct, e: mp_limb_t);
    pub fn _fmpz_poly_pow_trunc(
        res: *mut fmpz,
        poly: *const fmpz,
        e: mp_limb_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_pow_trunc(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        e: mp_limb_t,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_shift_left(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_shift_right(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_shift_left(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_shift_right(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_2norm(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_2norm(res: *mut fmpz, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_2norm_normalised_bits(poly: *const fmpz, len: mp_limb_signed_t) -> mp_limb_t;
    pub fn fmpz_poly_max_limbs(poly: *const fmpz_poly_struct) -> mp_limb_t;
    pub fn fmpz_poly_max_bits(poly: *const fmpz_poly_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_height(res: *mut fmpz, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_gcd_subresultant(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_gcd_subresultant(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_gcd_heuristic(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_poly_gcd_heuristic(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    ) -> c_int;
    pub fn _fmpz_poly_gcd_modular(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_gcd_modular(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_gcd(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_gcd(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_lcm(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_lcm(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_resultant_euclidean(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_resultant_euclidean(
        res: *mut fmpz,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_resultant_modular(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_resultant_modular(
        res: *mut fmpz,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_resultant(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_resultant(
        res: *mut fmpz,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_resultant_modular_div(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        divisor: *const fmpz,
        nbits: mp_limb_signed_t,
    );
    pub fn fmpz_poly_resultant_modular_div(
        res: *mut fmpz,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        divisor: *const fmpz,
        nbits: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_xgcd_modular(
        r: *mut fmpz,
        s: *mut fmpz,
        t: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_xgcd_modular(
        r: *mut fmpz,
        s: *mut fmpz_poly_struct,
        t: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_xgcd(
        r: *mut fmpz,
        s: *mut fmpz,
        t: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_xgcd(
        r: *mut fmpz,
        s: *mut fmpz_poly_struct,
        t: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_discriminant(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_discriminant(res: *mut fmpz, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_content(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_content(res: *mut fmpz, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_primitive_part(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_primitive_part(res: *mut fmpz_poly_struct, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_is_squarefree(poly: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn fmpz_poly_is_squarefree(poly: *const fmpz_poly_struct) -> c_int;
    pub fn _fmpz_poly_divrem_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_divrem_basecase(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_divrem_divconquer_recursive(
        Q: *mut fmpz,
        BQ: *mut fmpz,
        W: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn _fmpz_poly_divrem_divconquer(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_divrem_divconquer(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_divrem(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_divrem(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_div_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_div_basecase(
        Q: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_divremlow_divconquer_recursive(
        Q: *mut fmpz,
        QB: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn _fmpz_poly_div_divconquer_recursive(
        Q: *mut fmpz,
        temp: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn _fmpz_poly_div_divconquer(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_div_divconquer(
        Q: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_div(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_div(
        Q: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_preinvert(B_inv: *mut fmpz, B: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_preinvert(B_inv: *mut fmpz_poly_struct, B: *const fmpz_poly_struct);
    pub fn _fmpz_poly_div_preinv(
        Q: *mut fmpz,
        A: *const fmpz,
        len1: mp_limb_signed_t,
        B: *const fmpz,
        B_inv: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_div_preinv(
        Q: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        B_inv: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_divrem_preinv(
        Q: *mut fmpz,
        A: *const fmpz,
        len1: mp_limb_signed_t,
        B: *const fmpz,
        B_inv: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_divrem_preinv(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        B_inv: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_powers_precompute(B: *const fmpz, len: mp_limb_signed_t) -> *mut *mut fmpz;
    pub fn fmpz_poly_powers_precompute(
        pinv: *mut fmpz_poly_powers_precomp,
        poly: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_powers_clear(powers: *mut *mut fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_powers_clear(pinv: *mut fmpz_poly_powers_precomp);
    pub fn _fmpz_poly_rem_powers_precomp(
        A: *mut fmpz,
        m: mp_limb_signed_t,
        B: *const fmpz,
        n: mp_limb_signed_t,
        powers: *mut *mut fmpz,
    );
    pub fn fmpz_poly_rem_powers_precomp(
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        B_inv: *const fmpz_poly_powers_precomp,
    );
    pub fn _fmpz_poly_rem_basecase(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpz_poly_rem_basecase(
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_rem(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpz_poly_rem(
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_div_root(Q: *mut fmpz_poly_struct, A: *const fmpz_poly_struct, c: *const fmpz);
    pub fn _fmpz_poly_div_root(Q: *mut fmpz, A: *const fmpz, len: mp_limb_signed_t, c: *const fmpz);
    pub fn _fmpz_poly_inv_series_basecase(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_inv_series_basecase(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_inv_series_newton(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_inv_series_newton(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_inv_series(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_inv_series(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_div_series_basecase(
        Q: *mut fmpz,
        A: *const fmpz,
        Alen: mp_limb_signed_t,
        B: *const fmpz,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_div_series_divconquer(
        Q: *mut fmpz,
        A: *const fmpz,
        Alen: mp_limb_signed_t,
        B: *const fmpz,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_div_series(
        Q: *mut fmpz,
        A: *const fmpz,
        Alen: mp_limb_signed_t,
        B: *const fmpz,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_div_series_basecase(
        Q: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_div_series_divconquer(
        Q: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_div_series(
        Q: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_divides(
        q: *mut fmpz,
        a: *const fmpz,
        len1: mp_limb_signed_t,
        b: *const fmpz,
        len2: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_poly_divides(
        q: *mut fmpz_poly_struct,
        a: *const fmpz_poly_struct,
        b: *const fmpz_poly_struct,
    ) -> c_int;
    pub fn fmpz_poly_remove(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_poly_pseudo_divrem_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        d: *const mp_limb_t,
        A: *const fmpz,
        A_len: mp_limb_signed_t,
        B: *const fmpz,
        B_len: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpz_poly_pseudo_divrem_basecase(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        d: *const mp_limb_t,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_pseudo_divrem_divconquer(
        Q: *mut fmpz,
        R: *mut fmpz,
        d: *const mp_limb_t,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpz_poly_pseudo_divrem_divconquer(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        d: *mut mp_limb_t,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_pseudo_divrem_cohen(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpz_poly_pseudo_divrem_cohen(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_pseudo_rem_cohen(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
    pub fn fmpz_poly_pseudo_rem_cohen(
        R: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_pseudo_divrem(
        Q: *mut fmpz,
        R: *mut fmpz,
        d: *const mp_limb_t,
        A: *const fmpz,
        A_len: mp_limb_signed_t,
        B: *const fmpz,
        B_len: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpz_poly_pseudo_divrem(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        d: *const mp_limb_t,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_pseudo_div(
        Q: *mut fmpz,
        d: *const mp_limb_t,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpz_poly_pseudo_div(
        Q: *mut fmpz_poly_struct,
        d: *const mp_limb_t,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_pseudo_rem(
        R: *mut fmpz,
        d: *const mp_limb_t,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *const fmpz_preinvn_struct,
    );
    pub fn fmpz_poly_pseudo_rem(
        R: *mut fmpz_poly_struct,
        d: *const mp_limb_t,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_divlow_smodp(
        res: *mut fmpz,
        f: *const fmpz_poly_struct,
        g: *const fmpz_poly_struct,
        p: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_divhigh_smodp(
        res: *mut fmpz,
        f: *const fmpz_poly_struct,
        g: *const fmpz_poly_struct,
        p: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_derivative(rpoly: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_derivative(res: *mut fmpz_poly_struct, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_evaluate_divconquer_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
    );
    pub fn fmpz_poly_evaluate_divconquer_fmpz(
        res: *mut fmpz,
        poly: *const fmpz_poly_struct,
        a: *const fmpz,
    );
    pub fn _fmpz_poly_evaluate_horner_fmpz(
        res: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
    );
    pub fn fmpz_poly_evaluate_horner_fmpz(
        res: *mut fmpz,
        f: *const fmpz_poly_struct,
        a: *const fmpz,
    );
    pub fn _fmpz_poly_evaluate_fmpz(
        res: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
    );
    pub fn fmpz_poly_evaluate_fmpz(res: *mut fmpz, f: *const fmpz_poly_struct, a: *const fmpz);
    pub fn _fmpz_poly_evaluate_horner_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        anum: *const fmpz,
        aden: *const fmpz,
    );
    pub fn fmpz_poly_evaluate_horner_fmpq(res: *mut fmpq, f: *mut fmpz_poly_struct, a: *mut fmpq);
    pub fn _fmpz_poly_evaluate_divconquer_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        anum: *const fmpz,
        aden: *const fmpz,
    );
    pub fn fmpz_poly_evaluate_divconquer_fmpq(
        res: *mut fmpq,
        f: *const fmpz_poly_struct,
        a: *const fmpq,
    );
    pub fn _fmpz_poly_evaluate_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        anum: *const fmpz,
        aden: *const fmpz,
    );
    pub fn fmpz_poly_evaluate_fmpq(res: *mut fmpq, f: *const fmpz_poly_struct, a: *const fmpq);

    pub fn fmpz_poly_evaluate_mpq(
        res: *mut __mpq_struct,
        f: *const fmpz_poly_struct,
        a: *const __mpq_struct,
    );
    pub fn _fmpz_poly_evaluate_mod(
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    pub fn fmpz_poly_evaluate_mod(
        poly: *const fmpz_poly_struct,
        a: mp_limb_t,
        n: mp_limb_t,
    ) -> mp_limb_t;
    pub fn _fmpz_poly_evaluate_horner_d(poly: *const fmpz, n: mp_limb_signed_t, d: f64) -> f64;
    pub fn fmpz_poly_evaluate_horner_d(poly: *mut fmpz_poly_struct, d: f64) -> f64;
    pub fn _fmpz_poly_evaluate_horner_d_2exp(
        exp: *mut mp_limb_signed_t,
        poly: *const fmpz,
        n: mp_limb_signed_t,
        d: f64,
    ) -> f64;
    pub fn fmpz_poly_evaluate_horner_d_2exp(
        exp: *mut mp_limb_signed_t,
        poly: *const fmpz_poly_struct,
        d: f64,
    ) -> f64;
    pub fn _fmpz_poly_evaluate_horner_d_2exp2(
        exp: *mut mp_limb_signed_t,
        poly: *const fmpz,
        n: mp_limb_signed_t,
        d: f64,
        dexp: mp_limb_signed_t,
        prec_in: mp_limb_t,
    ) -> f64;
    pub fn fmpz_poly_evaluate_horner_d_2exp2(
        exp: *mut mp_limb_signed_t,
        poly: *const fmpz_poly_struct,
        d: f64,
        dexp: mp_limb_signed_t,
        prec: mp_limb_t,
    ) -> f64;
    pub fn _fmpz_poly_compose_horner(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_compose_horner(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_compose_divconquer(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_compose_divconquer(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_compose(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn fmpz_poly_compose(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_taylor_shift_horner(poly: *mut fmpz, c: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_taylor_shift_horner(
        g: *mut fmpz_poly_struct,
        f: *const fmpz_poly_struct,
        c: *const fmpz,
    );
    pub fn _fmpz_poly_taylor_shift_divconquer(poly: *mut fmpz, c: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_taylor_shift_divconquer(
        g: *mut fmpz_poly_struct,
        f: *const fmpz_poly_struct,
        c: *const fmpz,
    );
    pub fn _fmpz_poly_taylor_shift_multi_mod(poly: *mut fmpz, c: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_taylor_shift_multi_mod(
        g: *mut fmpz_poly_struct,
        f: *const fmpz_poly_struct,
        c: *const fmpz,
    );
    pub fn _fmpz_poly_taylor_shift(poly: *mut fmpz, c: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_taylor_shift(
        g: *mut fmpz_poly_struct,
        f: *const fmpz_poly_struct,
        c: *const fmpz,
    );
    pub fn _fmpz_poly_compose_series_brent_kung(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_compose_series_brent_kung(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_compose_series_horner(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_compose_series_horner(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_compose_series(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_compose_series(
        res: *mut fmpz_poly_struct,
        poly1: *const fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_revert_series_lagrange(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_revert_series_lagrange(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_revert_series_lagrange_fast(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_revert_series_lagrange_fast(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_revert_series_newton(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_revert_series_newton(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_revert_series(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_revert_series(
        Qinv: *mut fmpz_poly_struct,
        Q: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_sqrtrem_classical(
        res: *mut fmpz,
        r: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_poly_sqrtrem_classical(
        b: *mut fmpz_poly_struct,
        r: *mut fmpz_poly_struct,
        a: *const fmpz_poly_struct,
    ) -> c_int;
    pub fn _fmpz_poly_sqrtrem_divconquer(
        res: *mut fmpz,
        r: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        temp: *const fmpz,
    ) -> c_int;
    pub fn fmpz_poly_sqrtrem_divconquer(
        b: *mut fmpz_poly_struct,
        r: *mut fmpz_poly_struct,
        a: *const fmpz_poly_struct,
    ) -> c_int;
    pub fn _fmpz_poly_sqrt_classical(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_sqrt_classical(b: *mut fmpz_poly_struct, a: *const fmpz_poly_struct) -> c_int;
    pub fn _fmpz_poly_sqrt_divconquer(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        exact: c_int,
    ) -> c_int;
    pub fn fmpz_poly_sqrt_divconquer(b: *mut fmpz_poly_struct, a: *const fmpz_poly_struct)
        -> c_int;
    pub fn _fmpz_poly_sqrt_KS(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn fmpz_poly_sqrt_KS(b: *mut fmpz_poly_struct, a: *const fmpz_poly_struct) -> c_int;
    pub fn _fmpz_poly_sqrt(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn fmpz_poly_sqrt(b: *mut fmpz_poly_struct, a: *const fmpz_poly_struct) -> c_int;
    pub fn _fmpz_poly_sqrt_series(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_poly_sqrt_series(
        b: *mut fmpz_poly_struct,
        a: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_poly_power_sums_naive(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_power_sums_naive(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_power_sums(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_power_sums_to_poly(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_power_sums_to_poly(res: *mut fmpz_poly_struct, Q: *const fmpz_poly_struct);
    pub fn _fmpz_poly_signature(
        r1: *mut mp_limb_signed_t,
        r2: *mut mp_limb_signed_t,
        poly: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpz_poly_signature(
        r1: *mut mp_limb_signed_t,
        r2: *mut mp_limb_signed_t,
        poly: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_fprint(file: *mut FILE, poly: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn fmpz_poly_fprint(file: *mut FILE, poly: *const fmpz_poly_struct) -> c_int;
    pub fn _fmpz_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const c_char,
    ) -> c_int;
    pub fn fmpz_poly_fprint_pretty(
        file: *mut FILE,
        poly: *mut fmpz_poly_struct,
        x: *const c_char,
    ) -> c_int;
    pub fn _fmpz_poly_print_pretty(
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const c_char,
    ) -> c_int;
    pub fn _fmpz_poly_print(poly: *const fmpz, n: mp_limb_signed_t) -> c_int;
    pub fn fmpz_poly_print(poly: *mut fmpz_poly_struct) -> c_int;
    pub fn fmpz_poly_print_pretty(poly: *mut fmpz_poly_struct, x: *const c_char) -> c_int;
    pub fn fmpz_poly_fread(file: *mut FILE, poly: *const fmpz_poly_struct) -> c_int;
    pub fn fmpz_poly_fread_pretty(
        file: *mut FILE,
        poly: *const fmpz_poly_struct,
        x: *mut *mut c_char,
    ) -> c_int;
    pub fn fmpz_poly_read(poly: *mut fmpz_poly_struct) -> c_int;
    pub fn fmpz_poly_read_pretty(poly: *mut fmpz_poly_struct, x: *mut *mut c_char) -> c_int;
    pub fn fmpz_poly_debug(poly: *mut fmpz_poly_struct);
    /*
    pub fn fmpz_poly_get_nmod_poly(res: *mut nmod_poly_struct, poly: *mut fmpz_poly);
    pub fn fmpz_poly_set_nmod_poly(res: *mut fmpz_poly_struct, poly: *mut nmod_poly_struct);
    pub fn fmpz_poly_set_nmod_poly_unsigned(
        res: *mut fmpz_poly_struct,
        poly: *mut nmod_poly_struct,
    );
    pub fn _fmpz_poly_CRT_ui_precomp(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        m1: *mut fmpz,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        m2: mp_limb_t,
        m2inv: mp_limb_t,
        m1m2: *mut fmpz,
        c: mp_limb_t,
        sign: c_int,
    );
    pub fn _fmpz_poly_CRT_ui(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        m1: *mut fmpz,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        m2: mp_limb_t,
        m2inv: mp_limb_t,
        sign: c_int,
    );
    pub fn fmpz_poly_CRT_ui(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        m1: *mut fmpz,
        poly2: *mut nmod_poly_struct,
        sign: c_int,
    );
    */
    pub fn _fmpz_poly_product_roots_fmpz_vec(poly: *mut fmpz, xs: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_product_roots_fmpz_vec(
        poly: *mut fmpz_poly_struct,
        xs: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_product_roots_fmpq_vec(poly: *mut fmpz, xs: *const fmpq, n: mp_limb_signed_t);
    pub fn fmpz_poly_product_roots_fmpq_vec(
        poly: *mut fmpz_poly_struct,
        xs: *const fmpq,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_monomial_to_newton(poly: *mut fmpz, roots: *const fmpz, n: mp_limb_signed_t);
    pub fn _fmpz_poly_newton_to_monomial(poly: *mut fmpz, roots: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_poly_evaluate_fmpz_vec(
        res: *mut fmpz,
        f: *const fmpz_poly_struct,
        a: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_poly_interpolate_fmpz_vec(
        poly: *mut fmpz_poly_struct,
        xs: *const fmpz,
        ys: *const fmpz,
        n: mp_limb_signed_t,
    );
    /*
    pub fn fmpz_poly_hensel_build_tree(
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        fac: *mut nmod_poly_factor_struct,
    );
    */
    pub fn fmpz_poly_hensel_lift(
        Gout: *mut fmpz_poly_struct,
        Hout: *mut fmpz_poly_struct,
        Aout: *mut fmpz_poly_struct,
        Bout: *mut fmpz_poly_struct,
        f: *const fmpz_poly_struct,
        g: *const fmpz_poly_struct,
        h: *const fmpz_poly_struct,
        a: *const fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        p: *const fmpz,
        p1: *const fmpz,
    );
    pub fn _fmpz_poly_hensel_lift_without_inverse(
        G: *mut fmpz,
        H: *mut fmpz,
        f: *const fmpz,
        lenF: mp_limb_signed_t,
        g: *const fmpz,
        lenG: mp_limb_signed_t,
        h: *const fmpz,
        lenH: mp_limb_signed_t,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
        p1: *const fmpz,
    );
    pub fn fmpz_poly_hensel_lift_without_inverse(
        Gout: *mut fmpz_poly_struct,
        Hout: *mut fmpz_poly_struct,
        f: *const fmpz_poly_struct,
        g: *const fmpz_poly_struct,
        h: *const fmpz_poly_struct,
        a: *const fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        p: *const fmpz,
        p1: *const fmpz,
    );
    pub fn _fmpz_poly_hensel_lift_only_inverse(
        A: *mut fmpz,
        B: *mut fmpz,
        G: *const fmpz,
        lenG: mp_limb_signed_t,
        H: *const fmpz,
        lenH: mp_limb_signed_t,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
        p1: *const fmpz,
    );
    pub fn fmpz_poly_hensel_lift_only_inverse(
        Aout: *mut fmpz_poly_struct,
        Bout: *mut fmpz_poly_struct,
        G: *const fmpz_poly_struct,
        H: *const fmpz_poly_struct,
        a: *const fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        p: *const fmpz,
        p1: *const fmpz,
    );
    pub fn fmpz_poly_hensel_lift_tree_recursive(
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        j: mp_limb_signed_t,
        inv: mp_limb_signed_t,
        p0: *const fmpz,
        p1: *const fmpz,
    );
    pub fn fmpz_poly_hensel_lift_tree(
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        r: mp_limb_signed_t,
        p: *const fmpz,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        inv: mp_limb_signed_t,
    );
    /*
    pub fn _fmpz_poly_hensel_start_lift(
        lifted_fac: *mut fmpz_poly_factor,
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        local_fac: *mut nmod_poly_factor_struct,
        target_exp: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_poly_hensel_continue_lift(
        lifted_fac: *mut fmpz_poly_factor,
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        prev: mp_limb_signed_t,
        curr: mp_limb_signed_t,
        N: mp_limb_signed_t,
        p: *mut fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_hensel_lift_once(
        lifted_fac: *mut fmpz_poly_factor,
        f: *mut fmpz_poly_struct,
        local_fac: *mut nmod_poly_factor_struct,
        N: mp_limb_signed_t,
    );*/
    pub fn fmpz_poly_scalar_mul_mpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const __mpz_struct,
    );
    pub fn fmpz_poly_scalar_divexact_mpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const __mpz_struct,
    );
    pub fn fmpz_poly_scalar_fdiv_mpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *const fmpz_poly_struct,
        x: *const __mpz_struct,
    );
    pub fn fmpz_poly_set_coeff_mpz(
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
        x: *const __mpz_struct,
    );
    pub fn fmpz_poly_get_coeff_mpz(
        x: *mut __mpz_struct,
        poly: *const fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_bound_roots(bound: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
    pub fn fmpz_poly_bound_roots(bound: *mut fmpz, poly: *const fmpz_poly_struct);
    pub fn _fmpz_poly_num_real_roots_sturm(
        n_neg: *mut mp_limb_signed_t,
        n_pos: *mut mp_limb_signed_t,
        pol: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn fmpz_poly_num_real_roots_sturm(poly: *const fmpz_poly_struct) -> mp_limb_signed_t;
    pub fn _fmpz_poly_num_real_roots(pol: *const fmpz, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn fmpz_poly_num_real_roots(poly: *const fmpz_poly_struct) -> mp_limb_signed_t;
    pub fn fmpz_poly_CLD_bound(res: *mut fmpz, f: *const fmpz_poly_struct, n: mp_limb_signed_t);
    pub fn _fmpz_poly_cyclotomic(
        a: *mut fmpz,
        n: mp_limb_t,
        factors: mp_ptr,
        num_factors: mp_limb_signed_t,
        phi: mp_limb_t,
    );
    pub fn fmpz_poly_cyclotomic(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_is_cyclotomic(poly: *const fmpz, len: mp_limb_signed_t) -> mp_limb_t;
    pub fn fmpz_poly_is_cyclotomic(poly: *const fmpz_poly_struct) -> mp_limb_t;
    pub fn _fmpz_poly_cos_minpoly(f: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_cos_minpoly(f: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_swinnerton_dyer(T: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_swinnerton_dyer(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_chebyshev_t(coeffs: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_chebyshev_t(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_chebyshev_u(coeffs: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_chebyshev_u(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_legendre_pt(coeffs: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_legendre_pt(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_hermite_h(coeffs: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_hermite_h(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_hermite_he(coeffs: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_hermite_he(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_fibonacci(coeffs: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_poly_fibonacci(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn _fmpz_poly_eta_qexp(f: *mut fmpz, e: mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn fmpz_poly_eta_qexp(f: *mut fmpz_poly_struct, e: mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn _fmpz_poly_theta_qexp(f: *mut fmpz, e: mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn fmpz_poly_theta_qexp(f: *mut fmpz_poly_struct, e: mp_limb_signed_t, n: mp_limb_signed_t);
    pub fn fmpz_poly_add_si(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_poly_sub_si(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_poly_si_sub(
        res: *mut fmpz_poly_struct,
        c: mp_limb_signed_t,
        poly: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_add_fmpz(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        c: *const fmpz,
    );
    pub fn fmpz_poly_sub_fmpz(
        res: *mut fmpz_poly_struct,
        poly: *const fmpz_poly_struct,
        c: *const fmpz,
    );
    pub fn fmpz_poly_fmpz_sub(
        res: *mut fmpz_poly_struct,
        c: *const fmpz,
        poly: *const fmpz_poly_struct,
    );
}
