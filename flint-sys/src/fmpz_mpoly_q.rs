#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_mpoly_q.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::fmpz;
use crate::fmpz_mpoly::{fmpz_mpoly_ctx_struct, fmpz_mpoly_struct};
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_q_struct {
    pub num: fmpz_mpoly_struct,
    pub den: fmpz_mpoly_struct,
}

pub type fmpz_mpoly_q_t = [fmpz_mpoly_q_struct; 1usize];

extern "C" {
    /* Memory management */
    pub fn fmpz_mpoly_q_init(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_q_clear(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    /* Assignment */
    pub fn fmpz_mpoly_q_swap(
        x: *mut fmpz_mpoly_q_struct,
        y: *mut fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set_si(
        res: *mut fmpz_mpoly_q_struct,
        x: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Canonicalisation */
    pub fn fmpz_mpoly_q_canonicalise(
        x: *mut fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_is_canonical(
        res: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    /* Properties */
    pub fn fmpz_mpoly_q_is_zero(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_q_is_one(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_q_is_fmpz(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_q_is_fmpq(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mpoly_q_used_vars(
        used: *mut c_int,
        f: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_used_vars_num(
        used: *mut c_int,
        f: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_used_vars_den(
        used: *mut c_int,
        f: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Special values */
    pub fn fmpz_mpoly_q_zero(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_q_one(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_q_gen(
        res: *mut fmpz_mpoly_q_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Input and output */
    pub fn fmpz_mpoly_q_print_pretty(
        f: *const fmpz_mpoly_q_struct,
        x: *mut *const c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Random generation */
    pub fn fmpz_mpoly_q_randtest(
        res: *mut fmpz_mpoly_q_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bound: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Comparisons */
    pub fn fmpz_mpoly_q_equal(
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> c_int;
    /* Arithmetic */
    pub fn fmpz_mpoly_q_neg(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_add(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_sub(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_mul(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_div(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_inv(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );

    pub fn fmpz_mpoly_q_add_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_add_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );

    pub fn fmpz_mpoly_q_sub_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_sub_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );

    pub fn fmpz_mpoly_q_mul_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_mul_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );

    pub fn fmpz_mpoly_q_div_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_div_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );

    pub fn fmpz_mpoly_q_add_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_sub_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_mul_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_div_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Polynomial helper functions */
    pub fn fmpz_mpoly_gcd_assert_successful(
        res: *mut fmpz_mpoly_struct,
        x: *const fmpz_mpoly_struct,
        y: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Content */
    pub fn fmpz_mpoly_q_content(
        res: *mut fmpq,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    /* Evaluation */
    //pub fn fmpz_mpoly_q_evaluate_acb(acb_t res, f: *const fmpz_mpoly_q_struct, acb_srcptr x, prec: mp_limb_signed_t, ctx: *const fmpz_mpoly_ctx_struct);
}
