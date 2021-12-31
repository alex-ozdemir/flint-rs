#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/qadic.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::padic::{padic_ctx_struct, padic_print_mode, padic_struct};
use crate::padic_poly::{padic_poly_struct, padic_poly_t};
use libc::{c_char, c_int, FILE};

pub type qadic_t = padic_poly_t;
pub type qadic_struct = padic_poly_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qadic_ctx_struct {
    pub pctx: padic_ctx_struct,
    pub a: *mut fmpz,
    pub j: *mut mp_limb_signed_t,
    pub len: mp_limb_signed_t,
    pub var: *mut c_char,
}

pub type qadic_ctx_t = [qadic_ctx_struct; 1usize];

extern "C" {
    pub fn qadic_val(op: *const padic_poly_struct) -> mp_limb_signed_t;
    pub fn qadic_prec(op: *const padic_poly_struct) -> mp_limb_signed_t;
    pub fn qadic_ctx_init_conway(
        ctx: *mut qadic_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        min: mp_limb_signed_t,
        max: mp_limb_signed_t,
        var: *const c_char,
        mode: padic_print_mode,
    );
    pub fn qadic_ctx_init(
        ctx: *mut qadic_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        min: mp_limb_signed_t,
        max: mp_limb_signed_t,
        var: *const c_char,
        mode: padic_print_mode,
    );
    pub fn qadic_ctx_clear(ctx: *mut qadic_ctx_struct);
    pub fn qadic_ctx_degree(ctx: *const qadic_ctx_struct) -> mp_limb_signed_t;
    pub fn qadic_ctx_print(ctx: *const qadic_ctx_struct);
    pub fn qadic_init(x: *mut padic_poly_struct);
    pub fn qadic_init2(rop: *mut padic_poly_struct, prec: mp_limb_signed_t);
    pub fn qadic_clear(x: *mut padic_poly_struct);
    pub fn _fmpz_poly_reduce(
        R: *mut fmpz,
        lenR: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
    pub fn _fmpz_mod_poly_reduce(
        R: *mut fmpz,
        lenR: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        len: mp_limb_signed_t,
        p: *mut fmpz,
    );
    pub fn qadic_reduce(x: *mut padic_poly_struct, ctx: *const qadic_ctx_struct);
    pub fn qadic_randtest(
        x: *mut padic_poly_struct,
        state: *const flint_rand_s,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_randtest_not_zero(
        x: *mut padic_poly_struct,
        state: *const flint_rand_s,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_randtest_val(
        x: *mut padic_poly_struct,
        state: *const flint_rand_s,
        val: mp_limb_signed_t,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_randtest_int(
        x: *mut padic_poly_struct,
        state: *const flint_rand_s,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_zero(op: *mut padic_poly_struct);
    pub fn qadic_one(op: *mut padic_poly_struct);
    pub fn qadic_gen(x: *mut padic_poly_struct, ctx: *const qadic_ctx_struct);
    pub fn qadic_set_ui(rop: *mut padic_poly_struct, op: mp_limb_t, ctx: *const qadic_ctx_struct);
    pub fn qadic_get_padic(
        rop: *mut padic_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn qadic_set(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_set_fmpz_poly(
        rop: *mut padic_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_is_zero(op: *const padic_poly_struct) -> c_int;
    pub fn qadic_is_one(op: *const padic_poly_struct) -> c_int;
    pub fn qadic_equal(op1: *const padic_poly_struct, op2: *const padic_poly_struct) -> c_int;
    pub fn qadic_add(
        x: *mut padic_poly_struct,
        y: *const padic_poly_struct,
        z: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_sub(
        x: *mut padic_poly_struct,
        y: *const padic_poly_struct,
        z: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_neg(
        x: *mut padic_poly_struct,
        y: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_mul(
        x: *mut padic_poly_struct,
        y: *const padic_poly_struct,
        z: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn _qadic_inv(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn qadic_inv(
        x: *mut padic_poly_struct,
        y: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn _qadic_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: *const fmpz,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn qadic_pow(
        x: *mut padic_poly_struct,
        y: *const padic_poly_struct,
        e: *const fmpz,
        ctx: *const qadic_ctx_struct,
    );
    pub fn _qadic_exp_rectangular(
        rop: *mut fmpz,
        op: *const fmpz,
        v: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
        pN: *mut fmpz,
    );
    pub fn qadic_exp_rectangular(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn _qadic_exp_balanced(
        rop: *mut fmpz,
        op: *const fmpz,
        v: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
        pN: *const fmpz,
    );
    pub fn qadic_exp_balanced(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn _qadic_exp(
        rop: *mut fmpz,
        op: *const fmpz,
        v: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
        pN: *const fmpz,
    );
    pub fn qadic_exp(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn _qadic_log_rectangular(
        z: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
        pN: *const fmpz,
    );
    pub fn qadic_log_rectangular(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn _qadic_log_balanced(
        z: *mut fmpz,
        y: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
        pN: *const fmpz,
    );
    pub fn qadic_log_balanced(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn _qadic_log(
        z: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
        pN: *const fmpz,
    );
    pub fn qadic_log(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn _qadic_frobenius_a(
        rop: *mut fmpz,
        exp: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _qadic_frobenius(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn qadic_frobenius(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        e: mp_limb_signed_t,
        ctx: *const qadic_ctx_struct,
    );
    pub fn _qadic_teichmuller(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn qadic_teichmuller(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn _qadic_trace(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        pN: *const fmpz,
    );
    pub fn qadic_trace(
        rop: *mut padic_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn _qadic_norm_resultant(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _qadic_norm_analytic(
        rop: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _qadic_norm(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn qadic_norm(
        rop: *mut padic_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_norm_analytic(
        rop: *mut padic_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_norm_resultant(
        rop: *mut padic_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    );
    pub fn qadic_sqrt(
        rop: *mut padic_poly_struct,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn qadic_fprint_pretty(
        file: *mut FILE,
        op: *const padic_poly_struct,
        ctx: *const qadic_ctx_struct,
    ) -> c_int;
    pub fn qadic_print_pretty(op: *mut padic_poly_struct, ctx: *const qadic_ctx_struct) -> c_int;
    pub fn qadic_debug(op: *const padic_poly_struct) -> c_int;
}
