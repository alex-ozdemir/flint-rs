#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::arb::arb_t;
use crate::deps::*;
use crate::fmpq::fmpq;
use crate::fmpz::{fmpz, fmpz_t};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bernoulli_rev_struct {
    pub alloc: mp_limb_signed_t,
    pub prec: mp_limb_signed_t,
    pub max_power: mp_limb_signed_t,
    pub powers: *mut fmpz,
    pub pow_error: fmpz_t,
    pub prefactor: arb_t,
    pub two_pi_squared: arb_t,
    pub n: mp_limb_t,
}

pub type bernoulli_rev_t = [bernoulli_rev_struct; 1usize];

extern "C" {
    pub static mut bernoulli_cache_num: mp_limb_signed_t;
    pub static mut bernoulli_cache: *mut fmpq;
    pub fn bernoulli_cache_compute(n: mp_limb_signed_t);
    pub fn bernoulli_rev_init(iter: *mut bernoulli_rev_struct, nmax: mp_limb_t);
    pub fn bernoulli_rev_next(numer: *mut fmpz, denom: *mut fmpz, iter: *mut bernoulli_rev_struct);
    pub fn bernoulli_rev_clear(iter: *mut bernoulli_rev_struct);
    pub fn bernoulli_bound_2exp_si(n: mp_limb_t) -> mp_limb_signed_t;
    pub fn _bernoulli_fmpq_ui_zeta(num: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn _bernoulli_fmpq_ui(num: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn bernoulli_fmpq_ui(b: *mut fmpq, n: mp_limb_t);
}
