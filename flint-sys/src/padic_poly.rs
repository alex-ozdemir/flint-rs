#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/padic_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpq_poly::fmpq_poly_struct;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::padic::{padic_ctx_struct, padic_struct};
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct padic_poly_struct {
    pub coeffs: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub val: mp_limb_signed_t,
    pub N: mp_limb_signed_t,
}

pub type padic_poly_t = [padic_poly_struct; 1usize];

extern "C" {
    pub fn _fmpz_vec_ord_p(
        vec: *const fmpz,
        len: mp_limb_signed_t,
        p: *mut fmpz,
    ) -> mp_limb_signed_t;
    pub fn padic_poly_init(poly: *mut padic_poly_struct);
    pub fn padic_poly_init2(
        poly: *mut padic_poly_struct,
        alloc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn padic_poly_clear(poly: *mut padic_poly_struct);
    pub fn padic_poly_realloc(f: *mut padic_poly_struct, alloc: mp_limb_signed_t, p: *mut fmpz);
    pub fn padic_poly_fit_length(f: *mut padic_poly_struct, len: mp_limb_signed_t);
    pub fn _padic_poly_set_length(poly: *mut padic_poly_struct, len: mp_limb_signed_t);
    pub fn _padic_poly_normalise(f: *mut padic_poly_struct);
    pub fn _padic_poly_canonicalise(
        poly: *mut fmpz,
        v: *mut mp_limb_signed_t,
        len: mp_limb_signed_t,
        p: *mut fmpz,
    );
    pub fn padic_poly_canonicalise(poly: *mut padic_poly_struct, p: *mut fmpz);
    pub fn padic_poly_reduce(f: *mut padic_poly_struct, ctx: *mut padic_ctx_struct);
    pub fn padic_poly_truncate(poly: *mut padic_poly_struct, n: mp_limb_signed_t, p: *mut fmpz);
    pub fn padic_poly_degree(poly: *mut padic_poly_struct) -> mp_limb_signed_t;
    pub fn padic_poly_length(poly: *mut padic_poly_struct) -> mp_limb_signed_t;
    pub fn padic_poly_val(poly: *mut padic_poly_struct) -> mp_limb_signed_t;
    pub fn padic_poly_randtest(
        f: *mut padic_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_randtest_not_zero(
        f: *mut padic_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_randtest_val(
        f: *mut padic_poly_struct,
        state: *mut flint_rand_s,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set(
        f: *mut padic_poly_struct,
        g: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_padic(
        poly: *mut padic_poly_struct,
        x: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_si(
        poly: *mut padic_poly_struct,
        x: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_ui(
        poly: *mut padic_poly_struct,
        x: mp_limb_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_fmpz(
        poly: *mut padic_poly_struct,
        x: *mut fmpz,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_fmpq(
        poly: *mut padic_poly_struct,
        x: *mut fmpq,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_fmpz_poly(
        rop: *mut padic_poly_struct,
        op: *mut fmpz_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_fmpq_poly(
        rop: *mut padic_poly_struct,
        op: *mut fmpq_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_get_fmpz_poly(
        rop: *mut fmpz_poly_struct,
        op: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_get_fmpq_poly(
        rop: *mut fmpq_poly_struct,
        op: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_zero(poly: *mut padic_poly_struct);
    pub fn padic_poly_one(poly: *mut padic_poly_struct);
    pub fn padic_poly_swap(poly1: *mut padic_poly_struct, poly2: *mut padic_poly_struct);
    pub fn padic_poly_get_coeff_padic(
        c: *mut padic_struct,
        poly: *mut padic_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_set_coeff_padic(
        f: *mut padic_poly_struct,
        n: mp_limb_signed_t,
        c: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_equal(f: *mut padic_poly_struct, g: *mut padic_poly_struct) -> c_int;
    pub fn padic_poly_is_zero(poly: *mut padic_poly_struct) -> c_int;
    pub fn padic_poly_is_one(poly: *mut padic_poly_struct) -> c_int;
    pub fn _padic_poly_add(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op1: *const fmpz,
        val1: mp_limb_signed_t,
        len1: mp_limb_signed_t,
        N1: mp_limb_signed_t,
        op2: *const fmpz,
        val2: mp_limb_signed_t,
        len2: mp_limb_signed_t,
        N2: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_add(
        f: *mut padic_poly_struct,
        g: *mut padic_poly_struct,
        h: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_sub(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op1: *const fmpz,
        val1: mp_limb_signed_t,
        len1: mp_limb_signed_t,
        N1: mp_limb_signed_t,
        op2: *const fmpz,
        val2: mp_limb_signed_t,
        len2: mp_limb_signed_t,
        N2: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_sub(
        f: *mut padic_poly_struct,
        g: *mut padic_poly_struct,
        h: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_neg(
        f: *mut padic_poly_struct,
        g: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_scalar_mul_padic(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        c: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_scalar_mul_padic(
        rop: *mut padic_poly_struct,
        op: *mut padic_poly_struct,
        c: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_mul(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op1: *const fmpz,
        val1: mp_limb_signed_t,
        len1: mp_limb_signed_t,
        op2: *const fmpz,
        val2: mp_limb_signed_t,
        len2: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_mul(
        f: *mut padic_poly_struct,
        g: *mut padic_poly_struct,
        h: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_pow(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        e: mp_limb_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_pow(
        rop: *mut padic_poly_struct,
        op: *mut padic_poly_struct,
        e: mp_limb_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_inv_series(
        Qinv: *mut padic_poly_struct,
        Q: *mut padic_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_derivative(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_derivative(
        rop: *mut padic_poly_struct,
        op: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_shift_left(
        rop: *mut padic_poly_struct,
        op: *mut padic_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_shift_right(
        rop: *mut padic_poly_struct,
        op: *mut padic_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_evaluate_padic(
        u: *mut fmpz,
        v: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        poly: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *mut fmpz,
        b: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_evaluate_padic(
        y: *mut padic_struct,
        poly: *mut padic_poly_struct,
        x: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_compose(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op1: *const fmpz,
        val1: mp_limb_signed_t,
        len1: mp_limb_signed_t,
        op2: *const fmpz,
        val2: mp_limb_signed_t,
        len2: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_compose(
        rop: *mut padic_poly_struct,
        op1: *mut padic_poly_struct,
        op2: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_poly_compose_pow(
        rop: *mut fmpz,
        rval: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        op: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        k: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_compose_pow(
        rop: *mut padic_poly_struct,
        op: *mut padic_poly_struct,
        k: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_poly_debug(poly: *mut padic_poly_struct) -> c_int;
    pub fn _padic_poly_fprint(
        file: *mut FILE,
        poly: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_fprint(
        file: *mut FILE,
        poly: *mut padic_poly_struct,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn _padic_poly_print(
        poly: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_print(poly: *mut padic_poly_struct, ctx: *mut padic_ctx_struct) -> c_int;
    pub fn _padic_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        var: *const c_char,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_fprint_pretty(
        file: *mut FILE,
        poly: *mut padic_poly_struct,
        var: *const c_char,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn _padic_poly_print_pretty(
        file: *mut FILE,
        poly: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        var: *const c_char,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_print_pretty(
        poly: *mut padic_poly_struct,
        var: *const c_char,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn _padic_poly_is_canonical(
        op: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_is_canonical(op: *mut padic_poly_struct, ctx: *mut padic_ctx_struct)
        -> c_int;
    pub fn _padic_poly_is_reduced(
        op: *const fmpz,
        val: mp_limb_signed_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_poly_is_reduced(op: *mut padic_poly_struct, ctx: *mut padic_ctx_struct) -> c_int;
}
