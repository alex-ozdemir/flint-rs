#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_nmod_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::fq_nmod::{fq_nmod_ctx_struct, fq_nmod_struct};
use crate::fq_nmod_mat::fq_nmod_mat_struct;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_poly_struct {
    pub coeffs: *mut fq_nmod_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fq_nmod_poly_t = [fq_nmod_poly_struct; 1usize];

extern "C" {
    pub fn fq_nmod_poly_init(poly: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_poly_init2(
        poly: *mut fq_nmod_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_realloc(
        poly: *mut fq_nmod_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_truncate(
        poly: *mut fq_nmod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set_trunc(
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_fit_length(
        poly: *mut fq_nmod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_clear(poly: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn _fq_nmod_poly_normalise(poly: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn _fq_nmod_poly_normalise2(
        poly: *const fq_nmod_struct,
        length: *mut mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_set_length(
        poly: *mut fq_nmod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_length(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_degree(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_lead(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut fq_nmod_struct;
    pub fn fq_nmod_poly_randtest(
        f: *mut fq_nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_randtest_not_zero(
        f: *mut fq_nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_randtest_monic(
        f: *mut fq_nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_randtest_irreducible(
        f: *mut fq_nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_set(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set_fq_nmod(
        poly: *mut fq_nmod_poly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set_fmpz_mod_poly(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fmpz_mod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set_nmod_poly(
        rop: *mut fq_nmod_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_swap(
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_zero(
        rop: *mut fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_zero(poly: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_poly_one(poly: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_poly_gen(f: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn _fq_nmod_poly_make_monic(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_make_monic(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_reverse(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_reverse(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_deflation(
        input: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_t;
    pub fn fq_nmod_poly_deflate(
        result: *mut fq_nmod_poly_struct,
        input: *mut fq_nmod_poly_struct,
        deflation: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_inflate(
        result: *mut fq_nmod_poly_struct,
        input: *mut fq_nmod_poly_struct,
        inflation: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_get_coeff(
        x: *mut nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set_coeff(
        poly: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_set_coeff_fmpz(
        poly: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        x: *mut fmpz,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_is_gen(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_equal(
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_equal_trunc(
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_is_zero(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_is_one(op: *mut fq_nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct)
        -> c_int;
    pub fn fq_nmod_poly_is_unit(
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_equal_fq_nmod(
        poly: *mut fq_nmod_poly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_poly_add(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_add(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_add_si(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_add_series(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_sub(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_sub(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_sub_series(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_neg(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_neg(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_scalar_mul_fq_nmod(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_scalar_mul_fq_nmod(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_scalar_div_fq_nmod(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_scalar_div_fq_nmod(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_scalar_addmul_fq_nmod(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_scalar_addmul_fq_nmod(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_scalar_submul_fq_nmod(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_scalar_submul_fq_nmod(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        x: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mul_classical(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mul_classical(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mul_reorder(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mul_reorder(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mul_univariate(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mul_univariate(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mul_KS(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mul_KS(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mul(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mul(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mullow_classical(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mullow_classical(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mullow_KS(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mullow_KS(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mullow_univariate(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mullow_univariate(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mullow(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mullow(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mulhigh_classical(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        start: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mulhigh_classical(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        start: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mulhigh(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mulhigh(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        start: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mulmod(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mulmod(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        f: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_mulmod_preinv(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_nmod_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_mulmod_preinv(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        f: *mut fq_nmod_poly_struct,
        finv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_sqr_classical(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_sqr_classical(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_sqr_reorder(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_sqr_reorder(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_sqr_KS(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_sqr_KS(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_sqr(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_sqr(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_pow(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        e: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_pow(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        e: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_pow_trunc_binexp(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_pow_trunc_binexp(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_pow_trunc(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_pow_trunc(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_powmod_fmpz_binexp(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: *mut fmpz,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_powmod_fmpz_binexp(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: *mut fmpz,
        f: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_powmod_fmpz_binexp_preinv(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: *mut fmpz,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_nmod_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_powmod_fmpz_binexp_preinv(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: *mut fmpz,
        f: *mut fq_nmod_poly_struct,
        finv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_powmod_ui_binexp(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: mp_limb_t,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_powmod_ui_binexp(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: mp_limb_t,
        f: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_powmod_ui_binexp_preinv(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: mp_limb_t,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_nmod_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_powmod_ui_binexp_preinv(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: mp_limb_t,
        f: *mut fq_nmod_poly_struct,
        finv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_powmod_fmpz_sliding_preinv(
        res: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        e: *mut fmpz,
        k: mp_limb_t,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_nmod_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_powmod_fmpz_sliding_preinv(
        res: *mut fq_nmod_poly_struct,
        poly: *mut fq_nmod_poly_struct,
        e: *mut fmpz,
        k: mp_limb_t,
        f: *mut fq_nmod_poly_struct,
        finv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_powmod_x_fmpz_preinv(
        res: *mut fq_nmod_struct,
        e: *mut fmpz,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        finv: *const fq_nmod_struct,
        lenfinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_powmod_x_fmpz_preinv(
        res: *mut fq_nmod_poly_struct,
        e: *mut fmpz,
        f: *mut fq_nmod_poly_struct,
        finv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_shift_left(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_shift_left(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_shift_right(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_shift_right(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_hamming_weight(
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_hamming_weight(
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_gcd_euclidean(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_gcd_euclidean(
        G: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn _fq_nmod_poly_gcd(
        G: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_gcd(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_gcd_euclidean_f(
        f: *mut nmod_poly_struct,
        G: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_gcd_euclidean_f(
        f: *mut nmod_poly_struct,
        G: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_hgcd(
        M: *mut *mut fq_nmod_struct,
        lenM: *mut mp_limb_signed_t,
        A: *mut fq_nmod_struct,
        lenA: *mut mp_limb_signed_t,
        B: *mut fq_nmod_struct,
        lenB: *mut mp_limb_signed_t,
        a: *const fq_nmod_struct,
        lena: mp_limb_signed_t,
        b: *const fq_nmod_struct,
        lenb: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn _fq_nmod_poly_gcd_hgcd(
        G: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_gcd_hgcd(
        G: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_xgcd_euclidean_f(
        f: *mut nmod_poly_struct,
        G: *mut fq_nmod_struct,
        S: *mut fq_nmod_struct,
        fq_nmod: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_xgcd_euclidean_f(
        f: *mut nmod_poly_struct,
        G: *mut fq_nmod_poly_struct,
        S: *mut fq_nmod_poly_struct,
        fq_nmod: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_xgcd_euclidean(
        G: *mut fq_nmod_struct,
        S: *mut fq_nmod_struct,
        fq_nmod: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_xgcd_euclidean(
        G: *mut fq_nmod_poly_struct,
        S: *mut fq_nmod_poly_struct,
        fq_nmod: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_xgcd(
        G: *mut fq_nmod_struct,
        S: *mut fq_nmod_struct,
        fq_nmod: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_nmod_poly_xgcd(
        G: *mut fq_nmod_poly_struct,
        S: *mut fq_nmod_poly_struct,
        fq_nmod: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_remove(
        f: *mut fq_nmod_poly_struct,
        g: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_t;
    pub fn _fq_nmod_poly_div_basecase(
        Q: *mut fq_nmod_struct,
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_div_basecase(
        Q: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divrem_basecase(
        Q: *mut fq_nmod_struct,
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_divrem_basecase(
        Q: *mut fq_nmod_poly_struct,
        R: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divrem_divconquer_recursive(
        Q: *mut fq_nmod_struct,
        BQ: *mut fq_nmod_struct,
        W: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divrem_divconquer(
        Q: *mut fq_nmod_struct,
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_divrem_divconquer(
        Q: *mut fq_nmod_poly_struct,
        R: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divrem(
        Q: *mut fq_nmod_struct,
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_divrem(
        Q: *mut fq_nmod_poly_struct,
        R: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_rem(
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_rem(
        R: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_inv_series_newton(
        Qinv: *mut fq_nmod_struct,
        Q: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        cinv: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_inv_series_newton(
        Qinv: *mut fq_nmod_poly_struct,
        Q: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_inv_series(
        Qinv: *mut fq_nmod_struct,
        Q: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        cinv: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_inv_series(
        Qinv: *mut fq_nmod_poly_struct,
        Q: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_div_series(
        Q: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        Alen: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_div_series(
        Q: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_div_newton_n_preinv(
        Q: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        Binv: *const fq_nmod_struct,
        lenBinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_div_newton_n_preinv(
        Q: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        Binv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divrem_newton_n_preinv(
        Q: *mut fq_nmod_struct,
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        Binv: *const fq_nmod_struct,
        lenBinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_divrem_newton_n_preinv(
        Q: *mut fq_nmod_poly_struct,
        R: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        Binv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divrem_f(
        f: *mut nmod_poly_struct,
        Q: *mut fq_nmod_struct,
        R: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_divrem_f(
        f: *mut nmod_poly_struct,
        Q: *mut fq_nmod_poly_struct,
        R: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_divides(
        Q: *mut fq_nmod_struct,
        A: *const fq_nmod_struct,
        lenA: mp_limb_signed_t,
        B: *const fq_nmod_struct,
        lenB: mp_limb_signed_t,
        invB: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_divides(
        Q: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_poly_derivative(
        rop: *mut fq_nmod_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_derivative(
        rop: *mut fq_nmod_poly_struct,
        op: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_evaluate_fq_nmod(
        rop: *mut nmod_poly_struct,
        op: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        a: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_evaluate_fq_nmod(
        res: *mut nmod_poly_struct,
        f: *mut fq_nmod_poly_struct,
        a: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_evaluate_fq_nmod_vec(
        ys: *mut fq_nmod_struct,
        coeffs: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        xs: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_evaluate_fq_nmod_vec(
        ys: *mut fq_nmod_struct,
        poly: *mut fq_nmod_poly_struct,
        xs: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_tree_alloc(
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut *mut fq_nmod_poly_struct;
    pub fn _fq_nmod_poly_tree_free(
        tree: *mut *mut fq_nmod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_tree_build(
        tree: *mut *mut fq_nmod_poly_struct,
        roots: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_evaluate_fq_nmod_vec_fast_precomp(
        vs: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        plen: mp_limb_signed_t,
        tree: *const *mut fq_nmod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_evaluate_fq_nmod_vec_fast(
        ys: *mut fq_nmod_struct,
        poly: *const fq_nmod_struct,
        plen: mp_limb_signed_t,
        xs: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_evaluate_fq_nmod_vec_fast(
        ys: *mut fq_nmod_struct,
        poly: *mut fq_nmod_poly_struct,
        xs: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_evaluate_fq_nmod_vec_iter(
        ys: *mut fq_nmod_struct,
        coeffs: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        xs: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_evaluate_fq_nmod_vec_iter(
        ys: *mut fq_nmod_struct,
        poly: *mut fq_nmod_poly_struct,
        xs: *const fq_nmod_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_divconquer(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_divconquer(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_horner(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_horner(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose(
        rop: *mut fq_nmod_struct,
        op1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        op2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose(
        rop: *mut fq_nmod_poly_struct,
        op1: *mut fq_nmod_poly_struct,
        op2: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod(
        res: *mut fq_nmod_struct,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_nmod_struct,
        h: *const fq_nmod_struct,
        lenh: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly3: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod_preinv(
        res: *mut fq_nmod_struct,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_nmod_struct,
        h: *const fq_nmod_struct,
        lenh: mp_limb_signed_t,
        hinv: *const fq_nmod_struct,
        lenhinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod_preinv(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly3: *mut fq_nmod_poly_struct,
        poly3inv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod_horner(
        res: *mut fq_nmod_struct,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_nmod_struct,
        h: *const fq_nmod_struct,
        lenh: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod_horner(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly3: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod_horner_preinv(
        res: *mut fq_nmod_struct,
        f: *const fq_nmod_struct,
        lenf: mp_limb_signed_t,
        g: *const fq_nmod_struct,
        h: *const fq_nmod_struct,
        lenh: mp_limb_signed_t,
        hinv: *const fq_nmod_struct,
        lenhinv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod_horner_preinv(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly3: *mut fq_nmod_poly_struct,
        poly3inv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod_brent_kung(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly3: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod_brent_kung(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        poly3: *const fq_nmod_struct,
        len3: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod_brent_kung_preinv(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        poly2: *const fq_nmod_struct,
        poly3: *const fq_nmod_struct,
        len3: mp_limb_signed_t,
        poly3inv: *const fq_nmod_struct,
        len3inv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod_brent_kung_preinv(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly3: *mut fq_nmod_poly_struct,
        poly3inv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_reduce_matrix_mod_poly(
        A: *mut fq_nmod_mat_struct,
        B: *mut fq_nmod_mat_struct,
        f: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_precompute_matrix(
        A: *mut fq_nmod_mat_struct,
        poly1: *const fq_nmod_struct,
        poly2: *const fq_nmod_struct,
        len2: mp_limb_signed_t,
        poly2inv: *const fq_nmod_struct,
        len2inv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_precompute_matrix(
        A: *mut fq_nmod_mat_struct,
        poly1: *mut fq_nmod_poly_struct,
        poly2: *mut fq_nmod_poly_struct,
        poly2inv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut fq_nmod_struct,
        poly1: *const fq_nmod_struct,
        len1: mp_limb_signed_t,
        A: *mut fq_nmod_mat_struct,
        poly3: *const fq_nmod_struct,
        len3: mp_limb_signed_t,
        poly3inv: *const fq_nmod_struct,
        len3inv: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut fq_nmod_poly_struct,
        poly1: *mut fq_nmod_poly_struct,
        A: *mut fq_nmod_mat_struct,
        poly3: *mut fq_nmod_poly_struct,
        poly3inv: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_fprint_pretty(
        file: *mut FILE,
        poly: *mut fq_nmod_poly_struct,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_poly_fprint(
        file: *mut FILE,
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_fprint(
        file: *mut FILE,
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_poly_print(
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_print(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_poly_print_pretty(
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_poly_print_pretty(
        poly: *mut fq_nmod_poly_struct,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn _fq_nmod_poly_get_str_pretty(
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_nmod_poly_get_str_pretty(
        poly: *mut fq_nmod_poly_struct,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut c_char;
    pub fn _fq_nmod_poly_get_str(
        poly: *const fq_nmod_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_nmod_poly_get_str(
        poly: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut c_char;
}
