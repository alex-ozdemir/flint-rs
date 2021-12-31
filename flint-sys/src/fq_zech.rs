#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_zech.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fq_nmod::fq_nmod_ctx_struct;
use crate::nmod_mat::nmod_mat_struct;
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fq_zech_struct {
    pub value: mp_limb_t,
}

pub type fq_zech_t = [fq_zech_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_ctx_struct {
    pub qm1: mp_limb_t,
    pub qm1o2: mp_limb_t,
    pub qm1opm1: mp_limb_t,
    pub p: mp_limb_t,
    pub ppre: f64,
    pub prime_root: mp_limb_t,
    pub zech_log_table: *mut mp_limb_t,
    pub prime_field_table: *mut mp_limb_t,
    pub eval_table: *mut mp_limb_t,
    pub fq_nmod_ctx: *mut fq_nmod_ctx_struct,
    pub owns_fq_nmod_ctx: c_int,
    pub is_conway: c_int,
}

pub type fq_zech_ctx_t = [fq_zech_ctx_struct; 1usize];

extern "C" {
    pub fn fq_zech_ctx_init(
        ctx: *mut fq_zech_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn fq_zech_ctx_init_fq_nmod_ctx(
        ctx: *mut fq_zech_ctx_struct,
        ctxn: *mut fq_nmod_ctx_struct,
    );
    pub fn _fq_zech_ctx_init_conway(
        ctx: *mut fq_zech_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    ) -> c_int;
    pub fn fq_zech_ctx_init_conway(
        ctx: *mut fq_zech_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn fq_zech_ctx_init_random(
        ctx: *mut fq_zech_ctx_struct,
        p: *mut fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn fq_zech_ctx_init_modulus(
        ctx: *mut fq_zech_ctx_struct,
        modulus: *mut nmod_poly_struct,
        var: *const c_char,
    );
    pub fn fq_zech_ctx_randtest(ctx: *mut fq_zech_ctx_struct, state: *mut flint_rand_s);
    pub fn fq_zech_ctx_randtest_reducible(ctx: *mut fq_zech_ctx_struct, state: *mut flint_rand_s);
    pub fn fq_zech_ctx_clear(ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_ctx_modulus(ctx: *mut fq_zech_ctx_struct) -> *const nmod_poly_struct;
    pub fn fq_zech_ctx_degree(ctx: *mut fq_zech_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_zech_ctx_order(f: *mut fmpz, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_ctx_order_ui(ctx: *mut fq_zech_ctx_struct) -> mp_limb_t;
    pub fn fq_zech_ctx_fprint(file: *mut FILE, ctx: *mut fq_zech_ctx_struct) -> c_int;
    pub fn fq_zech_ctx_print(ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_init(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_init2(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_clear(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_reduce(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_add(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_sub(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_sub_one(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_neg(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul_fmpz(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        x: *mut fmpz,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul_si(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        x: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul_ui(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        x: mp_limb_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_sqr(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_inv(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: *mut fmpz,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *mut fmpz,
    );
    pub fn fq_zech_pow(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        e: *mut fmpz,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_pow_ui(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        e: mp_limb_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_sqrt(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn fq_zech_pth_root(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_is_square(op1: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct) -> c_int;
    pub fn fq_zech_randtest(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_randtest_not_zero(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_rand(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_rand_not_zero(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_equal(
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn fq_zech_is_zero(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct) -> c_int;
    pub fn fq_zech_is_one(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct) -> c_int;
    pub fn fq_zech_set(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_fmpz(rop: *mut fq_zech_struct, x: *mut fmpz, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_set_si(
        rop: *mut fq_zech_struct,
        x: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_ui(rop: *mut fq_zech_struct, x: mp_limb_t, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_swap(
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_zero(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_one(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_gen(rop: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_set_fq_nmod(
        rop: *mut fq_zech_struct,
        op: *mut nmod_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_get_fq_nmod(
        rop: *mut nmod_poly_struct,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_get_nmod_poly(
        a: *mut nmod_poly_struct,
        b: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_nmod_poly(
        a: *mut fq_zech_struct,
        b: *mut nmod_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_fprint_pretty(
        file: *mut FILE,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn fq_zech_print_pretty(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_fprint(
        file: *mut FILE,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn fq_zech_print(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_get_str(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct) -> *mut c_char;
    pub fn fq_zech_get_str_pretty(
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_zech_trace(rop: *mut fmpz, op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_frobenius(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        e: mp_limb_signed_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_norm(rop: *mut fmpz, op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_bit_pack(
        f: *mut fmpz,
        op: *mut fq_zech_struct,
        bit_size: mp_limb_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_bit_unpack(
        rop: *mut fq_zech_struct,
        f: *mut fmpz,
        bit_size: mp_limb_t,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn __fq_zech_ctx_prime(p: *mut fmpz, ctx: *mut fq_zech_ctx_struct);
    pub fn fq_zech_gcdinv(
        rop: *mut fq_zech_struct,
        inv: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_is_invertible(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct) -> c_int;
    pub fn fq_zech_is_invertible_f(
        rop: *mut fq_zech_struct,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn fq_zech_div(
        rop: *mut fq_zech_struct,
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_multiplicative_order(
        ord: *mut fmpz,
        op: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    ) -> c_int;
    pub fn fq_zech_get_nmod_mat(
        col: *mut nmod_mat_struct,
        a: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_nmod_mat(
        a: *mut fq_zech_struct,
        col: *mut nmod_mat_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_is_primitive(op: *mut fq_zech_struct, ctx: *mut fq_zech_ctx_struct) -> c_int;
    pub fn fq_zech_embed_gens(
        gen_sub: *mut fq_zech_struct,
        gen_sup: *mut fq_zech_struct,
        minpoly: *mut nmod_poly_struct,
        sub_ctx: *mut fq_zech_ctx_struct,
        sup_ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_embed_gens_naive(
        gen_sub: *mut fq_zech_struct,
        gen_sup: *mut fq_zech_struct,
        minpoly: *mut nmod_poly_struct,
        sub_ctx: *mut fq_zech_ctx_struct,
        sup_ctx: *mut fq_zech_ctx_struct,
    );
    pub fn _fq_zech_embed_gens_allombert(
        gen_sub: *mut fq_zech_struct,
        gen_sup: *mut fq_zech_struct,
        minpoly: *mut nmod_poly_struct,
        sub_ctx: *mut fq_zech_ctx_struct,
        sup_ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_embed_matrices(
        embed: *mut nmod_mat_struct,
        project: *mut nmod_mat_struct,
        gen_sub: *mut fq_zech_struct,
        sub_ctx: *mut fq_zech_ctx_struct,
        gen_sup: *mut fq_zech_struct,
        sup_ctx: *mut fq_zech_ctx_struct,
        gen_minpoly: *mut nmod_poly_struct,
    );
    pub fn fq_zech_embed_trace_matrix(
        res: *mut nmod_mat_struct,
        basis: *mut nmod_mat_struct,
        sub_ctx: *mut fq_zech_ctx_struct,
        sup_ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_embed_composition_matrix_sub(
        matrix: *mut nmod_mat_struct,
        gen: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_zech_embed_composition_matrix(
        matrix: *mut nmod_mat_struct,
        gen: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_embed_mul_matrix(
        matrix: *mut nmod_mat_struct,
        gen: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_embed_mono_to_dual_matrix(
        res: *mut nmod_mat_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_embed_dual_to_mono_matrix(
        res: *mut nmod_mat_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
    pub fn fq_zech_modulus_pow_series_inv(
        res: *mut nmod_poly_struct,
        ctx: *mut fq_zech_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_zech_modulus_derivative_inv(
        m_prime: *mut fq_zech_struct,
        m_prime_inv: *mut fq_zech_struct,
        ctx: *mut fq_zech_ctx_struct,
    );
}
