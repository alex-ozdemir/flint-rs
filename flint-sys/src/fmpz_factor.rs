#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use libc::{c_int, c_uchar};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_factor_struct {
    pub sign: c_int,
    pub p: *mut fmpz,
    pub exp: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub num: mp_limb_signed_t,
}

pub type fmpz_factor_t = [fmpz_factor_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ecm_s {
    pub t: mp_ptr,
    pub u: mp_ptr,
    pub v: mp_ptr,
    pub w: mp_ptr,
    pub x: mp_ptr,
    pub z: mp_ptr,
    pub a24: mp_ptr,
    pub ninv: mp_ptr,
    pub one: mp_ptr,
    pub GCD_table: *mut c_uchar,
    pub prime_table: *mut *mut c_uchar,
    pub n_size: mp_limb_t,
    pub normbits: mp_limb_t,
}

pub type ecm_t = [ecm_s; 1usize];

extern "C" {
    pub fn fmpz_factor_init(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_clear(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_print(factor: *const fmpz_factor_struct);
    pub fn _fmpz_factor_fit_length(factor: *mut fmpz_factor_struct, len: mp_limb_signed_t);
    pub fn _fmpz_factor_append_ui(factor: *mut fmpz_factor_struct, p: mp_limb_t, exp: mp_limb_t);
    pub fn _fmpz_factor_append(factor: *mut fmpz_factor_struct, p: *const fmpz, exp: mp_limb_t);
    pub fn _fmpz_factor_set_length(factor: *mut fmpz_factor_struct, newlen: mp_limb_signed_t);
    pub fn _fmpz_factor_concat(
        factor1: *mut fmpz_factor_struct,
        factor2: *const fmpz_factor_struct,
        exp: mp_limb_t,
    );
    pub fn _fmpz_factor_extend_factor_ui(factor: *mut fmpz_factor_struct, n: mp_limb_t);
    pub fn fmpz_factor_trial_range(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        start: mp_limb_t,
        num_primes: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_trial(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        num_primes: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_factor(factor: *mut fmpz_factor_struct, n: *const fmpz);
    pub fn fmpz_factor_no_trial(factor: *mut fmpz_factor_struct, n: *const fmpz);
    pub fn fmpz_factor_smooth(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        bits: mp_limb_signed_t,
        proved: c_int,
    ) -> c_int;
    pub fn fmpz_factor_si(factor: *mut fmpz_factor_struct, n: mp_limb_signed_t);
    pub fn fmpz_factor_pp1(
        factor: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_refine(res: *mut fmpz_factor_struct, f: *const fmpz_factor_struct);
    pub fn flint_mpn_sqr_and_add_a(
        y: mp_ptr,
        a: mp_ptr,
        n: mp_ptr,
        n_size: mp_limb_t,
        ninv: mp_ptr,
        normbits: mp_limb_t,
    );
    pub fn flint_mpn_factor_pollard_brent_single(
        factor: mp_ptr,
        n: mp_ptr,
        ninv: mp_ptr,
        a: mp_ptr,
        y: mp_ptr,
        n_size: mp_limb_t,
        normbits: mp_limb_t,
        max_iters: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_pollard_brent_single(
        p_factor: *mut fmpz,
        n_in: *const fmpz,
        yi: *const fmpz,
        ai: *const fmpz,
        max_iters: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_pollard_brent(
        factor: *mut fmpz,
        state: *const flint_rand_s,
        n: *const fmpz,
        max_tries: mp_limb_t,
        max_iters: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_expand_iterative(n: *mut fmpz, factor: *const fmpz_factor_struct);
    pub fn fmpz_factor_expand_multiexp(n: *mut fmpz, factor: *const fmpz_factor_struct);
    pub fn fmpz_factor_expand(n: *mut fmpz, factor: *const fmpz_factor_struct);
    pub fn fmpz_factor_euler_phi(res: *mut fmpz, fac: *const fmpz_factor_struct);
    pub fn fmpz_factor_moebius_mu(fac: *const fmpz_factor_struct) -> c_int;
    pub fn fmpz_factor_divisor_sigma(res: *mut fmpz, fac: *const fmpz_factor_struct, k: mp_limb_t);
    pub fn fmpz_factor_ecm_init(ecm_inf: *mut ecm_s, sz: mp_limb_t);
    pub fn fmpz_factor_ecm_clear(ecm_inf: *mut ecm_s);
    pub fn fmpz_factor_ecm_addmod(a: mp_ptr, b: mp_ptr, c: mp_ptr, n: mp_ptr, n_size: mp_limb_t);
    pub fn fmpz_factor_ecm_submod(x: mp_ptr, a: mp_ptr, b: mp_ptr, n: mp_ptr, n_size: mp_limb_t);
    pub fn fmpz_factor_ecm_double(
        x: mp_ptr,
        z: mp_ptr,
        x0: mp_ptr,
        z0: mp_ptr,
        n: mp_ptr,
        ecm_inf: *const ecm_s,
    );
    pub fn fmpz_factor_ecm_add(
        x: mp_ptr,
        z: mp_ptr,
        x1: mp_ptr,
        z1: mp_ptr,
        x2: mp_ptr,
        z2: mp_ptr,
        x0: mp_ptr,
        z0: mp_ptr,
        n: mp_ptr,
        ecm_inf: *const ecm_s,
    );
    pub fn fmpz_factor_ecm_mul_montgomery_ladder(
        x: mp_ptr,
        z: mp_ptr,
        x0: mp_ptr,
        z0: mp_ptr,
        k: mp_limb_t,
        n: mp_ptr,
        ecm_inf: *const ecm_s,
    );
    pub fn fmpz_factor_ecm_select_curve(
        f: mp_ptr,
        sig: mp_ptr,
        n: mp_ptr,
        ecm_inf: *const ecm_s,
    ) -> c_int;
    pub fn fmpz_factor_ecm_stage_I(
        f: mp_ptr,
        prime_array: *const mp_limb_t,
        num: mp_limb_t,
        B1: mp_limb_t,
        n: mp_ptr,
        ecm_inf: *const ecm_s,
    ) -> c_int;
    pub fn fmpz_factor_ecm_stage_II(
        f: mp_ptr,
        B1: mp_limb_t,
        B2: mp_limb_t,
        P: mp_limb_t,
        n: mp_ptr,
        ecm_inf: *const ecm_s,
    ) -> c_int;
    pub fn fmpz_factor_ecm(
        f: *mut fmpz,
        curves: mp_limb_t,
        B1: mp_limb_t,
        B2: mp_limb_t,
        state: *const flint_rand_s,
        n_in: *const fmpz,
    ) -> c_int;
    pub fn fmpz_factor_get_fmpz(
        z: *mut fmpz,
        factor: *const fmpz_factor_struct,
        i: mp_limb_signed_t,
    );
}
