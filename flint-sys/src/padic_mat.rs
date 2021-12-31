#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/padic_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq_mat::fmpq_mat_struct;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::padic::{padic_ctx_struct, padic_struct};
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct padic_mat_struct {
    pub mat: fmpz_mat_struct,
    pub val: mp_limb_signed_t,
    pub N: mp_limb_signed_t,
}

pub type padic_mat_t = [padic_mat_struct; 1usize];

extern "C" {
    pub fn padic_mat(A: *mut padic_mat_struct) -> *mut fmpz_mat_struct;
    pub fn padic_mat_entry(
        A: *mut padic_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpz;
    pub fn padic_mat_get_val(A: *mut padic_mat_struct) -> mp_limb_signed_t;
    pub fn padic_mat_get_prec(A: *mut padic_mat_struct) -> mp_limb_signed_t;
    pub fn padic_mat_nrows(A: *mut padic_mat_struct) -> mp_limb_signed_t;
    pub fn padic_mat_ncols(A: *mut padic_mat_struct) -> mp_limb_signed_t;
    pub fn padic_mat_init(A: *mut padic_mat_struct, r: mp_limb_signed_t, c: mp_limb_signed_t);
    pub fn padic_mat_init2(
        A: *mut padic_mat_struct,
        r: mp_limb_signed_t,
        c: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn padic_mat_clear(A: *mut padic_mat_struct);
    pub fn _padic_mat_canonicalise(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct);
    pub fn _padic_mat_reduce(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct);
    pub fn padic_mat_reduce(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct);
    pub fn padic_mat_is_empty(A: *mut padic_mat_struct) -> c_int;
    pub fn padic_mat_is_square(A: *mut padic_mat_struct) -> c_int;
    pub fn padic_mat_is_canonical(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct) -> c_int;
    pub fn padic_mat_is_reduced(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct) -> c_int;
    pub fn padic_mat_set(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_swap(A: *mut padic_mat_struct, B: *mut padic_mat_struct);
    pub fn padic_mat_zero(A: *mut padic_mat_struct);
    pub fn padic_mat_one(A: *mut padic_mat_struct);
    pub fn padic_mat_set_fmpq_mat(
        B: *mut padic_mat_struct,
        A: *mut fmpq_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_get_fmpq_mat(
        B: *mut fmpq_mat_struct,
        A: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_get_entry_padic(
        rop: *mut padic_struct,
        op: *mut padic_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_set_entry_padic(
        rop: *mut padic_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        op: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_equal(A: *mut padic_mat_struct, B: *mut padic_mat_struct) -> c_int;
    pub fn padic_mat_is_zero(A: *mut padic_mat_struct) -> c_int;
    pub fn padic_mat_fprint(
        file: *mut FILE,
        A: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_mat_fprint_pretty(
        file: *mut FILE,
        A: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    ) -> c_int;
    pub fn padic_mat_print(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct) -> c_int;
    pub fn padic_mat_print_pretty(A: *mut padic_mat_struct, ctx: *mut padic_ctx_struct) -> c_int;
    pub fn padic_mat_randtest(
        mat: *mut padic_mat_struct,
        state: *mut flint_rand_s,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_transpose(B: *mut padic_mat_struct, A: *mut padic_mat_struct);
    pub fn _padic_mat_add(
        C: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        B: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_add(
        C: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        B: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_mat_sub(
        C: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        B: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_sub(
        C: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        B: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_mat_neg(B: *mut padic_mat_struct, A: *mut padic_mat_struct);
    pub fn padic_mat_neg(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_mat_scalar_mul_padic(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        c: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_scalar_mul_padic(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        c: *mut padic_struct,
        ctx: *mut padic_ctx_struct,
    );
    pub fn _padic_mat_scalar_mul_fmpz(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        c: *mut fmpz,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_scalar_mul_fmpz(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        c: *mut fmpz,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_scalar_div_fmpz(
        B: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        c: *mut fmpz,
        ctx: *mut padic_ctx_struct,
    );
    pub fn padic_mat_mul(
        C: *mut padic_mat_struct,
        A: *mut padic_mat_struct,
        B: *mut padic_mat_struct,
        ctx: *mut padic_ctx_struct,
    );
}
