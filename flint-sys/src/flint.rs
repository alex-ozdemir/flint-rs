#![allow(non_camel_case_types)]

//! FLINT definitions used in multiple crates.

use crate::deps::*;
use libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flint_rand_s {
    pub gmp_state: gmp_randstate_t,
    pub gmp_init: c_int,
    pub __randval: mp_limb_t,
    pub __randval2: mp_limb_t,
}

//pub type flint_rand_t = [flint_rand_s; 1usize];
pub type flint_rand_t = flint_rand_s;
pub type flint_bitcnt_t = mp_limb_t;

pub type flint_mpfr = __mpfr_struct;

pub type thread_pool_handle = c_int;
