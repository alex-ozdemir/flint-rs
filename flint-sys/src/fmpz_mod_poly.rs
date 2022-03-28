#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fmpz_mod::fmpz_mod_ctx_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_char, c_int, c_void, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fmpz_mod_poly_struct {
    pub coeffs: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}

pub type fmpz_mod_poly_t = [fmpz_mod_poly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_res_struct {
    pub res: fmpz_t,
    pub lc: fmpz_t,
    pub len0: mp_limb_signed_t,
    pub len1: mp_limb_signed_t,
    pub off: mp_limb_signed_t,
}

pub type fmpz_mod_poly_res_t = [fmpz_mod_poly_res_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_frobenius_powers_2exp_struct {
    pub pow: *mut fmpz_mod_poly_struct,
    pub len: mp_limb_signed_t,
}

pub type fmpz_mod_poly_frobenius_powers_2exp_t =
    [fmpz_mod_poly_frobenius_powers_2exp_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_frobenius_powers_struct {
    pub pow: *mut fmpz_mod_poly_struct,
    pub len: mp_limb_signed_t,
}

pub type fmpz_mod_poly_frobenius_powers_t = [fmpz_mod_poly_frobenius_powers_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_matrix_precompute_arg_t {
    pub A: *mut fmpz_mat_struct,
    pub poly1: *mut fmpz_mod_poly_struct,
    pub poly2: *mut fmpz_mod_poly_struct,
    pub poly2inv: *mut fmpz_mod_poly_struct,
    pub ctx: *const fmpz_mod_ctx_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_compose_mod_precomp_preinv_arg_t {
    pub A: *mut fmpz_mat_struct,
    pub res: *mut fmpz_mod_poly_struct,
    pub poly1: *mut fmpz_mod_poly_struct,
    pub poly3: *mut fmpz_mod_poly_struct,
    pub poly3inv: *mut fmpz_mod_poly_struct,
    pub ctx: *const fmpz_mod_ctx_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_radix_struct {
    pub V: *mut fmpz,
    pub W: *mut fmpz,
    pub Rpow: *mut *mut fmpz,
    pub Rinv: *mut *mut fmpz,
    pub degR: mp_limb_signed_t,
    pub k: mp_limb_signed_t,
    pub invL: fmpz,
}

pub type fmpz_mod_poly_radix_t = [fmpz_mod_poly_radix_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_berlekamp_massey_struct {
    pub npoints: mp_limb_signed_t,
    pub R0: fmpz_mod_poly_t,
    pub R1: fmpz_mod_poly_t,
    pub V0: fmpz_mod_poly_t,
    pub V1: fmpz_mod_poly_t,
    pub qt: fmpz_mod_poly_t,
    pub rt: fmpz_mod_poly_t,
    pub points: fmpz_mod_poly_t,
}

pub type fmpz_mod_berlekamp_massey_t = [fmpz_mod_berlekamp_massey_struct; 1usize];

extern "C" {
    pub fn fmpz_mod_poly_init(poly: *mut fmpz_mod_poly_struct, ctx: *const fmpz_mod_ctx_struct);
    pub fn fmpz_mod_poly_init2(
        poly: *mut fmpz_mod_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_clear(poly: *mut fmpz_mod_poly_struct, ctx: *const fmpz_mod_ctx_struct);
    pub fn fmpz_mod_poly_realloc(
        poly: *mut fmpz_mod_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_fit_length(
        poly: *mut fmpz_mod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_normalise(poly: *mut fmpz_mod_poly_struct);
    pub fn _fmpz_mod_poly_set_length(poly: *mut fmpz_mod_poly_struct, len: mp_limb_signed_t);
    pub fn fmpz_mod_poly_truncate(
        poly: *mut fmpz_mod_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_trunc(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_irreducible(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_not_zero(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_monic(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_monic_irreducible(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_monic_primitive(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_trinomial(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_trinomial_irreducible(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        max_attempts: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_randtest_pentomial(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_randtest_pentomial_irreducible(
        f: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        max_attempts: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_randtest_sparse_irreducible(
        poly: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_degree(
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_length(
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_lead(
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> *mut fmpz;
    pub fn fmpz_mod_poly_is_monic(
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_one(
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_gen(
        op: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_set(
        poly1: *mut fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_swap(
        poly1: *mut fmpz_mod_poly_struct,
        poly2: *mut fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_reverse(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_reverse(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_zero(poly: *mut fmpz_mod_poly_struct, ctx: *const fmpz_mod_ctx_struct);
    pub fn fmpz_mod_poly_one(poly: *mut fmpz_mod_poly_struct, ctx: *const fmpz_mod_ctx_struct);
    pub fn fmpz_mod_poly_gen(poly: *mut fmpz_mod_poly_struct, ctx: *const fmpz_mod_ctx_struct);
    pub fn fmpz_mod_poly_zero_coeffs(
        poly: *mut fmpz_mod_poly_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_ui(
        f: *mut fmpz_mod_poly_struct,
        x: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_fmpz(
        poly: *mut fmpz_mod_poly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_fmpz_poly(
        f: *mut fmpz_mod_poly_struct,
        g: *const fmpz_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_get_fmpz_poly(
        f: *mut fmpz_poly_struct,
        g: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_equal(
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_equal_trunc(
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_zero(
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_set_coeff_fmpz(
        poly: *mut fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        x: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_coeff_ui(
        poly: *mut fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_coeff_si(
        poly: *mut fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_get_coeff_fmpz(
        x: *mut fmpz,
        poly: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_set_coeff_mpz(
        poly: *mut fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        x: *const __mpz_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_get_coeff_mpz(
        x: *mut __mpz_struct,
        poly: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_shift_left(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_shift_left(
        f: *mut fmpz_mod_poly_struct,
        g: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_shift_right(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_shift_right(
        f: *mut fmpz_mod_poly_struct,
        g: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_add(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_add(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_sub(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_add_series(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_sub(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_neg(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_sub_series(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_neg(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_scalar_mul_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_scalar_mul_fmpz(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        x: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_scalar_mul_ui(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: mp_limb_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_scalar_mul_ui(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        x: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_scalar_div_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_scalar_div_fmpz(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        x: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_mul(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_mul(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_mullow(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_mullow(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_sqr(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_sqr(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_mulmod(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_mulmod(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_mulmod_preinv(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        finv: *const fmpz,
        lenfinv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_mulmod_preinv(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_pow(
        rop: *mut fmpz_mod_poly_struct,
        op: *const fmpz_mod_poly_struct,
        e: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_pow_trunc(
        res: *mut fmpz,
        poly: *const fmpz,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_pow_trunc(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_pow_trunc_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_pow_trunc_binexp(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powmod_ui_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        e: mp_limb_t,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_powmod_ui_binexp(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        e: mp_limb_t,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powmod_ui_binexp_preinv(
        res: *mut fmpz,
        poly: *const fmpz,
        e: mp_limb_t,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        finv: *const fmpz,
        lenfinv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_powmod_ui_binexp_preinv(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        e: mp_limb_t,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powmod_fmpz_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        e: *const fmpz,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_powmod_fmpz_binexp(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        e: *const fmpz,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powmod_fmpz_binexp_preinv(
        res: *mut fmpz,
        poly: *const fmpz,
        e: *const fmpz,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        finv: *const fmpz,
        lenfinv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_powmod_fmpz_binexp_preinv(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        e: *const fmpz,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powmod_x_fmpz_preinv(
        res: *mut fmpz,
        e: *const fmpz,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        finv: *const fmpz,
        lenfinv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_powmod_x_fmpz_preinv(
        res: *mut fmpz_mod_poly_struct,
        e: *const fmpz,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_powmod_linear_fmpz_preinv(
        res: *mut fmpz_mod_poly_struct,
        a: *const fmpz,
        e: *const fmpz,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powers_mod_preinv_naive(
        res: *mut *mut fmpz,
        f: *const fmpz,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *const fmpz,
        glen: mp_limb_signed_t,
        ginv: *const fmpz,
        ginvlen: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_powers_mod_naive(
        res: *mut fmpz_mod_poly_struct,
        f: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        g: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_powers_mod_preinv_threaded_pool(
        res: *mut *mut fmpz,
        f: *const fmpz,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *const fmpz,
        glen: mp_limb_signed_t,
        ginv: *const fmpz,
        ginvlen: mp_limb_signed_t,
        p: *const fmpz,
        threads: *const thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_powers_mod_bsgs(
        res: *mut fmpz_mod_poly_struct,
        f: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        g: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_frobenius_powers_2exp_precomp(
        pow: *mut fmpz_mod_poly_frobenius_powers_2exp_struct,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        m: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_frobenius_powers_2exp_clear(
        pow: *mut fmpz_mod_poly_frobenius_powers_2exp_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_frobenius_power(
        res: *mut fmpz_mod_poly_struct,
        pow: *const fmpz_mod_poly_frobenius_powers_2exp_struct,
        f: *const fmpz_mod_poly_struct,
        m: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_frobenius_powers_precomp(
        pow: *mut fmpz_mod_poly_frobenius_powers_struct,
        f: *const fmpz_mod_poly_struct,
        finv: *const fmpz_mod_poly_struct,
        m: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_frobenius_powers_clear(
        pow: *mut fmpz_mod_poly_frobenius_powers_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_divrem_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_divrem_basecase(
        Q: *mut fmpz_mod_poly_struct,
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_div_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_div_basecase(
        Q: *mut fmpz_mod_poly_struct,
        A: *mut fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_div_newton_n_preinv(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        Binv: *const fmpz,
        lenBinv: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_div_newton_n_preinv(
        Q: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        Binv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_divrem_newton_n_preinv(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        Binv: *const fmpz,
        lenBinv: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_divrem_newton_n_preinv(
        Q: *mut fmpz_mod_poly_struct,
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        Binv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_remove(
        f: *mut fmpz_mod_poly_struct,
        p: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> mp_limb_t;
    pub fn _fmpz_mod_poly_rem_basecase(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_rem_basecase(
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_divrem_divconquer_recursive(
        Q: *mut fmpz,
        BQ: *mut fmpz,
        W: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn _fmpz_mod_poly_divrem_divconquer(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_divrem_divconquer(
        Q: *mut fmpz_mod_poly_struct,
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_divrem(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_divrem(
        Q: *mut fmpz_mod_poly_struct,
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_divrem_f(
        f: *mut fmpz,
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_divrem_f(
        f: *mut fmpz,
        Q: *mut fmpz_mod_poly_struct,
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_rem(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_rem(
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_rem_f(
        f: *mut fmpz,
        R: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_inv_series_newton(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        n: mp_limb_signed_t,
        cinv: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_inv_series_newton(
        Qinv: *mut fmpz_mod_poly_struct,
        Q: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_inv_series_newton_f(
        f: *mut fmpz,
        Qinv: *mut fmpz_mod_poly_struct,
        Q: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_inv_series(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        n: mp_limb_signed_t,
        cinv: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_inv_series(
        Qinv: *mut fmpz_mod_poly_struct,
        Q: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_inv_series_f(
        f: *mut fmpz,
        Qinv: *mut fmpz_mod_poly_struct,
        Q: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_div_series(
        Q: *mut fmpz,
        A: *const fmpz,
        Alen: mp_limb_signed_t,
        B: *const fmpz,
        Blen: mp_limb_signed_t,
        p: *const fmpz,
        n: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_div_series(
        Q: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_make_monic(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_make_monic_f(
        f: *mut fmpz,
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcd_euclidean(
        G: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcd_euclidean(
        G: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcd_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcd_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcd_f(
        f: *mut fmpz,
        G: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;

    // (mostly) correct up to this point

    pub fn fmpz_mod_poly_gcd_f(
        f: *mut fmpz,
        G: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_hgcd_recursive(
        M: *mut *mut fmpz,
        lenM: *mut mp_limb_signed_t,
        A: *mut fmpz,
        lenA: *mut mp_limb_signed_t,
        B: *mut fmpz,
        lenB: *mut mp_limb_signed_t,
        a: *const fmpz,
        lena: mp_limb_signed_t,
        b: *const fmpz,
        lenb: mp_limb_signed_t,
        P: *const fmpz,
        mod_: *const fmpz,
        flag: c_int,
        res: *const fmpz_mod_poly_res_struct,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_mod_poly_hgcd(
        M: *mut *mut fmpz,
        lenM: *mut mp_limb_signed_t,
        A: *mut fmpz,
        lenA: *mut mp_limb_signed_t,
        B: *mut fmpz,
        lenB: *mut mp_limb_signed_t,
        a: *const fmpz,
        lena: mp_limb_signed_t,
        b: *const fmpz,
        lenb: mp_limb_signed_t,
        mod_: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_mod_poly_gcd_hgcd(
        G: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        mod_: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcd_hgcd(
        G: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcd(
        G: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcd(
        G: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_xgcd_euclidean(
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_mod_poly_xgcd_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_xgcd_euclidean(
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        T: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_xgcd_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        T: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_xgcd_hgcd(
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        mod_: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_xgcd_hgcd(
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        T: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_xgcd(
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_mod_poly_xgcd_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_xgcd(
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        T: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_xgcd_f(
        f: *mut fmpz,
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        T: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcdinv_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invA: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcdinv_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcdinv_euclidean(
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        invA: *const fmpz,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcdinv_euclidean(
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcdinv(
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcdinv(
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_gcdinv_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_gcdinv_f(
        f: *mut fmpz,
        G: *mut fmpz_mod_poly_struct,
        S: *mut fmpz_mod_poly_struct,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_invmod(
        A: *mut fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        P: *const fmpz,
        lenP: mp_limb_signed_t,
        p: *const fmpz,
    ) -> c_int;
    pub fn _fmpz_mod_poly_invmod_f(
        f: *mut fmpz,
        A: *mut fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        P: *const fmpz,
        lenP: mp_limb_signed_t,
        p: *const fmpz,
    ) -> c_int;
    pub fn fmpz_mod_poly_invmod(
        A: *mut fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        P: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_invmod_f(
        f: *mut fmpz,
        A: *mut fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        P: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mod_poly_minpoly_bm(
        poly: *mut fmpz,
        seq: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_minpoly_bm(
        poly: *mut fmpz_mod_poly_struct,
        seq: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_minpoly_hgcd(
        poly: *mut fmpz,
        seq: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_minpoly_hgcd(
        poly: *mut fmpz_mod_poly_struct,
        seq: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_minpoly(
        poly: *mut fmpz,
        seq: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_poly_minpoly(
        poly: *mut fmpz_mod_poly_struct,
        seq: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_resultant_euclidean(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_resultant_euclidean(
        r: *mut fmpz,
        f: *const fmpz_mod_poly_struct,
        g: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_resultant_hgcd(
        res: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_resultant_hgcd(
        res: *mut fmpz,
        A: *const fmpz_mod_poly_struct,
        B: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_resultant(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_resultant(
        res: *mut fmpz,
        f: *const fmpz_mod_poly_struct,
        g: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_discriminant(
        d: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_discriminant(
        d: *mut fmpz,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_derivative(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_derivative(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_evaluate_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_evaluate_fmpz(
        res: *mut fmpz,
        poly: *const fmpz_mod_poly_struct,
        a: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_tree_alloc(len: mp_limb_signed_t) -> *mut *mut fmpz_poly_struct;
    pub fn _fmpz_mod_poly_tree_free(tree: *mut *mut fmpz_poly_struct, len: mp_limb_signed_t);
    pub fn _fmpz_mod_poly_tree_build(
        tree: *mut *mut fmpz_poly_struct,
        roots: *const fmpz,
        len: mp_limb_signed_t,
        mod_: *mut fmpz,
    );
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec_iter(
        ys: *mut fmpz,
        coeffs: *const fmpz,
        len: mp_limb_signed_t,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_evaluate_fmpz_vec_iter(
        ys: *mut fmpz,
        poly: *const fmpz_mod_poly_struct,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp(
        vs: *mut fmpz,
        poly: *const fmpz,
        plen: mp_limb_signed_t,
        tree: *const *const fmpz_poly_struct,
        len: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec_fast(
        ys: *mut fmpz,
        poly: *const fmpz,
        plen: mp_limb_signed_t,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_evaluate_fmpz_vec_fast(
        ys: *mut fmpz,
        poly: *const fmpz_mod_poly_struct,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec(
        ys: *mut fmpz,
        coeffs: *const fmpz,
        len: mp_limb_signed_t,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_evaluate_fmpz_vec(
        ys: *mut fmpz,
        poly: *const fmpz_mod_poly_struct,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_horner(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_horner(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_divconquer(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_divconquer(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod(
        res: *mut fmpz,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        g: *const fmpz,
        h: *const fmpz,
        lenh: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_mod(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        poly3: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod_brent_kung(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        poly3: *const fmpz,
        len3: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_mod_brent_kung(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        poly3: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_reduce_matrix_mod_poly(
        A: *mut fmpz_mat_struct,
        B: *const fmpz_mat_struct,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_precompute_matrix(
        A: *mut fmpz_mat_struct,
        poly1: *const fmpz,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        poly2inv: *const fmpz,
        len2inv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn _fmpz_mod_poly_precompute_matrix_worker(arg_ptr: *mut c_void);
    pub fn fmpz_mod_poly_precompute_matrix(
        A: *mut fmpz_mat_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        poly2inv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        A: *const fmpz_mat_struct,
        poly3: *const fmpz,
        len3: mp_limb_signed_t,
        poly3inv: *const fmpz,
        len3inv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv_worker(arg_ptr: *mut c_void);
    pub fn fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        A: *const fmpz_mat_struct,
        poly3: *const fmpz_mod_poly_struct,
        poly3inv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_preinv(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        poly3: *const fmpz,
        len3: mp_limb_signed_t,
        poly3inv: *const fmpz,
        len3inv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_mod_brent_kung_preinv(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        poly3: *const fmpz_mod_poly_struct,
        poly3inv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod_horner(
        res: *mut fmpz,
        f: *const fmpz,
        lenf: mp_limb_signed_t,
        g: *const fmpz,
        h: *const fmpz,
        lenh: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_mod_horner(
        res: *mut fmpz_mod_poly_struct,
        poly1: *const fmpz_mod_poly_struct,
        poly2: *const fmpz_mod_poly_struct,
        poly3: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_vec_preinv(
        res: *mut fmpz_mod_poly_struct,
        polys: *const fmpz_mod_poly_struct,
        len1: mp_limb_signed_t,
        l: mp_limb_signed_t,
        g: *const fmpz,
        glen: mp_limb_signed_t,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        polyinv: *const fmpz,
        leninv: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_compose_mod_brent_kung_vec_preinv(
        res: *mut fmpz_mod_poly_struct,
        polys: *const fmpz_mod_poly_struct,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *const fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        polyinv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded_pool(
        res: *mut fmpz_mod_poly_struct,
        polys: *const fmpz_mod_poly_struct,
        lenpolys: mp_limb_signed_t,
        l: mp_limb_signed_t,
        g: *const fmpz,
        glen: mp_limb_signed_t,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        polyinv: *const fmpz,
        leninv: mp_limb_signed_t,
        p: *const fmpz,
        threads: *const thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded_pool(
        res: *mut fmpz_mod_poly_struct,
        polys: *const fmpz_mod_poly_struct,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *const fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        polyinv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
        threads: *const thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded(
        res: *mut fmpz_mod_poly_struct,
        polys: *const fmpz_mod_poly_struct,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *const fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        polyinv: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_radix_init(
        Rpow: *mut *mut fmpz,
        Rinv: *const *const fmpz,
        R: *const fmpz,
        lenR: mp_limb_signed_t,
        k: mp_limb_signed_t,
        invL: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_radix_init(
        D: *mut fmpz_mod_poly_radix_struct,
        R: *mut fmpz_mod_poly_struct,
        degF: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_radix_clear(D: *mut fmpz_mod_poly_radix_struct);
    pub fn _fmpz_mod_poly_radix(
        B: *mut *mut fmpz,
        F: *const fmpz,
        Rpow: *mut *mut fmpz,
        Rinv: *mut *mut fmpz,
        degR: mp_limb_signed_t,
        k: mp_limb_signed_t,
        i: mp_limb_signed_t,
        W: *const fmpz,
        p: *const fmpz,
    );
    pub fn fmpz_mod_poly_radix(
        B: *mut *mut fmpz_mod_poly_struct,
        F: *const fmpz_mod_poly_struct,
        D: *const fmpz_mod_poly_radix_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_fprint(
        file: *mut FILE,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    ) -> c_int;
    pub fn fmpz_mod_poly_fprint(
        file: *mut FILE,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_fread(
        file: *mut FILE,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpz_mod_poly_struct,
        x: *const c_char,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mod_poly_print(poly: *const fmpz, len: mp_limb_signed_t, p: *const fmpz) -> c_int;
    pub fn fmpz_mod_poly_print(
        poly: *mut fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_print_pretty(
        poly: *const fmpz_mod_poly_struct,
        x: *const c_char,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mod_poly_product_roots_fmpz_vec(
        poly: *mut fmpz,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_product_roots_fmpz_vec(
        poly: *mut fmpz_poly_struct,
        xs: *const fmpz,
        n: mp_limb_signed_t,
        mod_: *const fmpz,
    );
    pub fn fmpz_mod_poly_find_distinct_nonzero_roots(
        roots: *const fmpz,
        P: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mod_poly_split_rabin(
        a: *mut fmpz_mod_poly_struct,
        b: *const fmpz_mod_poly_struct,
        f: *const fmpz_mod_poly_struct,
        halfp: *const fmpz,
        t: *const fmpz_mod_poly_struct,
        t2: *const fmpz_mod_poly_struct,
        randstate: *const flint_rand_s,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_init(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_start_over(
        B: *const fmpz_mod_berlekamp_massey_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_clear(
        B: *const fmpz_mod_berlekamp_massey_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_print(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_add_points(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        a: *const fmpz,
        count: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_add_zeros(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        count: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_add_point(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        a: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_add_point_ui(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        a: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_berlekamp_massey_reduce(
        B: *mut fmpz_mod_berlekamp_massey_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_berlekamp_massey_points(
        B: *mut fmpz_mod_berlekamp_massey_struct,
    ) -> *const fmpz;
    pub fn fmpz_mod_berlekamp_massey_point_count(
        B: *mut fmpz_mod_berlekamp_massey_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mod_berlekamp_massey_V_poly(
        B: *mut fmpz_mod_berlekamp_massey_struct,
    ) -> *const fmpz_mod_poly_struct;
    pub fn fmpz_mod_berlekamp_massey_R_poly(
        B: *mut fmpz_mod_berlekamp_massey_struct,
    ) -> *const fmpz_mod_poly_struct;
    pub fn fmpz_mod_poly_add_si(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_sub_si(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_si_sub(
        res: *mut fmpz_mod_poly_struct,
        c: mp_limb_signed_t,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_add_fmpz(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_sub_fmpz(
        res: *mut fmpz_mod_poly_struct,
        poly: *const fmpz_mod_poly_struct,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_fmpz_sub(
        res: *mut fmpz_mod_poly_struct,
        c: *const fmpz,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
