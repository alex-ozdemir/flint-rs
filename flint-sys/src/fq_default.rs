#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fq_default.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mod::fmpz_mod_ctx_struct;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq::{fq_ctx_t, fq_t};
use crate::fq_nmod::{fq_nmod_ctx_t, fq_nmod_t};
use crate::fq_zech::{fq_zech_ctx_t, fq_zech_t};
use crate::nmod_poly::nmod_poly_struct;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Copy, Clone)]
pub union fq_default_struct {
    pub fq: fq_t,
    pub fq_nmod: fq_nmod_t,
    pub fq_zech: fq_zech_t,
}

pub type fq_default_t = [fq_default_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fq_default_ctx_struct {
    pub type_: c_int,
    pub ctx: fq_default_ctx_struct_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fq_default_ctx_struct_ctx {
    pub fq: fq_ctx_t,
    pub fq_nmod: fq_nmod_ctx_t,
    pub fq_zech: fq_zech_ctx_t,
}

pub type fq_default_ctx_t = [fq_default_ctx_struct; 1usize];

extern "C" {
    pub fn fq_default_ctx_init_type(
        ctx: *mut fq_default_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
        type_: c_int,
    );
    pub fn fq_default_ctx_init(
        ctx: *mut fq_default_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const c_char,
    );
    pub fn fq_default_ctx_init_modulus_type(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const fmpz_mod_poly_struct,
        mod_ctx: *const fmpz_mod_ctx_struct,
        var: *const c_char,
        type_: c_int,
    );
    pub fn fq_default_ctx_init_modulus(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const fmpz_mod_poly_struct,
        mod_ctx: *const fmpz_mod_ctx_struct,
        var: *const c_char,
    );
    pub fn fq_default_ctx_init_modulus_nmod_type(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const nmod_poly_struct,
        var: *const c_char,
        type_: c_int,
    );
    pub fn fq_default_ctx_init_modulus_nmod(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const nmod_poly_struct,
        var: *const c_char,
    );
    pub fn fq_default_ctx_clear(ctx: *mut fq_default_ctx_struct);
    pub fn fq_default_ctx_type(ctx: *const fq_default_ctx_struct) -> c_int;
    pub fn fq_default_ctx_degree(ctx: *const fq_default_ctx_struct) -> mp_limb_signed_t;
    pub fn fq_default_ctx_prime(prime: *mut fmpz, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_ctx_modulus(p: *mut fmpz_mod_poly_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_ctx_order(f: *mut fmpz, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_ctx_fprint(file: *mut FILE, ctx: *const fq_default_ctx_struct) -> c_int;
    pub fn fq_default_ctx_print(ctx: *const fq_default_ctx_struct);
    pub fn fq_default_init(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_init2(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_clear(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_is_invertible(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_add(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_sub(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_sub_one(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_neg(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mul(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mul_fmpz(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mul_si(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_mul_ui(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        x: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_sqr(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_inv(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_div(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_pow(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        e: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_pow_ui(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        e: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_sqrt(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_pth_root(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_is_square(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_randtest(
        rop: *mut fq_default_struct,
        state: *const flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_randtest_not_zero(
        rop: *mut fq_default_struct,
        state: *const flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_rand(
        rop: *mut fq_default_struct,
        state: *const flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_rand_not_zero(
        rop: *mut fq_default_struct,
        state: *const flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_equal(
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_is_zero(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_is_one(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_set(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_fmpz(
        rop: *mut fq_default_struct,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_ui(
        rop: *mut fq_default_struct,
        x: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_si(
        rop: *mut fq_default_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_zero(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_one(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_swap(
        op1: *mut fq_default_struct,
        op2: *mut fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_gen(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_get_nmod_poly(
        poly: *mut nmod_poly_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_nmod_poly(
        op: *mut fq_default_struct,
        poly: *const nmod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_get_fmpz_mod_poly(
        poly: *mut fmpz_mod_poly_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_fmpz_mod_poly(
        op: *mut fq_default_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_get_fmpz_poly(
        poly: *mut fmpz_poly_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_fmpz_poly(
        op: *mut fq_default_struct,
        poly: *const fmpz_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_get_coeff_fmpz(
        c: *mut fmpz,
        op: *const fq_default_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_fprint(
        file: *mut FILE,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_print(op: *const fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_fprint_pretty(
        file: *mut FILE,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> c_int;
    pub fn fq_default_print_pretty(op: *const fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_get_str(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_default_get_str_pretty(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut c_char;
    pub fn fq_default_trace(
        rop: *mut fmpz,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_frobenius(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        e: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_norm(
        rop: *mut fmpz,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
}
