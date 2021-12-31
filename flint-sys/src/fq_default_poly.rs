#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_default_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq_default::{fq_default_ctx_struct, fq_default_struct};
use crate::fq_default_mat::fq_default_mat_struct;
use crate::fq_nmod_poly::fq_nmod_poly_t;
use crate::fq_poly::fq_poly_t;
use crate::fq_zech_poly::fq_zech_poly_t;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Copy, Clone)]
pub union fq_default_poly_struct {
    pub fq: fq_poly_t,
    pub fq_nmod: fq_nmod_poly_t,
    pub fq_zech: fq_zech_poly_t,
}

pub type fq_default_poly_t = [fq_default_poly_struct; 1usize];

extern "C" {
    pub fn fq_default_poly_init(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_init2(
        poly: *mut fq_default_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_realloc(
        poly: *mut fq_default_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_truncate(
        poly: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_trunc(
        poly1: *mut fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_fit_length(
        poly: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn _fq_default_poly_set_length(
        poly: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_clear(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_length(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_poly_degree(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_poly_randtest(
        f: *mut fq_default_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_randtest_not_zero(
        f: *mut fq_default_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_randtest_monic(
        f: *mut fq_default_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_randtest_irreducible(
        f: *mut fq_default_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_fq_default(
        poly: *mut fq_default_poly_struct,
        c: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_swap(
        op1: *mut fq_default_poly_struct,
        op2: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_zero(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_one(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_gen(f: *mut fq_default_poly_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_poly_make_monic(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_reverse(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_deflation(
        input: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_t;
    pub fn fq_default_poly_deflate(
        result: *mut fq_default_poly_struct,
        input: *const fq_default_poly_struct,
        deflation: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_inflate(
        result: *mut fq_default_poly_struct,
        input: *const fq_default_poly_struct,
        inflation: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_get_coeff(
        x: *mut fq_default_struct,
        poly: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_coeff(
        poly: *mut fq_default_poly_struct,
        n: mp_limb_signed_t,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_coeff_fmpz(
        poly: *mut fq_default_poly_struct,
        n: mp_limb_signed_t,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_nmod_poly(
        rop: *mut fq_default_poly_struct,
        op: *const nmod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_fmpz_mod_poly(
        rop: *mut fq_default_poly_struct,
        op: *const fmpz_mod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_fmpz_poly(
        rop: *mut fq_default_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_equal(
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_equal_trunc(
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_is_zero(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_is_one(
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_is_unit(
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_is_gen(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_equal_fq_default(
        poly: *const fq_default_poly_struct,
        c: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_add(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_add_si(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        c: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_add_series(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_sub(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_sub_series(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_neg(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_scalar_mul_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_scalar_div_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_scalar_addmul_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_scalar_submul_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_mul(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_mullow(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_mulhigh(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        start: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_mulmod(
        res: *mut fq_default_poly_struct,
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_sqr(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_pow(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        e: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_pow_trunc(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_powmod_fmpz_binexp(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        e: *const fmpz,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_powmod_ui_binexp(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        e: mp_limb_t,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_shift_left(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_shift_right(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_hamming_weight(
        op: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_default_poly_gcd(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_xgcd(
        G: *mut fq_default_poly_struct,
        S: *mut fq_default_poly_struct,
        T: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_remove(
        f: *mut fq_default_poly_struct,
        g: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_t;
    pub fn fq_default_poly_divrem(
        Q: *mut fq_default_poly_struct,
        R: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_rem(
        R: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_inv_series(
        Qinv: *mut fq_default_poly_struct,
        Q: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_div_series(
        Q: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_divides(
        Q: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_derivative(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_evaluate_fq_default(
        res: *mut fq_default_struct,
        f: *const fq_default_poly_struct,
        a: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_compose(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_compose_mod(
        res: *mut fq_default_poly_struct,
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        poly3: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fq_default_poly_struct,
        x: *const c_char,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_fprint(
        file: *mut FILE,
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_print(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_print_pretty(
        poly: *const fq_default_poly_struct,
        x: *const c_char,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_poly_get_str_pretty(
        poly: *const fq_default_poly_struct,
        x: *const c_char,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_default_poly_get_str(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_default_mat_charpoly(
        p: *mut fq_default_poly_struct,
        M: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mat_minpoly(
        p: *mut fq_default_poly_struct,
        X: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
}
