#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fq.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mod::{fmpz_mod_ctx_struct, fmpz_mod_ctx_t};
use crate::fmpz_mod_mat::fmpz_mod_mat_struct;
use crate::fmpz_mod_poly::{fmpz_mod_poly_struct, fmpz_mod_poly_t};
use crate::fmpz_poly::{fmpz_poly_struct, fmpz_poly_t};
use libc::{c_char, c_int, FILE};

pub type fq_t = fmpz_poly_t;
pub type fq_struct = fmpz_poly_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_ctx_struct {
    pub ctxp: fmpz_mod_ctx_t,
    pub sparse_modulus: c_int,
    pub is_conway: c_int,
    pub a: *mut fmpz,
    pub j: *mut mp_limb_signed_t,
    pub len: mp_limb_signed_t,
    pub modulus: fmpz_mod_poly_t,
    pub inv: fmpz_mod_poly_t,
    pub var: *mut c_char,
}

pub type fq_ctx_t = [fq_ctx_struct; 1usize];

extern "C" {
    pub fn fq_ctx_init(
        ctx: *mut fq_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn _fq_ctx_init_conway(
        ctx: *mut fq_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    ) -> c_int;
    pub fn fq_ctx_init_conway(
        ctx: *mut fq_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn fq_ctx_init_modulus(
        ctx: *mut fq_ctx_struct,
        modulus: *const fmpz_mod_poly_struct,
        ctxp: *const fmpz_mod_ctx_struct,
        var: *const c_char,
    );
    pub fn fq_ctx_randtest(ctx: *mut fq_ctx_struct, state: *const flint_rand_s);
    pub fn fq_ctx_randtest_reducible(ctx: *mut fq_ctx_struct, state: *const flint_rand_s);
    pub fn fq_ctx_clear(ctx: *mut fq_ctx_struct);
    pub fn fq_ctx_modulus(ctx: *const fq_ctx_struct) -> *const fmpz_mod_poly_struct;
    pub fn fq_ctx_degree(ctx: *const fq_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_ctx_prime(ctx: *const fq_ctx_struct) -> *const fmpz;
    pub fn fq_ctx_order(f: *mut fmpz, ctx: *const fq_ctx_struct);
    pub fn fq_ctx_fprint(file: *mut FILE, ctx: *const fq_ctx_struct) -> c_int;
    pub fn fq_ctx_print(ctx: *const fq_ctx_struct);
    pub fn fq_init(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_init2(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_clear(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn _fq_sparse_reduce(R: *mut fmpz, lenR: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn _fq_dense_reduce(R: *mut fmpz, lenR: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn _fq_reduce(R: *mut fmpz, lenR: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn fq_reduce(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_add(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sub(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sub_one(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_neg(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul_fmpz(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        x: *const fmpz,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul_si(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul_ui(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        x: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sqr(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_inv(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn _fq_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: *const fmpz,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_pow(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        e: *const fmpz,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_pow_ui(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        e: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sqrt(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> c_int;
    pub fn fq_pth_root(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_is_square(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> c_int;
    pub fn fq_randtest(
        rop: *mut fmpz_poly_struct,
        state: *const flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_randtest_dense(
        rop: *mut fmpz_poly_struct,
        state: *const flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_randtest_not_zero(
        rop: *mut fmpz_poly_struct,
        state: *const flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_rand(
        rop: *mut fmpz_poly_struct,
        state: *const flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_rand_not_zero(
        rop: *mut fmpz_poly_struct,
        state: *const flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_equal(
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> c_int;
    pub fn fq_is_zero(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> c_int;
    pub fn fq_is_one(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> c_int;
    pub fn fq_set(rop: *mut fmpz_poly_struct, op: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_set_fmpz(rop: *mut fmpz_poly_struct, x: *const fmpz, ctx: *const fq_ctx_struct);
    pub fn fq_set_ui(rop: *mut fmpz_poly_struct, x: mp_limb_t, ctx: *const fq_ctx_struct);
    pub fn fq_set_si(rop: *mut fmpz_poly_struct, x: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn fq_swap(
        op1: *mut fmpz_poly_struct,
        op2: *mut fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_zero(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_one(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_gen(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_get_fmpz_poly(
        a: *mut fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_set_fmpz_poly(
        a: *mut fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_get_fmpz_mod_poly(
        a: *mut fmpz_mod_poly_struct,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_set_fmpz_mod_poly(
        a: *mut fmpz_poly_struct,
        b: *const fmpz_mod_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_fprint(
        file: *mut FILE,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> c_int;
    pub fn fq_print(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_fprint_pretty(
        file: *mut FILE,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> c_int;
    pub fn fq_print_pretty(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> c_int;
    pub fn fq_get_str(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> *mut c_char;
    pub fn fq_get_str_pretty(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct)
        -> *mut c_char;
    pub fn _fq_trace(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_trace(rop: *mut fmpz, op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn _fq_frobenius(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_frobenius(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        e: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn _fq_norm(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_norm(rop: *mut fmpz, op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_bit_pack(
        f: *mut fmpz,
        op: *const fmpz_poly_struct,
        bit_size: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_bit_unpack(
        rop: *mut fmpz_poly_struct,
        f: *const fmpz,
        bit_size: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn __fq_ctx_prime(p: *mut fmpz, ctx: *const fq_ctx_struct);
    pub fn fq_gcdinv(
        rop: *mut fmpz_poly_struct,
        inv: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_is_invertible(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> c_int;
    pub fn fq_is_invertible_f(
        rop: *const fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> c_int;
    pub fn fq_div(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_multiplicative_order(
        ord: *mut fmpz,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> c_int;
    pub fn fq_get_fmpz_mod_mat(
        col: *mut fmpz_mod_mat_struct,
        a: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_set_fmpz_mod_mat(
        a: *mut fmpz_poly_struct,
        col: *const fmpz_mod_mat_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_is_primitive(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> c_int;
}
