#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [FLINT documentation](http://flintlib.org/doc/ulong_extras.html).

use crate::deps::*;
use crate::flint::*;
use libc::{c_char, c_int, c_uchar, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pair_s {
    pub x: mp_limb_t,
    pub y: mp_limb_t,
}

pub type n_pair_t = pair_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_factor_t {
    pub num: c_int,
    pub exp: [c_int; 15usize],
    pub p: [mp_limb_t; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_primes_struct {
    pub small_i: mp_limb_signed_t,
    pub small_num: mp_limb_signed_t,
    pub small_primes: *mut c_uint,
    pub sieve_a: mp_limb_t,
    pub sieve_b: mp_limb_t,
    pub sieve_i: mp_limb_signed_t,
    pub sieve_num: mp_limb_signed_t,
    pub sieve: *mut c_char,
}

pub type n_primes_t = [n_primes_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_ecm_s {
    pub x: mp_limb_t,
    pub z: mp_limb_t,
    pub a24: mp_limb_t,
    pub ninv: mp_limb_t,
    pub normbits: mp_limb_t,
    pub one: mp_limb_t,
    pub GCD_table: *mut c_uchar,
    pub prime_table: *mut *mut c_uchar,
}

pub type n_ecm_t = [n_ecm_s; 1usize];

extern "C" {
    pub fn n_primes_init(iter: *mut n_primes_struct);
    pub fn n_primes_clear(iter: *mut n_primes_struct);
    pub fn n_primes_extend_small(iter: *mut n_primes_struct, bound: mp_limb_t);
    pub fn n_primes_sieve_range(iter: *mut n_primes_struct, a: mp_limb_t, b: mp_limb_t);
    pub fn n_primes_jump_after(iter: *mut n_primes_struct, n: mp_limb_t);
    pub fn n_primes_next(iter: *mut n_primes_struct) -> mp_limb_t;
    pub static mut flint_primes_small: [c_uint; 0usize];
    pub static mut _flint_primes: [*mut mp_limb_t; 64usize];
    pub static mut _flint_prime_inverses: [*mut f64; 64usize];
    pub static mut _flint_primes_used: c_int;
    pub fn n_compute_primes(num_primes: mp_limb_t);
    pub fn n_cleanup_primes();
    pub fn n_primes_arr_readonly(n: mp_limb_t) -> *const mp_limb_t;
    pub fn n_prime_inverses_arr_readonly(n: mp_limb_t) -> *const f64;
    pub fn n_mul_checked(a: *mut mp_limb_t, b: mp_limb_t, c: mp_limb_t) -> c_int;
    pub fn n_add_checked(a: *mut mp_limb_t, b: mp_limb_t, c: mp_limb_t) -> c_int;
    pub fn n_randlimb(state: *mut flint_rand_s) -> mp_limb_t;
    pub fn n_randint(state: *mut flint_rand_s, limit: mp_limb_t) -> mp_limb_t;
    pub fn n_urandint(state: *mut flint_rand_s, limit: mp_limb_t) -> mp_limb_t;
    pub fn n_randbits(state: *mut flint_rand_s, bits: c_uint) -> mp_limb_t;
    pub fn n_randtest_bits(state: *mut flint_rand_s, bits: c_int) -> mp_limb_t;
    pub fn n_randtest(state: *mut flint_rand_s) -> mp_limb_t;
    pub fn n_randtest_not_zero(state: *mut flint_rand_s) -> mp_limb_t;
    pub fn n_randprime(state: *mut flint_rand_s, bits: mp_limb_t, proved: c_int) -> mp_limb_t;
    pub fn n_randtest_prime(state: *mut flint_rand_s, proved: c_int) -> mp_limb_t;
    pub fn n_pow(n: mp_limb_t, exp: mp_limb_t) -> mp_limb_t;
    pub fn n_flog(n: mp_limb_t, b: mp_limb_t) -> mp_limb_t;
    pub fn n_clog(n: mp_limb_t, b: mp_limb_t) -> mp_limb_t;
    pub fn n_precompute_inverse(n: mp_limb_t) -> f64;
    pub fn n_preinvert_limb(n: mp_limb_t) -> mp_limb_t;
    pub fn n_mod_precomp(a: mp_limb_t, n: mp_limb_t, ninv: f64) -> mp_limb_t;
    pub fn n_mod2_precomp(a: mp_limb_t, n: mp_limb_t, ninv: f64) -> mp_limb_t;
    pub fn n_divrem2_precomp(q: *mut mp_limb_t, a: mp_limb_t, n: mp_limb_t, npre: f64)
        -> mp_limb_t;
    pub fn n_divrem2_preinv(
        q: *mut mp_limb_t,
        a: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_div2_preinv(a: mp_limb_t, n: mp_limb_t, ninv: mp_limb_t) -> mp_limb_t;
    pub fn n_mod2_preinv(a: mp_limb_t, n: mp_limb_t, ninv: mp_limb_t) -> mp_limb_t;
    pub fn n_ll_mod_preinv(
        a_hi: mp_limb_t,
        a_lo: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_lll_mod_preinv(
        a_hi: mp_limb_t,
        a_mi: mp_limb_t,
        a_lo: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_mulmod_precomp(a: mp_limb_t, b: mp_limb_t, n: mp_limb_t, ninv: f64) -> mp_limb_t;
    pub fn n_mulmod2_preinv(a: mp_limb_t, b: mp_limb_t, n: mp_limb_t, ninv: mp_limb_t)
        -> mp_limb_t;
    pub fn n_mulmod2(a: mp_limb_t, b: mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_mulmod_preinv(
        a: mp_limb_t,
        b: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
        norm: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_powmod_ui_precomp(a: mp_limb_t, exp: mp_limb_t, n: mp_limb_t, npre: f64) -> mp_limb_t;
    pub fn n_powmod_precomp(
        a: mp_limb_t,
        exp: mp_limb_signed_t,
        n: mp_limb_t,
        npre: f64,
    ) -> mp_limb_t;
    pub fn n_powmod(a: mp_limb_t, exp: mp_limb_signed_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_powmod2_preinv(
        a: mp_limb_t,
        exp: mp_limb_signed_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_powmod2_ui_preinv(
        a: mp_limb_t,
        exp: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_powmod_ui_preinv(
        a: mp_limb_t,
        exp: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
        norm: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_powmod2(a: mp_limb_t, exp: mp_limb_signed_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_addmod(x: mp_limb_t, y: mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_submod(x: mp_limb_t, y: mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_negmod(x: mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_sqrtmod(a: mp_limb_t, p: mp_limb_t) -> mp_limb_t;
    pub fn n_sqrtmod_2pow(
        sqrt: *mut *mut mp_limb_t,
        a: mp_limb_t,
        exp: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn n_sqrtmod_primepow(
        sqrt: *mut *mut mp_limb_t,
        a: mp_limb_t,
        p: mp_limb_t,
        exp: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn n_sqrtmodn(
        sqrt: *mut *mut mp_limb_t,
        a: mp_limb_t,
        fac: *mut n_factor_t,
    ) -> mp_limb_signed_t;
    pub fn n_gcd(x: mp_limb_t, y: mp_limb_t) -> mp_limb_t;
    pub fn n_xgcd(a: *mut mp_limb_t, b: *mut mp_limb_t, x: mp_limb_t, y: mp_limb_t) -> mp_limb_t;
    pub fn n_gcdinv(a: *mut mp_limb_t, x: mp_limb_t, y: mp_limb_t) -> mp_limb_t;
    pub fn n_invmod(x: mp_limb_t, y: mp_limb_t) -> mp_limb_t;
    pub fn n_CRT(r1: mp_limb_t, m1: mp_limb_t, r2: mp_limb_t, m2: mp_limb_t) -> mp_limb_t;
    pub fn n_revbin(in_: mp_limb_t, bits: mp_limb_t) -> mp_limb_t;
    pub fn n_jacobi(x: mp_limb_signed_t, y: mp_limb_t) -> c_int;
    pub fn n_jacobi_unsigned(x: mp_limb_t, y: mp_limb_t) -> c_int;
    pub fn n_sqrt(a: mp_limb_t) -> mp_limb_t;
    pub fn n_sqrtrem(r: *mut mp_limb_t, a: mp_limb_t) -> mp_limb_t;
    pub fn n_is_square(x: mp_limb_t) -> c_int;
    pub fn n_cbrt_estimate(a: f64) -> f64;
    pub fn n_cbrt(a: mp_limb_t) -> mp_limb_t;
    pub fn n_cbrt_binary_search(x: mp_limb_t) -> mp_limb_t;
    pub fn n_cbrt_newton_iteration(n: mp_limb_t) -> mp_limb_t;
    pub fn n_cbrt_chebyshev_approx(n: mp_limb_t) -> mp_limb_t;
    pub fn n_cbrtrem(remainder: *mut mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_is_perfect_power235(n: mp_limb_t) -> c_int;
    pub fn n_is_perfect_power(root: *mut mp_limb_t, n: mp_limb_t) -> c_int;
    pub fn n_is_oddprime_small(n: mp_limb_t) -> c_int;
    pub fn n_is_oddprime_binary(n: mp_limb_t) -> c_int;
    pub fn n_is_probabprime_fermat(n: mp_limb_t, i: mp_limb_t) -> c_int;
    pub fn n_is_probabprime_fibonacci(n: mp_limb_t) -> c_int;
    pub fn n_is_probabprime_lucas(n: mp_limb_t) -> c_int;
    pub fn n_is_probabprime_BPSW(n: mp_limb_t) -> c_int;
    pub fn n_is_strong_probabprime_precomp(
        n: mp_limb_t,
        npre: f64,
        a: mp_limb_t,
        d: mp_limb_t,
    ) -> c_int;
    pub fn n_is_strong_probabprime2_preinv(
        n: mp_limb_t,
        ninv: mp_limb_t,
        a: mp_limb_t,
        d: mp_limb_t,
    ) -> c_int;
    pub fn n_is_probabprime(n: mp_limb_t) -> c_int;
    pub fn n_is_prime_pseudosquare(n: mp_limb_t) -> c_int;
    pub fn n_is_prime_pocklington(n: mp_limb_t, iterations: mp_limb_t) -> c_int;
    pub fn n_is_prime(n: mp_limb_t) -> c_int;
    pub fn n_nth_prime(n: mp_limb_t) -> mp_limb_t;
    pub fn n_nth_prime_bounds(lo: *mut mp_limb_t, hi: *mut mp_limb_t, n: mp_limb_t);
    pub fn n_prime_pi(n: mp_limb_t) -> mp_limb_t;
    pub fn n_prime_pi_bounds(lo: *mut mp_limb_t, hi: *mut mp_limb_t, n: mp_limb_t);
    pub fn n_remove(n: *mut mp_limb_t, p: mp_limb_t) -> c_int;
    pub fn n_remove2_precomp(n: *mut mp_limb_t, p: mp_limb_t, ppre: f64) -> c_int;
    pub fn n_factor_init(factors: *mut n_factor_t);
    pub fn n_factor_insert(factors: *mut n_factor_t, p: mp_limb_t, exp: mp_limb_t);
    pub fn n_factor_trial_range(
        factors: *mut n_factor_t,
        n: mp_limb_t,
        start: mp_limb_t,
        num_primes: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_factor_trial_partial(
        factors: *mut n_factor_t,
        n: mp_limb_t,
        prod: *mut mp_limb_t,
        num_primes: mp_limb_t,
        limit: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_factor_trial(
        factors: *mut n_factor_t,
        n: mp_limb_t,
        num_primes: mp_limb_t,
    ) -> mp_limb_t;
    pub fn n_factor_partial(
        factors: *mut n_factor_t,
        n: mp_limb_t,
        limit: mp_limb_t,
        proved: c_int,
    ) -> mp_limb_t;
    pub fn n_factor_power235(exp: *mut mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_factor_one_line(n: mp_limb_t, iters: mp_limb_t) -> mp_limb_t;
    pub fn n_factor_lehman(n: mp_limb_t) -> mp_limb_t;
    pub fn n_factor_SQUFOF(n: mp_limb_t, iters: mp_limb_t) -> mp_limb_t;
    pub fn n_factor(factors: *mut n_factor_t, n: mp_limb_t, proved: c_int);
    pub fn n_factor_pp1(n: mp_limb_t, B1: mp_limb_t, c: mp_limb_t) -> mp_limb_t;
    pub fn n_factor_pp1_wrapper(n: mp_limb_t) -> mp_limb_t;
    pub fn n_factor_pp1_table_insert(
        bits: mp_limb_signed_t,
        B1: mp_limb_signed_t,
        count: mp_limb_signed_t,
    );
    pub fn n_factor_pollard_brent_single(
        factor: *mut mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
        ai: mp_limb_t,
        xi: mp_limb_t,
        normbits: mp_limb_t,
        max_iters: mp_limb_t,
    ) -> c_int;
    pub fn n_factor_pollard_brent(
        factor: *mut mp_limb_t,
        state: *mut flint_rand_s,
        n_in: mp_limb_t,
        max_tries: mp_limb_t,
        max_iters: mp_limb_t,
    ) -> c_int;
    pub fn n_is_squarefree(n: mp_limb_t) -> c_int;
    pub fn n_moebius_mu(n: mp_limb_t) -> c_int;
    pub fn n_moebius_mu_vec(mu: *mut c_int, len: mp_limb_t);
    pub fn n_euler_phi(n: mp_limb_t) -> mp_limb_t;
    pub fn n_sizeinbase(n: mp_limb_t, base: c_int) -> c_int;
    pub fn n_nextprime(n: mp_limb_t, proved: c_int) -> mp_limb_t;
    pub fn n_factorial_mod2_preinv(n: mp_limb_t, p: mp_limb_t, pinv: mp_limb_t) -> mp_limb_t;
    pub fn n_factorial_fast_mod2_preinv(n: mp_limb_t, p: mp_limb_t, pinv: mp_limb_t) -> mp_limb_t;
    pub fn n_primitive_root_prime_prefactor(p: mp_limb_t, factors: *mut n_factor_t) -> mp_limb_t;
    pub fn n_primitive_root_prime(p: mp_limb_t) -> mp_limb_t;
    pub fn n_discrete_log_bsgs(b: mp_limb_t, a: mp_limb_t, n: mp_limb_t) -> mp_limb_t;
    pub fn n_root_estimate(a: f64, n: c_int) -> mp_limb_t;
    pub fn n_rootrem(remainder: *mut mp_limb_t, n: mp_limb_t, root: mp_limb_t) -> mp_limb_t;
    pub fn n_root(n: mp_limb_t, root: mp_limb_t) -> mp_limb_t;
    pub fn n_factor_ecm_double(
        x: *mut mp_limb_t,
        z: *mut mp_limb_t,
        x0: mp_limb_t,
        z0: mp_limb_t,
        n: mp_limb_t,
        n_ecm_inf: *mut n_ecm_s,
    );
    pub fn n_factor_ecm_add(
        x: *mut mp_limb_t,
        z: *mut mp_limb_t,
        x1: mp_limb_t,
        z1: mp_limb_t,
        x2: mp_limb_t,
        z2: mp_limb_t,
        x0: mp_limb_t,
        z0: mp_limb_t,
        n: mp_limb_t,
        n_ecm_inf: *mut n_ecm_s,
    );
    pub fn n_factor_ecm_mul_montgomery_ladder(
        x: *mut mp_limb_t,
        z: *mut mp_limb_t,
        x0: mp_limb_t,
        z0: mp_limb_t,
        k: mp_limb_t,
        n: mp_limb_t,
        n_ecm_inf: *mut n_ecm_s,
    );
    pub fn n_factor_ecm_select_curve(
        f: *mut mp_limb_t,
        sig: mp_limb_t,
        n: mp_limb_t,
        n_ecm_inf: *mut n_ecm_s,
    ) -> c_int;
    pub fn n_factor_ecm_stage_I(
        f: *mut mp_limb_t,
        prime_array: *const mp_limb_t,
        num: mp_limb_t,
        B1: mp_limb_t,
        n: mp_limb_t,
        n_ecm_inf: *mut n_ecm_s,
    ) -> c_int;
    pub fn n_factor_ecm_stage_II(
        f: *mut mp_limb_t,
        B1: mp_limb_t,
        B2: mp_limb_t,
        P: mp_limb_t,
        n: mp_limb_t,
        n_ecm_inf: *mut n_ecm_s,
    ) -> c_int;
    pub fn n_factor_ecm(
        f: *mut mp_limb_t,
        curves: mp_limb_t,
        B1: mp_limb_t,
        B2: mp_limb_t,
        state: *mut flint_rand_s,
        n: mp_limb_t,
    ) -> c_int;
    pub fn n_mulmod_precomp_shoup(w: mp_limb_t, p: mp_limb_t) -> mp_limb_t;
    pub fn n_mulmod_shoup(
        w: mp_limb_t,
        t: mp_limb_t,
        w_precomp: mp_limb_t,
        p: mp_limb_t,
    ) -> mp_limb_t;
}
