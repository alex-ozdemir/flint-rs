#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq::{fq_ctx_struct, fq_struct};
use crate::fq_mat::fq_mat_struct;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_poly_struct {
    pub coeffs: *mut fq_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fq_poly_t = [fq_poly_struct; 1usize];

extern "C" {
    pub fn fq_poly_init(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_init2(
        poly: *mut fq_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_realloc(
        poly: *mut fq_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_truncate(
        poly: *mut fq_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_set_trunc(
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_fit_length(
        poly: *mut fq_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_clear(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn _fq_poly_normalise(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn _fq_poly_normalise2(
        poly: *const fq_struct,
        length: *mut mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_set_length(
        poly: *mut fq_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_length(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_poly_degree(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_poly_lead(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> *mut fq_struct;
    pub fn fq_poly_randtest(
        f: *mut fq_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_randtest_not_zero(
        f: *mut fq_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_randtest_monic(
        f: *mut fq_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_randtest_irreducible(
        f: *mut fq_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_set(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_set(rop: *mut fq_poly_struct, op: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_set_fq(
        poly: *mut fq_poly_struct,
        c: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_set_fmpz_mod_poly(
        rop: *mut fq_poly_struct,
        op: *mut fmpz_mod_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_set_nmod_poly(
        rop: *mut fq_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_swap(
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_zero(rop: *mut fq_struct, len: mp_limb_signed_t, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_zero(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_one(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_gen(f: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn _fq_poly_make_monic(
        rop: *mut fq_struct,
        op: *const fq_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_make_monic(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_reverse(
        res: *mut fq_struct,
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_reverse(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_deflation(input: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> mp_limb_t;
    pub fn fq_poly_deflate(
        result: *mut fq_poly_struct,
        input: *mut fq_poly_struct,
        deflation: mp_limb_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_inflate(
        result: *mut fq_poly_struct,
        input: *mut fq_poly_struct,
        inflation: mp_limb_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_get_coeff(
        x: *mut fmpz_poly_struct,
        poly: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_set_coeff(
        poly: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_set_coeff_fmpz(
        poly: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        x: *mut fmpz,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_is_gen(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_equal(
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_equal_trunc(
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_is_zero(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_is_one(op: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_is_unit(op: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_equal_fq(
        poly: *mut fq_poly_struct,
        c: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_poly_add(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_add(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_add_si(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_add_series(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_sub(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_sub(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_sub_series(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_neg(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_neg(rop: *mut fq_poly_struct, op: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn _fq_poly_scalar_mul_fq(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_scalar_mul_fq(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_scalar_div_fq(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_scalar_div_fq(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_scalar_addmul_fq(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_scalar_addmul_fq(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_scalar_submul_fq(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_scalar_submul_fq(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        x: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mul_classical(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mul_classical(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mul_reorder(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mul_reorder(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mul_univariate(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mul_univariate(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mul_KS(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mul_KS(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mul(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mul(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mullow_classical(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mullow_classical(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mullow_KS(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mullow_KS(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mullow_univariate(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mullow_univariate(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mullow(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mullow(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mulhigh_classical(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        start: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mulhigh_classical(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        start: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mulhigh(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mulhigh(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        start: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mulmod(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mulmod(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_mulmod_preinv(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_mulmod_preinv(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        f: *mut fq_poly_struct,
        finv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_sqr_classical(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_sqr_classical(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_sqr_reorder(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_sqr_reorder(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_sqr_KS(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_sqr_KS(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_sqr(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_sqr(rop: *mut fq_poly_struct, op: *mut fq_poly_struct, ctx: *mut fq_ctx_struct);
    pub fn _fq_poly_pow(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        e: mp_limb_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_pow(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        e: mp_limb_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_pow_trunc_binexp(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_pow_trunc_binexp(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_pow_trunc(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_pow_trunc(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_powmod_fmpz_binexp(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: *mut fmpz,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_powmod_fmpz_binexp(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: *mut fmpz,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_powmod_fmpz_binexp_preinv(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: *mut fmpz,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_powmod_fmpz_binexp_preinv(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: *mut fmpz,
        f: *mut fq_poly_struct,
        finv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_powmod_ui_binexp(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: mp_limb_t,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_powmod_ui_binexp(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: mp_limb_t,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_powmod_ui_binexp_preinv(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: mp_limb_t,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_powmod_ui_binexp_preinv(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: mp_limb_t,
        f: *mut fq_poly_struct,
        finv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_powmod_fmpz_sliding_preinv(
        res: *mut fq_struct,
        poly: *const fq_struct,
        e: *mut fmpz,
        k: mp_limb_t,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_powmod_fmpz_sliding_preinv(
        res: *mut fq_poly_struct,
        poly: *mut fq_poly_struct,
        e: *mut fmpz,
        k: mp_limb_t,
        f: *mut fq_poly_struct,
        finv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_powmod_x_fmpz_preinv(
        res: *mut fq_struct,
        e: *mut fmpz,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_powmod_x_fmpz_preinv(
        res: *mut fq_poly_struct,
        e: *mut fmpz,
        f: *mut fq_poly_struct,
        finv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_shift_left(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_shift_left(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_shift_right(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_shift_right(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_hamming_weight(
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_hamming_weight(
        op: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_gcd_euclidean(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_gcd_euclidean(
        G: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn _fq_poly_gcd(
        G: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_gcd(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_gcd_euclidean_f(
        f: *mut fmpz_poly_struct,
        G: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_gcd_euclidean_f(
        f: *mut fmpz_poly_struct,
        G: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_hgcd(
        M: *mut *mut fq_struct,
        lenM: *mut mp_limb_signed_t,
        A: *mut fq_struct,
        lenA: *mut mp_limb_signed_t,
        B: *mut fq_struct,
        lenB: *mut mp_limb_signed_t,
        a: *const fq_struct,
        lena: mp_limb_signed_t,
        b: *const fq_struct,
        lenb: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn _fq_poly_gcd_hgcd(
        G: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_gcd_hgcd(
        G: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_xgcd_euclidean_f(
        f: *mut fmpz_poly_struct,
        G: *mut fq_struct,
        S: *mut fq_struct,
        fq: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_xgcd_euclidean_f(
        f: *mut fmpz_poly_struct,
        G: *mut fq_poly_struct,
        S: *mut fq_poly_struct,
        fq: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_xgcd_euclidean(
        G: *mut fq_struct,
        S: *mut fq_struct,
        fq: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_xgcd_euclidean(
        G: *mut fq_poly_struct,
        S: *mut fq_poly_struct,
        fq: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_xgcd(
        G: *mut fq_struct,
        S: *mut fq_struct,
        fq: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_poly_xgcd(
        G: *mut fq_poly_struct,
        S: *mut fq_poly_struct,
        fq: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_remove(
        f: *mut fq_poly_struct,
        g: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> mp_limb_t;
    pub fn _fq_poly_div_basecase(
        Q: *mut fq_struct,
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_div_basecase(
        Q: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divrem_basecase(
        Q: *mut fq_struct,
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_divrem_basecase(
        Q: *mut fq_poly_struct,
        R: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divrem_divconquer_recursive(
        Q: *mut fq_struct,
        BQ: *mut fq_struct,
        W: *mut fq_struct,
        A: *const fq_struct,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divrem_divconquer(
        Q: *mut fq_struct,
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_divrem_divconquer(
        Q: *mut fq_poly_struct,
        R: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divrem(
        Q: *mut fq_struct,
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_divrem(
        Q: *mut fq_poly_struct,
        R: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_rem(
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_rem(
        R: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_inv_series_newton(
        Qinv: *mut fq_struct,
        Q: *const fq_struct,
        n: mp_limb_signed_t,
        cinv: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_inv_series_newton(
        Qinv: *mut fq_poly_struct,
        Q: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_inv_series(
        Qinv: *mut fq_struct,
        Q: *const fq_struct,
        n: mp_limb_signed_t,
        cinv: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_inv_series(
        Qinv: *mut fq_poly_struct,
        Q: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_div_series(
        Q: *mut fq_struct,
        A: *const fq_struct,
        Alen: mp_limb_signed_t,
        B: *const fq_struct,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_div_series(
        Q: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_div_newton_n_preinv(
        Q: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        Binv: *const fq_struct,
        lenBinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_div_newton_n_preinv(
        Q: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        Binv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divrem_newton_n_preinv(
        Q: *mut fq_struct,
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        Binv: *const fq_struct,
        lenBinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_divrem_newton_n_preinv(
        Q: *mut fq_poly_struct,
        R: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        Binv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divrem_f(
        f: *mut fmpz_poly_struct,
        Q: *mut fq_struct,
        R: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_divrem_f(
        f: *mut fmpz_poly_struct,
        Q: *mut fq_poly_struct,
        R: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_divides(
        Q: *mut fq_struct,
        A: *const fq_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_struct,
        lenB: mp_limb_signed_t,
        invB: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_divides(
        Q: *mut fq_poly_struct,
        A: *mut fq_poly_struct,
        B: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_poly_derivative(
        rop: *mut fq_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_derivative(
        rop: *mut fq_poly_struct,
        op: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_evaluate_fq(
        rop: *mut fmpz_poly_struct,
        op: *const fq_struct,
        len: mp_limb_signed_t,
        a: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_evaluate_fq(
        res: *mut fmpz_poly_struct,
        f: *mut fq_poly_struct,
        a: *mut fmpz_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_evaluate_fq_vec(
        ys: *mut fq_struct,
        coeffs: *const fq_struct,
        len: mp_limb_signed_t,
        xs: *const fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_evaluate_fq_vec(
        ys: *mut fq_struct,
        poly: *mut fq_poly_struct,
        xs: *const fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_tree_alloc(
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> *mut *mut fq_poly_struct;
    pub fn _fq_poly_tree_free(
        tree: *mut *mut fq_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_tree_build(
        tree: *mut *mut fq_poly_struct,
        roots: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_evaluate_fq_vec_fast_precomp(
        vs: *mut fq_struct,
        poly: *const fq_struct,
        plen: mp_limb_signed_t,
        tree: *const *mut fq_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_evaluate_fq_vec_fast(
        ys: *mut fq_struct,
        poly: *const fq_struct,
        plen: mp_limb_signed_t,
        xs: *const fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_evaluate_fq_vec_fast(
        ys: *mut fq_struct,
        poly: *mut fq_poly_struct,
        xs: *const fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_evaluate_fq_vec_iter(
        ys: *mut fq_struct,
        coeffs: *const fq_struct,
        len: mp_limb_signed_t,
        xs: *const fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_evaluate_fq_vec_iter(
        ys: *mut fq_struct,
        poly: *mut fq_poly_struct,
        xs: *const fq_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_divconquer(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_divconquer(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_horner(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_horner(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose(
        rop: *mut fq_struct,
        op1: *const fq_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose(
        rop: *mut fq_poly_struct,
        op1: *mut fq_poly_struct,
        op2: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod(
        res: *mut fq_struct,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_struct,
        h: *const fq_struct,
        lenh: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly3: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod_preinv(
        res: *mut fq_struct,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_struct,
        h: *const fq_struct,
        lenh: mp_limb_signed_t,
        hinv: *const fq_struct,
        lenhinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod_preinv(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly3: *mut fq_poly_struct,
        poly3inv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod_horner(
        res: *mut fq_struct,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_struct,
        h: *const fq_struct,
        lenh: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod_horner(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly3: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod_horner_preinv(
        res: *mut fq_struct,
        f: *const fq_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_struct,
        h: *const fq_struct,
        lenh: mp_limb_signed_t,
        hinv: *const fq_struct,
        lenhinv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod_horner_preinv(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly3: *mut fq_poly_struct,
        poly3inv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod_brent_kung(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly3: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod_brent_kung(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        poly3: *const fq_struct,
        len3: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod_brent_kung_preinv(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_struct,
        poly3: *const fq_struct,
        len3: mp_limb_signed_t,
        poly3inv: *const fq_struct,
        len3inv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod_brent_kung_preinv(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly3: *mut fq_poly_struct,
        poly3inv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_reduce_matrix_mod_poly(
        A: *mut fq_mat_struct,
        B: *mut fq_mat_struct,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_precompute_matrix(
        A: *mut fq_mat_struct,
        poly1: *const fq_struct,
        poly2: *const fq_struct,
        len2: mp_limb_signed_t,
        poly2inv: *const fq_struct,
        len2inv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_precompute_matrix(
        A: *mut fq_mat_struct,
        poly1: *mut fq_poly_struct,
        poly2: *mut fq_poly_struct,
        poly2inv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut fq_struct,
        poly1: *const fq_struct,
        len1: mp_limb_signed_t,
        A: *mut fq_mat_struct,
        poly3: *const fq_struct,
        len3: mp_limb_signed_t,
        poly3inv: *const fq_struct,
        len3inv: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut fq_poly_struct,
        poly1: *mut fq_poly_struct,
        A: *mut fq_mat_struct,
        poly3: *mut fq_poly_struct,
        poly3inv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        x: *const c_char,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_fprint_pretty(
        file: *mut FILE,
        poly: *mut fq_poly_struct,
        x: *const c_char,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_poly_fprint(
        file: *mut FILE,
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_fprint(
        file: *mut FILE,
        poly: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_poly_print(
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_print(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn _fq_poly_print_pretty(
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        x: *const c_char,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_print_pretty(
        poly: *mut fq_poly_struct,
        x: *const c_char,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn _fq_poly_get_str_pretty(
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        x: *const c_char,
        ctx: *mut fq_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_poly_get_str_pretty(
        poly: *mut fq_poly_struct,
        x: *const c_char,
        ctx: *mut fq_ctx_struct,
    ) -> *mut c_char;
    pub fn _fq_poly_get_str(
        poly: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_poly_get_str(poly: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> *mut c_char;
    pub fn fq_mat_charpoly_danilevsky(
        p: *mut fq_poly_struct,
        A: *mut fq_mat_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_mat_charpoly(p: *mut fq_poly_struct, M: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_mat_minpoly(p: *mut fq_poly_struct, X: *mut fq_mat_struct, ctx: *mut fq_ctx_struct);
}
