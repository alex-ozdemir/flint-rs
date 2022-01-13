#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_nmod.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::nmod_mat::nmod_mat_struct;
use crate::nmod_poly::{nmod_poly_struct, nmod_poly_t};
use crate::nmod_vec::nmod_t;
use libc::{c_char, c_int, FILE};

pub type fq_nmod_t = nmod_poly_t;
pub type fq_nmod_struct = nmod_poly_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fq_nmod_ctx_struct {
    pub p: fmpz,
    pub mod_: nmod_t,
    pub sparse_modulus: c_int,
    pub is_conway: c_int,
    pub a: *mut mp_limb_t,
    pub j: *mut mp_limb_signed_t,
    pub len: mp_limb_signed_t,
    pub modulus: nmod_poly_t,
    pub inv: nmod_poly_t,
    pub var: *mut c_char,
}

pub type fq_nmod_ctx_t = [fq_nmod_ctx_struct; 1usize];

extern "C" {
    pub fn fq_nmod_ctx_init(
        ctx: *mut fq_nmod_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn _fq_nmod_ctx_init_conway(
        ctx: *mut fq_nmod_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    ) -> c_int;
    pub fn fq_nmod_ctx_init_conway(
        ctx: *mut fq_nmod_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn fq_nmod_ctx_init_modulus(
        ctx: *mut fq_nmod_ctx_struct,
        modulus: *mut nmod_poly_struct,
        var: *const c_char,
    );
    pub fn fq_nmod_ctx_randtest(ctx: *mut fq_nmod_ctx_struct, state: *mut flint_rand_s);
    pub fn fq_nmod_ctx_randtest_reducible(ctx: *mut fq_nmod_ctx_struct, state: *mut flint_rand_s);
    pub fn fq_nmod_ctx_clear(ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_ctx_modulus(ctx: *mut fq_nmod_ctx_struct) -> *const nmod_poly_struct;
    pub fn fq_nmod_ctx_degree(ctx: *mut fq_nmod_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_nmod_ctx_order(f: *mut fmpz, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_ctx_fprint(file: *mut FILE, ctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn fq_nmod_ctx_print(ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_init(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_init2(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_clear(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn _fq_nmod_sparse_reduce(
        R: *mut mp_limb_t,
        lenR: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_dense_reduce(
        R: *mut mp_limb_t,
        lenR: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_reduce(R: *mut mp_limb_t, lenR: mp_limb_signed_t, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_reduce(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_add(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        op2: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_sub(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        op2: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_sub_one(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_neg(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mul(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        op2: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mul_fmpz(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        x: *mut fmpz,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mul_si(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        x: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_mul_ui(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        x: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_sqr(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_inv(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_pow(
        rop: *mut mp_limb_t,
        op: *const mp_limb_t,
        len: mp_limb_signed_t,
        e: *mut fmpz,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_pow(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        e: *mut fmpz,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_pow_ui(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        e: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_sqrt(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_pth_root(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_is_square(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn fq_nmod_randtest(
        rop: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_randtest_dense(
        rop: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_randtest_not_zero(
        rop: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_rand(
        rop: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_rand_not_zero(
        rop: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_equal(
        op1: *mut nmod_poly_struct,
        op2: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_is_zero(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn fq_nmod_is_one(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn fq_nmod_cmp(
        a: *mut nmod_poly_struct,
        b: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_set(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_set_fmpz(rop: *mut nmod_poly_struct, x: *mut fmpz, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_set_si(
        rop: *mut nmod_poly_struct,
        x: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_set_ui(rop: *mut nmod_poly_struct, x: mp_limb_t, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_swap(
        op1: *mut nmod_poly_struct,
        op2: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_zero(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_one(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_gen(rop: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_get_nmod_poly(
        a: *mut nmod_poly_struct,
        b: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_set_nmod_poly(
        a: *mut nmod_poly_struct,
        b: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_fprint(
        file: *mut FILE,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_print(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_fprint_pretty(
        file: *mut FILE,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_print_pretty(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_get_str(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> *mut c_char;
    pub fn fq_nmod_get_str_pretty(
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> *mut c_char;
    pub fn _fq_nmod_trace(
        rop: *mut fmpz,
        op: *const mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_trace(rop: *mut fmpz, op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn _fq_nmod_frobenius(
        rop: *mut mp_limb_t,
        op: *const mp_limb_t,
        len: mp_limb_signed_t,
        e: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_frobenius(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        e: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_nmod_norm(
        rop: *mut fmpz,
        op: *const mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_norm(rop: *mut fmpz, op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_bit_pack(
        f: *mut fmpz,
        op: *mut nmod_poly_struct,
        bit_size: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_bit_unpack(
        rop: *mut nmod_poly_struct,
        f: *mut fmpz,
        bit_size: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn __fq_nmod_ctx_prime(p: *mut fmpz, ctx: *mut fq_nmod_ctx_struct);
    pub fn fq_nmod_gcdinv(
        rop: *mut nmod_poly_struct,
        inv: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_is_invertible(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
    pub fn fq_nmod_is_invertible_f(
        rop: *mut nmod_poly_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_div(
        rop: *mut nmod_poly_struct,
        op1: *mut nmod_poly_struct,
        op2: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_multiplicative_order(
        ord: *mut fmpz,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
    pub fn fq_nmod_get_nmod_mat(
        col: *mut nmod_mat_struct,
        a: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_set_nmod_mat(
        a: *mut nmod_poly_struct,
        col: *mut nmod_mat_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_nmod_is_primitive(op: *mut nmod_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
