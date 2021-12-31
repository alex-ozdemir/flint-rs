#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::{fmpz, fmpz_t};
use crate::nmod_vec::nmod_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_ctx {
    pub n: fmpz_t,
    pub add_fxn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut fmpz,
            arg2: *mut fmpz,
            arg3: *mut fmpz,
            arg4: *const fmpz_mod_ctx,
        ),
    >,
    pub sub_fxn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut fmpz,
            arg2: *mut fmpz,
            arg3: *mut fmpz,
            arg4: *const fmpz_mod_ctx,
        ),
    >,
    pub mul_fxn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut fmpz,
            arg2: *mut fmpz,
            arg3: *mut fmpz,
            arg4: *const fmpz_mod_ctx,
        ),
    >,
    pub mod_: nmod_t,
    pub n_limbs: [mp_limb_t; 3usize],
    pub ninv_limbs: [mp_limb_t; 3usize],
}
pub type fmpz_mod_ctx_struct = fmpz_mod_ctx;
pub type fmpz_mod_ctx_t = [fmpz_mod_ctx_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_ctx_init(ctx: *mut fmpz_mod_ctx_struct, n: *const fmpz);
}
extern "C" {
    pub fn fmpz_mod_ctx_init_ui(ctx: *mut fmpz_mod_ctx_struct, n: mp_limb_t);
}
extern "C" {
    pub fn fmpz_mod_ctx_clear(ctx: *mut fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_ctx_modulus(ctx: *const fmpz_mod_ctx_struct) -> *const fmpz;
}
extern "C" {
    pub fn fmpz_mod_ctx_get_modulus_mpz_read_only(
        m: *mut __mpz_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_ctx_set_modulus(ctx: *mut fmpz_mod_ctx_struct, n: *const fmpz);
}
extern "C" {
    pub fn fmpz_mod_ctx_set_modulus_ui(ctx: *mut fmpz_mod_ctx_struct, n: mp_limb_t);
}
extern "C" {
    pub fn fmpz_mod_is_canonical(
        a: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_assert_canonical(a: *const fmpz, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_is_one(
        a: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_equal_fmpz(
        a: *const fmpz,
        b: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_equal_si(
        a: *const fmpz,
        b: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_set_fmpz(a: *mut fmpz, b: *const fmpz, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_set_ui(a: *mut fmpz, b: mp_limb_t, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_set_si(a: *mut fmpz, b: mp_limb_signed_t, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn _fmpz_mod_add1(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_add2s(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_add2(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_addN(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_add(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_add_fmpz(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_add_ui(
        a: *mut fmpz,
        b: *const fmpz,
        c: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_add_si(
        a: *mut fmpz,
        b: *const fmpz,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_sub1(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_sub2s(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_sub2(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_subN(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_sub(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_sub_fmpz(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_sub_ui(
        a: *mut fmpz,
        b: *const fmpz,
        c: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_sub_si(
        a: *mut fmpz,
        b: *const fmpz,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_fmpz_sub(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_ui_sub(
        a: *mut fmpz,
        b: mp_limb_t,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_si_sub(
        a: *mut fmpz,
        b: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_neg(a: *mut fmpz, b: *const fmpz, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn _fmpz_mod_mul1(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mul2s(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mul2(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn _fmpz_mod_mulN(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mul(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_addmul(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        d: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mul_fmpz(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mul_ui(
        a: *mut fmpz,
        b: *const fmpz,
        c: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_mul_si(
        a: *mut fmpz,
        b: *const fmpz,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_is_invertible(
        a: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_inv(a: *mut fmpz, b: *const fmpz, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_divides(
        a: *const fmpz,
        b: *const fmpz,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_pow_ui(
        a: *mut fmpz,
        b: *const fmpz,
        pow: mp_limb_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_pow_fmpz(
        a: *mut fmpz,
        b: *const fmpz,
        pow: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_mod_rand(a: *mut fmpz, state: *const flint_rand_s, ctx: *const fmpz_mod_ctx_struct);
}
extern "C" {
    pub fn fmpz_mod_rand_not_zero(
        a: *mut fmpz,
        state: *const flint_rand_s,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_discrete_log_pohlig_hellman_table_entry_struct {
    pub gammapow: fmpz_t,
    pub cm: mp_limb_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_discrete_log_pohlig_hellman_entry_struct {
    pub exp: mp_limb_signed_t,
    pub prime: mp_limb_t,
    pub gamma: fmpz_t,
    pub gammainv: fmpz_t,
    pub startingbeta: fmpz_t,
    pub co: fmpz_t,
    pub startinge: fmpz_t,
    pub idem: fmpz_t,
    pub cbound: mp_limb_t,
    pub dbound: mp_limb_t,
    pub table: *mut fmpz_mod_discrete_log_pohlig_hellman_table_entry_struct,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_discrete_log_pohlig_hellman_struct {
    pub fpctx: fmpz_mod_ctx_t,
    pub pm1: fmpz_t,
    pub alpha: fmpz_t,
    pub alphainv: fmpz_t,
    pub num_factors: mp_limb_signed_t,
    pub entries: *mut fmpz_mod_discrete_log_pohlig_hellman_entry_struct,
}
pub type fmpz_mod_discrete_log_pohlig_hellman_t =
    [fmpz_mod_discrete_log_pohlig_hellman_struct; 1usize];
extern "C" {
    pub fn fmpz_mod_discrete_log_pohlig_hellman_init(
        L: *mut fmpz_mod_discrete_log_pohlig_hellman_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_discrete_log_pohlig_hellman_clear(
        L: *mut fmpz_mod_discrete_log_pohlig_hellman_struct,
    );
}
extern "C" {
    pub fn fmpz_mod_discrete_log_pohlig_hellman_precompute_prime(
        L: *const fmpz_mod_discrete_log_pohlig_hellman_struct,
        p: *const fmpz,
    ) -> f64;
}
extern "C" {
    pub fn fmpz_mod_discrete_log_pohlig_hellman_run(
        x: *mut fmpz,
        L: *const fmpz_mod_discrete_log_pohlig_hellman_struct,
        y: *const fmpz,
    );
}
extern "C" {
    pub fn fmpz_mod_discrete_log_pohlig_hellman_primitive_root(
        L: *const fmpz_mod_discrete_log_pohlig_hellman_struct,
    ) -> *const fmpz;
}
extern "C" {
    pub fn fmpz_next_smooth_prime(a: *mut fmpz, b: *const fmpz) -> ::std::os::raw::c_int;
}
